use crate::ast::{AstNode, AstNode::*, MonadicVerb, DyadicVerb};
use pest::{Parser, error::Error, iterators::Pair};

#[derive(Parser)]
#[grammar = "jax.pest"]
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
    /*match pair.as_rule() {

    }*/
    unimplemented!()
}

fn parse_dyadic_verb(pair: Pair<Rule>, lhs: AstNode, rhs: AstNode) -> AstNode::DyadicOp {
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

fn parse_monadic_verb(pair: Pair<Rule>, expr: AstNode) -> AstNode::MonadicOp {
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