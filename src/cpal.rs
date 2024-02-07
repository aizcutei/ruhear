#![allow(unused)]
use anyhow;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};

pub type RUBuffers = Vec<Vec<f32>>;

pub struct RUHear {
    pub callback: Arc<Mutex<dyn FnMut(RUBuffers) + Send>>,
    host: cpal::Host,
    device: cpal::Device,
    format: cpal::SupportedStreamConfig,
    stream: Option<cpal::Stream>,
}

impl RUHear {
    pub fn new(callback: Arc<Mutex<dyn FnMut(RUBuffers) + Send>>) -> Self {
        let host = cpal::default_host();
        let device = host.default_output_device().unwrap();
        let format = device.default_output_config().unwrap();
        Self {
            callback,
            host,
            device,
            format,
            stream: None,
        }
    }

    pub fn start(&mut self) -> Result<(), anyhow::Error> {
        if self.stream.is_none() {
            let callback = self.callback.clone();
            let channels = &self.format.channels().clone();
            let channels = *channels as usize;
            let stream = match self.format.sample_format() {
                cpal::SampleFormat::I8 => self.device.build_input_stream(
                    &self.format.config(),
                    move |data: &[i8], &_| {
                        let mut bufs = vec![vec![]; channels];
                        for (i, sample) in data.chunks(channels).enumerate() {
                            for (j, &channel) in sample.iter().enumerate() {
                                bufs[j].push(channel as f32 / i8::MAX as f32);
                            }
                        }
                        if let Ok(mut callback) = callback.lock() {
                            (*callback)(bufs);
                        }
                    },
                    |e| eprintln!("{}", e),
                    None,
                ),
                cpal::SampleFormat::U8 => self.device.build_input_stream(
                    &self.format.config(),
                    move |data: &[u8], &_| {
                        let mut bufs = vec![vec![]; channels];
                        for (i, sample) in data.chunks(channels).enumerate() {
                            for (j, &channel) in sample.iter().enumerate() {
                                bufs[j].push((channel as f32 / u8::MAX as f32) * 2.0 - 1.0 as f32);
                            }
                        }
                        if let Ok(mut callback) = callback.lock() {
                            (*callback)(bufs);
                        }
                    },
                    |e| eprintln!("{}", e),
                    None,
                ),
                cpal::SampleFormat::I16 => self.device.build_input_stream(
                    &self.format.config(),
                    move |data: &[i16], &_| {
                        let mut bufs = vec![vec![]; channels];
                        for (i, sample) in data.chunks(channels).enumerate() {
                            for (j, &channel) in sample.iter().enumerate() {
                                bufs[j].push(channel as f32 / i16::MAX as f32);
                            }
                        }
                        if let Ok(mut callback) = callback.lock() {
                            (*callback)(bufs);
                        }
                    },
                    |e| eprintln!("{}", e),
                    None,
                ),
                cpal::SampleFormat::U16 => self.device.build_input_stream(
                    &self.format.config(),
                    move |data: &[u16], &_| {
                        let mut bufs = vec![vec![]; channels];
                        for (i, sample) in data.chunks(channels).enumerate() {
                            for (j, &channel) in sample.iter().enumerate() {
                                bufs[j].push((channel as f32 / u16::MAX as f32) * 2.0 - 1.0 as f32);
                            }
                        }
                        if let Ok(mut callback) = callback.lock() {
                            (*callback)(bufs);
                        }
                    },
                    |e| eprintln!("{}", e),
                    None,
                ),
                cpal::SampleFormat::I32 => self.device.build_input_stream(
                    &self.format.config(),
                    move |data: &[i32], &_| {
                        let mut bufs = vec![vec![]; channels];
                        for (i, sample) in data.chunks(channels).enumerate() {
                            for (j, &channel) in sample.iter().enumerate() {
                                bufs[j].push(channel as f32 / i32::MAX as f32);
                            }
                        }
                        if let Ok(mut callback) = callback.lock() {
                            (*callback)(bufs);
                        }
                    },
                    |e| eprintln!("{}", e),
                    None,
                ),
                cpal::SampleFormat::U32 => self.device.build_input_stream(
                    &self.format.config(),
                    move |data: &[u32], &_| {
                        let mut bufs = vec![vec![]; channels];
                        for (i, sample) in data.chunks(channels).enumerate() {
                            for (j, &channel) in sample.iter().enumerate() {
                                bufs[j].push((channel as f32 / u32::MAX as f32) * 2.0 - 1.0 as f32);
                            }
                        }
                        if let Ok(mut callback) = callback.lock() {
                            (*callback)(bufs);
                        }
                    },
                    |e| eprintln!("{}", e),
                    None,
                ),
                cpal::SampleFormat::I64 => self.device.build_input_stream(
                    &self.format.config(),
                    move |data: &[i64], &_| {
                        let mut bufs = vec![vec![]; channels];
                        for (i, sample) in data.chunks(channels).enumerate() {
                            for (j, &channel) in sample.iter().enumerate() {
                                bufs[j].push(channel as f32 / i64::MAX as f32);
                            }
                        }
                        if let Ok(mut callback) = callback.lock() {
                            (*callback)(bufs);
                        }
                    },
                    |e| eprintln!("{}", e),
                    None,
                ),
                cpal::SampleFormat::U64 => self.device.build_input_stream(
                    &self.format.config(),
                    move |data: &[u64], &_| {
                        let mut bufs = vec![vec![]; channels];
                        for (i, sample) in data.chunks(channels).enumerate() {
                            for (j, &channel) in sample.iter().enumerate() {
                                bufs[j].push((channel as f32 / u64::MAX as f32) * 2.0 - 1.0 as f32);
                            }
                        }
                        if let Ok(mut callback) = callback.lock() {
                            (*callback)(bufs);
                        }
                    },
                    |e| eprintln!("{}", e),
                    None,
                ),
                cpal::SampleFormat::F32 => self.device.build_input_stream(
                    &self.format.config(),
                    move |data: &[f32], &_| {
                        let mut bufs = vec![vec![]; channels];
                        for (i, sample) in data.chunks(channels).enumerate() {
                            for (j, &channel) in sample.iter().enumerate() {
                                bufs[j].push(channel as f32);
                            }
                        }
                        if let Ok(mut callback) = callback.lock() {
                            (*callback)(bufs);
                        }
                    },
                    |e| eprintln!("{}", e),
                    None,
                ),
                cpal::SampleFormat::F64 => self.device.build_input_stream(
                    &self.format.config(),
                    move |data: &[f64], &_| {
                        let mut bufs = vec![vec![]; channels];
                        for (i, sample) in data.chunks(channels).enumerate() {
                            for (j, &channel) in sample.iter().enumerate() {
                                bufs[j].push(channel as f32);
                            }
                        }
                        if let Ok(mut callback) = callback.lock() {
                            (*callback)(bufs);
                        }
                    },
                    |e| eprintln!("{}", e),
                    None,
                ),
                sample_format => {
                    panic!("unsupported format {:?}", sample_format);
                }
            }?;
            self.stream = Some(stream);
        }
        if let Some(stream) = &self.stream {
            stream.play();
        }
        Ok(())
    }

    pub fn stop(&self) -> Result<(), anyhow::Error> {
        if let Some(stream) = &self.stream {
            stream.pause()?;
        }
        Ok(())
    }
}
