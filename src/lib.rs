mod lexical_analysis;

use lexical_analysis::{err::LexicalAnalysisResult, lexical_analysis, KeyType};
pub fn compile(code: &str, key_words_json: &str) -> LexicalAnalysisResult<Vec<KeyType>> {
    lexical_analysis(code, key_words_json)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(1, 1)
    }
}
