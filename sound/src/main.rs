extern crate alsa;

use alsa::{Direction, ValueOr};
use alsa::pcm::{PCM, HwParams, Format, Access, State};

fn sound_init(pcm: &PCM, samplerate: u32, channelnum: u32) -> alsa::pcm::IO<i16> {
    // Set hardware parameters: 48000 Hz / Mono / 16 bit
    let hwp = HwParams::any(&pcm).unwrap();
    hwp.set_channels(channelnum).unwrap();
    hwp.set_rate(samplerate, ValueOr::Nearest).unwrap();
    hwp.set_format(Format::s16()).unwrap();
    hwp.set_access(Access::RWInterleaved).unwrap();
    pcm.hw_params(&hwp).unwrap();
    let io = pcm.io_i16().unwrap();

    // Make sure we don't start the stream too early
    let hwp = pcm.hw_params_current().unwrap();
    let swp = pcm.sw_params_current().unwrap();
    swp.set_start_threshold(hwp.get_buffer_size().unwrap() - hwp.get_period_size().unwrap()).unwrap();
    pcm.sw_params(&swp).unwrap();
    return io;
}

fn main() {
    // Open default playback device
    let pcm = PCM::new("default", Direction::Playback, false).unwrap();
    let io = sound_init(&pcm, 48000, 1);

    // Make a sine wave
    let mut buf = [0i16; 1024];
    for (i, a) in buf.iter_mut().enumerate() {
        *a = ((i as f32 * 2.0 * ::std::f32::consts::PI / 128.0).sin() * 8192.0) as i16
    }

    // Play it back for 2 seconds.
    for _ in 0..2*48000/1024 {
        assert_eq!(io.writei(&buf[..]).unwrap(), 1024);
    }

    if pcm.state() != State::Running { pcm.start().unwrap() }; // In case the buffer was larger than 2 seconds, start the stream manually.
    pcm.drain().unwrap(); // Wait for the stream to finish playback.
}
