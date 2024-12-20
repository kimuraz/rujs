use std::collections::HashMap;
use std::sync::LazyLock;

#[derive(Debug)]
pub struct SymbolTrie {
    children: HashMap<char, SymbolTrie>,
    is_last: bool,
}

static DELIMITERS: [&str; 9] = ["(", ")", "{", "}", "[", "]", ",", ";", "."];

impl SymbolTrie {
    fn new(symbols: &[&str]) -> Self {
        let mut root = SymbolTrie {
            children: HashMap::new(),
            is_last: false,
        };

        for &symbol in symbols {
            root.insert(symbol);
        }

        root
    }

    fn insert(&mut self, symbol: &str) {
        let mut node = self;
        for ch in symbol.chars() {
            node = node.children.entry(ch).or_insert_with(SymbolTrie::new_empty);
        }
        node.is_last = true;
    }

    fn new_empty() -> Self {
        SymbolTrie {
            children: HashMap::new(),
            is_last: false,
        }
    }

    pub fn is_boundary(ch: char) -> bool {
        let s: &str = &ch.to_string();
        ch.is_whitespace() || DELIMITERS.contains(&s)
    }

    pub fn match_symbol<'a>(&self, code: &'a [char], start: usize, needs_boundary: bool) -> Option<(String, usize)> {
        let mut node = self;
        let mut matched = String::new();
        let mut last_match = None;

        for (i, &ch) in code.iter().enumerate().skip(start) {
            if let Some(child) = node.children.get(&ch) {
                matched.push(ch);
                node = child;

                if node.is_last {
                    if needs_boundary {
                        if i + 1 >= code.len() || Self::is_boundary(code[i + 1]) {
                            last_match = Some((matched.clone(), i + 1));
                        }
                    } else {
                        last_match = Some((matched.clone(), i + 1));
                    }
                }
            } else {
                break;
            }
        }

        last_match
    }
}

pub static OPERATORS_TRIE: LazyLock<SymbolTrie> = LazyLock::new(|| {
    SymbolTrie::new(&[
        "+", "-", "*", "**", "/", "%", "==", "!=", "<", "<=", ">",
        ">=", "&&", "||", "!", "=", "+=", "-=", "*=", "/=", "%=",
        "===", "..."
    ])
});

pub static DELIMITERS_TRIE: LazyLock<SymbolTrie> = LazyLock::new(|| {
    SymbolTrie::new(&DELIMITERS)
});

pub static KEYWORDS_TRIE: LazyLock<SymbolTrie> = LazyLock::new(|| {
    SymbolTrie::new(&[
        "let", "const", "var", "if", "else", "for", "while", "do", "break",
        "continue", "return", "function", "true", "false", "null", "undefined",
        "new", "this", "delete", "typeof", "in", "instanceof", "void", "catch",
        "try", "finally", "switch", "case", "default", "throw", "class", "extends",
        "super", "import", "export", "from", "as", "await", "async", "yield",
    ])
});


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_operator_match() {
        let code: Vec<char> = "==".chars().collect();
        let result = OPERATORS_TRIE.match_symbol(&code, 0, false);
        assert_eq!(result, Some(("==".to_string(), 2)));
    }

    #[test]
    fn test_non_operator_match() {
        let code: Vec<char> = "foo".chars().collect();
        let result = OPERATORS_TRIE.match_symbol(&code, 0, false);
        assert_eq!(result, None);
    }

    #[test]
    fn test_longer_operator_match() {
        let code: Vec<char> = "===".chars().collect();
        let result = OPERATORS_TRIE.match_symbol(&code, 0, false);
        assert_eq!(result, Some(("===".to_string(), 3)));
    }

    #[test]
    fn test_partial_operator_match() {
        let code: Vec<char> = "=".chars().collect();
        let result = OPERATORS_TRIE.match_symbol(&code, 0, false);
        assert_eq!(result, Some(("=".to_string(), 1)));
    }

    #[test]
    fn test_operator_with_spaces() {
        let code: Vec<char> = " == ".chars().collect();
        let result = OPERATORS_TRIE.match_symbol(&code, 1, false);
        assert_eq!(result, Some(("==".to_string(), 3)));
    }

    #[test]
    fn test_delimiter_match() {
        let code: Vec<char> = "(".chars().collect();
        let result = DELIMITERS_TRIE.match_symbol(&code, 0, false);
        assert_eq!(result, Some(("(".to_string(), 1)));
    }

    #[test]
    fn test_non_delimiter_match() {
        let code: Vec<char> = "foo".chars().collect();
        let result = DELIMITERS_TRIE.match_symbol(&code, 0, false);
        assert_eq!(result, None);
    }

    #[test]
    fn test_code_with_dot() {
        let code: Vec<char> = "obj.prop".chars().collect();

        let result_dot = DELIMITERS_TRIE.match_symbol(&code, 3, false);
        assert_eq!(result_dot, Some((".".to_string(), 4)));
    }

    #[test]
    fn test_code_with_spread_operator() {
        let code: Vec<char> = "obj.props(...args)".chars().collect();

        let result_spread = OPERATORS_TRIE.match_symbol(&code, 10, false);
        assert_eq!(result_spread, Some(("...".to_string(), 13)));
    }

    #[test]
    fn test_multiple_delimiters() {
        let code: Vec<char> = "{[(".chars().collect();
        let result1 = DELIMITERS_TRIE.match_symbol(&code, 0, false);
        let result2 = DELIMITERS_TRIE.match_symbol(&code, 1, false);
        let result3 = DELIMITERS_TRIE.match_symbol(&code, 2, false);

        assert_eq!(result1, Some(("{".to_string(), 1)));
        assert_eq!(result2, Some(("[".to_string(), 2)));
        assert_eq!(result3, Some(("(".to_string(), 3)));
    }

    #[test]
    fn test_delimiter_with_spaces() {
        let code: Vec<char> = " { ".chars().collect();
        let result = DELIMITERS_TRIE.match_symbol(&code, 1, false);
        assert_eq!(result, Some(("{".to_string(), 2)));
    }

    #[test]
    fn test_partial_delimiter() {
        let code: Vec<char> = "}".chars().collect();
        let result = DELIMITERS_TRIE.match_symbol(&code, 0, false);
        assert_eq!(result, Some(("}".to_string(), 1)));
    }

    #[test]
    fn test_keyword_match() {
        let code: Vec<char> = "let".chars().collect();
        let result = KEYWORDS_TRIE.match_symbol(&code, 0, false);
        assert_eq!(result, Some(("let".to_string(), 3)));
    }

    #[test]
    fn test_non_keyword_match() {
        let code: Vec<char> = "foo".chars().collect();
        let result = KEYWORDS_TRIE.match_symbol(&code, 0, false);
        assert_eq!(result, None);
    }

    #[test]
    fn test_keyword_with_prefix() {
        let code: Vec<char> = "letVar".chars().collect();
        let result = KEYWORDS_TRIE.match_symbol(&code, 0, true);
        assert_eq!(result, None);
    }

    #[test]
    fn test_partial_keyword_match() {
        let code: Vec<char> = "le".chars().collect();
        let result = KEYWORDS_TRIE.match_symbol(&code, 0, true);
        assert_eq!(result, None);
    }

    #[test]
    fn test_keyword_with_spaces() {
        let code: Vec<char> = " let ".chars().collect();
        let result = KEYWORDS_TRIE.match_symbol(&code, 1, false);
        assert_eq!(result, Some(("let".to_string(), 4)));
    }
}
