
use rodio::{OutputStream, OutputStreamHandle, Sink};

pub fn build_sound_controller() -> Box<dyn SoundController> {
    match DefaultDeviceSoundController::new() {
        Ok(controller) => Box::new(controller),
        Err(_) => Box::new(NoOpSoundController {}),
    }
}

pub trait SoundController {
    fn play(&self);
    fn stop(&self);
}

pub struct DefaultDeviceSoundController {
    stream_handle: OutputStreamHandle,
    stream: OutputStream,
    sink: Sink,
}

impl DefaultDeviceSoundController {
    pub fn new() -> Result<Self, String> {
        let res = rodio::OutputStream::try_default();
        let (stream, stream_handle) = match res {
            Ok(value) => value,
            Err(_) => {
                eprintln!("Unable to initialize sound device. No sound will be available.");
                return Err(String::from("Unable to initialize audio."));
            }
        };

        let sink = match rodio::Sink::try_new(&stream_handle) {
            Ok(value) => value,
            Err(_) => {
                eprintln!("Unable to initialize sound device. No sound will be available.");
                return Err(String::from("Unable to initialize audio."));
            }
        };

        let source = rodio::source::SineWave::new(800);
        sink.append(source);
        Ok(DefaultDeviceSoundController {
            stream: stream,
            stream_handle: stream_handle,
            sink: sink,
        })
    }
}

impl SoundController for DefaultDeviceSoundController {
    fn play(&self) {
        self.sink.play();
    }

    fn stop(&self) {
        self.sink.pause();
    }
}

pub struct NoOpSoundController {}

impl SoundController for NoOpSoundController {
    fn play(&self) {}

    fn stop(&self) {}
}
