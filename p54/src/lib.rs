// – fn expand_key(key: &[u8; 16]) -> [__m128i; 11]: expands 128-bit key to round keys
// – fn encrypt1(keys: &[__m128i; 11], block: &mut [u8; 16]): encrypts one 128-bit block
// – fn decrypt1(keys: &[__m128i; 11], block: &mut [u8; 16]): decrypts one 128-bit block
// – fn encrypt8(keys: &[__m128i; 11], block: &mut [u8; 128]): encrypts eight 128-bit block – fn decrypt8(keys: &[__m128i; 11], block: &mut [u8; 128]): decrypts eight 128-bit block

// #[target_feature(enable = "neon")]
// #[target_feature(enable = "aes")]
// fn expand_key(key: &[u8; 16]) -> [__m128i; 11] {}

pub mod crypto;
