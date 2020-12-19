
#[derive(Debug, PartialEq, Clone, Copy)]
enum Token {
    Lparen,
    Rparen,
    Int(u64),
    Add,
    Mul,
    Eof,
}

fn get_number<T: Iterator<Item = char>>(c: char, iter: &mut std::iter::Peekable<T>) -> u64 {
    let mut number = c.to_string().parse::<u64>().unwrap();
    while let Some(Ok(digit)) = iter.peek().map(|c| c.to_string().parse::<u64>()) {
        number = number * 10 + digit;
        iter.next();
    }
    number
}

fn lex(input: &str) -> Vec<Token> {
    let mut iter = input.chars().peekable();
    let mut result = vec![];
    loop {
        let tok = match iter.next() {
            Some(c) => match c {
                '(' => Some(Token::Lparen),
                ')' => Some(Token::Rparen),
                '+' => Some(Token::Add),
                '*' => Some(Token::Mul),
                '0'..='9' => Some(Token::Int(get_number(c, &mut iter))),
                _ => None,
            },
            None => Some(Token::Eof),
        };

        tok.map(|t| result.push(t));
        if let Some(Token::Eof) = tok {
            return result;
        }
    }
}

#[derive(Debug)]
enum AstNode {
    Int(u64),
    Binop(Box<AstNode>, Token, Box<AstNode>),
}

fn parse_expr<T: Iterator<Item = Token>>(iter: &mut std::iter::Peekable<T>, min_bp: u8) -> AstNode {
    //First we parse the left-hand expr: an int or parenthesised expr
    let mut lhs = match iter.next().unwrap() {
        Token::Int(i) => AstNode::Int(i),
        Token::Lparen => {
            let lhs = parse_expr(iter, 0);
            assert_eq!(iter.next().unwrap(), Token::Rparen);
            lhs
        }
        _ => panic!("ohno"),
    };

    //Now we keep building expressions until we find an operator
    //lower binding power
    loop {
        let op = match iter.peek().unwrap() {
            Token::Eof => break,
            Token::Rparen => break,
            Token::Lparen => break,
            op @ Token::Add | op @ Token::Mul => *op,
            t => panic!("bad token: {:?}", t),
        };

        let (l_bp, r_bp) = binding_power(op);
        if l_bp < min_bp {
            break;
        }
        iter.next();

        let rhs = parse_expr(iter, r_bp);
        lhs = AstNode::Binop(Box::new(lhs), op, Box::new(rhs));
    }

    lhs
}

fn binding_power(tok: Token) -> (u8, u8) {
    match tok {
        Token::Mul => (1, 2),
        Token::Add => (3, 4),
        _ => panic!("onno"),
    }
}

fn parse(tokens: &[Token]) -> AstNode {
    parse_expr(&mut tokens.iter().copied().peekable(), 0)
}
/*
pub struct JIT {
    builder_context: FunctionBuilderContext,
    ctx: codegen::Context,
    data_ctx: DataContext,
    module: SimpleJITModule,
}

impl JIT {
    fn new() -> JIT {
        let builder = SimpleJITBuilder::new(cranelift_module::default_libcall_names());
        let module = SimpleJITModule::new(builder);
        JIT {
            builder_context: FunctionBuilderContext::new(),
            ctx: module.make_context(),
            data_ctx: DataContext::new(),
            module,
        }
    }

    fn translate(&mut self, ast: &AstNode) {
        let int = types::I32;

        self.ctx.func.signature.returns.push(AbiParam::new(int));

        let mut builder = FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_context);

        let entry_block = builder.create_block();
        builder.switch_to_block(entry_block);
        builder.seal_block(entry_block);


    }

    fn compile(&mut self, input: &str, name: &str) {
        let ast = parse(&lex(input));
        self.translate(ast);


        let id = self.module.declare_function(name, Linkake::External, &self.ctx.func.signature).unwrap();
        self.module.define_function(id, &mut self.context, &mut codegen::binemit::NullTrapSink {});   self.module.clear_context(&mut self.ctx);
        self.module.finalize_definitions();

        let code = self.module.get_finalized_function(id);

        Ok(code)
    }
}*/

fn eval(ast: &AstNode) -> u64 {
    match ast {
        AstNode::Int(i) => *i,
        AstNode::Binop(lhs, Token::Add, rhs) => eval(lhs) + eval(rhs),
        AstNode::Binop(lhs, Token::Mul, rhs) => eval(lhs) * eval(rhs),
        _ => panic!("wtf"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2() {
        let solution: u64 = include_str!("../input/day18.txt")
            .lines()
            .map(|line| eval(&parse(&lex(line))))
            .sum();
        println!("Solution: {}", solution);
    }
}
