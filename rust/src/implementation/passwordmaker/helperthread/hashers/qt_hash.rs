use std::borrow::{BorrowMut, Borrow};
use passwordmaker_rs::Hasher;
use libc::size_t;

pub(crate) struct Md4;
pub(crate) struct Md5;
pub(crate) struct Sha1;
pub(crate) struct Sha256;

pub(crate) struct QHasher<T>(T);

pub(crate) type QtMd4 = QHasher<Md4>;
pub(crate) type QtMd5 = QHasher<Md5>;
pub(crate) type QtSha1 = QHasher<Sha1>;
pub(crate) type QtSha256 = QHasher<Sha256>;

impl<T> Hasher for QHasher<T> where T:QtHasher {
    type Output = T::Output;
    
    fn hash(input : &[u8]) -> Self::Output {
        let mut result = T::Output::default();
        let required_bytes = result.borrow().len();
        let computed_bytes = unsafe{ 
            pwm_qhash(
                T::QT_ALGO_NUMBER, 
                input.as_ptr(), 
                input.len() as size_t, 
                result.borrow_mut().as_mut_ptr(), 
                required_bytes)
        };
        assert_eq!(computed_bytes, required_bytes); //no point to forward this to caller. It's a code bug, plain and simple.
        result
    }
}

pub(crate) trait QtHasher{
    type Output : Default + BorrowMut<[u8]>;
    const QT_ALGO_NUMBER : size_t;
}

impl QtHasher for Md4 {
    type Output = [u8;16];
    const QT_ALGO_NUMBER : size_t = 0;
}

impl QtHasher for Md5 {
    type Output = [u8;16];
    const QT_ALGO_NUMBER : size_t = 1;
}

impl QtHasher for Sha1 {
    type Output = [u8;20];
    const QT_ALGO_NUMBER : size_t = 2;
}

impl QtHasher for Sha256 {
    type Output = [u8;32];
    const QT_ALGO_NUMBER : size_t = 4;
}

#[cfg(test)]
use rust_testhelper::pwm_qhash;
#[cfg(not(test))]
extern "C"{
    fn pwm_qhash(algorithm : size_t, input : *const u8, input_length : size_t, output : *mut u8, output_capacity : size_t) -> size_t;
} 

/// Those tests are testing the integration of Qt's QCryptographicHash function. They are functional tests, NOT unit tests.
#[cfg(test)]
mod qt_hash_tests{
    use super::*;
    fn get_simple_string_as_bytes() -> &'static [u8] {
        "I am a simple string and I like simple things. I like dancing in the rain, I like eating hamburgers, and I like you.".as_bytes()
    }
    fn get_complex_string_as_bytes() -> &'static [u8] {
        "I am a complex string and do complex stuff. I like 🕺 in the 🌧️. I like eating 🍔, and I ❤️ you.".as_bytes()
    }
    #[test]
    fn md4_simple_string_test(){
        let hash = QtMd4::hash(get_simple_string_as_bytes());
        let expected = vec![0x14, 0x1f, 0x1f, 0x1d, 0xef, 0x4a, 0x0d, 0x15, 0x1d, 0xb5, 0x5f, 0x7c, 0xb8, 0x96, 0xca, 0x99];
        assert_eq!(hash.borrow(), expected);
    }
    #[test]
    fn md4_complex_string_test(){
        let hash = QtMd4::hash(get_complex_string_as_bytes());
        let expected = vec![0xc1, 0xfb, 0xe3, 0x4d, 0x72, 0x75, 0xb3, 0xa5, 0x3a, 0xc3, 0x45, 0xcf, 0x90, 0x34, 0x81, 0xf7];
        assert_eq!(hash.borrow(), expected);
    }

    #[test]
    fn md5_simple_string_test(){
        let hash = QtMd5::hash(get_simple_string_as_bytes());
        let expected = vec![0x4b, 0x2e, 0x18, 0x22, 0x45, 0x43, 0xf3, 0x96, 0xee, 0x79, 0x53, 0x18, 0x90, 0x1b, 0xb9, 0x7f];
        assert_eq!(hash.borrow(), expected);
    }
    #[test]
    fn md5_complex_string_test(){
        let hash = QtMd5::hash(get_complex_string_as_bytes());
        let expected = vec![0x70, 0x31, 0x35, 0xd9, 0x38, 0x55, 0x1d, 0x2a, 0xae, 0xfa, 0xd9, 0x38, 0x07, 0x91, 0x11, 0xfe ];
        assert_eq!(hash.borrow(), expected);
    }

    #[test]
    fn sha1_simple_string_test(){
        let hash = QtSha1::hash(get_simple_string_as_bytes());
        let expected = vec![0xa1, 0x0a, 0x15, 0x18, 0x99, 0x29, 0x9d, 0xc7, 0xa6, 0x48, 0x36, 0x11, 0x44, 0xb3, 0x94, 0x09, 0x87, 0x3a, 0x39, 0xf3];
        assert_eq!(hash.borrow(), expected);
    }
    #[test]
    fn sha1_complex_string_test(){
        let hash = QtSha1::hash(get_complex_string_as_bytes());
        let expected = vec![0x30, 0x79, 0xcd, 0xbc, 0x66, 0x09, 0xad, 0x24, 0x99, 0x44, 0xe5, 0x52, 0x25, 0xdf, 0xb4, 0x68, 0xfd, 0x5f, 0xb9, 0x8f ];
        assert_eq!(hash.borrow(), expected);
    }

    #[test]
    fn sha256_simple_string_test(){
        let hash = QtSha256::hash(get_simple_string_as_bytes());
        let expected = vec![0xd4, 0xaf, 0x13, 0x6a, 0x87, 0x62, 0x12, 0xf1, 0x93, 0x7d, 0xd1, 0x71, 0xab, 0xa1, 0xfa, 0x3e, 0x3b, 0x8e, 0xc5, 0x68,
            0xed, 0x42, 0x46, 0x9d, 0xf0, 0x9b, 0xd0, 0xd8, 0xd8, 0x39, 0x09, 0x93];
        assert_eq!(hash.borrow(), expected);
    }
    #[test]
    fn sha256_complex_string_test(){
        let hash = QtSha256::hash(get_complex_string_as_bytes());
        let expected = vec![0x01, 0x3d, 0x93, 0x17, 0x45, 0x18, 0x29, 0x41, 0x6a, 0x09, 0xb5, 0x65, 0x1b, 0x81, 0x32, 0x88, 0xce, 0x83, 0xad, 0x92,
            0x04, 0x0f, 0x24, 0x13, 0x57, 0x8d, 0xd1, 0xa5, 0xe8, 0x3a, 0x73, 0xaa    ];
        assert_eq!(hash.borrow(), expected);
    }
}