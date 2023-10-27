use std::collections::HashSet;
use unicode_normalization::is_nfc_quick;
use unicode_normalization::is_nfd_quick;
use unicode_normalization::is_nfkc_quick;
use unicode_normalization::is_nfkd_quick;
use unicode_normalization::IsNormalized;

/// Given a slice of Strings, this function will attempt to detect the Unicode normalization used
/// in each String.
/// There are 4 different Unicode normalizations: NFC, NFD, NFKC, NFKD. Which ever one lists uses
/// isn't a concern. What IS a concern is if one list uses MORE THAN ONE normalization.
/// Thus, this functions counts how many DIFFERENT normalizations it finds. If it's more than 1
/// type, it returns false, since the list does not have what I call "uniform Unicdoe
/// normalization." Elsewhere, we warn the user about this.
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
        // If we've already found more than 1 normalization, we can skip the
        // rest of the list and return false
        if types_of_normalizations_discovered.len() > 1 {
            return false;
        }
    }
    types_of_normalizations_discovered.len() == 1
}

#[test]
fn can_detect_non_uniform_unicode_normalization_in_a_given_list() {
    let normalization_type_1 = "sécréter";
    let normalization_type_2 = "sécréter";
    let non_uniform_list = vec![
        normalization_type_1.to_string(),
        normalization_type_2.to_string(),
    ];
    assert!(!uniform_unicode_normalization(&non_uniform_list));

    let uniform_list = vec![
        "alpha".to_string(),
        "beta".to_string(),
        "charlie".to_string(),
    ];
    assert!(uniform_unicode_normalization(&uniform_list));

    let uniform_list2 = vec![
        "alpha".to_string(),
        "beta".to_string(),
        normalization_type_1.to_string(), // add one word with an accented character
        "charlie".to_string(),
        normalization_type_1.to_string(), // twice
    ];
    // Should still be detected as uniform
    assert!(uniform_unicode_normalization(&uniform_list2));
}
