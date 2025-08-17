use super::*;
use ::blake2::Blake2bVarCore;
use ::blake2::digest::{Update, VariableOutput};
use blake2::digest::core_api::RtVariableCoreWrapper;

pub struct BLAKE512 {
    hash: BITS512,
    hasher: RtVariableCoreWrapper<Blake2bVarCore>,
}

impl Hasher for BLAKE512 {
    fn new() -> Self {
        BLAKE512 {
            hash: [0; 64],
            // Variable size was done instead of using Blake2b512 type alias as the concrete type
            // that would need to be defined in the struct BLAKE512 is silly
            hasher: ::blake2::Blake2bVar::new(64).unwrap(),
        }
    }

    fn digest(&mut self, bytes: impl AsRef<[u8]>) {
        self.hasher.update(bytes.as_ref())
    }

    fn complete(mut self) -> HashReturn {
        let res = self.hasher.finalize_boxed();
        self.hash.copy_from_slice(&res);
        HashReturn::BLAKE512(self.hash)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_blake512_hash() {
        let input = "HelloWorld";
        let expected_output = "8dc77b2e140c3601a9fdd146684dea960124c514b999314be65fafe189cecee9bb1395cc80826aa1b8464de775678d13bfd332c51aafd026b9b5a67e606430f3";

        let mut blake2 = BLAKE512::new();

        blake2.digest(input);

        let result = blake2.complete().into_bytes();

        let hash_out = crate::hex_table::u8_array_to_lower_hex_string(&result).unwrap();

        assert_eq!(hash_out, expected_output);
    }
}
