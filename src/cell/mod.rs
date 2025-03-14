use {
    crate::DocumentReader,
    std::{
        io,
        io::Read,
    },
};

mod color;
mod contents;
mod grid;
mod render_style;
mod text;
mod text_style;
mod time;
mod ty;

pub use {
    color::*,
    contents::*,
    grid::*,
    render_style::*,
    text::*,
    text_style::*,
    time::*,
    ty::*,
};

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
