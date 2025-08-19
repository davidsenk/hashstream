use super::*;
use ::sha2::{Digest, Sha256};

pub struct SHA256 {
    hash: BITS256,
    hasher: Sha256,
}

impl Hasher for SHA256 {
    fn new() -> Self {
        SHA256 {
            hash: [0; 32],
            hasher: Sha256::new(),
        }
    }

    fn digest(&mut self, bytes: impl AsRef<[u8]>) {
        self.hasher.update(bytes)
    }

    fn complete(mut self) -> HashReturn {
        let res = self.hasher.finalize();
        self.hash.copy_from_slice(&res);
        HashReturn::SHA256(self.hash)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sha256_hash() {
        let input = "HelloWorld";
        let expected_output = "872e4e50ce9990d8b041330c47c9ddd11bec6b503ae9386a99da8584e9bb12c4";

        let mut sha256 = SHA256::new();

        sha256.digest(input);

        let result = sha256.complete().into_bytes();

        let hash_out = crate::hex_table::u8_array_to_lower_hex_string(&result).unwrap();

        assert_eq!(hash_out, expected_output);
    }
}
