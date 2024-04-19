use crate::buffers::*;
use crate::float::*;

pub struct SampleMut {
    pub buffer: Buffer<Stereo<f32>>,
}

impl SampleMut {
    pub fn from(buffer: Buffer<Stereo<f32>>) -> Self {
        Self { buffer }
    }

    /*pub fn load(path: &str) -> Self {
        let mut reader = hound::WavReader::open(path).unwrap();

        let size = reader.samples::<i16>().len();
        let buffer = AudioBuffer::new(size);
        let sample = SampleMut { buffer, path: path.to_string() };

        sample.load_sample_async(path);

        sample
    }

    pub fn cached(path: &str) -> Self {
        SampleCache::load_mut(path)
    }

    pub fn from(buffer: AudioBuffer, path: String) -> Self {
        SampleMut { buffer, path }
    }

    pub fn save(&self) {
        SampleCache::insert(&self);

        println!("WARNING: Save assuming sample rate of 44100");

        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: 44100,
            bits_per_sample: 32,
            sample_format: hound::SampleFormat::Float,
        };

        let mut writer = hound::WavWriter::create(self.path.clone(), spec).unwrap();

        for sample in &self.buffer {
            match writer.write_sample(*sample) {
                Ok(_) => (),
                Err(_) => println!("Error writing sample"),
            }
        }
    }*/

    fn load_sample_async(&self, path: &str) {
        let _path = path.to_string();
        //let mut block = self.buffer.clone();
        //^^^ Need reference counted buffer

        /*std::thread::spawn(move || {
            let mut reader = hound::WavReader::open(path).unwrap();

            let mut i = 0;
            reader.samples::<i16>()
                .fold(0.0, |_, v| {
                    let sample = v.unwrap() as f32 * (1.0/32768.0);
                    block.as_mut()[i] = sample;

                    i += 1;
                    0.0
                }
            );

            println!("Sample loaded.");
        });*/
    }

    /*fn as_ptr(&self) -> *const f32 {
        self.buffer.as_ptr()
    }

    fn as_mut_ptr(&mut self) -> *mut f32 {
        self.buffer.as_mut_ptr()
    }*/

    fn size(&self) -> usize {
        self.buffer.capacity()
    }
}

/*impl FloatBuffer for SampleMut {
}*/

/*impl<'a> IntoIterator for &'a SampleMut {
    type Item = &'a f32;
    type IntoIter = slice::Iter<'a, f32>;

    fn into_iter(self) -> slice::Iter<'a, f32> {
        self.as_ref().into_iter()
    }
}

impl<'a> IntoIterator for &'a mut SampleMut {
    type Item = &'a mut f32;
    type IntoIter = slice::IterMut<'a, f32>;

    fn into_iter(self) -> slice::IterMut<'a, f32> {
        self.as_mut().into_iter()
    }
}*/

/*impl<'a> IntoIterator for &'a SampleMut {
    type Item = &'a f32;
    type IntoIter = slice::Iter<'a, f32>;

    fn into_iter(self) -> slice::Iter<'a, f32> {
        unsafe {
            self.buffer.as_ref().into_iter()
        }
    }
}*/
