use {
    super::{
        TextStyle,
        Time,
    },
    crate::DocumentReader,
    nonmax::NonMaxU32,
    std::{
        io,
        io::Read,
    },
};

#[derive(Debug)]
pub struct CellText {
    pub text: String,
    pub relative_size: i32,
    pub image_index: Option<NonMaxU32>,
    pub style: TextStyle,
    pub last_edit: Time,
}

impl CellText {
    pub fn read<R>(reader: &mut DocumentReader<R>) -> io::Result<Self>
    where
        R: Read,
    {
        let text = reader.read_string()?;
        let relative_size = -reader.read_i32()?;
        let image_index = NonMaxU32::new(reader.read_u32()?);
        let style = TextStyle::from_bits(reader.read_u32()?)
            .ok_or_else(|| io::Error::other("unknown style bits"))?;
        let last_edit = Time::read(reader)?;

        Ok(Self {
            text,
            relative_size,
            image_index,
            style,
            last_edit,
        })
    }
}
