#[cfg(test)]

mod tests {
    use std::str::FromStr;
    use bitcoin_hashes::Hash;
    use bitcoin_hashes::sha1;

    #[test]
    fn play() {
        let pre_image = b"sushi";
        let expected_sha1 = sha1::Hash::from_str("451dcf9913bf3b329c05c2a46ad555eeae267ba8").unwrap();
        let hash = sha1::Hash::hash(pre_image);
        assert_eq!(hash, expected_sha1)
    }
}
