fn main() {
    println!("Hello, world!");
}

mod battista {
    const ALPHABET: [u8; 26] = *b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const A: u8 = b'A';
    const Z: u8 = b'Z';
    const WRAP: u8 = 26;

    fn sanitize(raw: &str) -> impl Iterator<Item = u8> + '_ {
        raw.bytes().filter_map(|char| match char {
            A..=Z => Some(char),
            b'a'..=b'z' => Some(char - (b'a' - A)),
            _ => None,
        })
    }

    pub fn decrypt(ciphered_message: &str, key: &str) -> String {
        let mut key_chars = key.bytes().map(|key_char| key_char - A).cycle();
        let message = sanitize(ciphered_message)
            .map(|char| {
                let key_char = key_chars.next().unwrap();
                ((char + WRAP - A) - key_char) % WRAP + A
            })
            .collect();
        String::from_utf8(message).unwrap()
    }
}

#[test]
fn successful_decryption() {
    let key = "WHYRUST";
    let ciphered_message = "PVCDJG PAYCMY JR KUC";

    let clear_message = battista::decrypt(ciphered_message, key);
    assert_eq!(clear_message, "TOEMPOWEREVERYONE")
}
