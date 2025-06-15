#[derive(Debug, PartialEq, Clone)]
pub enum RegExp {
    Normal(char),
    Any,
    ZeroOrMore(Box<RegExp>),
    Or(Box<RegExp>, Box<RegExp>),
    Str(Vec<RegExp>),
}