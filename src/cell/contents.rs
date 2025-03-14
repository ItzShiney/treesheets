use {
    crate::DocumentReader,
    std::{
        io,
        io::Read,
    },
};

const SELECTION: u8 = 1 << 7;

const IS_TEXT: u8 = 0;
const IS_GRID: u8 = 1;
const IS_TEXT_AND_GRID: u8 = 2;
const IS_NEITHER_TEXT_NOR_GRID: u8 = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CellContents {
    pub starts_selection: bool,
    pub has_text: bool,
    pub has_grid: bool,
}

impl CellContents {
    pub fn read<R>(reader: &mut DocumentReader<R>) -> io::Result<Self>
    where
        R: Read,
    {
        let mut byte = reader.read_byte()?;

        let starts_selection = byte & SELECTION != 0;

        byte &= !SELECTION;
        let has_text = matches!(byte, IS_TEXT | IS_TEXT_AND_GRID);
        let has_grid = matches!(byte, IS_GRID | IS_TEXT_AND_GRID);

        if !has_text && !has_grid && byte != IS_NEITHER_TEXT_NOR_GRID {
            return Err(io::Error::other("invalid cell type"));
        }

        Ok(Self {
            starts_selection,
            has_text,
            has_grid,
        })
    }
}
