mod minimum_entropy_tests {
    use phraze::*;

    #[test]
    fn can_calculate_number_of_words_to_use_given_minimum_entropy() {
        // With a list_length of 8192 (2^13), each word
        // adds exactly 13 bits of entropy to a passphrase
        let list_length = 8192;
        // So a 4-word passphrase gives 52 bits of entropy (13*4)
        // Thus, if user asks for a minimum of 51 bits of entropy
        let desired_minimum_entropy = Some(51);

        // Phrase should calculate that user needs 4 words from
        // this hypothetical list
        assert_eq!(
            calculate_number_words_needed(None, desired_minimum_entropy, list_length),
            4
        );
    }
}
