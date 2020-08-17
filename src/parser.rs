use crate::ast::{AstNode, AstNode::*};
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

fn parse_dyadic_verb(pair: Pair<Rule>, lhs: AstNode, rhs: AstNode) -> AstNode {
   AstNode::DyadicOp {
       lhs: Box::new(lhs),
       rhs: Box::new(rhs),
       verb: match pair.as_str() {
           _ => panic!("Unexpected verb: {}", pair.as_str())
       },
   }
}