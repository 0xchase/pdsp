use std::{io::BufReader, fs::File};

use crate::{Float, Loadable};

fn temp() {
    let wavetable = Wavetable::<f32, 2048>::new();
}

pub struct Wavetable<F: Float, const C: usize> {
    pub table: Vec<[F; C]>
}

impl<F: Float, const C: usize> Wavetable<F, C> {
    pub fn new() -> Self {
        Wavetable {
            table: vec![[F::ZERO; C]; 16]
        }
    }

    pub const fn len(&self) -> usize {
        C
    }
}

impl<F: Float, const C: usize> From<Vec<[F; C]>> for Wavetable<F, C> {
    fn from(table: Vec<[F; C]>) -> Self {
        Wavetable {
            table
        }
    }
}

impl<F: Float, const C: usize> Loadable for Wavetable<F, C> {
    fn load(path: &str) -> Result<Self, String> where Self: Sized {
        match hound::WavReader::open(path.to_string()) {
            Ok(reader) => {
                return load_wavetable(path, reader)
            }
            Err(e) => {
                Err(e.to_string())
            }
        }
    }
}

fn load_wavetable<F: Float, const C: usize>(path: &str, mut reader: hound::WavReader<BufReader<File>>) -> Result<Wavetable<F, C>, String> {
    let mut wavetable = Wavetable::<F, C>::new();
    let length = wavetable.len();

    let spec = reader.spec();
    let channels = spec.channels as usize;

    if channels != 1 {
        panic!("Unsupported wavetable channel count");
    }

    println!("{:?}", spec);


    if spec.sample_format == hound::SampleFormat::Float {
        if spec.bits_per_sample == 16 || spec.bits_per_sample == 32 {
            let size = reader.samples::<f32>().len();
            let mut i = 0;

            println!("Length: {}", size);

            reader.samples::<f32>().for_each(| v | {
                let v = f32::from(v.unwrap());
                wavetable.table[i / length][length * i / size] = F::from(v);
                i += 1;
            });

            return Ok(wavetable);
        }

        return Err(String::from("Unsupported wavetable bit depth"));

    } else if spec.sample_format == hound::SampleFormat::Int {
        if spec.bits_per_sample == 16 {
            let size = reader.samples::<i16>().len();
            let mut i = 0;

            reader.samples::<i16>().for_each(| v | {
                let v = f32::from(v.unwrap());
                wavetable.table[i / length][length * i / size] = F::from(v);
                i += 1;
            });

            return Ok(wavetable);
        } else if spec.bits_per_sample == 24 || spec.bits_per_sample == 32 {
            let size = reader.samples::<i32>().len();
            let mut i = 0;

            reader.samples::<i32>().for_each(| v | {
                let v = v.unwrap() as i32;
                wavetable.table[i / length][length * i / size] = F::from(v as f32);
                i += 1;
            });

            return Ok(wavetable);
        }

        return Err(String::from("Unsupported wavetable bit depth"));
    }

    return Err(String::from("Unsupported sample fomat"));
}
