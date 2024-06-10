use p54::crypto::{decrypt, encrypt};

fn main() {
    let plaintext = "backendengineer.io".to_string();

    let key_str = "thiskeystrmustbe32charlongtowork".to_string();

    let encrypted_data = encrypt(key_str.clone(), plaintext);

    println!("encrypted_data: {:?}", encrypted_data.clone());

    let original = decrypt(key_str, encrypted_data);

    println!("original: {:?}", original);
}
