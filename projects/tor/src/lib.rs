#[cfg(test)]
mod tests {
  use bitcoin::secp256k1::rand::prelude::thread_rng;
  use bitcoin::secp256k1::rand::Rng;
  use sha1::{Digest, Sha1};

  /// Refer to tor control spec for HashedControlPassword
  /// https://gitweb.torproject.org/torspec.git/tree/control-spec.txt
  #[test]
  fn test_tor_hashed_password() {
    let expected = "16:AA611955E35C4164600ECDB927F1319E62F208D1827EB93A10DFECB9B2".to_lowercase();

    let password = "testpasswordfortor";
    let salt = thread_rng().gen::<[u8; 8]>().to_vec();
    let salt_hex = hex::encode(salt);
    let to_hash = salt_hex.clone() + password;

    let mut hasher = Sha1::new();
    hasher.update(to_hash.as_bytes());
    let hashed_password = hasher.finalize();
    let mut hashed_password_hex = hex::encode(&hashed_password);
    hashed_password_hex.insert_str(0, &salt_hex);
    hashed_password_hex.insert_str(0, "16:");
    assert_eq!(hashed_password_hex, expected);
  }
}
