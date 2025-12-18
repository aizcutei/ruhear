#![allow(dead_code)]
use anyhow::Result;
use screencapturekit::prelude::*;
use std::sync::{Arc, Mutex};

use crate::RUBuffers;
struct ErrorHandler;

impl SCStreamDelegateTrait for ErrorHandler {
    fn did_stop_with_error(&self, error: SCError) {
        println!("Stream Error! {error}");
    }
}

struct OutputHandler {
    callback: Arc<Mutex<dyn FnMut(RUBuffers) + Send>>,
}

fn bytes_to_f32_samples(data: &[u8]) -> Vec<f32> {
    if data.is_empty() {
        return Vec::new();
    }

    if data.len() % 4 == 0 {
        let mut out = Vec::with_capacity(data.len() / 4);
        for chunk in data.chunks_exact(4) {
            out.push(f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]));
        }
        return out;
    }

    if data.len() % 2 == 0 {
        let mut out = Vec::with_capacity(data.len() / 2);
        for chunk in data.chunks_exact(2) {
            let sample = i16::from_le_bytes([chunk[0], chunk[1]]);
            out.push(f32::from(sample) / 32768.0);
        }
        return out;
    }

    Vec::new()
}

impl SCStreamOutputTrait for OutputHandler {
    fn did_output_sample_buffer(
        &self,
        sample_buffer: CMSampleBuffer,
        _of_type: SCStreamOutputType,
    ) {
        let Some(audio) = sample_buffer.audio_buffer_list() else {
            return;
        };

        let mut audio_buffers: RUBuffers = Vec::new();

        if audio.num_buffers() == 1 {
            let buffer = audio.get(0).unwrap();
            let channels = buffer.number_channels as usize;
            let samples = bytes_to_f32_samples(buffer.data());

            if channels <= 1 {
                audio_buffers.push(samples);
            } else {
                let frames = samples.len() / channels;
                audio_buffers = (0..channels).map(|_| Vec::with_capacity(frames)).collect();
                for frame_idx in 0..frames {
                    for ch in 0..channels {
                        audio_buffers[ch].push(samples[frame_idx * channels + ch]);
                    }
                }
            }
        } else {
            for buffer in audio.iter() {
                audio_buffers.push(bytes_to_f32_samples(buffer.data()));
            }
        }

        let mut callback = self.callback.lock().unwrap();
        callback(audio_buffers);
    }
}

#[derive(Debug, Clone)]
enum RUDevice {
    MacosDisplay(SCDisplay),
    // MacosApplication(SCRunningApplication),
    // MacosWindow(SCWindow),
}

pub struct RUHear {
    pub callback: Arc<Mutex<dyn FnMut(RUBuffers) + Send>>,
    device_list: Vec<RUDevice>,
    device: Option<RUDevice>,
    stream: Option<SCStream>,
}

impl RUHear {
    pub fn new(callback: Arc<Mutex<dyn FnMut(RUBuffers) + Send>>) -> Self {
        let content = SCShareableContent::get().expect("Failed to get shareable content");
        let displays = content.displays();
        let display = displays.first().cloned().expect("No display found");
        Self {
            callback: callback.clone(),
            device_list: displays
                .iter()
                .map(|display| RUDevice::MacosDisplay(display.clone()))
                .collect(),
            device: Some(RUDevice::MacosDisplay(display.clone())),
            stream: None,
        }
    }

    pub fn start(&mut self) -> Result<(), anyhow::Error> {
        let filter = match self.device.clone().unwrap() {
            RUDevice::MacosDisplay(display) => {
                let excluded_windows: [&SCWindow; 0] = [];
                SCContentFilter::builder()
                    .display(&display)
                    .exclude_windows(&excluded_windows)
                    .build()
            }
        };

        let config = SCStreamConfiguration::new()
            .with_width(2)
            .with_height(2)
            .with_captures_audio(true)
            .with_excludes_current_process_audio(true);

        let mut stream = SCStream::new_with_delegate(&filter, &config, ErrorHandler {});
        let output_handler = OutputHandler {
            callback: self.callback.clone(),
        };
        stream.add_output_handler(output_handler, SCStreamOutputType::Audio);

        stream.start_capture().map_err(|e| anyhow::anyhow!(e))?;
        self.stream = Some(stream);
        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), anyhow::Error> {
        if let Some(stream) = self.stream.take() {
            stream.stop_capture().map_err(|e| anyhow::anyhow!(e))
        } else {
            anyhow::bail!("Stream not found")
        }
    }
}
