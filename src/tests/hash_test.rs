use algorithm::hash;
use hex_literal::hex;

#[test]
fn test_run() -> Result<(), String> {
     assert_eq!(
          hash::run()[..],
          hex!("697929f9ce49b2c6136db1deb9b452754552cf79df29c29de0db6e705942dedc")
     );
     Ok(())
}
