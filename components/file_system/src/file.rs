use super::settings::get_io_type;
use crate::io_rate_limiter::{IoOp, IoRateLimiter};
use std::fmt::{self, Debug, Formatter};
#[allow(unused)]
use std::io::{Read, Result, Seek, SeekFrom, Write};
use std::sync::Arc;
use std::fs;
use std::path::Path;

use super::io_rate_limiter::get_io_rate_limiter;

pub struct File {
    inner: fs::File,
    limiter: Option<Arc<IoRateLimiter>>,
}

impl Debug for File {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}


impl File {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<File> {
        let inner = fs::File::open(path)?;
        Ok(File {
            inner,
            limiter: get_io_rate_limiter(),
        })
    }

    #[cfg(test)]
    pub fn open_with_limiter<P: AsRef<Path>>(
        path: P,
        limiter: Option<Arc<IoRateLimiter>>,
    ) -> Result<File> {
        let inner = fs::File::open(path)?;
        Ok(File { inner, limiter })
    }

    pub fn create<P: AsRef<Path>>(path: P) -> Result<File> {
        let inner = fs::File::create(path)?;
        Ok(File {
            inner,
            limiter: get_io_rate_limiter(),
        })
    }

    #[cfg(test)]
    pub fn create_with_limiter<P: AsRef<Path>>(
        path: P,
        limiter: Option<Arc<IoRateLimiter>>,
    ) -> Result<File> {
        let inner = fs::File::create(path)?;
        Ok(File { inner, limiter })
    }

    pub fn from_raw_file(file: fs::File) -> Result<File> {
        Ok(File {
            inner: file,
            limiter: get_io_rate_limiter(),
        })
    }

    pub fn sync_all(&self) -> Result<()> {
        self.inner.sync_all()
    }

    pub fn sync_data(&self) -> Result<()> {
        self.inner.sync_data()
    }

    pub fn set_len(&self, size: u64) -> Result<()> {
        self.inner.set_len(size)
    }

    pub fn metadata(&self) -> Result<fs::Metadata> {
        self.inner.metadata()
    }

    pub fn try_clone(&self) -> Result<File> {
        let inner = self.inner.try_clone()?;
        Ok(File {
            inner,
            limiter: get_io_rate_limiter(),
        })
    }

    pub fn set_permissions(&self, perm: fs::Permissions) -> Result<()> {
        self.inner.set_permissions(perm)
    }
}

impl Read for File {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        if let Some(limiter) = &mut self.limiter {
            let mut remains = buf.len();
            let mut pos = 0;
            while remains > 0 {
                let allowed = limiter.request(get_io_type(), IoOp::Read, remains);
                let read = self.inner.read(&mut buf[pos..pos + allowed])?;
                pos += read;
                remains -= read;
                if read == 0 {
                    break;
                }
            }
            Ok(pos)
        } else {
            self.inner.read(buf)
        }
    }
}

impl Seek for File {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        self.inner.seek(pos)
    }
}

impl Write for File {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        if let Some(limiter) = &mut self.limiter {
            let mut remains = buf.len();
            let mut pos = 0;
            while remains > 0 {
                let allowed = limiter.request(get_io_type(), IoOp::Write, remains);
                let written = self.inner.write(&buf[pos..pos + allowed])?;
                pos += written;
                remains -= written;
                if written == 0 {
                    break;
                }
            }
            Ok(pos)
        } else {
            self.inner.write(buf)
        }
    }

    fn flush(&mut self) -> Result<()> {
        self.inner.flush()
    }
}
