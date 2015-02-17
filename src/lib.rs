#![feature(io)]

extern crate "rustc-serialize" as rustc_serialize;

use std::io::{Read, Seek};

/*
 * TODO: Use an enum with a custom impl of Decoder to control the endianness
 *
 * TODO: figure out how to get offsets based on a packed layout
 */
#[repr(packed)]
#[derive(RustcDecodable, RustcEncodable)]
struct Super {
    s_magic: Le<u32>,
    inodes: Le<u32>,
    mkfs_time: Le<u32>,
    block_size: Le<u32>,
    fragments: Le<u32>,
    compression: Le<u16>,
    block_log: Le<u16>,
    flags: Le<u16>,
    no_ids: Le<u16>,
    s_major: Le<u16>,
    s_minor: Le<u16>,
    root_inode: Le<u64>,
    bytes_used: Le<u64>,
    id_table_start: Le<u64>,
    xattr_id_table_start: Le<u64>,
    inode_table_start: Le<u64>,
    directory_table_start: Le<u64>,
    fragment_table_start: Le<u64>,
    lookup_table_start: Le<u64>,
}

pub struct File<'a, R: Read + Seek + 'a> {
    /* FIXME: mutability is only for Seek+Reader's limitation in needing a cursor.
     * Once we have it, change to ReaderAt and use a non-mut reference
     */
    a: &'a mut R
}

impl<'b, R: Read+Seek + 'b> File<'b, R> {
    fn new(r: &'b mut R) -> File<'b, R>
    {
        File { a : r }
    }

    fn size(&mut self) -> u64
    {
        a.seek(io::SeekFrom::Start(
    }
}

#[test]
fn file_new() {
    use std::fs;
    let mut io = fs::File::open("tmp/empty.squashfs").unwrap();
    let f = File::new(&mut io);
}
