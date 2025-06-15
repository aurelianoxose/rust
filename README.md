# rust

First_task

Your task is to implement a simple regular expression parser. We will have a parser that outputs the following AST of a regular expression:

enum RegExp {
  Normal(char),                  // A character that is not in "()*|."
  Any,                           // Any character
  ZeroOrMore(Box<RegExp>),       // Zero or more occurances of the same regexp
  Or(Box<RegExp>, Box<RegExp>),  // A choice between 2 regexps
  Str(Vec<RegExp>),              // A sequence of regexps
}
As with the usual regular expressions, Any is denoted by the '.' character, ZeroOrMore is denoted by a subsequent '*' and Or is denoted by '|'. Brackets, ( and ), are allowed to group a sequence of regular expressions into the Str object.

Or is not associative, so "a|(a|a)" and "(a|a)|a" are both valid regular expressions, whereas "a|a|a" is not.

Operator precedences from highest to lowest are: *, sequencing and |. Therefore the followings hold:

// Box::new() parts are omitted from Or and ZeroOrMore variants for readability
"ab*"    -> Str(vec![Normal('a'), ZeroOrMore(Normal('b'))])
"(ab)*"  -> ZeroOrMore(Str(vec![Normal('a'), Normal('b')]))
"ab|a"   -> Or(Str(vec![Normal('a'), Normal('b')]), Normal('a'))
"a(b|a)" -> Str(vec![Normal('a'), Or(Normal('b'), Normal('a'))])
"a|b*"   -> Or(Normal('a'), ZeroOrMore(Normal('b')))
"(a|b)*" -> ZeroOrMore(Or(Normal('a'), Normal('b')))
Some examples:

// Box::new() parts are omitted from Or and ZeroOrMore variants for readability
"a"          -> Normal('a')
"ab"         -> Str(vec![Normal('a'), Normal('b')])
"a.*"        -> Str(vec![Normal('a'), ZeroOrMore(Any)])
"(a.*)|(bb)" -> Or(
                  Str(vec![Normal('a'), ZeroOrMore(Any)]), 
                  Str(vec![Normal('b'), Normal('b')])
                )
The followings are invalid regexps and the parser should return Nothing in Haskell / 0 in C or C++ / null in JavaScript or C# / None in Python or Rust / new Void() in Java/Void() in Kotlin:

"", ")(", "*", "a(", "()", "a**", etc.

Feel free to use any parser combinator libraries available on codewars, or implement the parser "from scratch".
