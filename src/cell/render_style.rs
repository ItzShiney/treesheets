use {
    crate::DocumentReader,
    std::io::{
        self,
        Read,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RenderStyle {
    Grid,
    Bubble,
    Line,
}

impl RenderStyle {
    pub fn read<R>(reader: &mut DocumentReader<R>) -> io::Result<Self>
    where
        R: Read,
    {
        Ok(match reader.read_byte()? {
            0 => Self::Grid,
            1 => Self::Bubble,
            2 => Self::Line,
            _ => return Err(io::Error::other("invalid cell type")),
        })
    }
}
