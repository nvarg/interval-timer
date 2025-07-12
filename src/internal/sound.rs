use rodio::{OutputStream, Sink, Source, buffer::SamplesBuffer};
use std::thread;

pub struct SoundFile {
    buffer: Vec<i16>,
    sample_rate: u32,
    ready: bool,
}

impl SoundFile {
    pub fn new() -> Self {
        Self {
            sample_rate: SAMPLE_RATE,
            buffer: vec![],
            ready: false,
        }
    }

    pub fn load_file(&mut self, file: &str) -> Result<(), String> {
        self.ready = false;
        let file = std::fs::File::open(file).map_err(|v| v.to_string())?;

        let reader = std::io::BufReader::new(file);
        let decoder = rodio::Decoder::new(reader).map_err(|v| v.to_string())?;

        self.sample_rate = decoder.sample_rate();
        self.buffer = decoder.buffered().collect();
        self.ready = true;
        Ok(())
    }

    pub fn play(&self, volume: f32) {
        let samples = SamplesBuffer::new(1, self.sample_rate, &self.buffer[..]);
        thread::spawn(move || {
            play_buffer(samples, volume);
        });
    }

    pub fn is_ready(&self) -> bool {
        self.ready
    }
}

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

pub fn play_sound(volume: f32) {
    thread::spawn(move || {
        let source = SamplesBuffer::new(1, SAMPLE_RATE, &SAMPLES[..]);
        play_buffer(source, volume);
    });
}

fn play_buffer(buffer: SamplesBuffer<i16>, volume: f32) {
    let (_stream, handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&handle).unwrap();

    sink.append(buffer);
    sink.set_volume(volume);
    sink.play();
    sink.sleep_until_end();
}
