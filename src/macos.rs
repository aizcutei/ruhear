use screencapturekit::{
    cm_sample_buffer::CMSampleBuffer,
    sc_content_filter::{InitParams, SCContentFilter},
    sc_display::SCDisplay,
    sc_error_handler::StreamErrorHandler,
    sc_output_handler::SCStreamOutputType,
    sc_output_handler::StreamOutput,
    sc_shareable_content::SCShareableContent,
    sc_stream::SCStream,
    sc_stream_configuration::SCStreamConfiguration,
};
use std::sync::{Arc, Mutex};

pub type AudioBuffers = Vec<Vec<f32>>;
struct ErrorHandler;

impl StreamErrorHandler for ErrorHandler {
    fn on_error(&self) {
        panic!("Stream Error!")
    }
}

struct OutputHandler {
    callback: Arc<Mutex<dyn FnMut(AudioBuffers) + Send>>,
}

impl StreamOutput for OutputHandler {
    fn did_output_sample_buffer(
        &self,
        sample_buffer: CMSampleBuffer,
        _of_type: SCStreamOutputType,
    ) {
        let audio = sample_buffer.sys_ref.get_av_audio_buffer_list();
        let mut audio_buffers = Vec::new();
        for channel in audio {
            let data = channel.data;
            let mut f32_data = Vec::new();
            for i in 0..data.len() / 4 {
                let mut f32_bytes = [0u8; 4];
                f32_bytes.copy_from_slice(&data[i * 4..i * 4 + 4]);
                let f32 = f32::from_le_bytes(f32_bytes);
                f32_data.push(f32);
            }
            audio_buffers.push(f32_data);
            // call the callback
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
    pub callback: Arc<Mutex<dyn FnMut(AudioBuffers) + Send>>,
    device_list: Vec<RUDevice>,
    device: Option<RUDevice>,
    stream: Option<SCStream>,
}

impl RUHear {
    pub fn new(callback: Arc<Mutex<dyn FnMut(AudioBuffers) + Send>>) -> Self {
        let content = SCShareableContent::current();
        let displays = content.displays;
        let display = displays.clone().first().unwrap().to_owned();
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

    // pub fn set_callback(&mut self, callback: Arc<Mutex<dyn FnMut(AudioBuffers) + Send>>) {
    //     self.callback = callback;
    // }

    pub fn start(&mut self) {
        let params = match self.device.clone().unwrap() {
            RUDevice::MacosDisplay(display) => InitParams::Display(display),
        };
        let filter = SCContentFilter::new(params);
        let config = SCStreamConfiguration {
            width: 2,
            height: 2,
            captures_audio: true,
            excludes_current_process_audio: true,
            ..Default::default()
        };
        let mut stream = SCStream::new(filter, config, ErrorHandler {});
        let output_handler = OutputHandler {
            callback: self.callback.clone(),
        };
        stream.add_output(output_handler, SCStreamOutputType::Audio);
        let _ = stream.start_capture();
        self.stream = Some(stream);
    }

    pub fn stop(&mut self) {
        if let Some(stream) = self.stream.take() {
            let _ = stream.stop_capture();
        }
    }
}
