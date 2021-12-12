use rand::rngs::OsRng;
use rsa::{Hash, PaddingScheme, PublicKey, RsaPrivateKey, RsaPublicKey};
use sha2::Digest;

pub fn run() {
     let mut rng = OsRng;
     let bits = 512;
     let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
     let public_key = RsaPublicKey::from(&private_key);

     // Signature
     let message = b"Hello world!";
     let digest = sha2::Sha256::digest(message).to_vec();
     println!("message: {:?}", message);

     let signature = private_key
          .sign(
               PaddingScheme::new_pkcs1v15_sign(Some(Hash::SHA2_256)),
               &digest,
          )
          .expect("failed to signature");

     println!("Signature: {:?}", signature);

     // Verify
     public_key
          .verify(
               PaddingScheme::new_pkcs1v15_sign(Some(Hash::SHA2_256)),
               &digest,
               &signature,
          )
          .expect("failed to verify");
}
