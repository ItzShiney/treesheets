use {
    crate::DocumentReader,
    std::{
        fmt::{
            self,
            Debug,
        },
        io::{
            self,
            Read,
        },
        time::Duration,
    },
};

mod contents;
mod grid;
mod text;

pub use {
    contents::*,
    grid::*,
    text::*,
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RenderStyle {
    #[default]
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
            _ => return Err(io::Error::other("invalid render style")),
        })
    }
}

#[derive(Clone, Copy)]
pub struct Time {
    pub ms: u64,
}

impl Time {
    pub fn read<R>(reader: &mut DocumentReader<R>) -> io::Result<Self>
    where
        R: Read,
    {
        let ms = reader.read_u64()?;
        Ok(Self { ms })
    }
}

impl Debug for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", Duration::from_millis(self.ms))
    }
}

// TODO: learn what actually this is and rename accordingly
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

#[derive(Debug)]
pub struct Cell {
    pub ty: CellType,
    pub bg_color: Color,
    pub text_color: Color,
    pub render_style: RenderStyle,
    pub starts_selection: bool,
    pub text: Option<CellText>,
    pub grid: Option<CellGrid>,
}

impl Cell {
    pub fn read<R>(reader: &mut DocumentReader<R>) -> io::Result<Self>
    where
        R: Read,
    {
        let ty = CellType::read(reader)?;
        let bg_color = Color::read(reader)?;
        let text_color = Color::read(reader)?;
        let render_style = RenderStyle::read(reader)?;

        let CellContents {
            starts_selection,
            has_text,
            has_grid,
        } = CellContents::read(reader)?;

        let text = has_text.then(|| CellText::read(reader)).transpose()?;
        let grid = has_grid.then(|| CellGrid::read(reader)).transpose()?;

        Ok(Self {
            ty,
            bg_color,
            text_color,
            render_style,
            starts_selection,
            text,
            grid,
        })
    }
}
