mod minimum_entropy_tests {
    use phraze::*;

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
