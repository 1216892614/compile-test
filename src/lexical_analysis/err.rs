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
