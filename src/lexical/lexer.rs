use crate::lexical::symbols::{DELIMITERS_TRIE, KEYWORDS_TRIE, OPERATORS_TRIE};

#[derive(Debug, PartialEq)]
pub enum Token {
    Keyword(String),
    Identifier(String),
    Number(String),
    StringLiteral(String),
    Operator(String),
    Delimiter(String),
    EOF,
}

pub struct Lexer {
    code: Vec<char>,
    position: usize,
    current_char: Option<char>,
}


impl Lexer {
    pub fn new(code: &str) -> Self {
        let mut lexer = Lexer {
            code: code.chars().collect(),
            position: 0,
            current_char: None,
        };
        lexer.current_char = lexer.code.get(lexer.position).cloned();
        lexer
    }

    fn advance(&mut self) {
        self.position += 1;
        self.current_char = if self.position < self.code.len() {
            Some(self.code[self.position])
        } else {
            None
        };
    }


    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    pub fn next_token(&mut self) -> Token {
        while let Some(ch) = self.current_char {
            if ch.is_whitespace() {
                self.skip_whitespace();
                continue;
            }

            if ch.is_alphabetic() || ch == '_' || ch == '$' {
                return self.lex_identifier_or_keyword();
            }

            if ch.is_digit(10) {
                return self.lex_number();
            }

            if ch == '"' || ch == '\'' {
                return self.lex_string();
            }

            if let Some(token) = self.lex_operator_or_delimiter() {
                return token;
            }


            self.advance();
        }

        Token::EOF
    }

    fn lex_operator_or_delimiter(&mut self) -> Option<Token> {
        if let Some((symbol, end)) = OPERATORS_TRIE.match_symbol(&self.code, self.position, false) {
            for _i in self.position..end {
                self.advance();
            }
            return Some(Token::Operator(symbol));
        }

        if let Some((symbol, end)) = DELIMITERS_TRIE.match_symbol(&self.code, self.position, false) {
            for _i in self.position..end {
                self.advance();
            }
            return Some(Token::Delimiter(symbol));
        }

        None
    }

    fn lex_number(&mut self) -> Token {
        let mut value = String::new();

        while let Some(ch) = self.current_char {
            if ch.is_digit(10) || ch == '.' {
                value.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        Token::Number(value)
    }

    fn lex_identifier_or_keyword(&mut self) -> Token {
        let mut value = String::new();

        while let Some(ch) = self.current_char {
            if ch.is_alphanumeric() || ch == '_' || ch == '$' {
                value.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        if KEYWORDS_TRIE.match_symbol(&value.chars().collect::<Vec<_>>(), 0, true).is_some() {
            Token::Keyword(value)
        } else {
            Token::Identifier(value)
        }
    }

    fn lex_string(&mut self) -> Token {
        let quote = self.current_char.unwrap();
        self.advance();

        let mut value = String::new();
        while let Some(ch) = self.current_char {
            if ch == quote {
                self.advance();
                break;
            } else if ch == '\\' {
                self.advance();
                if let Some(escaped_char) = self.current_char {
                    value.push(match escaped_char {
                        'n' => '\n',
                        't' => '\t',
                        '\\' => '\\',
                        '"' => '"',
                        '\'' => '\'',
                        _ => escaped_char,
                    });
                    self.advance();
                }
            } else {
                value.push(ch);
                self.advance();
            }
        }

        Token::StringLiteral(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keywords() {
        let input = "let const var if else while do break continue return";
        let mut lexer = Lexer::new(input);

        let keywords = vec![
            "let", "const", "var", "if", "else", "while", "do", "break", "continue", "return",
        ];

        for keyword in keywords {
            assert_eq!(lexer.next_token(), Token::Keyword(keyword.to_string()));
            }

        assert_eq!(lexer.next_token(), Token::EOF);
    }

    #[test]
    fn test_identifiers() {
        let input = "myVar another_variable _leadingUnderscore $dollarSign123";
        let mut lexer = Lexer::new(input);

        let identifiers = vec![
            "myVar",
            "another_variable",
            "_leadingUnderscore",
            "$dollarSign123",
        ];

        for identifier in identifiers {
            assert_eq!(lexer.next_token(), Token::Identifier(identifier.to_string()));
        }

        assert_eq!(lexer.next_token(), Token::EOF);
    }

    #[test]
    fn test_numbers() {
        let input = "42 3.14 0.99";
        let mut lexer = Lexer::new(input);

        let numbers = vec!["42", "3.14", "0.99"];

        for number in numbers {
            assert_eq!(lexer.next_token(), Token::Number(number.to_string()));
        }

        assert_eq!(lexer.next_token(), Token::EOF);
    }

    #[test]
    fn test_strings() {
        let input = r#""hello" 'world' "multi-line\nstring""#;
        let mut lexer = Lexer::new(input);

        let strings = vec!["hello", "world", "multi-line\nstring"];

        for string in strings {
            assert_eq!(lexer.next_token(), Token::StringLiteral(string.to_string()));
        }

        assert_eq!(lexer.next_token(), Token::EOF);
    }

    #[test]
    fn test_operators() {
        let input = "= == === + - * / % && || ! < <= > >= +=";
        let mut lexer = Lexer::new(input);

        let operators = vec![
            "=", "==", "===", "+", "-", "*", "/", "%", "&&", "||", "!", "<", "<=", ">", ">=", "+=",
        ];

        for operator in operators {
            assert_eq!(lexer.next_token(), Token::Operator(operator.to_string()));
        }

        assert_eq!(lexer.next_token(), Token::EOF);
    }

    #[test]
    fn test_delimiters() {
        let input = "( ) { } [ ] , ; .";
        let mut lexer = Lexer::new(input);

        let delimiters = vec![
            "(", ")", "{", "}", "[", "]", ",", ";", ".",
        ];

        for delimiter in delimiters {
            assert_eq!(lexer.next_token(), Token::Delimiter(delimiter.to_string()));
        }

        assert_eq!(lexer.next_token(), Token::EOF);
    }

    #[test]
    fn test_combined_input() {
        let input = r#"let x = 42; if (x > 10) { x += 5; }"#; // Parentheses is fucking up
        let mut lexer = Lexer::new(input);

        let expected_tokens = vec![
            Token::Keyword("let".to_string()),
            Token::Identifier("x".to_string()),
            Token::Operator("=".to_string()),
            Token::Number("42".to_string()),
            Token::Delimiter(";".to_string()),
            Token::Keyword("if".to_string()),
            Token::Delimiter("(".to_string()),
            Token::Identifier("x".to_string()),
            Token::Operator(">".to_string()),
            Token::Number("10".to_string()),
            Token::Delimiter(")".to_string()),
            Token::Delimiter("{".to_string()),
            Token::Identifier("x".to_string()),
            Token::Operator("+=".to_string()),
            Token::Number("5".to_string()),
            Token::Delimiter(";".to_string()),
            Token::Delimiter("}".to_string()),
            Token::EOF,
        ];

        for expected in expected_tokens {
            let actual = lexer.next_token();
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_edge_cases() {
        let input = r#"let _ = 0; let $ = 5; let camelCase = true;"#;
        let mut lexer = Lexer::new(input);

        let expected_tokens = vec![
            Token::Keyword("let".to_string()),
            Token::Identifier("_".to_string()),
            Token::Operator("=".to_string()),
            Token::Number("0".to_string()),
            Token::Delimiter(";".to_string()),
            Token::Keyword("let".to_string()),
            Token::Identifier("$".to_string()),
            Token::Operator("=".to_string()),
            Token::Number("5".to_string()),
            Token::Delimiter(";".to_string()),
            Token::Keyword("let".to_string()),
            Token::Identifier("camelCase".to_string()),
            Token::Operator("=".to_string()),
            Token::Keyword("true".to_string()),
            Token::Delimiter(";".to_string()),
            Token::EOF,
        ];

        for expected in expected_tokens {
            assert_eq!(lexer.next_token(), expected);
        }
    }

    #[test]
    fn test_unexpected_characters() {
        let input = r#"@ # ^ ~ ` | ? \"#;
        let mut lexer = Lexer::new(input);

        let expected_tokens = vec![
            Token::EOF,
        ];

        for expected in expected_tokens {
            assert_eq!(lexer.next_token(), expected);
        }
    }
}
