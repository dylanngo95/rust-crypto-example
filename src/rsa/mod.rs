use rand::rngs::OsRng;
use rsa::{PaddingScheme, PublicKey, RsaPrivateKey, RsaPublicKey};

pub fn run() {
     let mut rng = OsRng;
     let bits = 512;
     let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
     let public_key = RsaPublicKey::from(&private_key);

     // Encrypt
     let plain_text = b"Hello world!";
     let cipher_text = public_key
          .encrypt(
               &mut rng,
               PaddingScheme::new_pkcs1v15_encrypt(),
               &plain_text[..],
          )
          .expect("failed to encrypt");
     println!("Cipher Text: {:?}", cipher_text);

     // Decrypt
     let decrypted_text = private_key
          .decrypt(PaddingScheme::new_pkcs1v15_encrypt(), &cipher_text)
          .expect("failed to decrypt");

     println!("Plain text: {}", String::from_utf8_lossy(&decrypted_text));
}
