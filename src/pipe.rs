use std::{
    io::{self, Write},
    time::Duration,
};

use anyhow::Result;
use hound::{SampleFormat, WavSpec};
use tracing::{debug, info};

use crate::audio::Audio;

pub(crate) fn run_pipe(timeout: Option<Duration>) -> Result<()> {
    let mut audio = Audio::new()?;
    let format = audio.format();

    let spec = WavSpec {
        channels: format.channels() as u16,
        sample_rate: format.sample_rate(),
        bits_per_sample: format.bits_per_sample() as u16,
        sample_format: SampleFormat::Int,
    };

    let mut stdout = io::stdout().lock();

    let header = spec.into_header_for_infinite_file();
    stdout.write_all(&header)?;

    info!("Audio device initialized.");
    debug!("Format: {:?}", format);

    audio.start()?;

    loop {
        let buffer = audio.read()?;

        let mut bytes = Vec::with_capacity(buffer.samples().len() * format.bytes_per_sample());

        for sample in buffer.samples() {
            bytes.extend_from_slice(&sample.to_le_bytes());
        }

        stdout.write_all(&bytes)?;

        debug!(
            "position={} frames={}",
            buffer.position().frames(),
            buffer.frames()
        );

        if timeout.is_some_and(|timeout| buffer.position().duration(format) >= timeout) {
            break;
        }
    }

    stdout.flush()?;
    audio.stop()?;

    Ok(())
}
