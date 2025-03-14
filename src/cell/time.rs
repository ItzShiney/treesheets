use {
    crate::DocumentReader,
    std::{
        fmt::Debug,
        io::{
            self,
            Read,
        },
        time::Duration,
    },
};

#[derive(Clone, Copy)]
pub struct Time {
    pub millis: u64,
}

impl Time {
    pub fn read<R>(reader: &mut DocumentReader<R>) -> io::Result<Self>
    where
        R: Read,
    {
        Ok(Self {
            millis: reader.read_u64()?,
        })
    }
}

impl Debug for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", Duration::from_millis(self.millis))
    }
}
