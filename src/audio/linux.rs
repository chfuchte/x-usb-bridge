use alsa::{
    Direction, ValueOr,
    pcm::{Access, Format, HwParams, PCM},
};

use crate::audio::{AudioBuffer, AudioFormat, AudioPosition};

const DEVICE: &str = "hw:3,0";
const FRAMES_PER_READ: usize = 1024;

pub(crate) struct LinuxAudio {
    pcm: PCM,
    format: AudioFormat,
    position: AudioPosition,
}

impl LinuxAudio {
    pub(crate) fn new(format: AudioFormat) -> anyhow::Result<Self> {
        let pcm = PCM::new(DEVICE, Direction::Capture, false)?;

        {
            let params = HwParams::any(&pcm)?;
            params.set_access(Access::RWInterleaved)?;
            params.set_format(Format::s32())?;
            params.set_channels(format.channels as u32)?;
            params.set_rate(format.sample_rate, ValueOr::Nearest)?;
            pcm.hw_params(&params)?;
        }

        pcm.prepare()?;

        Ok(Self {
            pcm,
            format,
            position: AudioPosition::new(0),
        })
    }

    pub(crate) fn start(&mut self) -> anyhow::Result<()> {
        self.pcm.prepare()?;

        Ok(())
    }

    pub(crate) fn stop(&mut self) -> anyhow::Result<()> {
        self.pcm.drop()?;

        Ok(())
    }

    pub(crate) fn read(&mut self) -> anyhow::Result<AudioBuffer> {
        let io = self.pcm.io_i32()?;

        let mut samples = vec![0i32; FRAMES_PER_READ * self.format.channels];

        let frames = loop {
            match io.readi(&mut samples) {
                Ok(frames) => break frames,
                Err(error) => {
                    self.pcm.recover(error.errno(), true)?;
                }
            }
        };

        samples.truncate(frames * self.format.channels);

        let position = self.position;
        self.position = AudioPosition::new(self.position.frames + frames as u64);

        Ok(AudioBuffer::new(self.format, position, samples))
    }
}
