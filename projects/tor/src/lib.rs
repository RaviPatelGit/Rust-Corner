#[cfg(test)]
mod tests {
  use bitcoin::secp256k1::rand::prelude::thread_rng;
  use bitcoin::secp256k1::rand::Rng;
  use sha1::{Digest, Sha1};
  use std::char;

  /// Refer to tor control spec for HashedControlPassword
  /// https://gitweb.torproject.org/torspec.git/tree/control-spec.txt
  #[test]
  fn test_tor_hashed_password() {
    // generate an even number that can be divided by count subsequent sections. (Thanks Roman)
    const EXPBIAS: u8 = 6;

    let expected = "16:AA611955E35C4164600ECDB927F1319E62F208D1827EB93A10DFECB9B2".to_lowercase();

    let password = "testpasswordfortor";

    let mut salt = thread_rng().gen::<[u8; 8]>().to_vec();
    println!("salt: {:?}", salt);
    salt.push(96); // indicator_unicode
    let c = salt[8] as u64;
    println!("indicator_char: {:?}", char::from_u32(c as u32));
    println!("indicator_unicode: {:?}", c);

    let count: u64 = (16 + (&15)) << ((c >> 4) + EXPBIAS as u64);

    println!("count: {:?}", count);

    let salt_hex = hex::encode(salt);
    println!("salt_hex: {:?}", salt_hex);

    let to_hash = salt_hex.clone() + password;
    println!("to_hash_hex: {:?}", to_hash);

    let mut hasher = Sha1::new();
    hasher.update(to_hash.as_bytes());
    let hashed_password = hasher.finalize();
    let mut hashed_password_hex = hex::encode(&hashed_password);
    hashed_password_hex.insert_str(0, &salt_hex);
    hashed_password_hex.insert_str(0, "16:");

    assert_eq!(hashed_password_hex, expected);
  }
}
