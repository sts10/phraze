mod minimum_entropy_tests {
    use phraze::*;

    #[test]
    fn can_read_in_lists_and_find_appropriate_number_of_words() {
        let list = fetch_list(ListChoice::Medium);
        assert!(list.len() == 8192);

        let list = fetch_list(ListChoice::Long);
        assert!(list.len() == 17576);

        let list = fetch_list(ListChoice::Qwerty);
        assert!(list.len() == 1296);

        let list = fetch_list(ListChoice::Alpha);
        assert!(list.len() == 1296);

        let list = fetch_list(ListChoice::Eff);
        assert!(list.len() == 7776);

        let list = fetch_list(ListChoice::Effshort);
        assert!(list.len() == 1296);

        let list = fetch_list(ListChoice::Mnemonicode);
        assert!(list.len() == 1633);
    }

    #[test]
    fn can_read_in_lists_without_any_blank_words() {
        let list = fetch_list(ListChoice::Medium);
        assert!(!list.contains(&""));
        assert!(!list.contains(&"\n"));
        assert!(list.contains(&"abbey"));

        let list = fetch_list(ListChoice::Long);
        assert!(!list.contains(&"\n"));
        assert!(!list.contains(&""));

        let list = fetch_list(ListChoice::Qwerty);
        assert!(!list.contains(&"\n"));
        assert!(!list.contains(&""));

        let list = fetch_list(ListChoice::Alpha);
        assert!(!list.contains(&"\n"));
        assert!(!list.contains(&""));

        let list = fetch_list(ListChoice::Eff);
        assert!(!list.contains(&"\n"));
        assert!(!list.contains(&""));

        let list = fetch_list(ListChoice::Effshort);
        assert!(!list.contains(&"\n"));
        assert!(!list.contains(&""));

        let list = fetch_list(ListChoice::Mnemonicode);
        assert!(!list.contains(&"\n"));
        assert!(!list.contains(&""));
    }
}
