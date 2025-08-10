mod blake256;
mod crc32;
mod md5;
mod null_hash;
mod sha1;
mod sha256;
mod sha3_256;

use crate::Args;

use std::sync::Arc;

type BITS256 = [u8; 32];
type BITS160 = [u8; 20];

type BITS128 = [u8; 16];

type BITS32 = u32;

type ArcU8 = Arc<[u8]>;

macro_rules! arc_u8_sized {
    ($size: literal) => {
        Arc::new([0u8, $size])
    };
}

fn arc_u8_empty() -> ArcU8 {
    arc_u8_sized!(0)
}

#[derive(Debug, PartialEq)]
pub enum HashReturn {
    CRC32(BITS32),
    SHA256(BITS256),
    SHA3_256(BITS256),
    BLAKE256(BITS256),
    SHA1(BITS160),
    MD5(BITS128),
    RAW(ArcU8),
}

impl HashReturn {
    pub fn into_bytes(self) -> Arc<[u8]> {
        match self {
            HashReturn::CRC32(inner) => Arc::new(inner.to_le_bytes()),
            HashReturn::SHA256(inner) => Arc::new(inner),
            HashReturn::SHA3_256(inner) => Arc::new(inner),
            HashReturn::BLAKE256(inner) => Arc::new(inner),
            HashReturn::SHA1(inner) => Arc::new(inner),
            HashReturn::MD5(inner) => Arc::new(inner),
            HashReturn::RAW(inner) => inner,
        }
    }
}

pub trait Hasher {
    fn new() -> Self;
    fn digest(&mut self, bytes: impl AsRef<[u8]>);
    fn complete(self) -> HashReturn;
}

pub struct Hashes {}
