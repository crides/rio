use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric0, one_of},
    combinator::{map_res, recognize},
    multi::many0,
    sequence::{delimited, terminated, tuple},
    IResult,
    error::ParseError,
};

#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub typ: Option<String>,
}

#[derive(Debug)]
pub struct TypeDef {
    pub name: String,
    pub fields: Vec<Field>,
}

pub fn space(input: &str) -> IResult<&str, ()> {
    let (input, _) = many0(one_of(" \t"))(input)?;
    Ok((input, ()))
}

pub fn linespace(input: &str) -> IResult<&str, ()> {
    let (input, _) = many0(one_of(" \t\n\r"))(input)?;
    Ok((input, ()))
}

pub fn ident(input: &str) -> IResult<&str, String> {
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

pub fn opt_typed_name(input: &str) -> IResult<&str, Field> {
    let (input, name) = ident(input)?;
    let res: IResult<&str, &str, ()> = tag(":")(input);
    match res {
        Ok((input, _)) => {
            let (input, typ) = ident(input)?;
            Ok((input, Field { name, typ: Some(typ) }))
        },
        _ => {
            Ok((input, Field { name, typ: None }))
        }
    }
}

// /// custom `ws!`
// fn wrap_space<O1, F>(inner: F) -> impl Fn(&str) -> IResult<&str, O1>
// where
//     F: Fn(&str) -> IResult<&str, O1>,
// {
//     move |input| {
//         let (input, _) = space(input)?;
//         let (input, res) = inner(input)?;
//         let (input, _) = space(input)?;
//         Ok((input, res))
//     }
// }

/// Recognizes a comma separated list, with an optional trailing comma
pub fn csl<O1, F>(inner: F) -> impl Fn(&str) -> IResult<&str, Vec<O1>>
where
    F: Fn(&str) -> IResult<&str, O1>,
{
    move |mut input| {
        let mut result = Vec::new();
        loop {
            input = linespace(input)?.0;
            match inner(input) {
                Err(_) => break,
                Ok((rest, inner_res)) => {
                    input = rest;
                    result.push(inner_res);
                }
            };
            input = linespace(input)?.0;
            let res: IResult<&str, &str, ()> = tag(",")(input);
            match res {
                Ok((rest, _)) => {
                    input = rest;
                }
                _ => break,
            }
        }
        input = linespace(input)?.0;
        Ok((input, result))
    }
}

pub fn type_def(input: &str) -> IResult<&str, TypeDef> {
    let (input, _) = terminated(tag("type"), space)(input)?;
    let (input, name) = ident(input)?;
    let (input, _) = tag("{")(input)?;
    let (input, fields) = csl(opt_typed_name)(input)?;
    let (input, _) = tag("}")(input)?;
    let (input, _) = linespace(input)?;
    Ok((input, TypeDef { name, fields }))
}

