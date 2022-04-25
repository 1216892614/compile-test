mod keep_for_use {
    use super::err::{LexicalAnalysisErr, LexicalAnalysisResult};
    use serde_json::{Map, Value};
    use std::collections::HashMap;

    #[derive(Debug)]
    pub(super) struct Keep {
        pub(super) keep_words: HashMap<String, String>,
        pub(super) keep_symbol: HashMap<String, String>,
        pub(super) keep_complex_symbol: HashMap<String, String>,
    }

    pub(super) fn keepwords_deserialization(jsonc: &str) -> LexicalAnalysisResult<Keep> {
        let root_json: Value = match serde_json::from_str(jsonc) {
            Ok(v) => v,
            Err(e) => return Err(LexicalAnalysisErr::FailToSerializationKeyWordsJson(e)),
        };

        let mut keep = Keep {
            keep_words: HashMap::new(),
            keep_symbol: HashMap::new(),
            keep_complex_symbol: HashMap::new(),
        };

        match root_json {
            Value::Object(object) => {
                insert_all_keepwords(&mut keep, object)?;
            }
            _ => return Err(LexicalAnalysisErr::FailToReadKeyWordsJsonTree),
        }

        Ok(keep)
    }

    fn insert_all_keepwords(
        keep: &mut Keep,
        object: Map<String, Value>,
    ) -> LexicalAnalysisResult<()> {
        for (source, symbol) in object {
            if super::is_made_entirely_alphanumeric_or_empty(&source) {
                keep.keep_words
                    .insert(source, safe_get_str_from_value(symbol)?);
            } else if source.len() > 1 && !super::is_made_entirely_alphanumeric_or_empty(&source) {
                keep.keep_complex_symbol
                    .insert(source, safe_get_str_from_value(symbol)?);
            } else if source.len() == 1 && !super::is_made_entirely_alphanumeric_or_empty(&source) {
                keep.keep_symbol
                    .insert(source, safe_get_str_from_value(symbol)?);
            } else {
                return Err(LexicalAnalysisErr::UnsupportedKeywords(source));
            }
        }

        Ok(())
    }

    fn safe_get_str_from_value(symbol: Value) -> LexicalAnalysisResult<String> {
        if let Value::String(s) = symbol {
            Ok(s)
        } else {
            return Err(LexicalAnalysisErr::SymbolOfKeywordsIsNotString);
        }
    }
}

fn is_made_entirely_alphanumeric_or_empty(str: &str) -> bool {
    str.chars()
        .all(|i| matches!(i,'a'..='z' | 'A'..='Z' | '_' | '0'..='9'))
}

#[derive(Debug)]
pub enum KeyType {
    KeepWord(String),
    Identifier(String),
    Value(i32),
}

use code_split::CodeSpliting;
use std::collections::HashMap;

fn code_split(
    code: String,
    complex_key_symbol: HashMap<String, String>,
) -> Result<Vec<String>, err::LexicalAnalysisErr> {
    let mut code_split = CodeSpliting::new(complex_key_symbol);
    for c in code.chars() {
        match c {
            'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => code_split.insert_alphanumeric_to_buf(c),
            ' ' | '\n' | '\t' => code_split.insert_not_empty_buf_to_result(),
            _ => code_split.insert_symbol_to_buf(c),
        }
    }
    //TODO
    println!("result: {:?}", code_split.result);
    Ok(code_split.result)
}

mod code_split {
    use std::collections::HashMap;

    use crate::is_made_entirely_alphanumeric_or_empty;

    pub(super) struct CodeSpliting {
        pub(super) result: Vec<String>,
        buf: String,
        keep_complex_symbol: HashMap<String, String>,
    }

    impl CodeSpliting {
        pub(super) fn new(keep_complex_symbol: HashMap<String, String>) -> Self {
            CodeSpliting {
                result: Vec::new(),
                buf: String::new(),
                keep_complex_symbol,
            }
        }

        pub(super) fn insert_not_empty_buf_to_result(&mut self) {
            if is_made_entirely_alphanumeric_or_empty(&self.buf) && !self.buf.is_empty() {
                self.move_alphanumeric_buf_to_result();
            } else {
                self.move_not_empty_symbol_buf_to_result();
            };
        }

        pub(super) fn insert_alphanumeric_to_buf(&mut self, c: char) {
            match &mut self.buf {
                buf if is_made_entirely_alphanumeric_or_empty(&buf) => buf.push(c),
                _ => {
                    self.move_not_empty_symbol_buf_to_result();
                    self.buf.push(c);
                }
            };
        }

        pub(super) fn insert_symbol_to_buf(&mut self, c: char) {
            match &mut self.buf {
                buf if buf.len() == 0 => self.buf.push(c),
                buf if is_made_entirely_alphanumeric_or_empty(&buf) => {
                    self.move_alphanumeric_buf_to_result();
                    self.buf.push(c);
                }
                _ => self.buf.push(c),
            };
        }

        fn move_alphanumeric_buf_to_result(&mut self) {
            self.result.push(self.buf.clone());
            self.buf.clear();
        }

        fn move_not_empty_symbol_buf_to_result(&mut self) {
            match self.buf.len() {
                0 => return,
                1 => self.result.push(self.buf.clone()),
                _ => self.matching_symbol_arr_and_move(),
            };
            self.buf.clear();
        }

        fn matching_symbol_arr_and_move(&mut self) {
            for i in (1..self.buf.len()).rev() {
                let mut close_2 = String::new();

                match self.buf.chars().nth(i - 1) {
                    Some(c) => close_2.push(c),
                    None => continue,
                }
                match self.buf.chars().nth(i) {
                    Some(c) => close_2.push(c),
                    None => continue,
                }

                self.insert_symbol(close_2);
            }
            self.clear_buf_to_result();
        }

        fn insert_symbol(&mut self, close_2: String) {
            match self.keep_complex_symbol.get(&close_2) {
                Some(_) => {
                    self.result.push(close_2);
                    self.buf = self.buf.chars().rev().collect();
                    self.buf.pop();
                    self.buf.pop();
                    self.buf = self.buf.chars().rev().collect();
                }
                None => {
                    self.buf = self.buf.chars().rev().collect();
                    self.result.push(self.buf.pop().unwrap().to_string());
                    self.buf = self.buf.chars().rev().collect();
                }
            }
        }

        fn clear_buf_to_result(&mut self) {
            for i in self.buf.chars() {
                self.result.push(i.to_string());
            }
            self.buf.clear();
        }
    }
}

fn words_match(
    word_vec: Vec<String>,
    keep_words: keep_for_use::Keep,
) -> err::LexicalAnalysisResult<Vec<KeyType>> {
    let mut result: Vec<KeyType> = Vec::new();
    for word in word_vec {
        match word {
            word if is_made_entirely_alphanumeric_or_empty(&word) => {
                match keep_words.keep_words.get(&word) {
                    Some(word) => result.push(KeyType::KeepWord(word.to_string())),
                    None => result.push(match word.parse::<i32>() {
                        Ok(num) => KeyType::Value(num),
                        Err(_) => KeyType::Identifier(word),
                    }),
                }
            }
            word if word.len() > 1 => match keep_words.keep_complex_symbol.get(&word) {
                Some(word) => result.push(KeyType::KeepWord(word.to_string())),
                None => {
                    return Err(err::LexicalAnalysisErr::UndefinedKeywords(format!(
                        "<ComplexSymbol: {}>",
                        word
                    )))
                }
            },
            _ => match keep_words.keep_symbol.get(&word) {
                Some(word) => result.push(KeyType::KeepWord(word.to_string())),
                None => {
                    return Err(err::LexicalAnalysisErr::UndefinedKeywords(format!(
                        "<SingleSymbol: {}>",
                        word
                    )))
                }
            },
        };
    }
    Ok(result)
}

pub fn lexical_analysis(
    code: &str,
    key_words_json: &str,
) -> err::LexicalAnalysisResult<Vec<KeyType>> {
    let key_words = keep_for_use::keepwords_deserialization(key_words_json)?;
    Ok(words_match(
        code_split(code.to_string(), key_words.keep_complex_symbol.clone())?,
        key_words,
    )?)
}

mod err {
    use serde_json;
    use std::fmt::Display;

    pub type LexicalAnalysisResult<T> = Result<T, LexicalAnalysisErr>;

    #[derive(Debug)]
    pub enum LexicalAnalysisErr {
        FailToSerializationKeyWordsJson(serde_json::Error),
        FailToReadKeyWordsJsonTree,
        SymbolOfKeywordsIsNotString,
        UnsupportedKeywords(String),
        UndefinedKeywords(String),
    }

    impl Display for LexicalAnalysisErr {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match &*self {
                LexicalAnalysisErr::FailToSerializationKeyWordsJson(ref e) => e.fmt(f),
                LexicalAnalysisErr::FailToReadKeyWordsJsonTree => write!(
                    f,
                    "Be sure your keywords JSON same as: {{ \"source str\": \"symbol str\" }}."
                ),
                LexicalAnalysisErr::SymbolOfKeywordsIsNotString => write!(
                    f,
                    "Be sure your keywords JSON same as: {{ \"source str\": \"symbol str\" }}."
                ),
                LexicalAnalysisErr::UnsupportedKeywords(s) => {
                    write!(
                        f,
                        "{:?} is not a supported word for this lexical_analysis.",
                        s
                    )
                }
                LexicalAnalysisErr::UndefinedKeywords(s) => {
                    write!(f, "{:?} is not a defined keep word for this language.", s)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        code_split, err, keep_for_use::keepwords_deserialization, lexical_analysis, KeyType,
    };
    use std::io::Read;

    #[test]
    fn it_works_0() {
        println!(
            "{:?}",
            run_lexical_analysis(
                " int main()
                {
                    return 1;
                }"
            )
            .unwrap()
        );
    }

    #[test]
    fn it_works_1() {
        println!(
            "{:?}",
            run_lexical_analysis(
                " int main()
                {
                    int a=10,b=20,max;
                    if (a>=b)
                         max=a;
                    else max=b;
                    return 1;
                }"
            )
            .unwrap()
        );
    }

    #[test]
    fn it_works_2() {
        println!(
            "{:?}",
            run_lexical_analysis(
                " int while if else return
                =
                + - * /
                < <= > >= == !=
                # @"
            )
        );
    }

    fn run_lexical_analysis(test_set: &str) -> err::LexicalAnalysisResult<Vec<KeyType>> {
        let mut json = std::fs::File::open("static/keep_str.jsonc").unwrap();
        let mut txt = String::new();
        json.read_to_string(&mut txt).unwrap();
        lexical_analysis(test_set, &txt)
    }

    #[test]
    fn code_split_works() {
        let code = " int main()
        {
            int a=10,b=20,max;
            if (a>=b)
            max=a;
            else max=>b;
            return 1;
            @
        }";

        let mut json = std::fs::File::open("static/keep_str.jsonc").unwrap();
        let mut txt = String::new();
        json.read_to_string(&mut txt).unwrap();

        println!(
            "{:?}",
            code_split(
                code.to_string(),
                keepwords_deserialization(&txt).unwrap().keep_complex_symbol
            )
        );
    }
}
