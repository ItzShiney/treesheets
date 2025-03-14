use {
    crate::{
        reader::DocumentReader,
        Cell,
        Image,
    },
    flate2::bufread::ZlibDecoder,
    std::{
        fs::File,
        io::{
            self,
            BufReader,
            Read,
        },
        path::Path,
    },
};

#[derive(Debug)]
pub struct Document {
    pub selection_width: u8,
    pub selection_height: u8,
    pub zoom_level: u8,

    pub images: Vec<Image>,
    pub root: Cell,
    pub tags: Vec<String>,
}

impl Document {
    pub fn read<P>(path: P) -> io::Result<Self>
    where
        P: AsRef<Path>,
    {
        let mut reader = DocumentReader::from(BufReader::new(File::open(path)?));

        if reader.read_bytes()? != *b"TSFF" {
            return Err(io::Error::other("not a TreeSheets file"));
        }
        if reader.read_byte()? != 23 {
            return Err(io::Error::other("unsupported file version"));
        }

        let selection_width = reader.read_byte()?;
        let selection_height = reader.read_byte()?;
        let zoom_level = reader.read_byte()?;

        let images = Image::read(&mut reader)?;

        let mut reader = DocumentReader::from(ZlibDecoder::new(reader.into_inner()));
        let root = Cell::read(&mut reader)?;
        let tags = Self::read_tags(&mut reader)?;

        Ok(Self {
            selection_width,
            selection_height,
            zoom_level,
            images,
            root,
            tags,
        })
    }

    fn read_tags<R>(reader: &mut DocumentReader<R>) -> io::Result<Vec<String>>
    where
        R: Read,
    {
        let mut res = Vec::default();
        loop {
            let tag = reader.read_string()?;
            if tag.is_empty() {
                break;
            }
            res.push(tag);
        }
        Ok(res)
    }
}
