/// This whole module may look dumb, but that might change, once the public interface of passwordmaker_rs starts to deviate from the saved data.

use passwordmaker_rs::{HashAlgorithm,UseLeetWhenGenerating,LeetLevel};

pub(super) fn convert_hash_algorithm(stored_algo : &crate::implementation::profiles::HashAlgorithm) -> HashAlgorithm {
    use crate::implementation::profiles::HashAlgorithm as PHashAlgorithm;
    match stored_algo {
        PHashAlgorithm::Md4 => HashAlgorithm::Md4,
        PHashAlgorithm::HmacMd4 => HashAlgorithm::HmacMd4,
        PHashAlgorithm::Md5 => HashAlgorithm::Md5,
        PHashAlgorithm::Md5Version06 => HashAlgorithm::Md5Version06,
        PHashAlgorithm::HmacMd5 => HashAlgorithm::HmacMd5,
        PHashAlgorithm::HmacMd5Version06 => HashAlgorithm::HmacMd5Version06,
        PHashAlgorithm::Sha1 => HashAlgorithm::Sha1,
        PHashAlgorithm::HmacSha1 => HashAlgorithm::HmacSha1,
        PHashAlgorithm::Sha256 => HashAlgorithm::Sha256,
        PHashAlgorithm::HmacSha256 => HashAlgorithm::HmacSha256,
        PHashAlgorithm::Ripemd160 => HashAlgorithm::Ripemd160,
        PHashAlgorithm::HmacRipemd160 => HashAlgorithm::HmacRipemd160,
    }
}

pub(super) fn convert_leet(stored_leet : &crate::implementation::profiles::UseLeetWhenGenerating) -> UseLeetWhenGenerating {
    use crate::implementation::profiles::UseLeetWhenGenerating as PUseLeetWhenGenerating;
    match stored_leet {
        PUseLeetWhenGenerating::NotAtAll => UseLeetWhenGenerating::NotAtAll,
        PUseLeetWhenGenerating::Before { level } => UseLeetWhenGenerating::Before { level: convert_leet_level(level) },
        PUseLeetWhenGenerating::After { level } => UseLeetWhenGenerating::After { level: convert_leet_level(level) },
        PUseLeetWhenGenerating::BeforeAndAfter { level } => UseLeetWhenGenerating::BeforeAndAfter { level: convert_leet_level(level) },
    }
}

fn convert_leet_level(stored_level : &crate::implementation::profiles::LeetLevel) -> LeetLevel {
    use crate::implementation::profiles::LeetLevel as PLeetLevel;
    match stored_level {
        PLeetLevel::One => LeetLevel::One,
        PLeetLevel::Two => LeetLevel::Two,
        PLeetLevel::Three => LeetLevel::Three,
        PLeetLevel::Four => LeetLevel::Four,
        PLeetLevel::Five => LeetLevel::Five,
        PLeetLevel::Six => LeetLevel::Six,
        PLeetLevel::Seven => LeetLevel::Seven,
        PLeetLevel::Eight => LeetLevel::Eight,
        PLeetLevel::Nine => LeetLevel::Nine,
    }
}