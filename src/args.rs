use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about = None, long_about = None)]
pub struct Args {
    #[arg(default_value_t = false)]
    /// SHA2-256
    #[arg(long)]
    pub sha256: bool,
    /// SHA3-256
    #[arg(long)]
    pub sha3_256: bool,
    /// SHA1
    #[arg(long)]
    pub sha1: bool,
    /// CRC_32_ISO_HDLC
    #[arg(long)]
    pub crc32_iso: bool,
    /// CRC_32_CKSUM
    #[arg(long)]
    pub crc32_posix: bool,
    /// CRC_32_XFER
    #[arg(long)]
    pub crc32_xfer: bool,
    /// Blake2b256
    #[arg(long)]
    pub blake256: bool,
    /// Blake2b512
    #[arg(long)]
    pub blake512: bool,
    /// Blake3
    #[arg(long)]
    pub blake3: bool,
    /// MD5
    #[arg(long)]
    pub md5: bool,
}
