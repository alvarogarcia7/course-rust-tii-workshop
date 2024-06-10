// Implement in lib.rs a simplified BigUint4096 type, i.e. unsigned 4096 bit integer
// ● Under the hood use array of u64s
// ● Browse the standard library docs for potentially useful methods:
// https://doc.rust-lang.org/stable/std/primitive.u64.html
// ● Implement basic arithmetic traits
// ● Implement conversion methods from hex-encoded &str and to hex-
// encoded String (be careful about endianness)
// ● Add integration tests for the type

/// BigUint64
/// Note: the Least Significant Bit (LSB) is stored on position 0
#[derive(Debug, PartialEq)]
pub struct BigUintGeneric<const SIZE: usize> {
    value: [u64; SIZE],
}

// Source: https://doc.rust-lang.org/reference/items/generics.html
// // Used as a field of a struct.
// struct Foo<const N: usize>([i32; N]);
//
// impl<const N: usize> Foo<N> {
//     // Used as an associated constant.
//     const CONST: usize = N * 4;
// }

/// Bad implementation of a BigUint4096
/// Warning: There might be defects. Do not use for production, this is only a sandbox where to test the implementation.
impl<const N: usize> BigUintGeneric<N> {
    pub fn from(initial_value: [u64; N]) -> Self {
        Self {
            value: initial_value,
        }
    }

    pub fn new() -> Self {
        Self::from([0; N])
    }
    pub fn sum(&mut self, another: &Self) {
        let mut carry = 0;
        for i in 0..self.value.len() {
            carry = self.sum_one_limb(i, another, carry)
        }
        if carry != 0 {
            panic!("You are overflowing the structure")
        }
    }

    pub fn multiply(&mut self, another: &BigUintGeneric<N>) {
        let mut carry = 0;
        for i in 0..self.value.len() {
            let [x, carry1] = Self::multiply_two_numbers(self.value[i], another.value[i]);
            let (y, carry2) = Self::sum_two_numbers(x, carry);

            self.value[i] = y;

            carry = carry2 + carry1
        }
        if carry != 0 {
            panic!("You are overflowing the structure")
        }
    }

    fn sum_two_numbers(a: u64, b: u64) -> (u64, u64) {
        match a.checked_add(b) {
            None => {
                // 7 + 4 = 11
                // MAX = 9 (one digit)
                // 11 % 10 = 1

                // MAX - 7 = 2
                // 4 - 2 - 1 = 1 (one digit, no overflows)
                let x = u64::MAX - a;
                let y = b - x - 1;
                (y, 1u64)
            }
            Some(non_overflow) => (non_overflow, 0),
        }
    }
    fn sum_one_limb(&mut self, i: usize, another: &Self, carry: u64) -> u64 {
        let (x, carry1) = Self::sum_two_numbers(self.value[i], another.value[i]);
        let (y, carry2) = Self::sum_two_numbers(x, carry);

        self.value[i] = y;

        carry2 + carry1
    }

    const HIGHER: u64 = 0xFFFFFFFF00000000;
    const LOWER: u64 = 0x000000000FFFFFFFF;
    fn multiply_two_numbers(op1: u64, op2: u64) -> [u64; 2] {
        //  const HIGHER: u64 = 0xFFFFFFFF00000000;
        //  const LOWER: u64 = 0x000000000FFFFFFFF;
        //  let c_32 = (a & HIGHER) >> 32;
        //  let d_32 = a & LOWER;
        //
        //  let e_32 = (b & HIGHER) >> 32;
        //  let f_32 = b & LOWER;
        //
        //  let fd_64 = f_32 * d_32;
        //  let fc_64 = f_32 * c_32;
        //  let ed_64 = e_32 * d_32;
        //  let ec_64 = e_32 * c_32;
        //
        //  let r1_64 = fd_64;
        //  let (r2_64, c2_1) = Self::sum_two_numbers(fc_64, ed_64);
        //  let (r3_64, r4_1) = Self::sum_two_numbers(ec_64, c2_1);
        //
        //  // // let (t, u) = Self::sum_two_numbers(c * e, e * f);
        //  // // let (t2, u2) = Self::sum_two_numbers(c * f, e * d);
        //  // println!("INT 0x{r1_32:8X}{r2_32:8X}");
        //  let r_1 = ((r2_64 & LOWER) << 32) + (r1_64 & LOWER);
        //  let r_2 = ((r4_1 & LOWER) << 32) + (r3_64 & LOWER);
        //
        //  // println!("RES 0x{r_1:8X}-{r_2:8X}");
        //  [r_1, r_2]
        //  // [r1, r2, r3, r4]
        //         thread 'integration::multiply_one_limb' panicked at p34/tests/integration.rs:82:9:
        // assertion `left == right` failed
        //   left: BigUint4096 { value: [8589934593, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] }
        //  right: BigUint4096 { value: [18446744073709551614, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] }

        let a_32 = (op1 & Self::HIGHER) >> 32;
        let b_32 = op1 & Self::LOWER;

        let c_32 = (op2 & Self::HIGHER) >> 32;
        let d_32 = op2 & Self::LOWER;

        let bd_64 = b_32 * d_32;
        let ad_64 = a_32 * d_32;
        let bc_64 = b_32 * c_32;
        let ac_64 = a_32 * c_32;

        let [bd_l_32, bd_h_32] = Self::split(bd_64);
        let [ad_l_32, ad_h_32] = Self::split(ad_64);
        let [bc_l_32, bc_h_32] = Self::split(bc_64);
        let [ac_l_32, ac_h_32] = Self::split(ac_64);

        let chunk_1 = bd_l_32;
        // let r_2_64: u64 = (bd_h_32 + ad_l_32 + bc_l_32).try_into().unwrap();
        let chunk_2_64 = (<u32 as Into<u64>>::into(bd_h_32))
            + (<u32 as Into<u64>>::into(ad_l_32))
            + (<u32 as Into<u64>>::into(bc_l_32));
        let [chunk_2, c1] = Self::split(chunk_2_64);

        let chunk_3_64: u64 = (<u32 as Into<u64>>::into(ad_h_32))
            + (<u32 as Into<u64>>::into(bc_h_32))
            + (<u32 as Into<u64>>::into(ac_l_32))
            + (<u32 as Into<u64>>::into(c1));
        // let r_3_64: u64 = (ad_h_32 + bc_h_32 + ac_l_32 + c1).try_into().unwrap();
        let [chunk_3, c2] = Self::split(chunk_3_64);

        let chunk_4 = ac_h_32 + c2;

        let result_1: u64 =
            (<u32 as Into<u64>>::into(chunk_2) << 32) + <u32 as Into<u64>>::into(chunk_1);
        let result_2: u64 =
            (<u32 as Into<u64>>::into(chunk_4) << 32) + <u32 as Into<u64>>::into(chunk_3);
        // let r__2: u64 = ((r_4 << 32) + r_3).try_into().unwrap();

        [result_1, result_2]
    }

    fn split(ab: u64) -> [u32; 2] {
        let low: u32 = (ab & Self::LOWER).try_into().unwrap();
        let high: u32 = ((ab & Self::HIGHER) >> 32).try_into().unwrap();

        [low, high]
    }
}

impl<const N: usize> Default for BigUintGeneric<N> {
    fn default() -> Self {
        Self::new()
    }
}

#[macro_export]
macro_rules! build_biguint {
    () => {
        BigUintGeneric::new();
    };
    // Define the first few values, everything else is zero
    ($($value:expr $(,)?)+) => {
        {
            // let mut v: Vec<u64> = vec![];
            // $(
            // v.push($value);
            // )*
            // // TODO Question: by letting the `let variable` define the type, how may I get the length of the needed array?. How do I know if it's 64 or 92, etc?
            // BigUintGeneric::from(v)
            // TODO: Defect: this macro can only generate up to BigUintGeneric::<64>
            let mut v = [0;64];
            let mut i = 0;
            $(
            v[i] = $value;
            i += 1;
            )*
            BigUintGeneric::from(v)
        }
    };
    // AGB: Don't do this because we allow two ways of creating from an array - NO
    // AGB: Don't do this because we force the user to specify 1u64 as the literal - NO
    // Define all values using the array syntax
    // ([$value:expr; $len:expr]) => {
    //     {
    //         BigUint4096::from([$value; $len])
    //     }
    // };
}
