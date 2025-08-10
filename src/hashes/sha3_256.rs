use super::*;

pub struct SHA3_256 {
    hash: BITS256,
}

impl Hasher for SHA3_256 {
    fn new() -> Self {
        SHA3_256 { hash: [0; 32] }
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
    fn test_sha3_256_hash() {
        todo!();
    }
}
