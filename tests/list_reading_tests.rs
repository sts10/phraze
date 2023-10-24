mod minimum_entropy_tests {
    use phraze::*;

    #[test]
    fn can_read_in_lists_and_find_appropriate_number_of_words() {
        let list = make_list(List::Long);
        assert!(list.len() == 17576);

        let list = make_list(List::Qwerty);
        assert!(list.len() == 1296);

        let list = make_list(List::Alpha);
        assert!(list.len() == 1296);

        let list = make_list(List::Eff);
        assert!(list.len() == 7776);

        let list = make_list(List::Effshort);
        assert!(list.len() == 1296);

        let list = make_list(List::Mnemonicode);
        assert!(list.len() == 1633);
    }

    #[test]
    fn can_read_in_lists_without_any_blank_words() {
        let list = make_list(List::Medium);
        assert!(!list.contains(&""));
        assert!(!list.contains(&"\n"));
        assert!(list.contains(&"abbey"));

        let list = make_list(List::Long);
        assert!(!list.contains(&"\n"));
        assert!(!list.contains(&""));

        let list = make_list(List::Qwerty);
        assert!(!list.contains(&"\n"));
        assert!(!list.contains(&""));

        let list = make_list(List::Alpha);
        assert!(!list.contains(&"\n"));
        assert!(!list.contains(&""));

        let list = make_list(List::Eff);
        assert!(!list.contains(&"\n"));
        assert!(!list.contains(&""));

        let list = make_list(List::Effshort);
        assert!(!list.contains(&"\n"));
        assert!(!list.contains(&""));

        let list = make_list(List::Mnemonicode);
        assert!(!list.contains(&"\n"));
        assert!(!list.contains(&""));
    }
}
