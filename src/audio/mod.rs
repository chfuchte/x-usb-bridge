mod audio;

#[cfg(target_os = "linux")]
mod linux;

pub(crate) use audio::*;
