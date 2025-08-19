use super::*;
use crc::Table;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CRC32TYPE {
    CKSUM,
    XFER,
    ISO,
}

pub struct CRC32 {
    crc32type: CRC32TYPE,
    digester: ::crc::Digest<'static, u32, Table<16>>,
}

/// CRC_32_ISO_HDLC
const CRC32_ISO: ::crc::Crc<u32, Table<16>> =
    ::crc::Crc::<u32, Table<16>>::new(&::crc::CRC_32_ISO_HDLC);
pub fn crc32_iso() -> CRC32 {
    CRC32 {
        crc32type: CRC32TYPE::ISO,
        digester: CRC32_ISO.digest(),
    }
}

/// CRC_32_XFER
const CRC32_XFER: ::crc::Crc<u32, Table<16>> =
    ::crc::Crc::<u32, Table<16>>::new(&::crc::CRC_32_XFER);
pub fn crc32_xfer() -> CRC32 {
    CRC32 {
        crc32type: CRC32TYPE::XFER,
        digester: CRC32_XFER.digest(),
    }
}

/// CRC_32_CKSUM
const CRC32_CKSUM: ::crc::Crc<u32, Table<16>> =
    ::crc::Crc::<u32, Table<16>>::new(&::crc::CRC_32_CKSUM);
pub fn crc32_cksum() -> CRC32 {
    CRC32 {
        crc32type: CRC32TYPE::CKSUM,
        digester: CRC32_CKSUM.digest(),
    }
}
impl Hasher for CRC32 {
    /// Defaults to CRC32_ISO_HDLC, most likely choice for CRC32
    /// Use the other crc32_* fns to get other crc32 hasher types
    fn new() -> Self {
        crc32_iso()
    }

    fn digest(&mut self, bytes: impl AsRef<[u8]>) {
        self.digester.update(bytes.as_ref())
    }

    fn complete(self) -> HashReturn {
        HashReturn::CRC32(self.digester.finalize(), self.crc32type)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_crc32_hashes() {
        let input_1 = "Hello";
        let input_2 = "World";

        let expected_iso = "77770c79";
        let expected_cksum = "b9e23d07";
        let expected_xfer = "9810e2bb";

        let mut iso = CRC32::new();
        let mut cksum = crc32_cksum();
        let mut xfer = crc32_xfer();

        iso.digest(input_1);
        iso.digest(input_2);
        cksum.digest(input_1);
        cksum.digest(input_2);
        xfer.digest(input_1);
        xfer.digest(input_2);

        let iso_result = iso.complete().into_bytes();
        let cksum_result = cksum.complete().into_bytes();
        let xfer_result = xfer.complete().into_bytes();

        let iso_hash = crate::hex_table::u8_array_to_lower_hex_string(&iso_result).unwrap();
        let cksum_hash = crate::hex_table::u8_array_to_lower_hex_string(&cksum_result).unwrap();
        let xfer_hash = crate::hex_table::u8_array_to_lower_hex_string(&xfer_result).unwrap();

        assert_eq!(iso_hash, expected_iso);
        assert_eq!(cksum_hash, expected_cksum);
        assert_eq!(xfer_hash, expected_xfer);
    }
}
