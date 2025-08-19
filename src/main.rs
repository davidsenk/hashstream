#![cfg(not(feature = "nobin"))]

mod args;
use args::Args;

use hashstream::{CRC32TYPE, HashType, Hashes};

use clap::Parser;

use std::io::{Read, stdin};
use std::process::exit;

fn main() {
    let args = Args::parse();
    let mut hashes = Vec::new();

    if args.sha256 || args.all {
        hashes.push(HashType::SHA256)
    }

    if args.sha3_256 || args.all {
        hashes.push(HashType::SHA3_256)
    }

    if args.sha1 || args.all {
        hashes.push(HashType::SHA1)
    }

    if args.crc32_iso || args.all {
        hashes.push(HashType::CRC32(CRC32TYPE::ISO))
    }

    if args.crc32_posix || args.all {
        hashes.push(HashType::CRC32(CRC32TYPE::CKSUM))
    }

    if args.crc32_xfer || args.all {
        hashes.push(HashType::CRC32(CRC32TYPE::XFER))
    }

    if args.blake256 || args.all {
        hashes.push(HashType::BLAKE256)
    }

    if args.blake512 || args.all {
        hashes.push(HashType::BLAKE512)
    }

    if args.blake3 || args.all {
        hashes.push(HashType::BLAKE3)
    }

    if args.md5 || args.all {
        hashes.push(HashType::MD5)
    }

    if hashes.is_empty() {
        eprintln!("Error: At least one hashtype or --all is required");
        exit(255);
    }

    let mut hashers = Hashes::new(&hashes);

    let stdin = stdin();
    let mut lock = stdin.lock();

    let mut buf = [0u8; 1_000_000];

    while let Ok(count) = lock.read(&mut buf) {
        if count > 0 {
            hashers.digest(&buf[0..count])
        } else {
            break;
        }
    }

    let ret = hashers.complete();

    for hash in ret {
        println!("{}", hash.serialize())
    }
}
