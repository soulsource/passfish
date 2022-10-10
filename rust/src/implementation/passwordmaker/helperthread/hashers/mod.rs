mod qt_hash;
use passwordmaker_rs::{Hasher, HasherList};
use ripemd::Digest;


pub(super) use qt_hash::{QtMd4, QtMd5, QtSha1, QtSha256};
pub(super) struct Ripemd160;

impl Hasher for Ripemd160{
    type Output = [u8;20];

    fn hash(input : &[u8]) -> Self::Output {
        let hash = ripemd::Ripemd160::digest(input);
        hash.into()
    }
}

pub(super) struct PassFishHashers;
impl HasherList for PassFishHashers {
    type MD4 = QtMd4;
    type MD5 = QtMd5;
    type SHA1 = QtSha1;
    type SHA256 = QtSha256;
    type RIPEMD160 = Ripemd160;
}

impl passwordmaker_rs::Md4 for QtMd4 {}
impl passwordmaker_rs::Md5 for QtMd5 {}
impl passwordmaker_rs::Sha1 for QtSha1 {}
impl passwordmaker_rs::Sha256 for QtSha256 {}
impl passwordmaker_rs::Ripemd160  for Ripemd160 {}
