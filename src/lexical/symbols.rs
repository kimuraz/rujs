use std::collections::HashMap;
use std::sync::LazyLock;

#[derive(Debug)]
struct SymbolTrie {
    children: HashMap<char, SymbolTrie>,
    is_last: bool,
}

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

    pub fn match_symbol<'a>(&self, code: &'a [char], start: usize) -> Option<(String, usize)> {
        let mut node = self;
        let mut matched = String::new();
        let mut last_match = None;

        for (i, &ch) in code.iter().enumerate().skip(start) {
            if let Some(child) = node.children.get(&ch) {
                matched.push(ch);
                node = child;

                if node.is_last {
                    last_match = Some((matched.clone(), i + 1));
                }
            } else {
                break;
            }
        }

        last_match
    }

}

static OPERATORS_TRIE: LazyLock<SymbolTrie> = LazyLock::new(|| {
    SymbolTrie::new(&[
        "+", "-", "*", "**", "/", "%", "==", "!=", "<", "<=", ">",
        ">=", "&&", "||", "!", "=", "+=", "-=", "*=", "/=", "%=",
        "===", ".", "...",
    ])
});

static DELIMITERS_TRIE: LazyLock<SymbolTrie> = LazyLock::new(|| {
    SymbolTrie::new(&["(", ")", "{", "}", "[", "]", ",", ";", ":"])
});

static KEYWORDS_TRIE: LazyLock<SymbolTrie> = LazyLock::new(|| {
    SymbolTrie::new(&[
        "if", "else", "for", "while", "do", "break", "continue", "return", 
        "function", "var", "let", "const", "true", "false", "null", "undefined",
        "new", "class", "extends", "this", "super", "import", "from",
    ])
});


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_operator_match() {
        let code: Vec<char> = "==".chars().collect();
        let result = OPERATORS_TRIE.match_symbol(&code, 0);
        assert_eq!(result, Some(("==".to_string(), 2))); // Matches "=="
    }

    #[test]
    fn test_non_operator_match() {
        let code: Vec<char> = "foo".chars().collect();
        let result = OPERATORS_TRIE.match_symbol(&code, 0);
        assert_eq!(result, None);
    }

    #[test]
    fn test_longer_operator_match() {
        let code: Vec<char> = "===".chars().collect();
        let result = OPERATORS_TRIE.match_symbol(&code, 0);
        assert_eq!(result, Some(("===".to_string(), 3)));
    }

    #[test]
    fn test_partial_operator_match() {
        let code: Vec<char> = "=".chars().collect();
        let result = OPERATORS_TRIE.match_symbol(&code, 0);
        assert_eq!(result, Some(("=".to_string(), 1)));
    }

    #[test]
    fn test_operator_with_spaces() {
        let code: Vec<char> = " == ".chars().collect();
        let result = OPERATORS_TRIE.match_symbol(&code, 1);
        assert_eq!(result, Some(("==".to_string(), 3)));
    }

    #[test]
    fn test_delimiter_match() {
        let code: Vec<char> = "(".chars().collect();
        let result = DELIMITERS_TRIE.match_symbol(&code, 0);
        assert_eq!(result, Some(("(".to_string(), 1)));
    }

    #[test]
    fn test_non_delimiter_match() {
        let code: Vec<char> = "foo".chars().collect();
        let result = DELIMITERS_TRIE.match_symbol(&code, 0);
        assert_eq!(result, None);
    }

    #[test]
    fn test_code_with_dot_and_spread() {
        let code: Vec<char> = "obj.prop".chars().collect();

        let result_dot = OPERATORS_TRIE.match_symbol(&code, 3);
        assert_eq!(result_dot, Some((".".to_string(), 4)));
    }

    #[test]
    fn test_code_with_spread_operator() {
        let code: Vec<char> = "...args".chars().collect();

        let result_spread = OPERATORS_TRIE.match_symbol(&code, 0);
        assert_eq!(result_spread, Some(("...".to_string(), 3)));
    }

    #[test]
    fn test_multiple_delimiters() {
        let code: Vec<char> = "{[(".chars().collect();
        let result1 = DELIMITERS_TRIE.match_symbol(&code, 0);
        let result2 = DELIMITERS_TRIE.match_symbol(&code, 1);
        let result3 = DELIMITERS_TRIE.match_symbol(&code, 2);

        assert_eq!(result1, Some(("{".to_string(), 1)));
        assert_eq!(result2, Some(("[".to_string(), 2)));
        assert_eq!(result3, Some(("(".to_string(), 3)));
    }

    #[test]
    fn test_delimiter_with_spaces() {
        let code: Vec<char> = " { ".chars().collect();
        let result = DELIMITERS_TRIE.match_symbol(&code, 1);
        assert_eq!(result, Some(("{".to_string(), 2)));
    }

    #[test]
    fn test_partial_delimiter() {
        let code: Vec<char> = "}".chars().collect();
        let result = DELIMITERS_TRIE.match_symbol(&code, 0);
        assert_eq!(result, Some(("}".to_string(), 1)));
    }

    #[test]
    fn test_keyword_match() {
        let code: Vec<char> = "let".chars().collect();
        let result = KEYWORDS_TRIE.match_symbol(&code, 0);
        assert_eq!(result, Some(("let".to_string(), 3)));
    }

    #[test]
    fn test_non_keyword_match() {
        let code: Vec<char> = "foo".chars().collect();
        let result = KEYWORDS_TRIE.match_symbol(&code, 0);
        assert_eq!(result, None); // "foo" is not a keyword
    }

    #[test]
    fn test_keyword_with_prefix() {
        let code: Vec<char> = "letVar".chars().collect();
        let result = KEYWORDS_TRIE.match_symbol(&code, 0);
        assert_eq!(result, Some(("let".to_string(), 3))); // Matches "let" as a prefix
    }

    #[test]
    fn test_partial_keyword_match() {
        let code: Vec<char> = "le".chars().collect();
        let result = KEYWORDS_TRIE.match_symbol(&code, 0);
        assert_eq!(result, None);
    }

    #[test]
    fn test_keyword_with_spaces() {
        let code: Vec<char> = " let ".chars().collect();
        let result = KEYWORDS_TRIE.match_symbol(&code, 1);
        assert_eq!(result, Some(("let".to_string(), 4)));
    }
}
