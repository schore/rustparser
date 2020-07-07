#[allow(dead_code)]
#[derive(Clone, Debug)]
struct ParserOutput<A: Clone>(Option<(A, String)>);
struct Parser<A: Clone>(Box<dyn Fn(&String)->ParserOutput<A>>);

impl<A: Clone> ParserOutput<A> {
    fn only_if<F>(&self, f: F) -> ParserOutput<A>
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

    fn map<B: Clone, F>(&self, f: F) -> ParserOutput<B>
    where
        F: FnOnce(&A) -> B,
    {
        match &self.0 {
            Some((v, s)) => {
                return ParserOutput(Some((f(v), s.clone())));
            }
            None => return ParserOutput(None),
        }
    }

    fn and_then<B: Clone>(&self, parser: &Parser<B>) -> ParserOutput<B>
    {
        match &self.0 {
            Some((_, s)) => parser.parse(&s),
            None => ParserOutput(None),
        }
    }
}

impl<A: Clone> Parser<A>
// where
    // F: Fn(&String) -> ParserOutput<A>,
{
    fn new<F>(f: F) -> Parser<A>
    where F: 'static + Fn(&String) -> ParserOutput<A>
    {
        Parser(Box::new(f))
    }

    fn parse(&self, input: &String) -> ParserOutput<A> {
        (self.0)(input)
    }

    fn or(self, p2 :Parser<A>) -> Parser<A>
    where A : 'static
    {
        Parser::new(move |input : &String| {
            let result = self.parse(input);
            if result.0.is_some() {
                return result;
            }
            return p2.parse(input);
        })
    }

}


fn item(input: &String) -> ParserOutput<char> {
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

fn is_numeric(input: &String) -> ParserOutput<char> {
    item(input).only_if(|c| c.is_numeric())
}

fn is_alphabetic<'a>(input: &String) -> ParserOutput<char> {
    item(input).only_if(|c| c.is_alphabetic())
}

fn get_all<A: Clone>(input: &String, parser: &Parser<A>) -> ParserOutput<Vec<A>>
{
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

fn at_least_one<A: Clone>(input: &String, parser: &Parser<A>) -> ParserOutput<Vec<A>>
where
{
    get_all(input, parser).only_if(|v| v.len() > 0)
}

fn word(input: &String) -> ParserOutput<String> {
    at_least_one(input, &Parser::new(is_alphabetic)).map(|r| r.into_iter().collect())
}

fn white_space(input: &String) -> ParserOutput<char> {
    item(input).only_if(|c| c.is_whitespace())
}

fn clear_white_space(input: &String) -> ParserOutput<()> {
    get_all(input, &Parser::new(white_space)).map(|_| ())
}

fn keyword(input: &String, keyword: &str) -> ParserOutput<()> {
    word(input).only_if(|w| w == keyword).map(|_| ())
}

#[derive(Clone, Debug)]
enum Action {
    DoAction(String),
    Entry(String),
    Exit(String)
}

fn _action(input: &String, action: &'static str) -> ParserOutput<String> {
    clear_white_space(input)
        .and_then(&Parser::new(move |x| keyword(x, action)))
        .and_then(&Parser::new(clear_white_space))
        .and_then(&Parser::new(word))
}

fn do_action(input: &String) -> ParserOutput<Action> {
    _action(input, "do")
        .map(|s| Action::DoAction(s.clone()))
}

fn entry_action(input: &String) -> ParserOutput<Action> {
    _action(input, "entry")
        .map(|s| Action::Entry(s.clone()))
}

fn exit_action(input: &String) -> ParserOutput<Action> {
    _action(input, "exit")
        .map(|s| Action::Exit(s.clone()))
}

fn action(input: &String) -> ParserOutput<Action> {
    Parser::new(&do_action)
        .or(Parser::new(&entry_action))
        .or(Parser::new(&exit_action))
        .parse(input)
}

fn main() {
    println!("{:#?}", item(&"Foo".to_string()));
    println!("{:#?}", item(&"Ba".to_string()));
    println!("{:#?}", item(&"B".to_string()));
    println!("{:#?}", item(&"".to_string()));

    println!("{:#?}", is_numeric(&"".to_string()));
    println!("{:#?}", is_numeric(&"a".to_string()));
    println!("{:#?}", is_numeric(&"01".to_string()));

    println!("{:#?}", get_all(&"01f".to_string(), &Parser::new(is_numeric)));
    println!("{:#?}", at_least_one(&"".to_string(), &Parser::new(item)));
    // println!("{:#?}", at_least_one("1", &item));
    println!(
        "{:#?}",
        at_least_one(&"12f".to_string(), &Parser::new(is_numeric))
    );

    println!("{:#?}", word(&"Als ich".to_string()));
    println!("{:#?}", word(&"Als1 ich".to_string()));

    println!("{:#?}", clear_white_space(&"  aba".to_string()));
    println!("{:#?}", keyword(&"Foo Bar".to_string(), "Foo"));
    println!("{:#?}", action(&" entry Namaear bla".to_string()));
}
