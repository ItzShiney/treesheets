use {
    extended::Extended as f80,
    std::{
        io::{
            self,
            Read,
        },
        mem,
        slice,
    },
};

pub struct DocumentReader<R>(R);

impl<R> From<R> for DocumentReader<R> {
    fn from(reader: R) -> Self {
        Self(reader)
    }
}

impl<R> DocumentReader<R> {
    pub fn into_inner(self) -> R {
        self.0
    }
}

impl<R> DocumentReader<R>
where
    R: Read,
{
    pub fn read_bool(&mut self) -> io::Result<bool> {
        Ok(self.read_byte()? != 0)
    }

    pub fn read_byte(&mut self) -> io::Result<u8> {
        let mut res = 0;
        self.0.read_exact(slice::from_mut(&mut res))?;
        Ok(res)
    }

    pub fn read_i32(&mut self) -> io::Result<i32> {
        let mut buf = [0; mem::size_of::<i32>()];
        self.0.read_exact(&mut buf)?;
        Ok(i32::from_le_bytes(buf))
    }

    pub fn read_u32(&mut self) -> io::Result<u32> {
        let mut buf = [0; mem::size_of::<u32>()];
        self.0.read_exact(&mut buf)?;
        Ok(u32::from_le_bytes(buf))
    }

    pub fn read_u64(&mut self) -> io::Result<u64> {
        let mut buf = [0; mem::size_of::<u64>()];
        self.0.read_exact(&mut buf)?;
        Ok(u64::from_le_bytes(buf))
    }

    pub fn read_double(&mut self) -> io::Result<f64> {
        let mut buf = [0; 10];
        self.0.read_exact(&mut buf)?;
        Ok(f80::from_be_bytes(buf).to_f64())
    }

    pub fn read_bytes<const N: usize>(&mut self) -> io::Result<[u8; N]> {
        let mut res = [0; N];
        self.0.read_exact(&mut res)?;
        Ok(res)
    }

    pub fn read_bytes_len(&mut self, len: usize) -> io::Result<Vec<u8>> {
        let mut res = vec![0; len];
        self.0.read_exact(&mut res)?;
        Ok(res)
    }

    pub fn read_string(&mut self) -> io::Result<String> {
        let len = usize::try_from(self.read_u32()?).map_err(io::Error::other)?;
        self.read_bytes_len(len)
            .and_then(|bytes| String::from_utf8(bytes).map_err(io::Error::other))
    }
}
