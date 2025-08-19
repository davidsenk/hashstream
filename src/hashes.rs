mod blake256;

mod blake3;
mod blake512;
mod crc32;
mod md5;
mod null_hash;
mod sha1;
mod sha256;
mod sha3_256;

use std::sync::Arc;

type BITS512 = [u8; 64];

type BITS256 = [u8; 32];
type BITS160 = [u8; 20];

type BITS128 = [u8; 16];

type BITS32 = u32;

type ArcU8 = Arc<[u8]>;

macro_rules! arc_u8_sized {
    ($size: literal) => {
        Arc::new([0u8, $size])
    };
}

fn arc_u8_empty() -> ArcU8 {
    arc_u8_sized!(0)
}

fn bits512_default() -> BITS512 {
    [0; 64]
}

#[derive(Debug, PartialEq)]
pub enum HashReturn {
    CRC32(BITS32, crc32::CRC32TYPE),
    SHA256(BITS256),
    SHA3_256(BITS256),
    BLAKE256(BITS256),
    BLAKE512(BITS512),
    BLAKE3(BITS256),
    SHA1(BITS160),
    MD5(BITS128),
    RAW(ArcU8),
}

impl HashReturn {
    pub fn into_bytes(self) -> Arc<[u8]> {
        match self {
            HashReturn::CRC32(inner, _) => Arc::new(inner.to_be_bytes()),
            HashReturn::SHA256(inner) => Arc::new(inner),
            HashReturn::SHA3_256(inner) => Arc::new(inner),
            HashReturn::BLAKE256(inner) => Arc::new(inner),
            HashReturn::BLAKE512(inner) => Arc::new(inner),
            HashReturn::BLAKE3(inner) => Arc::new(inner),
            HashReturn::SHA1(inner) => Arc::new(inner),
            HashReturn::MD5(inner) => Arc::new(inner),
            HashReturn::RAW(inner) => inner,
        }
    }
}

pub trait Hasher {
    fn new() -> Self;
    fn digest(&mut self, bytes: impl AsRef<[u8]>);
    fn complete(self) -> HashReturn;
}

#[derive(Debug, Copy, Clone)]
pub enum HashType {
    CRC32(crc32::CRC32TYPE),
    SHA256,
    SHA3_256,
    BLAKE256,
    BLAKE512,
    BLAKE3,
    SHA1,
    MD5,
    Null,
}

// Why? because static dispatch is why!
// TODO: Make the concrete types return this too
//  Maybe even a macro that does the below boilerplate? idk
enum HasherHolder {
    CRC32(crc32::CRC32),
    SHA256(sha256::SHA256),
    SHA3_256(sha3_256::SHA3_256),
    BLAKE256(blake256::BLAKE256),
    BLAKE512(blake512::BLAKE512),
    BLAKE3(blake3::BLAKE3),
    SHA1(sha1::SHA1),
    MD5(md5::MD5),
    NULL(null_hash::NullHash),
}

pub struct Hashes {
    hashers: Vec<HasherHolder>,
}

impl Hashes {
    pub fn new(types: &[HashType]) -> Self {
        let mut hashers = Vec::with_capacity(types.len());
        for t in types {
            match t {
                HashType::CRC32(t) => match t {
                    crc32::CRC32TYPE::ISO => hashers.push(HasherHolder::CRC32(crc32::crc32_iso())),
                    crc32::CRC32TYPE::CKSUM => {
                        hashers.push(HasherHolder::CRC32(crc32::crc32_cksum()))
                    }
                    crc32::CRC32TYPE::XFER => hashers.push(HasherHolder::CRC32(crc32_xfer())),
                },
                HashType::SHA256 => hashers.push(HasherHolder::SHA256(sha256::SHA256::new())),
                HashType::SHA3_256 => {
                    hashers.push(HasherHolder::SHA3_256(sha3_256::SHA3_256::new()))
                }
                HashType::BLAKE256 => {
                    hashers.push(HasherHolder::BLAKE256(blake256::BLAKE256::new()))
                }
                HashType::BLAKE512 => {
                    hashers.push(HasherHolder::BLAKE512(blake512::BLAKE512::new()))
                }
                HashType::BLAKE3 => hashers.push(HasherHolder::BLAKE3(blake3::BLAKE3::new())),
                HashType::SHA1 => hashers.push(HasherHolder::SHA1(sha1::SHA1::new())),
                HashType::MD5 => hashers.push(HasherHolder::MD5(md5::MD5::new())),
                HashType::Null => hashers.push(HasherHolder::NULL(null_hash::NullHash::new())),
            }
        }

        Hashes { hashers }
    }

    pub fn digest(&mut self, bytes: impl AsRef<[u8]>) {
        for hasher in &mut self.hashers {
            match hasher {
                HasherHolder::CRC32(hasher) => hasher.digest(&bytes),
                HasherHolder::SHA256(hasher) => hasher.digest(&bytes),
                HasherHolder::SHA3_256(hasher) => hasher.digest(&bytes),
                HasherHolder::BLAKE256(hasher) => hasher.digest(&bytes),
                HasherHolder::BLAKE512(hasher) => hasher.digest(&bytes),
                HasherHolder::BLAKE3(hasher) => hasher.digest(&bytes),
                HasherHolder::SHA1(hasher) => hasher.digest(&bytes),
                HasherHolder::MD5(hasher) => hasher.digest(&bytes),
                HasherHolder::NULL(hasher) => hasher.digest(&bytes),
            }
        }
    }

    pub fn complete(mut self) -> Vec<HashReturn> {
        let mut ret = Vec::with_capacity(self.hashers.len());

        for hasher in self.hashers {
            match hasher {
                HasherHolder::CRC32(hasher) => ret.push(hasher.complete()),
                HasherHolder::SHA256(hasher) => ret.push(hasher.complete()),
                HasherHolder::SHA3_256(hasher) => ret.push(hasher.complete()),
                HasherHolder::BLAKE256(hasher) => ret.push(hasher.complete()),
                HasherHolder::BLAKE512(hasher) => ret.push(hasher.complete()),
                HasherHolder::BLAKE3(hasher) => ret.push(hasher.complete()),
                HasherHolder::SHA1(hasher) => ret.push(hasher.complete()),
                HasherHolder::MD5(hasher) => ret.push(hasher.complete()),
                HasherHolder::NULL(hasher) => ret.push(hasher.complete()),
            }
        }

        ret
    }
}

#[cfg(test)]
mod test {
    use crate::hashes::crc32::CRC32TYPE;
    use crate::hashes::{HashReturn, HashType, Hashes};

    #[test]
    fn test_multi_hash() {
        let hash_types = [
            HashType::CRC32(CRC32TYPE::CKSUM),
            HashType::CRC32(CRC32TYPE::ISO),
            HashType::CRC32(CRC32TYPE::XFER),
            HashType::SHA256,
            HashType::SHA3_256,
            HashType::BLAKE256,
            HashType::BLAKE512,
            HashType::BLAKE3,
            HashType::SHA1,
            HashType::MD5,
            //HashType::Null,
        ];
        let mut hashes = Hashes::new(&hash_types);

        hashes.digest("HashStream2025");

        let ret = hashes.complete();

        // Not using .into_bytes() to properly check each hash type
        for hash in ret {
            match hash {
                HashReturn::CRC32(hash, t) => {
                    let hash_out =
                        crate::hex_table::u8_array_to_lower_hex_string(&hash.to_be_bytes())
                            .unwrap();
                    match t {
                        CRC32TYPE::CKSUM => {
                            assert_eq!(hash_out, "5f4f456d");
                        }
                        CRC32TYPE::XFER => {
                            assert_eq!(hash_out, "99e560da");
                        }
                        CRC32TYPE::ISO => {
                            assert_eq!(hash_out, "66eac774");
                        }
                    }
                }
                HashReturn::SHA256(hash) => {
                    let hash_out = crate::hex_table::u8_array_to_lower_hex_string(&hash).unwrap();
                    assert_eq!(
                        hash_out,
                        "9975cc7fe9ae0c307eae57ac62a099f07749d0d7d01362e92435941ca18a635e"
                    );
                }
                HashReturn::SHA3_256(hash) => {
                    let hash_out = crate::hex_table::u8_array_to_lower_hex_string(&hash).unwrap();
                    assert_eq!(
                        hash_out,
                        "47eec6a5beaf06f0bf5f0ee06f0cae374fd46b968849d2b65de2d8623e907e16"
                    );
                }
                HashReturn::BLAKE256(hash) => {
                    let hash_out = crate::hex_table::u8_array_to_lower_hex_string(&hash).unwrap();
                    assert_eq!(
                        hash_out,
                        "850e58e95935ccde7320cc41ff99c41877b4e8956f6c5c09d0a5afd40c659519"
                    );
                }
                HashReturn::BLAKE512(hash) => {
                    let hash_out = crate::hex_table::u8_array_to_lower_hex_string(&hash).unwrap();
                    assert_eq!(
                        hash_out,
                        "29f9c9c6f598e570fa6d036f68bc9e22d34cab2359a1dcf738d9a32a98721a5f0b7de34f20429c814529d3fac96e68a2d37460aeea355a3712e30f601ea3e607"
                    );
                }
                HashReturn::BLAKE3(hash) => {
                    let hash_out = crate::hex_table::u8_array_to_lower_hex_string(&hash).unwrap();
                    assert_eq!(
                        hash_out,
                        "6323d7085cd64ef68d16b2b53ec50de2b65f0f6e9d6fc44833de8925790eb3e6"
                    );
                }
                HashReturn::SHA1(hash) => {
                    let hash_out = crate::hex_table::u8_array_to_lower_hex_string(&hash).unwrap();
                    assert_eq!(hash_out, "d07e867570b94d81178a644e4b0359dc354872f1");
                }
                HashReturn::MD5(hash) => {
                    let hash_out = crate::hex_table::u8_array_to_lower_hex_string(&hash).unwrap();
                    assert_eq!(hash_out, "e001d16d3d95cf04ec4a86ecb4c5ab6a");
                }
                HashReturn::RAW(_) => {
                    todo!("Shouldn't be here for this test...")
                }
            }
        }
    }
}
