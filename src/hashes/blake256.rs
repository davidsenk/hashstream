use super::*;
use ::blake2::Blake2bVarCore;
use ::blake2::digest::{Update, VariableOutput};
use blake2::digest::core_api::RtVariableCoreWrapper;

pub struct BLAKE256 {
    hash: BITS256,
    hasher: RtVariableCoreWrapper<Blake2bVarCore>,
}

impl Hasher for BLAKE256 {
    fn new() -> Self {
        BLAKE256 {
            hash: [0; 32],
            hasher: ::blake2::Blake2bVar::new(32).unwrap(),
        }
    }

    fn digest(&mut self, bytes: impl AsRef<[u8]>) {
        self.hasher.update(bytes.as_ref())
    }

    fn complete(mut self) -> HashReturn {
        let res = self.hasher.finalize_boxed();
        self.hash.copy_from_slice(&res);
        HashReturn::BLAKE256(self.hash)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_blake256_hash() {
        let input = "HelloWorld";
        let expected_output = "27159ce7d992c98fb04d5e9a59e43e75f77882b676fc6b2ccb8e952c2373da3e";

        let mut blake2 = BLAKE256::new();

        blake2.digest(input);

        let result = blake2.complete().into_bytes();

        let hash_out = crate::hex_table::u8_array_to_lower_hex_string(&result).unwrap();

        assert_eq!(hash_out, expected_output);
    }
}
