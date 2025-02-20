mod minimum_entropy_tests {
    use phraze::*;

    #[test]
    fn can_accurately_calculate_the_number_of_words_to_put_in_a_passphrase_given_a_desired_number_of_words()
     {
        assert_eq!(calculate_number_words_needed(Some(8), None, 0, 4000), 8);
    }

    #[test]
    fn can_accurately_calculate_the_number_of_words_to_put_in_a_passphrase_given_a_strength_count()
    {
        // 100 / 13 == a little over 7, so need 8 words to satisfy
        assert_eq!(calculate_number_words_needed(None, None, 1, 8192), 8);
        // 120 / 13 == a little over 9, so need 10 words to satisfy
        assert_eq!(calculate_number_words_needed(None, None, 2, 8192), 10);
    }

    #[test]
    fn can_accurately_calculate_the_number_of_words_to_put_in_a_passphrase_given_a_desired_minimum_entropy()
     {
        assert_eq!(calculate_number_words_needed(None, Some(102), 0, 8192), 8);
        assert_eq!(calculate_number_words_needed(None, Some(106), 0, 8192), 9);
    }

    #[test]
    fn can_calculate_number_of_words_to_use_given_minimum_entropy() {
        // With a list_length of 8192 (2^13), each word
        // adds exactly 13 bits of entropy to a passphrase
        let list_length = 8192;
        // So a 4-word passphrase gives 52 bits of entropy (13*4)
        // Thus, if user asks for a minimum of 51 bits of entropy
        let desired_minimum_entropy = 51;

        // Phrase should calculate that user needs 4 words from
        // this hypothetical list
        assert_eq!(
            convert_minimum_entropy_to_number_of_words(desired_minimum_entropy, list_length),
            4
        );
    }
}
