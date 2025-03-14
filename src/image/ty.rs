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
