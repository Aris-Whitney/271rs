#[macro_export]
macro_rules! choice {
    ( $x:expr, $y:expr, $z:expr ) => {
        (($x) & ($y)) ^ (!(($x)) & ($z))
    };
}

#[macro_export]
macro_rules! median {
    ( $x:expr, $y:expr, $z:expr ) => {
        (($x) & ($y)) | (($x) & ($z)) | (($y) & ($z))
    };
}

#[macro_export]
macro_rules! rotate {
    ( $val:expr, $shift:expr ) => {{
        const BITS: u32 = 64;
        let shift_amt = ($shift % BITS) as u32;
        (($val >> shift_amt) | ($val << (BITS - shift_amt))) & 0xFFFFFFFFFFFFFFFF
    }};
}

