// In lib.rs create a public hash_map! macro which allows to create HashMap like this:
// let map = hash_map!(
// 42 => true, 64 => false, 128 => true,
// );
// Write an integration test fo the macro

#[macro_export]
// Source: src/macros.rs - vec!
macro_rules! hashmap {
    () => {
        HashMap::<u64, bool>::new();
    };
    ($($key:expr => $value:expr $(,)?)+) => {
        HashMap::<u64, bool>::from([$(($key, $value) ,)*]);
    };
}
