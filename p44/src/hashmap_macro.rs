// In lib.rs create a public hash_map! macro which allows to create HashMap like this:
// let map = hash_map!(
// 42 => true, 64 => false, 128 => true,
// );
// Write an integration test fo the macro

#[macro_export]
// Source: src/macros.rs
macro_rules! hashmap {
    () => {
        HashMap::<u64, u64>::new();
    }; // ($elem:expr; $n:expr) => (
       //     $crate::__rust_force_expr!($crate::vec::from_elem($elem, $n))
       // );
       // ($($x:expr),+ $(,)?) => (
       //     $crate::__rust_force_expr!(<[_]>::into_vec(
       //         // This rustc_box is not required, but it produces a dramatic improvement in compile
       //         // time when constructing arrays with many elements.
       //         #[rustc_box]
       //         $crate::boxed::Box::new([$($x),+])
       //     ))
       // );
}
