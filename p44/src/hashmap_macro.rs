// In lib.rs create a public hash_map! macro which allows to create HashMap like this:
// let map = hash_map!(
// 42 => true, 64 => false, 128 => true,
// );
// Write an integration test fo the macro

#[macro_export]
// Source: src/macros.rs - vec!
macro_rules! hashmap {
    () => {
        HashMap::new();
    };
    ($($key:expr => $value:expr $(,)?)+) => {
        // Source: slides 4-3.pdf, slide number 8
        {
            let mut hashmap = HashMap::new();
            $(hashmap.insert($key, $value);)*
            hashmap
        }
    };
}

// #[macro_export]
// // Source: src/macros.rs - vec!
// macro_rules! hashmap_poorgeneric {
//     ( < $key_type:ty , $value_type:ty >)  => {
//         HashMap::<$key_type, $value_type>::new();
//     };
//     ( < $key_type:ty , $value_type:ty > , $($key:expr => $value:expr $(,)?)+)  => {
//         HashMap::<$key_type, $value_type>::from([$(($key, $value) ,)*]);
//     };
// }
