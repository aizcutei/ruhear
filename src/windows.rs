use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Host,
};

pub type AudioBuffers = Vec<Vec<f32>>;

pub struct RUHear {
    pub callback: Arc<Mutex<dyn FnMut(AudioBuffers) + Send>>,
    host: Host,
    device: cpal::Device,
    format: cpal::Format,
    stream: Option<cpal::Stream>,
}

impl RUHear {
    pub fn new(callback: Arc<Mutex<Box<dyn FnMut(AudioBuffers)>>>) -> Self {
        let host = cpal::default_host();
        let device = host.default_input_device().unwrap();
        let format = device.default_input_format().unwrap();
        Self {
            callback,
            host,
            device,
            format,
            stream: None,
        }
    }

    pub fn start(&mut self) {
        if self.stream.is_none() {
            let callback = self.callback.clone();
            let stream = match self.format.sample_format() {
                cpal::SampleFormat::F32 => self.device.build_input_stream(
                    &self.format.config(),
                    move |data: &[f32], &_| {
                        let audio_buffers: Vec<Vec<f32>> = data
                            .chunks_exact(self.format.channels as usize)
                            .enumerate()
                            .fold(
                                vec![vec![]; self.format.channels as usize],
                                |mut acc, (i, x)| {
                                    acc[i] = x.to_vec();
                                    acc
                                },
                            );
                        if let Ok(mut callback) = callback.lock() {
                            (*callback)(audio_buffers);
                        }
                    },
                    |e| eprintln!("{}", e),
                    None,
                ),
                cpal::SampleFormat::I32 => self.device.build_input_stream(
                    &self.format.config(),
                    move |data: &[i32], &_| {
                        if let Ok(mut callback) = callback.lock() {
                            (*callback)(data.iter().map(|&x| x as f32).collect());
                        }
                    },
                    |e| eprintln!("{}", e),
                    None,
                ),
                sample_format => {
                    panic!("unsupported format {:?}", sample_format);
                }
            }
            .unwrap();
            self.stream = Some(stream);
        }
        self.stream.as_ref().unwrap().play().unwrap();
    }

    pub fn stop(&self) {
        self.stream.as_ref().unwrap().pause().unwrap();
    }
}
