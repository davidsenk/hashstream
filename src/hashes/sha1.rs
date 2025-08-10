use super::*;

pub struct SHA1 {
    hash: BITS160,
}

impl Hasher for SHA1 {
    fn new() -> Self {
        SHA1 { hash: [0; 20] }
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
    fn test_sha1_hash() {
        todo!();
    }
}
