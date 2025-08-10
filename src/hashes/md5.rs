use super::*;

pub struct MD5 {
    hash: BITS128,
}

impl Hasher for MD5 {
    fn new() -> Self {
        MD5 { hash: [0; 16] }
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
    fn test_md5_hash() {
        todo!();
    }
}
