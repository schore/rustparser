struct Parser<'a, T> {
    parser: Box<dyn Fn(String) -> Option<(T, String)> + 'a>,
}

#[allow(dead_code)]
impl<'a, A: Clone> Parser<'a, A>
where
    A: 'a,
{
    fn parse(&self, str: String) -> Option<(A, String)> {
        let out = (self.parser)(str);
        return out;
    }

    fn map<B>(&'a self, f: &'a Fn(&A) -> B) -> Parser<'a, B> {
        return Parser {
            parser: Box::new(move |str| self.parse(str).map(|(s, str)| (f(&s), str))),
        };
    }

    fn new(inp: A) -> Parser<'a, A> {
        Parser {
            parser: Box::new(move |str| Some((inp.clone(), str))),
        }
    }

    fn and_then<B: Clone>(&'a self, f: &'a Fn(&A) -> Parser<'a, B>) -> Parser<'a, B> {
        Parser {
            parser: Box::new(move |str| {
                let temp = self.parse(str);
                match temp {
                    Some((a, out)) => f(&a).parse(out),
                    None => None,
                }
            }),
        }
    }

}


fn is_true<'a, A :Clone>(p :Parser<'a, A>, f: &'a Fn(&A) -> bool) -> Parser<'a, A>
{
    Parser {
        parser: Box::new(move |str| {
            let (a, st) = p.parse(str)?;
            if f(&a) {
                return Some((a, st));
            }
            None
        })
    }
}

fn item() -> Parser<'static, char> {
    Parser {
        parser: Box::new(move |str: String| {
            if str.len() > 0 {
                return Some((str.chars().next().unwrap(), str[1..].to_string()));
            }
            None
        }),
    }
}

fn is_numeric() -> Parser<'static, char> {
    is_true(item(),
            &|a :& char| a.is_numeric()
           )
}


fn myvectormap<A, B, F>(f: &F) -> impl Fn(Vec<A>) -> Vec<B> + '_
where
    F: Fn(&A) -> B,
{
    move |inp| inp.iter().map(f).collect()
}

fn main() {
    let a = Parser {
        parser: Box::new(|x| {
            return Some((0, x));
        }),
    };

    let b = a.parse("Hello, world!".to_string());

    println!("{:#?}", b);

    let a: fn(&i32) -> i32 = |&x| x + 1;
    let f = myvectormap(&a);

    println!("{:#?}", f(vec!(1, 2, 3)));
    println!("{:#?}", item().parse("Foo".to_string()));
    println!("{:#?}", item().parse("".to_string()));
    println!("{:#?}", is_numeric().parse("".to_string()));
    println!("{:#?}", is_numeric().parse("a".to_string()));
    println!("{:#?}", is_numeric().parse("01".to_string()));
}
