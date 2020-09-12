
#[derive(Clone, Debug)]
///The output of the parser
pub struct ParserOutput<A: Clone>(Option<(A, String)>);
///A parser
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

    /// Only returns a valid output if the function f returns true.
    ///
    /// Allows to check if the output matches a condition. So it is
    /// possible to extend an existing parser.
    ///
    /// * `f` function
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

/// A parser function which returns the first character of an
/// string
///
/// # Example
///
/// "Foo" -> Some('F', "oo")
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
