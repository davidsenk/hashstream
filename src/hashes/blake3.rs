use super::*;

pub struct BLAKE3 {
    hash: BITS256,
    hasher: ::blake3::Hasher,
}

impl Hasher for BLAKE3 {
    fn new() -> Self {
        BLAKE3 {
            hash: [0; 32],
            hasher: ::blake3::Hasher::new(),
        }
    }

    fn digest(&mut self, bytes: impl AsRef<[u8]>) {
        self.hasher.update(bytes.as_ref());
    }

    fn complete(self) -> HashReturn {
        let res = self.hasher.finalize();
        let mut ret = BITS256::default();
        ret.copy_from_slice(res.as_bytes());
        HashReturn::BLAKE3(ret)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_blake3_hash() {
        let input = "HelloWorld";
        let expected_output = "0588e8d8f7d781f64f170e58bad59d3d4de9fd6e7a61af0109819b2a41d09309";

        let mut blake3 = BLAKE3::new();

        blake3.digest(input);

        let result = blake3.complete().into_bytes();

        let hash_out = crate::hex_table::u8_array_to_lower_hex_string(&result).unwrap();

        assert_eq!(hash_out, expected_output);
    }
}
