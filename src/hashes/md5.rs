use super::*;
use ::md5::{Digest, Md5};

pub struct MD5 {
    hash: BITS128,
    hasher: Md5,
}

impl Hasher for MD5 {
    fn new() -> Self {
        MD5 {
            hash: [0; 16],
            hasher: Md5::new(),
        }
    }

    fn digest(&mut self, bytes: impl AsRef<[u8]>) {
        self.hasher.update(bytes)
    }

    fn complete(mut self) -> HashReturn {
        let res = self.hasher.finalize();
        self.hash.copy_from_slice(&res);
        HashReturn::MD5(self.hash)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_md5_hash() {
        let input = "HelloWorld";
        let expected_output = "68e109f0f40ca72a15e05cc22786f8e6";

        let mut md5 = MD5::new();

        md5.digest(input);

        let result = md5.complete().into_bytes();

        let hash_out = crate::hex_table::u8_array_to_lower_hex_string(&result).unwrap();

        assert_eq!(hash_out, expected_output);
    }
}
