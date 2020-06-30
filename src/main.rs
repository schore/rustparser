#[allow(dead_code)]
#[derive(Clone, Debug)]
struct ParserOutput<A: Clone>(Option<(A, String)>);
struct Parser<A: Clone, F>(F)
where
    F: Fn(&String) -> ParserOutput<A>;

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

    fn and_then<B: Clone, F>(&self, parser: &Parser<B, F>) -> ParserOutput<B>
    where
        F: Fn(&String) -> ParserOutput<B>,
    {
        match &self.0 {
            Some((_, s)) => parser.parse(&s),
            None => ParserOutput(None),
        }
    }
}

impl<A: Clone, F> Parser<A, F>
where
    F: Fn(&String) -> ParserOutput<A>,
{
    fn parse(&self, input: &String) -> ParserOutput<A> {
        (self.0)(input)
    }
}

fn or<A: Clone, F1, F2>(input: &String, p1: &Parser<A, F1>, p2: &Parser<A, F2>) -> ParserOutput<A>
where
    F1: Fn(&String) -> ParserOutput<A>,
    F2: Fn(&String) -> ParserOutput<A>,
{
    let result1 = p1.parse(input);
    match &result1.0 {
        Some(_) => result1,
        None => p2.parse(input),
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

fn get_all<A: Clone, F>(input: &String, parser: &Parser<A, F>) -> ParserOutput<Vec<A>>
where
    F: Fn(&String) -> ParserOutput<A>,
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

fn at_least_one<A: Clone, F>(input: &String, parser: &Parser<A, F>) -> ParserOutput<Vec<A>>
where
    F: Fn(&String) -> ParserOutput<A>,
{
    get_all(input, parser).only_if(|v| v.len() > 0)
}

fn word(input: &String) -> ParserOutput<String> {
    at_least_one(input, &Parser(is_alphabetic)).map(|r| r.into_iter().collect())
}

fn white_space(input: &String) -> ParserOutput<char> {
    item(input).only_if(|c| c.is_whitespace())
}

fn clear_white_space(input: &String) -> ParserOutput<()> {
    get_all(input, &Parser(white_space)).map(|_| ())
}

fn keyword(input: &String, keyword: &str) -> ParserOutput<()> {
    word(input).only_if(|w| w == keyword).map(|_| ())
}

fn action(input: &String, action: &str) -> ParserOutput<String> {
    clear_white_space(input)
        .and_then(&Parser(|x| keyword(x, action)))
        .and_then(&Parser(clear_white_space))
        .and_then(&Parser(word))
}

fn main() {
    println!("{:#?}", item(&"Foo".to_string()));
    println!("{:#?}", item(&"Ba".to_string()));
    println!("{:#?}", item(&"B".to_string()));
    println!("{:#?}", item(&"".to_string()));

    println!("{:#?}", is_numeric(&"".to_string()));
    println!("{:#?}", is_numeric(&"a".to_string()));
    println!("{:#?}", is_numeric(&"01".to_string()));

    println!("{:#?}", get_all(&"01f".to_string(), &Parser(is_numeric)));
    println!("{:#?}", at_least_one(&"".to_string(), &Parser(item)));
    // println!("{:#?}", at_least_one("1", &item));
    println!(
        "{:#?}",
        at_least_one(&"12f".to_string(), &Parser(is_numeric))
    );

    println!("{:#?}", word(&"Als ich".to_string()));
    println!("{:#?}", word(&"Als1 ich".to_string()));

    println!("{:#?}", clear_white_space(&"  aba".to_string()));
    println!("{:#?}", keyword(&"Foo Bar".to_string(), "Foo"));
    println!("{:#?}", action(&" do Namaear bla".to_string(), "do"));
}
