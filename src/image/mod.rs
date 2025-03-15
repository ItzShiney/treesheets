use {
    crate::reader::DocumentReader,
    std::{
        fmt::{
            self,
            Debug,
        },
        io::{
            self,
            Read,
        },
    },
};

mod ty;

pub use ty::*;

pub struct Image {
    pub ty: ImageType,
    pub scale: f64,
    pub data: Vec<u8>,
}

impl Debug for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Image")
            .field("ty", &self.ty)
            .field("scale", &self.scale)
            .field("data", &[..])
            .finish()
    }
}

impl Image {
    pub fn read<R>(reader: &mut DocumentReader<R>) -> io::Result<Vec<Self>>
    where
        R: Read,
    {
        let mut images = Vec::default();
        loop {
            let ty = match reader.read_byte()? {
                b'D' => break,
                value => ImageType::try_from(value).map_err(io::Error::other)?,
            };
            let scale = reader.read_double()?;
            let data = {
                let len = usize::try_from(reader.read_u64()?).map_err(io::Error::other)?;
                reader.read_bytes_len(len)?
            };

            images.push(Self { ty, scale, data });
        }
        Ok(images)
    }
}
