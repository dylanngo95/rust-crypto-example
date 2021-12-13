use sha2::{Digest, Sha256};
use std::{fs, io};

pub fn run() -> Vec<u8> {
     let mut file = fs::File::open("./src/hash/index.html").unwrap();
     let mut hasher = Sha256::new();
     let n = io::copy(&mut file, &mut hasher);
     let result = hasher.finalize();

     println!("Bytes processed: {:?}", n);
     println!("Hash: {:x}", result);

     result.to_vec()
}
