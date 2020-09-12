#[cfg(test)]
mod tests {

    use parser::*;

    #[derive(Clone, Debug, PartialEq)]
    pub enum Action {
        DoAction(String),
        Entry(String),
        Exit(String),
    }

    pub fn _action(input: &String, action: &'static str) -> ParserOutput<String> {
        clear_white_space(input)
            .and_then(&Parser::new(move |x| keyword(x, action)))
            .and_then(&Parser::new(clear_white_space))
            .and_then(&Parser::new(word))
    }

    pub fn do_action(input: &String) -> ParserOutput<Action> {
        _action(input, "do").map(|s| Action::DoAction(s.clone()))
    }

    pub fn entry_action(input: &String) -> ParserOutput<Action> {
        _action(input, "entry").map(|s| Action::Entry(s.clone()))
    }

    pub fn exit_action(input: &String) -> ParserOutput<Action> {
        _action(input, "exit").map(|s| Action::Exit(s.clone()))
    }

    pub fn action(input: &String) -> ParserOutput<Action> {
        Parser::new(&do_action)
            .or(Parser::new(&entry_action))
            .or(Parser::new(&exit_action))
            .parse(input)
    }

    #[test]
    fn test_item() {
        let item = Parser::new(item);
        assert_eq!(item.parse("Foo").unwrap(), ('F', "oo".to_string()));
        assert_eq!(item.parse("B").unwrap(), ('B', "".to_string()));
        assert_eq!(item.parse("").is_valid(), false)
    }

    #[test]
    fn test_is_numeric() {
        let numeric = Parser::new(is_numeric);
        assert_eq!(numeric.parse("").is_valid(), false);
        assert_eq!(numeric.parse("a").is_valid(), false);
        assert_eq!(numeric.parse("01").unwrap(), ('0', "1".to_string()));
    }

    #[test]
    fn test_word() {
        let word = Parser::new(word);
        assert_eq!(
            word.parse("Als ich").unwrap(),
            ("Als".to_string(), " ich".to_string())
        );
        assert_eq!(
            word.parse("Als1 ich").unwrap(),
            ("Als".to_string(), "1 ich".to_string())
        );
    }

    #[test]
    fn test_get_all() {
        let all = Parser::new(|s: &String| get_all(s, &Parser::new(|str| is_numeric(str))));

        assert_eq!(all.parse("01f").unwrap(), (vec!('0', '1'), "f".to_string()));
    }

    #[test]
    fn test_at_least_one() {
        let one_number = Parser::new(|s| at_least_one(s, &Parser::new(is_numeric)));
        assert_eq!(one_number.parse("abc").is_valid(), false);
        assert_eq!(
            one_number.parse("12f").unwrap(),
            (vec!('1', '2'), "f".to_string())
        );
    }

    #[test]
    fn test_clear_whitespace() {
        let clear_ws = Parser::new(clear_white_space);
        assert_eq!(clear_ws.parse("  aba").unwrap(), ((), "aba".to_string()));
    }

    #[test]
    fn test_keyword() {
        let kw = Parser::new(|s| keyword(s, "foo"));
        assert_eq!(kw.parse("foo bar").unwrap(), ((), " bar".to_string()));
    }

    #[test]
    fn test_action() {
        let action = Parser::new(action);
        assert_eq!(
            action.parse(" entry Namear bla").unwrap(),
            (Action::Entry("Namear".to_string()), " bla".to_string())
        );
    }
}
