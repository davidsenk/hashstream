use super::*;

pub struct SHA256 {
    hash: BITS256,
}

impl Hasher for SHA256 {
    fn new() -> Self {
        SHA256 { hash: [0; 32] }
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
    fn test_sha256_hash() {
        todo!();
    }
}
