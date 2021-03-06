use std::iter::Peekable;

#[derive(Debug, PartialEq)]
enum Token {
    Type(Type),
    Expression(Expression),
    Semicolon,
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

                let string = Token::Type(eat_string(c, &mut it));

                result.push(string);
            }
            'a'...'z' => {
                it.next();

                let expression = Token::Expression(eat_expression(c, &mut it));

                result.push(expression);
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
            ' ' => {
                it.next();
            }
            ';' => {
                result.push(Token::Semicolon);
                it.next();
            }
            _ => {
                return Err(format!("unexpected character {}", c));
            }
        }
    }

    Ok(result)
}

fn eat_string<T: Iterator<Item = char>>(_c: char, iter: &mut Peekable<T>) -> Type {
    let mut string = String::new();

    while let Some(character) = iter.peek() {
        let mut c = *character;

        if c == '\\' {
            c = iter.next().unwrap(); // TODO: Handle Err()

            // TODO: Handle \t, \n and so on.
            // println!("Recognized escape sequence, escaped char: {}", c);
        }
        if c == '"' {
            // early abort
            iter.next();
            break;
        }

        // TODO: Panic when the character is not [a-zA-Z]
        string.push_str(&c.to_string());
        iter.next();
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
    let input = r##"echo "Hallo \\Welt";"##;

    println!("{}", interpret(input));
}

fn interpret(input: &'static str) -> String {
    let mut ast = lex(&input.trim().to_owned())
        .unwrap()
        .into_iter()
        .peekable();

    let mut output = String::new();

    while let Some(node) = ast.peek() {
        if *node == Token::Expression(Expression::EchoExpression) {
            ast.next();
            if let Some(Token::Type(Type::String(string))) = ast.next() {
                output.push_str(&string);
            }
        }

        ast.next();
    }

    output
}

#[cfg(test)]
mod tests {
    use super::{lex, Expression, Token, Type};

    #[test]
    fn integer_parsing() {
        let result = lex(&"1234".to_owned());

        assert!(result.is_ok());

        assert_eq!(result.unwrap(), vec![Token::Type(Type::Number(1234))]);
    }

    #[test]
    fn basic_output() {
        let result = lex(&"echo \"Hallo Welt\";".to_owned());

        assert!(result.is_ok());

        assert_eq!(
            result.unwrap(),
            vec![
                Token::Expression(Expression::EchoExpression),
                Token::Type(Type::String(String::from("Hallo Welt"))),
                Token::Semicolon
            ]
        );
    }

    #[test]
    fn escaped_output() {
        let result = lex(&r##"echo "Hallo \\Welt";"##.to_owned());

        assert!(result.is_ok());

        assert_eq!(
            result.unwrap(),
            vec![
                Token::Expression(Expression::EchoExpression),
                Token::Type(Type::String(String::from("Hallo \\Welt"))),
                Token::Semicolon
            ]
        );
    }
}
