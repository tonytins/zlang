use crate::ast::{AstNode, AstNode::*, MonadicVerb, DyadicVerb};
use pest::{Parser, error::Error, iterators::Pair};

#[derive(Parser)]
#[grammar = "jlang.pest"]
pub struct ZParser;

pub fn parse(source: &str) -> Result<Vec<AstNode>, Error<Rule>> {

    let mut ast = vec![];
    let pairs = ZParser::parse(Rule::program, source)?;

    for pair in pairs {
        match pair.as_rule() {
            Rule::expr => {
                ast.push(Print(Box::new(build_ast_from_expr(pair))));
            }
            _ => {}
        }
    }

    Ok(ast)
}

fn build_ast_from_expr(pair: Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        Rule::expr => build_ast_from_expr(pair.into_inner().next().unwrap()),
        Rule::monadicExpr => {
            let mut pair = pair.into_inner();
            let verb = pair.next().unwrap();
            let expr = pair.next().unwrap();
            let expr = build_ast_from_expr(expr);
            parse_monadic_verb(verb, expr);
        }
    }
}

fn build_ast_from_term(pair: Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        Pair::integer => {
            let istr = pair.as_str();
            let (sign, istr) = match  &istr[..1] {
                "_" => (-1, &istr[1..]),
                _ => (1, &istr[..]),
            };
            let integer: i32 = istr.parse().unwrap();
            AstNode::Integer(sign * integer)
        }
    }
}

fn parse_dyadic_verb(pair: Pair<Rule>, lhs: AstNode, rhs: AstNode) -> AstNode {
   AstNode::DyadicOp {
       lhs: Box::new(lhs),
       rhs: Box::new(rhs),
       verb: match pair.as_str() {
           "+" => DyadicVerb::Plus,
           "*" => DyadicVerb::Times,
           "-" => DyadicVerb::Minus,
           "<" => DyadicVerb::LessThan,
           "=" => DyadicVerb::Equal,
           ">" => DyadicVerb::LargerThan,
           "%" => DyadicVerb::Divide,
           "^" => DyadicVerb::Power,
           "|" => DyadicVerb::Residue,
           "#" => DyadicVerb::Copy,
           ">." => DyadicVerb::LargerOf,
           ">:" => DyadicVerb::LargerOrEqual,
           "$" => DyadicVerb::Shape,
           _ => panic!("Unexpected verb: {}", pair.as_str())
       },
   }
}

fn parse_monadic_verb(pair: Pair<Rule>, expr: AstNode) -> AstNode {
    AstNode::MonadicOp {
        verb: match pair.as_str() {
            ">:" => MonadicVerb::Increment,
            "*:" => MonadicVerb::Square,
            "-" => MonadicVerb::Negate,
            "%" => MonadicVerb::Reciprocal,
            "#" => MonadicVerb::Tally,
            ">." => MonadicVerb::Ceiling,
            "$" => MonadicVerb::ShapeOf,
            _ => panic!("Unsupported verb: {}", pair.as_str())
        },
        expr: Box::new(expr),
    }
}