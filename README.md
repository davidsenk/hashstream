# hashstream

A combined streaming hasher.

Hashstream is designed for needing multiple different hash types from a single stream / file simultaneously.

Hashstream does not perform any file IO on its own, it only reads and writes to stdio. You must pipe `|` or `<` redirect input to hashstream for processing.

Useful for coordinating systems that provide different hashtypes for stored blobs.

# Usage

` dd if=/dev/zero bs=4M count=250 status=progress | hashstream --sha256 `
` hashstream --all < /home/user/somefile.dat `

```
Usage: hashstream [OPTIONS]

Options:
      --all          Enable all hash types, can not be used with any other hash type flag [DEFAULT]
      --sha256       SHA2-256
      --sha3-256     SHA3-256
      --sha1         SHA1
      --crc32-iso    CRC_32_ISO_HDLC
      --crc32-posix  CRC_32_CKSUM
      --crc32-xfer   CRC_32_XFER
      --blake256     Blake2b256
      --blake512     Blake2b512
      --blake3       Blake3
      --md5          MD5
  -h, --help         Print help
  -V, --version      Print version

```

# TODO:

 - Multithreading? Maybe higher performance for multiple hashes at same time? 
 - More hash types? idk what else may be useful
 - Passthrough processing? Maybe output hashes to stderr or socket / named pipe?
 - More perf tuning? Is faster than `*sum` utils on Arch for me, but slower under Alma and MacOS...
 - Polynym binary? i.e. like busybox, selects set of hashes based on binary / link name?
