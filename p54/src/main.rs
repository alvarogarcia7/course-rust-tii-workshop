use p54::crypto::{decrypt, encrypt};

fn main() {
    let p_s = "backendengineer.io AND MORE DATA IS ALSO POSSIBLE";
    let plaintext_string = p_s.as_bytes();
    let plaintext = plaintext_string.to_vec();

    let key_as_vec = "thiskeystrmustbe32charlongtowork".as_bytes();
    assert_eq!(key_as_vec.len(), 32);
    let key: [u8; 32] = <[u8; 32]>::try_from(key_as_vec).unwrap();

    let encrypted_data = encrypt(&key, plaintext);

    let hex_encoded_encrypted_data = hex::encode(&encrypted_data).clone();
    println!(
        "encrypted_data: {:?}, len = {:?}",
        hex_encoded_encrypted_data,
        hex_encoded_encrypted_data.len()
    );

    let decrypted_plaintext = decrypt(&key, encrypted_data);

    let as_string = String::from_utf8(decrypted_plaintext.clone())
        .expect("failed to convert vector of bytes to string");
    println!("original: {:?}", as_string);

    assert_eq!(decrypted_plaintext, plaintext_string);
    println!("The original and decrypted values match.")
}
