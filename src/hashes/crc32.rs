use super::*;

pub struct CRC32 {
    hash: BITS32,
}

impl Hasher for CRC32 {
    fn new() -> Self {
        CRC32 { hash: 0 }
    }

    fn digest(&mut self, bytes: impl AsRef<[u8]>) {
        todo!()
    }

    fn complete(self) -> HashReturn {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_crc32_hash() {
        todo!();
    }
}
