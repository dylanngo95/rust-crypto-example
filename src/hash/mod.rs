use sha2::{Digest, Sha256};
use std::{fs, io};

pub fn run() {
     let mut file = fs::File::open("./src/hash/index.html").unwrap();

     // create a Sha256 object
     let mut hasher = Sha256::new();

     let n = io::copy(&mut file, &mut hasher);

     // read hash digest and consume hasher
     let result = hasher.finalize();

     println!("Bytes processed: {:?}", n);

     println!("Hash: {:x}", result);
}
