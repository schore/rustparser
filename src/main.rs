pub mod parser {

    #[derive(Clone, Debug)]
    pub struct ParserOutput<A: Clone>(Option<(A, String)>);
    pub struct Parser<A: Clone>(Box<dyn Fn(&String) -> ParserOutput<A>>);

    impl<A: Clone> ParserOutput<A> {
        pub fn only_if<F>(&self, f: F) -> ParserOutput<A>
        where
            F: FnOnce(&A) -> bool,
        {
            match &self.0 {
                Some((value, s)) => {
                    if f(&value) {
                        return ParserOutput(Some((value.clone(), s.clone())));
                    }
                    return ParserOutput(None);
                }
                None => return ParserOutput(None),
            }
        }

        pub fn map<B: Clone, F>(&self, f: F) -> ParserOutput<B>
        where
            F: Fn(&A) -> B,
        {
            match &self.0 {
                Some((v, s)) => {
                    return ParserOutput(Some((f(v), s.clone())));
                }
                None => return ParserOutput(None),
            }
        }

        pub fn and_then<B: Clone>(&self, parser: &Parser<B>) -> ParserOutput<B> {
            match &self.0 {
                Some((_, s)) => parser.parse(s),
                None => ParserOutput(None),
            }
        }

        pub fn unwrap(self) -> (A, String) {
            self.0.unwrap()
        }

        pub fn is_valid(&self) -> bool {
            self.0.is_some()
        }
    }

    impl<A: Clone> Parser<A> {
        pub fn new<F>(f: F) -> Parser<A>
        where
            F: 'static + Fn(&String) -> ParserOutput<A>,
        {
            Parser(Box::new(f))
        }

        pub fn parse<S: Into<String>>(&self, input: S) -> ParserOutput<A> {
            (self.0)(&input.into())
        }

        pub fn or(self, p2: Parser<A>) -> Parser<A>
        where
            A: 'static,
        {
            Parser::new(move |input: &String| {
                let result = self.parse(input);
                if result.0.is_some() {
                    return result;
                }
                return p2.parse(input);
            })
        }

        pub fn map<B: Clone, F>(self, f: F) -> Parser<B>
        where
            F: 'static + Fn(&A) -> B,
            A: 'static,
        {
            Parser::new(move |input: &String| self.parse(input).map(&f))
        }

        pub fn only_if<F>(self, f: F) -> Parser<A>
        where
            F: 'static + Fn(&A) -> bool,
            A: 'static,
        {
            Parser::new(move |input: &String| {
                let result = self.parse(input);
                if result.0.is_some() && f(&result.0.unwrap().0) {
                    return self.parse(input);
                }
                return ParserOutput(None);
            })
        }
    }

    pub fn item(input: &String) -> ParserOutput<char> {
        let item = input.chars().next();
        let rest = input.get(1..);

        if item.and(rest).is_some() {
            return ParserOutput(Some((
                item.unwrap_or_default(),
                rest.unwrap_or_default().to_string(),
            )));
        }

        ParserOutput(None)
    }

    pub fn is_numeric(input: &String) -> ParserOutput<char> {
        item(input).only_if(|c| c.is_numeric())
    }

    pub fn is_alphabetic<'a>(input: &String) -> ParserOutput<char> {
        item(input).only_if(|c| c.is_alphabetic())
    }

    pub fn get_all<A: Clone>(input: &String, parser: &Parser<A>) -> ParserOutput<Vec<A>> {
        let mut ret: Vec<A> = Vec::new();
        let mut rest = input.clone();
        loop {
            match parser.parse(&rest).0 {
                Some((value, s)) => {
                    ret.push(value);
                    rest = s;
                }
                None => break,
            }
        }
        ParserOutput(Some((ret, rest)))
    }

    pub fn at_least_one<A: Clone>(input: &String, parser: &Parser<A>) -> ParserOutput<Vec<A>> {
        get_all(input, parser).only_if(|v| v.len() > 0)
    }

    pub fn word(input: &String) -> ParserOutput<String> {
        at_least_one(input, &Parser::new(is_alphabetic)).map(|r| r.into_iter().collect())
    }

    pub fn white_space(input: &String) -> ParserOutput<char> {
        item(input).only_if(|c| c.is_whitespace())
    }

    pub fn clear_white_space(input: &String) -> ParserOutput<()> {
        get_all(input, &Parser::new(white_space)).map(|_| ())
    }

    pub fn keyword(input: &String, keyword: &str) -> ParserOutput<()> {
        word(input).only_if(|w| w == keyword).map(|_| ())
    }

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
        assert_eq!(action.parse(" entry Namear bla").unwrap(),
                   (Action::Entry("Namear".to_string()), " bla".to_string()));
    }
}
