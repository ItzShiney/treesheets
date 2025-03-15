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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum ImageType {
    Png = b'I',
    Jpeg = b'J',
}

impl TryFrom<u8> for ImageType {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            b'I' => Self::Png,
            b'J' => Self::Jpeg,
            _ => return Err("invalid image type"),
        })
    }
}

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
