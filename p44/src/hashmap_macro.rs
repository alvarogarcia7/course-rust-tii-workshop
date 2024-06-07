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
    ($key:expr => $value:expr $(,)?) => {
        HashMap::<u64, bool>::from([($key, $value)]);
    }; // ($($x:expr),+ $(,)?) => (
       //     $crate::__rust_force_expr!(<[_]>::into_vec(
       //         // This rustc_box is not required, but it produces a dramatic improvement in compile
       //         // time when constructing arrays with many elements.
       //         #[rustc_box]
       //         $crate::boxed::Box::new([$($x),+])
       //     ))
       // );
}
