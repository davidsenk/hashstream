use super::*;
use ::sha3::{Digest, Sha3_256};

pub struct SHA3_256 {
    hash: BITS256,
    hasher: Sha3_256,
}

impl Hasher for SHA3_256 {
    fn new() -> Self {
        SHA3_256 {
            hash: [0; 32],
            hasher: Sha3_256::new(),
        }
    }

    fn digest(&mut self, bytes: impl AsRef<[u8]>) {
        self.hasher.update(bytes)
    }

    fn complete(mut self) -> HashReturn {
        let res = self.hasher.finalize();
        self.hash.copy_from_slice(&res);
        HashReturn::SHA3_256(self.hash)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sha3_256_hash() {
        let input = "HelloWorld";
        let expected_output = "964b398ecd55793d8ca93e01274efe1377a70c8dc358fdca17cb4e94a9ed7777";

        let mut sha3_256 = SHA3_256::new();

        sha3_256.digest(input);

        let result = sha3_256.complete().into_bytes();

        let hash_out = crate::hex_table::u8_array_to_lower_hex_string(&result).unwrap();

        assert_eq!(hash_out, expected_output);
    }
}
