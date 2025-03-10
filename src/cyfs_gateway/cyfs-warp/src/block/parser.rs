use super::block::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{space0},
    combinator::{map, opt},
    multi::many0,
    sequence::{delimited},
    IResult,
    Parser,
};
use shlex;

pub struct CommandParser {
    // Command name
    name: String,
    // Command args
    args: Vec<String>,
}

impl CommandParser {
    pub fn parse(block: &str) -> Result<Block, String> {
        let lines: Vec<&str> = Self::split_lines(block);
        let mut block = Block::new();

        if lines.is_empty() {
            warn!("Empty block");
            return Ok(block);
        }

        for (i, line) in lines.iter().enumerate() {
            let parsed_line = Self::parse_line(line)?;
            if let Some(ref label) = parsed_line.label {
                block.label_map.insert(label.clone(), i);
            }
            block.lines.push(parsed_line);
        }

        Ok(block)
    }

    // Block contains multiple lines, first split them into lines
    // The lines are separated by '\n' or '\r\n' or '\r' for different OS
    fn split_lines(block: &str) -> Vec<&str> {
        block
            .split(|c| c == '\n' || c == '\r')
            .map(|line| line.trim_end_matches('\r')) // Remove '\r' at the end of line if any
            .filter(|line| !line.is_empty()) // Filter out empty lines
            .collect()
    }

    // Parse a single line, support label and expressions
    fn parse_line(line: &str) -> Result<Line, String> {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            return Ok(Line {
                label: None,
                expressions: Vec::new(),
            });
        }

        // First try to split the line into label and expressions, the label is start with "label: "
        let (label, expr_input) = if let Some((label_part, rest)) = trimmed.split_once(": ") {
            (Some(label_part.trim().to_string()), rest.trim())
        } else {
            (None, trimmed)
        };

        let (_, expressions) =
            Self::parse_expressions(expr_input).map_err(|e| {
                let msg = format!("Parse expressions error: {}, {:?}", expr_input, e);
                error!("{}", msg);
                msg
            })?;


        Ok(Line { label, expressions })
    }

    // Parse expressions with operators
    fn parse_expressions(input: &str) -> IResult<&str, Vec<(Expression, Operator)>> {
        many0(|i| {
            let (i, expr) = Self::parse_expression(i)?;
            let (i, op) = opt(alt((
                map(tag("&&"), |_| Operator::And),
                map(tag("||"), |_| Operator::Or),
            ))).parse(i)?;

            Ok((i, (expr, op.unwrap_or(Operator::None))))
        }).parse(input)

        /*
        let (input, exprs) = many0(tuple((
            Self::parse_expression,
            opt(alt((
                map(tag("&&"), |_| Operator::And),
                map(tag("||"), |_| Operator::Or),
            ))),
        ))).parse(input)
        .map_err(|e| {
            let msg = format!("Parse error: {}, {:?}", input, e);
            error!("{}", msg);
            e
        })?;

        let exprs = exprs
            .into_iter()
            .map(|(expr, op)| (expr, op.unwrap_or(Operator::None)))
            .collect();

        Ok((input, exprs))
        */
    }

    // Parse expression with brackets or command
    fn parse_expression(input: &str) -> IResult<&str, Expression> {
        alt((Self::parse_group, Self::parse_command)).parse(input)
    }

    // Parse group of expressions with brackets
    fn parse_group(input: &str) -> IResult<&str, Expression> {
        let mut parser = delimited(tag("("), Self::parse_expressions, tag(")"));
        let (input, expressions) = parser.parse(input)
            .map_err(|e| {
                let msg = format!("Parse group error: {}, {:?}", input, e);
                error!("{}", msg);
                e
            })?;

        Ok((input, Expression::Group(expressions)))
    }

    // Parse command
    fn parse_command(input: &str) -> IResult<&str, Expression> {
        let (input, tokens) = map(
            many0(delimited(
                space0,
                |i| Ok((i, shlex::split(i).unwrap_or_default())),
                space0,
            )),
            |token_sets| {
                if token_sets.is_empty() {
                    return Expression::Command(Command {
                        name: "".to_string(),
                        args: Vec::new(),
                    });
                }
                
                let tokens = token_sets[0].clone();
                let name = tokens[0].clone();
                if name.to_ascii_lowercase() == "goto" && tokens.len() > 1 {
                    Expression::Goto(tokens[1].clone())
                } else {
                    let args = tokens[1..].to_vec();
                    Expression::Command(Command { name, args })
                }
            },
        ).parse(input).map_err(|e| {
            let msg = format!("Parse command error: {}, {:?}", input, e);
            error!("{}", msg);
            e
        })?;

        Ok((input, tokens))
    }
}
