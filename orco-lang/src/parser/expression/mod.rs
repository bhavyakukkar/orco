use super::*;

/// Parsers for a block expression
pub mod block;

/// Expect an expression, error if there is no
pub fn expect(parser: &mut Parser) -> ir::Expression {
    if let Some(expression) = parse(parser) {
        expression
    } else {
        parser.expected_error("expression");
        ir::Expression::Error
    }
}

/// Parse an expression
pub fn parse(parser: &mut Parser) -> Option<ir::Expression> {
    if parser.match_keyword("return") {
        Some(ir::Expression::Return(Box::new(expect(parser))))
    } else if let Some(expression) = binary_expression(parser, 0) {
        Some(expression)
    } else {
        None
    }
}

/// Parse a binary expression with a set level of precedance
pub fn binary_expression(parser: &mut Parser, level: usize) -> Option<ir::Expression> {
    use ir::expression::BinaryOp;
    let operators = [
        vec![
            (Operator::Plus, BinaryOp::Add),
            (Operator::Minus, BinaryOp::Sub),
        ],
        vec![
            (Operator::Star, BinaryOp::Mul),
            (Operator::Slash, BinaryOp::Div),
            (Operator::Percent, BinaryOp::Mod),
        ],
    ];

    if level >= operators.len() {
        return unit_expression(parser);
    }

    let mut expression = binary_expression(parser, level + 1)?;
    loop {
        let mut any = false;
        for &(operator, operation) in &operators[level] {
            if parser.match_opertor(operator) {
                expression = ir::Expression::BinaryOp(
                    Box::new(expression),
                    operation,
                    Box::new(binary_expression(parser, level + 1)?),
                );
                any = true;
            }
        }
        if !any {
            break;
        }
    }
    Some(expression)
}

/// Parse a unit expression
pub fn unit_expression(parser: &mut Parser) -> Option<ir::Expression> {
    if parser.match_error() {
        Some(ir::Expression::Error)
    } else if let Some(constant) = parser.match_constant() {
        Some(ir::Expression::Constant(constant))
    } else if let Some(block) = block::parse(parser) {
        Some(ir::Expression::Block(block))
    } else if let Some(name) = parser.match_ident() {
        if parser.match_opertor(Operator::LParen) {
            let mut args = Vec::new();
            while let Some(expression) = expression::parse(parser) {
                args.push(expression);
                if !parser.match_opertor(Operator::Comma) {
                    break;
                }
            }
            parser.expect_operator(Operator::RParen);
            Some(ir::Expression::FunctionCall { name, args })
        } else {
            todo!("Variables");
        }
    } else {
        None
    }
}
