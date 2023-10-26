use std::collections::HashSet;
use unicode_normalization::is_nfc_quick;
use unicode_normalization::is_nfd_quick;
use unicode_normalization::is_nfkc_quick;
use unicode_normalization::is_nfkd_quick;
use unicode_normalization::IsNormalized;

pub fn uniform_unicode_normalization(list: &[String]) -> bool {
    let mut types_of_normalizations_discovered = HashSet::new();
    for word in list {
        if is_nfc_quick(word.chars()) == IsNormalized::Yes {
            types_of_normalizations_discovered.insert("NFC");
        } else if is_nfd_quick(word.chars()) == IsNormalized::Yes {
            types_of_normalizations_discovered.insert("NFD");
        } else if is_nfkc_quick(word.chars()) == IsNormalized::Yes {
            types_of_normalizations_discovered.insert("NFKC");
        } else if is_nfkd_quick(word.chars()) == IsNormalized::Yes {
            types_of_normalizations_discovered.insert("NFKD");
        }
        if types_of_normalizations_discovered.len() > 1 {
            return false;
        }
    }
    types_of_normalizations_discovered.len() == 1
}
