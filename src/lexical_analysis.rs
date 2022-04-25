pub mod err;
pub use err::{LexicalAnalysisErr, LexicalAnalysisResult};
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

use std::collections::HashMap;
mod code_split;
use code_split::CodeSpliting;
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
    Ok(code_split.result)
}

mod keep_for_use;
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

#[cfg(test)]
mod tests {
    use super::{
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
