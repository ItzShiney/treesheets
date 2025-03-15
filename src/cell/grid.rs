use {
    super::{
        Cell,
        Color,
    },
    crate::DocumentReader,
    std::{
        io::{
            self,
            Read,
        },
        iter,
    },
};

#[derive(Debug)]
pub struct CellGrid {
    pub width: u32,
    pub height: u32,
    pub border_color: Color,
    pub border_width: u32,
    // TODO learn what actually this is and rename accordingly
    pub is_vertical_text_and_grid: bool,
    pub is_folded: bool,
    pub column_widths: Vec<u32>,
    pub cells: Vec<Cell>,
}

impl CellGrid {
    pub fn read<R>(reader: &mut DocumentReader<R>) -> io::Result<Self>
    where
        R: Read,
    {
        let width = reader.read_u32()?;
        let height = reader.read_u32()?;
        let border_color = Color::read(reader)?;
        let border_width = reader.read_u32()?;
        let is_vertical_text_and_grid = reader.read_bool()?;
        let is_folded = reader.read_bool()?;

        let column_widths = iter::repeat_with(|| reader.read_u32())
            .take(usize::try_from(width).map_err(io::Error::other)?)
            .collect::<Result<_, _>>()?;
        let cells = iter::repeat_with(|| Cell::read(reader))
            .take(usize::try_from(width * height).map_err(io::Error::other)?)
            .collect::<Result<_, _>>()?;

        Ok(Self {
            width,
            height,
            border_color,
            border_width,
            is_vertical_text_and_grid,
            is_folded,
            column_widths,
            cells,
        })
    }
}
