use {
    crate::DocumentReader,
    std::{
        io,
        io::Read,
    },
};

// TODO learn what actually this is and rename accordingly
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CellType {
    Data,
    Code,
    VarD,
    ViewH,
    VarU,
    ViewV,
}

impl CellType {
    pub fn read<R>(reader: &mut DocumentReader<R>) -> io::Result<Self>
    where
        R: Read,
    {
        Ok(match reader.read_byte()? {
            0 => Self::Data,
            1 => Self::Code,
            2 => Self::VarD,
            3 => Self::ViewH,
            4 => Self::VarU,
            5 => Self::ViewV,
            _ => return Err(io::Error::other("invalid cell type")),
        })
    }
}
