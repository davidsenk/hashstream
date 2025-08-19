use super::*;
use ::sha1::{Digest, Sha1};

pub struct SHA1 {
    hash: BITS160,
    hasher: Sha1,
}

impl Hasher for SHA1 {
    fn new() -> Self {
        SHA1 {
            hash: [0; 20],
            hasher: Sha1::new(),
        }
    }

    fn digest(&mut self, bytes: impl AsRef<[u8]>) {
        self.hasher.update(bytes)
    }

    fn complete(mut self) -> HashReturn {
        let res = self.hasher.finalize();
        self.hash.copy_from_slice(&res);
        HashReturn::SHA1(self.hash)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sha1_hash() {
        let input = "HelloWorld";
        let expected_output = "db8ac1c259eb89d4a131b253bacfca5f319d54f2";

        let mut sha1 = SHA1::new();

        sha1.digest(input);

        let result = sha1.complete().into_bytes();

        let hash_out = crate::hex_table::u8_array_to_lower_hex_string(&result).unwrap();

        assert_eq!(hash_out, expected_output);
    }
}
