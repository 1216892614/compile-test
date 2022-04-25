
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

fn insert_all_keepwords(keep: &mut Keep, object: Map<String, Value>) -> LexicalAnalysisResult<()> {
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
