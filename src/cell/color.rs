use {
    crate::DocumentReader,
    std::{
        io,
        io::Read,
    },
};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn read<R>(reader: &mut DocumentReader<R>) -> io::Result<Self>
    where
        R: Read,
    {
        let [r, g, b, _] = reader.read_bytes()?;
        Ok(Self { r, g, b })
    }
}
