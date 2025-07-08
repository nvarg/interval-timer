use rodio::{OutputStream, Sink, buffer::SamplesBuffer};
use std::thread;

const SAMPLE_RATE: u32 = 44100;
const FREQ: f32 = 523.25; // C5
const DURATION_MS: u32 = 250;
const NUM_SAMPLES: usize = (SAMPLE_RATE as u32 * DURATION_MS / 1000) as usize;

static SAMPLES: [i16; NUM_SAMPLES] = {
    const fn generate_samples() -> [i16; NUM_SAMPLES] {
        let mut buf = [0i16; NUM_SAMPLES];
        let mut n = 0;
        while n < NUM_SAMPLES {
            let t = n as f32 / SAMPLE_RATE as f32;
            let phase = (t * FREQ) % 1.0;
            let triangle = if phase < 0.5 {
                4.0 * phase - 1.0
            } else {
                3.0 - 4.0 * phase
            };
            let fade = 1.0 - (n as f32 / NUM_SAMPLES as f32);
            let sample = triangle * fade * 0.8;
            buf[n] = (sample * i16::MAX as f32) as i16;
            n += 1;
        }
        buf
    }

    generate_samples()
};

pub fn play_sound(vol: f32) {
    thread::spawn(move || {
        let (_stream, handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&handle).unwrap();

        let source = SamplesBuffer::new(1, SAMPLE_RATE, &SAMPLES[..]);

        sink.append(source);
        sink.set_volume(vol);
        sink.play();
        sink.sleep_until_end();
    });
}
