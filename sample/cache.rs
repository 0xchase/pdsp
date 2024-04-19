use std::sync::RwLock;

use crate::loadable::Loadable;
use crate::sample::sample::*;
use crate::buffers::*;
use crate::event::*;
use crate::float::*;
use crate::float::sample::*;

use hound::{Sample, WavReader};
use std::io::BufReader;
use std::fs::File;

use lazy_static::*;

lazy_static! {
    static ref SAMPLE_CACHE_STEREO: RwLock<Vec<SampleFile<Stereo<f32>>>> = RwLock::new(Vec::new());
}

use std::sync::Arc;

impl Loadable for SampleFile<Stereo<f32>> {
    fn load(path: &str) -> Result<Self, String> {
        /* Load sample from cache */
        for sample in &*SAMPLE_CACHE_STEREO.read().unwrap() {
            if sample.path() == path {
                return Ok(sample.clone());
            }
        }

        /* Load sample from files asynchronously */
        println!("Loading sample {}", path);

        match hound::WavReader::open(path.to_string()) {
            Ok(reader) => {
                let spec = reader.spec();

                if spec.bits_per_sample == 16 {
                    Ok(load_sample_file_i16(path, reader, spec.channels))
                } else if spec.bits_per_sample == 24 {
                    Ok(load_sample_file_i24(path, reader, spec.channels))
                } else {
                    Err("Unsupported WAV bit depth".to_string())
                }
            }
            Err(e) => {
                Err(e.to_string())
            }
        }
    }

    /*fn path(&self) -> String {
        self.path.clone()
    }*/
}

/*pub trait FileLoad<T> {
    fn load(_path: &str) -> T {
        panic!("File loader note implemented");
    }
}

impl FileLoad<SampleFile<Stereo>> for SampleFile<Stereo> {
    fn load(path: &str) -> SampleFile<Stereo> {
        /* Load sample from cache */
        for sample in &*SAMPLE_CACHE_STEREO.read().unwrap() {
            if sample.path() == path {
                return sample.clone();
            }
        }

        /* Load sample from files asynchronously */
        println!("Loading {}", path);

        let reader = hound::WavReader::open(path.to_string()).unwrap();
        let spec = reader.spec();

        if spec.bits_per_sample == 16 {
            load_sample_file_i16(path, reader, spec.channels)
        } else if spec.bits_per_sample == 24 {
            load_sample_file_i24(path, reader, spec.channels)
        } else {
            panic!("Unsupported WAV bit depth");
        }
    }
}*/

fn load_sample_file_i16(path: &str, mut reader: hound::WavReader<BufReader<File>>, channels: u16) -> SampleFile<Stereo<f32>> {
    let sample_rate = reader.spec().sample_rate;
    let size = reader.samples::<i16>().len();
    let mut buffer_new = Buffer::init(Stereo { left: 0.0, right: 0.0 }, size / 2);

    let mut i = 0;
    reader.samples::<i16>()
        .fold(0.0, |_, v| {
            let sample = v.unwrap().as_i16() as f32 * (1.0 / (i16::MAX as f32));
            if i % 2 == 0 {
                buffer_new.as_slice_mut()[i / channels as usize].left = sample;
            } else {
                buffer_new.as_slice_mut()[i / channels as usize].right = sample;
            }

            i += 1;
            0.0
        }
    );

    let mut sample = SampleFile::from(Arc::new(buffer_new), path.to_string());
    for note_name in NOTE_NAMES {
        if path.contains(note_name) {
            sample.pitch = Some(num_to_pitch(name_to_num(note_name).unwrap()));
        }
    }

    SAMPLE_CACHE_STEREO.write().unwrap().push(sample.clone());
    return sample;
}

fn load_sample_file_i24(path: &str, mut reader: hound::WavReader<BufReader<File>>, channels: u16) -> SampleFile<Stereo<f32>> {
    // let sample_rate = reader.spec().sample_rate;
    let size = reader.samples::<i32>().len();
    let mut buffer_new = Buffer::init(Stereo { left: 0.0, right: 0.0 }, size / 2);

    let mut i = 0;
    reader.samples::<i32>()
        .fold(0.0, |_, v| {
            let sample = v.unwrap() as f32 * (1.0 / (16777215 as f32));
            if i % 2 == 0 {
                buffer_new.as_slice_mut()[i / channels as usize].left = sample;
            } else {
                buffer_new.as_slice_mut()[i / channels as usize].right = sample;
            }

            i += 1;
            0.0
        }
    );

    let mut sample = SampleFile::from(Arc::new(buffer_new), path.to_string());
    for note_name in NOTE_NAMES {
        if path.contains(note_name) {
            sample.set_pitch(num_to_pitch(name_to_num(note_name).unwrap()));
        }
    }

    SAMPLE_CACHE_STEREO.write().unwrap().push(sample.clone());
    return sample;
}
