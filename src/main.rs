use std::iter::Peekable;

#[derive(Debug, PartialEq)]
enum Token {
    Type(Type),
    Expression(Expression),
}

#[derive(Debug, PartialEq)]
enum Type {
    String(String),
    Number(i64),
    // Paren,
    // Array(Box<Type>),
}

#[derive(Debug, PartialEq)]
enum Expression {
    EchoExpression,
}

fn lex(input: &String) -> Result<Vec<Token>, String> {
    let mut result = Vec::new();

    let mut it = input.chars().peekable();
    while let Some(&c) = it.peek() {
        match c {
            '"' => {
                it.next();

                result.push(Token::Type(eat_string(c, &mut it)));
            }
            'a'...'z' => {
                it.next();

                result.push(Token::Expression(eat_expression(c, &mut it)));
            }
            '0'...'9' => {
                it.next();
                let n = get_number(c, &mut it);
                result.push(Token::Type(n));
            }
            /*'+' | '*' => {
                result.push(LexItem::Op(c));
                it.next();
            }
            '(' | ')' | '[' | ']' | '{' | '}' => {
                result.push(LexItem::Paren(c));
                it.next();
            }*/
            ' ' | ';' => {
                it.next();
            }
            _ => {
                return Err(format!("unexpected character {}", c));
            }
        }
    }

    Ok(result)
}

fn eat_string<T: Iterator<Item = char>>(c: char, iter: &mut Peekable<T>) -> Type {
    let mut string = String::new();

    while let Some(character) = iter.next() {
        if character == '"' {
            break;
        }

        // TODO: Panic when the character is not [a-zA-Z]
        string.push_str(&character.to_string());
    }

    Type::String(string)
}

fn eat_expression<T: Iterator<Item = char>>(c: char, iter: &mut Peekable<T>) -> Expression {
    let mut string = String::new();

    string.push_str(&c.to_string());

    while let Some(character) = iter.peek() {
        if *character == ' ' {
            break;
        }

        // TODO: Panic when the character is not [a-zA-Z]
        string.push_str(&character.to_string());

        iter.next();
    }

    if string == "echo" {
        return Expression::EchoExpression;
    }

    unreachable!("Unknown expression");
}

fn get_number<T: Iterator<Item = char>>(c: char, iter: &mut Peekable<T>) -> Type {
    let mut number = c
        .to_string()
        .parse::<i64>()
        .expect("The caller should have passed a digit.");
    while let Some(Ok(digit)) = iter.peek().map(|c| c.to_string().parse::<i64>()) {
        number = number * 10 + digit;
        iter.next();
    }

    Type::Number(number)
}

fn main() {
    // println!("{:?}", lex(&"4323 3".trim().to_owned()).unwrap());
    let mut ast = lex(&"echo \"1236\"; echo ".trim().to_owned())
        .unwrap()
        .into_iter()
        .peekable();

    while let Some(node) = ast.peek() {
        if *node == Token::Expression(Expression::EchoExpression) {
            ast.next();
            if let Some(Token::Type(Type::String(string))) = ast.next() {
                println!("Printing out the string: \"{}\"", string);
            }
        }

        ast.next();
    }
}
