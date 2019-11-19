use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric0, one_of},
    combinator::{map_res, recognize},
    multi::many0,
    sequence::{delimited, tuple},
    IResult,
};

struct Field {
    name: String,
    typ: Option<String>,
}

struct TypeDef {
    name: String,
    fields: Vec<Field>,
}

fn space(input: &str) -> IResult<&str, ()> {
    let (input, _) = many0(one_of(" \t"))(input)?;
    Ok((input, ()))
}

fn ident(input: &str) -> IResult<&str, String> {
    map_res(
        delimited(
            space,
            recognize(tuple((
                alt((tag("_"), alpha1)),
                alt((tag("_"), alphanumeric0)),
            ))),
            space,
        ),
        |s| -> Result<_, ()> { Ok(s.into()) },
    )(input)
}

fn main() {
    let test_str = " asdf ";
    println!("{:?}", ident(test_str));
}
