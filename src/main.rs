use clap::{Parser, ValueEnum};
use rodio::{OutputStreamBuilder, Sink};

mod metronome;
mod utils;

use metronome::Metronome;

#[derive(Debug, Clone, Copy, ValueEnum)]
enum ClickSound {
    C,
    B,
    A,
}
impl ToString for ClickSound {
    fn to_string(&self) -> String {
        match self {
            ClickSound::C => "c".to_string(),
            ClickSound::B => "b".to_string(),
            ClickSound::A => "a".to_string(),
        }
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    tempo: u64,

    #[arg(short, long, default_value_t = 4)]
    division: u8,

    #[arg(long, default_value_t = ClickSound::C)]
    sound: ClickSound,

    #[arg(short, long, default_value_t = 50)]
    swing: i16,
}

fn main() {
    let args = Args::parse();

    let stream_handle =
        OutputStreamBuilder::open_default_stream().expect("open default audio stream");
    let sink = Sink::connect_new(&stream_handle.mixer());

    let mut base_path = match dirs::document_dir() {
        Some(v) => {
            let mut binding = v.clone();
            binding.as_mut_os_string().clone()
        }
        None => panic!("no home dir found"),
    };

    let click = match args.sound {
        ClickSound::A => {
            base_path.push("/rust-click/a.wav");
            base_path.into_string().unwrap()
        }
        ClickSound::B => {
            base_path.push("/rust-click/b.wav");
            base_path.into_string().unwrap()
        }
        ClickSound::C => {
            base_path.push("/rust-click/c.wav");
            base_path.into_string().unwrap()
        }
    };

    let sample_rate = stream_handle.config().sample_rate();
    let metronome = Metronome::new(&click, args.tempo, sample_rate, args.division, args.swing);

    sink.append(metronome);
    sink.sleep_until_end();
}
