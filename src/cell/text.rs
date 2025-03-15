use {
    super::{
        TextStyle,
        Time,
    },
    crate::DocumentReader,
    std::io::{
        self,
        Read,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MaybeImageIndex(u32);

impl MaybeImageIndex {
    pub fn read<R>(reader: &mut DocumentReader<R>) -> io::Result<Self>
    where
        R: Read,
    {
        reader.read_u32().map(Self)
    }

    pub const fn is_some(self) -> bool {
        self.0 != u32::MAX
    }

    pub fn into_index(self) -> Option<u32> {
        self.is_some().then_some(self.0)
    }
}

#[derive(Debug)]
pub struct CellText {
    pub text: String,
    pub relative_size: i32,
    pub image_index: MaybeImageIndex,
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
        let image_index = MaybeImageIndex::read(reader)?;
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
