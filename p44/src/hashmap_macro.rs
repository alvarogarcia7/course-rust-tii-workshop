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

#[macro_export]
// Source: src/macros.rs - vec!
macro_rules! hashmap_poorgeneric {
    ( $(<)+ $key_type:ty $(,)+ $value_type:ty $(>)+)  => {
        HashMap::<$key_type, $value_type>::new();
    };
    ( $(<)+ $key_type:ty $(,)+ $value_type:ty $(>)+ $(,)+ $($key:expr => $value:expr $(,)?)+)  => {
        HashMap::<$key_type, $value_type>::from([$(($key, $value) ,)*]);
    };
}
