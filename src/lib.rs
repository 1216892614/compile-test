mod keep_for_use {
    use super::err::{LexicalAnalysisErr, LexicalAnalysisResult};
    use serde_json::{Map, Value};
    use std::collections::HashMap;

    #[derive(Debug)]
    pub(super) struct Keep {
        KeepWords: HashMap<String, String>,
        KeepSymbol: HashMap<String, String>,
        KeepComplexSymbol: HashMap<String, String>,
    }

    pub(super) fn keepwords_deserialization(jsonc: &str) -> LexicalAnalysisResult<Keep> {
        let root_json: Value = match serde_json::from_str(jsonc) {
            Ok(v) => v,
            Err(e) => return Err(LexicalAnalysisErr::FailToSerializationKeyWordsJson(e)),
        };

        let mut keep = Keep {
            KeepWords: HashMap::new(),
            KeepSymbol: HashMap::new(),
            KeepComplexSymbol: HashMap::new(),
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
            if super::is_made_entirely_alphanumeric(&source) {
                keep.KeepWords
                    .insert(source, safe_get_str_from_value(symbol)?);
            } else if source.len() > 1 && !super::is_made_entirely_alphanumeric(&source) {
                keep.KeepComplexSymbol
                    .insert(source, safe_get_str_from_value(symbol)?);
            } else if source.len() == 1 && !super::is_made_entirely_alphanumeric(&source) {
                keep.KeepSymbol
                    .insert(source, safe_get_str_from_value(symbol)?);
            } else {
                return Err(LexicalAnalysisErr::unsupported_keywords(source));
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

fn is_made_entirely_alphanumeric(str: &str) -> bool {
    str.chars()
        .any(|i| matches!(i,'a'..='z' | 'A'..='Z' | '_' | '0'..='9'))
}

pub enum KeyType<'a> {
    KeepWord(&'a str),
    Identifier(&'a str),
    Value(i32),
}

fn code_split(code: String) -> Result<Vec<String>, err::LexicalAnalysisErr> {
    let mut result = Vec::new();
    let mut buf = String::new();
    for c in code.chars() {
        code_split::distinguish_between_vocabulary_boundaries(&c, &mut buf, &mut result);
    }
    Ok(result)
}

mod code_split {
    use crate::is_made_entirely_alphanumeric;

    pub(super) fn distinguish_between_vocabulary_boundaries(
        char_for_now: &char,
        buf: &mut String,
        result: &mut Vec<String>,
    ) {
        match char_for_now {
            ' ' | '\n' | '\t' => push_buf_not_empty_to_result(buf, result),
            'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => {
                push_letter_num_to_result(char_for_now, buf, result)
            }
            _ => todo!(),
        }
    }

    fn push_buf_not_empty_to_result(buf: &mut String, result: &mut Vec<String>) {
        if buf.len() != 0 {
            result.push(buf.to_string())
        }
        buf.clear();
    }

    fn push_letter_num_to_result(char_for_now: &char, buf: &mut String, result: &mut Vec<String>) {
        match buf {
            buf if is_made_entirely_alphanumeric(buf) || buf.len() == 0 => buf.push(*char_for_now),
            _ => todo!(),
        }
    }
}

fn words_match<'a>(word_vec: Vec<String>) -> err::LexicalAnalysisResult<KeyType<'a>> {
    todo!()
}

pub fn lexical_analysis(code: &str) -> err::LexicalAnalysisResult<KeyType> {
    Ok(words_match(code_split(code.to_string())?)?)
}

mod err {
    use serde_json;
    use std::{error::Error, fmt::Display};

    pub type LexicalAnalysisResult<T> = Result<T, LexicalAnalysisErr>;

    #[derive(Debug)]
    pub enum LexicalAnalysisErr {
        FailToSerializationKeyWordsJson(serde_json::Error),
        FailToReadKeyWordsJsonTree,
        SymbolOfKeywordsIsNotString,
        unsupported_keywords(String),
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
                LexicalAnalysisErr::unsupported_keywords(s) => {
                    write!(f, "{:?} is not a supported word for this language.", s)
                }
            }
        }
    }

    impl Error for LexicalAnalysisErr {}
}

#[cfg(test)]
mod tests {
    use crate::{lexical_analysis, KeyType};

    #[test]
    fn it_works() {
        let test_set = vec![
            " int main()
            {
                return 1;
            }",
            " int main()
            {
                int a=10,b=20,max;
                if (a>=b)
                     max=a;
               else max=b;
                return 1;
            }",
            " int while if else return
            =
            + - * /
            < <= > >= == !=
            # @",
            " @
            #",
        ];
        let mut word_map: Vec<KeyType> = Vec::new();
        for i in test_set {
            word_map.push(lexical_analysis(i).unwrap());
        }
    }

    use crate::keep_for_use::{keepwords_deserialization, Keep};
    use std::io::Read;
    #[test]
    fn json_to_keywords() {
        let mut json = std::fs::File::open("static/keep_str.jsonc").unwrap();
        let mut txt = String::new();
        json.read_to_string(&mut txt).unwrap();
        println!("{:?}", keepwords_deserialization(&txt))
    }
}
