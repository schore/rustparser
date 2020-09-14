
#[derive(Clone, Debug)]
///The output of the parser
pub struct ParserOutput<A: Clone>(pub Result<(A, String), String>);

///A parser
pub struct Parser<A: Clone>(pub Box<dyn Fn(&String) -> ParserOutput<A>>);


impl<A: Clone> ParserOutput<A> {
    pub fn only_if_message<F>(&self, f: F, message: String) -> ParserOutput<A>
    where
        F: FnOnce(&A) -> bool,
    {
        match &self.0 {
            Ok((value, s)) => {
                if f(&value) {
                    return ParserOutput(Ok((value.clone(), s.clone())));
                }
                return ParserOutput(Err(message.to_string()));
            }
            Err(s) => return ParserOutput(Err(s.clone())),
        }
    }

    pub fn only_if<F>(&self, f: F) -> ParserOutput<A>
    where
        F: FnOnce(&A) -> bool,
    {
        self.only_if_message(f, "If not matched".to_string())
    }



    pub fn map<B: Clone, F>(&self, f: F) -> ParserOutput<B>
    where
        F: Fn(&A) -> B,
    {
        match &self.0 {
            Ok((v, s)) => {
                return ParserOutput(Ok((f(v), s.clone())));
            }
            Err(s) => return ParserOutput(Err(s.clone())),
        }
    }


    pub fn and_then<B: Clone>(&self, parser: &Parser<B>) -> ParserOutput<B> {
        match &self.0 {
            Ok((_, s)) => parser.parse(s),
            Err(s) => ParserOutput(Err(s.clone())),
        }
    }

    pub fn unwrap(&self) -> (A, String) {
        self.0.clone().unwrap()
    }

    pub fn is_valid(&self) -> bool {
        self.0.is_ok()
    }

    pub fn set_error(&self, message :String) -> ParserOutput<A>{
        match self.0 {
            Ok(_) =>self.clone(),
            Err(_) => ParserOutput(Err(message))
        }
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
            if result.is_valid() {
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
            if result.is_valid() && f(&result.0.unwrap().0) {
                return self.parse(input);
            }
            return ParserOutput(Err("if not matched".to_string()));
        })
    }

    pub fn and_then<B: Clone>(self, p2 :Parser<B> ) -> Parser<B>
    where
        A: 'static,
        B: 'static
    {
        Parser::new(move |input: &String| {
            self.parse(input)
                .and_then(&p2)
        })
    }

    pub fn all(self) -> Parser<Vec<A>>
    where
        A: 'static

    {
        Parser::new(move |s :&String| get_all(s, &self))
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
        return ParserOutput(Ok((
            item.unwrap_or_default(),
            rest.unwrap_or_default().to_string(),
        )));
    }

    ParserOutput(Err("Nothing left to parse".to_string()))
}

pub fn is_numeric(input: &String) -> ParserOutput<char> {
    item(input).only_if_message(|c| c.is_numeric(), "Expecting numeric".to_string())
}

pub fn is_alphabetic<'a>(input: &String) -> ParserOutput<char> {
    item(input).only_if_message(|c| c.is_alphabetic(), "Expecting alphabetic".to_string())
}

pub fn get_all<A: Clone>(input: &String, parser: &Parser<A>) -> ParserOutput<Vec<A>> {
    let mut ret: Vec<A> = Vec::new();
    let mut rest = input.clone();
    loop {
        match parser.parse(&rest).0 {
            Ok((value, s)) => {
                ret.push(value);
                rest = s;
            }
            Err(_) => break,
        }
    }
    ParserOutput(Ok((ret, rest)))
}

pub fn at_least_one<A: Clone>(input: &String, parser: &Parser<A>) -> ParserOutput<Vec<A>> {
    get_all(input, parser).only_if_message(|v| v.len() > 0, "Expecting something".to_string())
}

pub fn word(input: &String) -> ParserOutput<String> {
    at_least_one(input, &Parser::new(is_alphabetic))
        .map(|r| r.into_iter().collect())
}

pub fn white_space(input: &String) -> ParserOutput<char> {
    item(input).only_if_message(|c| c.is_whitespace(), "Expecting whitespace".to_string())
}

pub fn clear_white_space(input: &String) -> ParserOutput<()> {
    get_all(input, &Parser::new(white_space)).map(|_| ())
}

pub fn keyword(input: &String, keyword: &str) -> ParserOutput<()> {
    word(input).only_if(|w| w == keyword)
        .set_error(format!("Expecting {}", keyword))
        .map(|_| ())
}

pub fn special_char (input: &String, c :char) -> ParserOutput<()> {
    item(input).only_if(|i| i == &c)
        .set_error(format!("Expecting {}", c))
        .map(|_| ())
}
