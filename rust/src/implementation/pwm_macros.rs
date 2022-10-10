extern crate passwordmaker_macros;
pub use self::passwordmaker_macros::*;

pub trait EnumVariantCount {
    fn variant_count() -> usize;
}

#[cfg(test)]
mod pwm_macro_tests {
    use super::*;

    #[allow(dead_code)]
    enum Nest{ A, B }
    #[allow(dead_code)]
    #[derive(EnumVariantCount)]
    enum TestNum {
        A(usize),
        B,
        C(Nest),
        D,
        E{a : usize, b: f64}
    }
    #[test]
    fn enum_variant_count_test(){
        assert_eq!(TestNum::variant_count(), 5);
    }
}