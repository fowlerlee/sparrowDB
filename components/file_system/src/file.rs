use super::settings::get_io_type;
use crate::io_rate_limiter::{IoOp, IoRateLimiter};
use std::fmt::{self, Debug, Formatter};
use std::io::{Read, Result, Write};
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
