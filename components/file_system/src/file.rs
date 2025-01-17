use super::settings::get_io_type;
use crate::io_rate_limiter::{IoOp, IoRateLimiter};
use std::fmt::{self, Debug, Formatter};
#[allow(unused)]
use std::io::{Read, Result, Seek, SeekFrom, Write};
use std::sync::Arc;

pub struct File {
    inner: std::fs::File,
    limiter: Option<Arc<IoRateLimiter>>,
}

impl Debug for File {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.inner)
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
