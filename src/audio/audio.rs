use std::time::Duration;

pub(crate) const DEFAULT_SAMPLE_RATE: u32 = 44_100;
pub(crate) const DEFAULT_CHANNELS: usize = 32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SampleFormat {
    S32Le,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct AudioFormat {
    sample_rate: u32,
    channels: usize,
    sample_format: SampleFormat,
}

impl AudioFormat {
    pub(crate) const fn x32() -> Self {
        Self {
            sample_rate: DEFAULT_SAMPLE_RATE,
            channels: DEFAULT_CHANNELS,
            sample_format: SampleFormat::S32Le,
        }
    }

    pub(crate) const fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    pub(crate) const fn channels(&self) -> usize {
        self.channels
    }

    pub(crate) const fn bytes_per_sample(&self) -> usize {
        match self.sample_format {
            SampleFormat::S32Le => 4,
        }
    }

    pub(crate) const fn bits_per_sample(&self) -> usize {
        match self.sample_format {
            SampleFormat::S32Le => self.bytes_per_sample() * 8,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct AudioPosition {
    frames: u64,
}

impl AudioPosition {
    pub(crate) const fn new(frames: u64) -> Self {
        Self { frames }
    }

    pub(crate) const fn frames(self) -> u64 {
        self.frames
    }

    pub(crate) fn duration(self, format: AudioFormat) -> Duration {
        Duration::from_secs_f64(self.frames as f64 / format.sample_rate as f64)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AudioState {
    Ready,
    Running,
    Stopped,
}

#[derive(Debug)]
pub(crate) struct AudioBuffer {
    format: AudioFormat,
    position: AudioPosition,
    samples: Vec<i32>,
}

impl AudioBuffer {
    pub(crate) fn new(format: AudioFormat, position: AudioPosition, samples: Vec<i32>) -> Self {
        Self {
            format,
            position,
            samples,
        }
    }

    pub(crate) fn position(&self) -> AudioPosition {
        self.position
    }

    pub(crate) fn samples(&self) -> &[i32] {
        &self.samples
    }

    pub(crate) fn frames(&self) -> usize {
        self.samples.len() / self.format.channels
    }
}

pub(crate) struct Audio {
    format: AudioFormat,
    state: AudioState,

    #[cfg(target_os = "linux")]
    backend: crate::audio::linux::LinuxAudio,
}

impl Audio {
    pub(crate) fn new() -> anyhow::Result<Self> {
        let format = AudioFormat::x32();

        Ok(Self {
            format,
            state: AudioState::Ready,

            #[cfg(target_os = "linux")]
            backend: crate::audio::linux::LinuxAudio::new(format)?,
        })
    }

    pub(crate) fn format(&self) -> AudioFormat {
        self.format
    }

    pub(crate) fn start(&mut self) -> anyhow::Result<()> {
        #[cfg(target_os = "linux")]
        self.backend.start()?;

        self.state = AudioState::Running;

        Ok(())
    }

    pub(crate) fn stop(&mut self) -> anyhow::Result<()> {
        #[cfg(target_os = "linux")]
        self.backend.stop()?;

        self.state = AudioState::Stopped;

        Ok(())
    }

    pub(crate) fn read(&mut self) -> anyhow::Result<AudioBuffer> {
        #[cfg(target_os = "linux")]
        {
            return Ok(self.backend.read()?);
        }

        #[cfg(not(target_os = "linux"))]
        {
            unreachable!("non-Linux builds are disabled");
        }
    }
}
