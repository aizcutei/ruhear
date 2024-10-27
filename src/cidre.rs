use anyhow::Result;
use cidre::{
    arc::Retained,
    cm, define_obj_type, dispatch, objc,
    sc::{
        self,
        stream::{Output, OutputImpl},
    },
};
use futures::executor::block_on;
use std::sync::{Arc, Mutex};

type RUBuffers = Vec<Vec<f32>>;

struct OutputHandlerInner {
    callback: Arc<Mutex<dyn FnMut(RUBuffers) + Send>>,
}

impl OutputHandlerInner {
    fn process(&mut self, sample_buf: &mut cm::SampleBuf) {
        let audio_buf_list = sample_buf.audio_buf_list::<2>().unwrap();
        let list = audio_buf_list.list();
        let buffers = list.buffers;
        let size = buffers[0].data_bytes_size as usize;
        let mut audio_buffers = Vec::new();
        for buf in buffers {
            let mut data_vec = vec![];
            unsafe {
                let data_slice = std::slice::from_raw_parts_mut(buf.data, size);
                data_vec = data_slice.to_vec();
            }
            let mut f32_data = Vec::new();
            for i in 0..data_vec.len() / 4 {
                let mut f32_bytes = [0u8; 4];
                f32_bytes.copy_from_slice(&data_vec[i * 4..i * 4 + 4]);
                let f32_val = f32::from_le_bytes(f32_bytes);
                f32_data.push(f32_val);
            }
            audio_buffers.push(f32_data);
        }
        let mut callback = self.callback.lock().unwrap();
        callback(audio_buffers);
    }
}

define_obj_type!(
    OutputHandler + OutputImpl,
    OutputHandlerInner,
    OUTPUT_HANDLER
);

impl Output for OutputHandler {}

#[objc::add_methods]
impl OutputImpl for OutputHandler {
    extern "C" fn impl_stream_did_output_sample_buf(
        &mut self,
        _cmd: Option<&cidre::objc::Sel>,
        _stream: &sc::Stream,
        sample_buf: &mut cm::SampleBuf,
        kind: sc::OutputType,
    ) {
        println!("impl_stream_did_output_sample_buf");
        match kind {
            sc::OutputType::Screen => {}
            sc::OutputType::Audio => self.inner_mut().process(sample_buf),
            sc::OutputType::Mic => {}
        }
    }
}

pub struct RUHear {
    pub callback: Arc<Mutex<dyn FnMut(RUBuffers) + Send>>,
    stream: Option<Retained<cidre::sc::Stream>>,
}

impl RUHear {
    pub fn new(callback: Arc<Mutex<dyn FnMut(RUBuffers) + Send>>) -> Self {
        let content = block_on(sc::ShareableContent::current());
        if let Ok(content) = content {
            let displays = content.displays().clone();
            let display = displays.first().unwrap();
            let filter = sc::ContentFilter::with_display_excluding_windows(
                display,
                &cidre::ns::Array::new(),
            );
            let q = dispatch::Queue::serial_with_ar_pool();
            let mut cfg = sc::StreamCfg::new();
            cfg.set_width(2);
            cfg.set_height(2);
            unsafe {
                cfg.set_captures_audio(true);
                cfg.set_excludes_current_process_audio(false);
            }
            let handler = OutputHandler::with(OutputHandlerInner {
                callback: callback.clone(),
            });
            let stream = sc::Stream::new(&filter, &cfg);
            stream
                .add_stream_output(handler.as_ref(), sc::OutputType::Audio, Some(&q))
                .unwrap();
            Self {
                callback,
                stream: Some(stream),
            }
        } else {
            panic!("Unable to find displays!")
        }
    }

    pub fn start(&mut self) -> Result<(), anyhow::Error> {
        if let Some(stream) = self.stream.as_ref() {
            match block_on(stream.start()) {
                Ok(_) => {
                    // println!("Started");
                    Ok(())
                }
                Err(e) => Err(anyhow::anyhow!("Error starting stream: {:?}", e)),
            }
        } else {
            anyhow::bail!("Stream not found")
        }
    }

    pub fn stop(&mut self) -> Result<(), anyhow::Error> {
        if let Some(stream) = self.stream.take() {
            match block_on(stream.stop()) {
                Ok(_) => {
                    // println!("Stopped");
                    Ok(())
                }
                Err(e) => Err(anyhow::anyhow!("Error stopping stream: {:?}", e)),
            }
        } else {
            anyhow::bail!("Stream not found")
        }
    }
}
