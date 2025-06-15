use super::*;

/// Does not actually hash anything, will contain last [u8] digested
/// If len digested > 32 then only last 32 bytes digested will be contained
/// If len digest < 32 will contain entire digested bytes + either 0, or
/// any other digested remnants that were > len last digest.
///
/// i.e. starting state is [0, 0, 0, ..]
/// digest [1, 2]
/// state to [1, 2, 0, ..]
/// digest [3, 4]
/// state to [3, 4, 0, ..]
/// digest [0, 1, 2, .., 31]
/// state to [0, 1, 2, .., 31]
/// digest [5, 6]
/// state to [5, 6, 2, .., 31]
pub struct NullHash {
    hash: BITS256,
}

impl NullHash {
    pub fn new() -> Self {
        NullHash { hash: [0; 32] }
    }
    pub fn digest(&mut self, bytes: impl AsRef<[u8]>) {
        let bytes = bytes.as_ref();

        let start_point = if bytes.len() < 32 {
            0
        } else {
            bytes.len() - 32
        };

        let bytes = &bytes[start_point..];

        for (loc, byte) in self.hash.iter_mut().zip(bytes) {
            *loc = *byte;
        }
    }
    pub fn complete(self) -> BITS256 {
        self.hash
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_null_hash() {
        let mut hasher = NullHash::new();

        for i in 0..32u8 {
            hasher.digest([i; 1]);
        }

        assert_eq!(*hasher.complete().first().unwrap(), 31);
    }

    #[test]
    fn test_null_hash_oversized() {
        let mut hasher = NullHash::new();

        let mut source_bytes = [0u8; 255];

        for (byte, i) in source_bytes.iter_mut().zip(0..=u8::MAX) {
            *byte = i;
        }

        hasher.digest(source_bytes);

        let result = hasher.complete();

        let start = 255 - 32u8;
        for (byte, i) in result.iter().zip(start..=255) {
            assert_eq!(*byte, i);
        }
    }
}
