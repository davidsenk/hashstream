use super::*;

pub struct BLAKE256 {
    hash: BITS256,
}

impl Hasher for BLAKE256 {
    fn new() -> Self {
        BLAKE256 { hash: [0; 32] }
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
    fn test_blake256_hash() {
        todo!();
    }
}
