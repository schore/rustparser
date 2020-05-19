#[allow(dead_code)]


type ParserOutput<'a, A> = Option<(A, &'a str)>;
type Parser<A> = Fn (&str) -> ParserOutput<A>;

fn get_value<'a, A>(s :&'a ParserOutput<A>) -> Option<&'a A> {
    let a = s.as_ref()?;
    Some(&a.0)
}

fn get_str<'a, A>(s :&'a ParserOutput<A> ) -> Option<&'a str> {
    let a = s.as_ref()?;
    Some(&a.1)
}

fn parserMap<'a, A,B>(s :ParserOutput<'a, A>, f : &Fn(A) -> B) -> ParserOutput<'a, B> {
    s.map(|(v, rest)| (f(v), rest))
}

fn item(input :&str) -> ParserOutput<char> {
    let item = input.chars().next()?;
    let rest = input.get(1..)?;
    Some((item, rest))
}

fn get_if<'a, A>(inp : &'a str, p :&Parser<A>, f : &Fn(&A) -> bool) -> ParserOutput<'a, A> {
    let (a, out) = p(inp)?;
    if f(&a) {
        return Some((a,out));
    }
    None
}

fn is_numeric<'a>(input :&'a str) -> ParserOutput<char> {
    get_if(input, &item, &|c| c.is_numeric())
}

fn is_alphabetic<'a>(input :&'a str) -> ParserOutput<char> {
    get_if(input, &item, &|c| c.is_alphabetic())
}

fn get_all<'a, A>(input :&'a str, p :&Parser<A>) -> ParserOutput<'a, Vec<A>> {
    let mut ret : Vec<A> = Vec::new();
    let mut rest = input;

    loop {
        match p(rest) {
            Some((a, s)) => {
                ret.push(a);
                rest = s;
            }
            None => break
        }
    }

    Some((ret, rest))
}

fn at_least_one<'a, A>(input :&'a str, p :&Parser<A>) -> ParserOutput<'a, Vec<A>> {
    let ret = get_all(input, p);
    if get_value(&ret).map_or(false, | a | a.len() > 0) {
        return ret;
    }
    None
}

fn word<'a>(input :&'a str) -> ParserOutput<'a, String> {
    parserMap(at_least_one(input, &is_alphabetic),
              &|a| a.into_iter().collect() )
}

fn white_space<'a>(input :&'a str)  -> ParserOutput<'a, char> {
     get_if(input, &item, &|c| c.is_whitespace())
}

fn clear_white_space<'a>(input :&'a str) -> ParserOutput<'a, ()>{
    parserMap(get_all(input, &white_space),
              &|_| ())
}

fn keyword<'a>(input :&'a str, keyword :& str)  -> ParserOutput<'a, ()> {
    word(input).and_then(&|(v,rest)| {
        if v == keyword
          { return Some(((),rest));
          }
        None

    })
}

fn main() {
    println!("{:#?}", item("Foo"));
    println!("{:#?}", item("Ba"));
    println!("{:#?}", item("B"));
    println!("{:#?}", item(""));

    println!("{:#?}", is_numeric(""));
    println!("{:#?}", is_numeric("a"));
    println!("{:#?}", is_numeric("01"));


    println!("{:#?}", get_all("01f", &is_numeric));
    println!("{:#?}", at_least_one("", &item));
    println!("{:#?}", at_least_one("1", &item));
    println!("{:#?}", at_least_one("12f", &is_numeric));

    println!("{:#?}", word("Als ich"));
    println!("{:#?}", word("Als1 ich"));


    println!("{:#?}", clear_white_space("  aba"));
    println!("{:#?}", keyword("Foo Bar", "Foo"));
}
