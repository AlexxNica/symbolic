extern crate symbolic_symcache;
extern crate symbolic_debuginfo;
extern crate symbolic_common;

use std::env;
use std::fs;
use std::io::Read;

use symbolic_symcache::SymCache;
use symbolic_debuginfo::FatObject;
use symbolic_common::ByteView;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        panic!("takes example one argument");
    }

    let filename = &args[1];
    let bv = if env::var("NO_MMAP").as_ref().map(|x| x.as_str()) == Ok("1") {
        let mut f = fs::File::open(filename).unwrap();
        let mut out = Vec::new();
        f.read_to_end(&mut out).unwrap();
        ByteView::from_vec(out)
    } else {
        ByteView::from_path(&args[1]).unwrap()
    };

    let fat_obj = FatObject::parse(bv).unwrap();
    let objects = fat_obj.objects().unwrap();
    let cache = SymCache::from_object(&objects[0]).unwrap();

    println!("Cache file size: {}", cache.size());
}