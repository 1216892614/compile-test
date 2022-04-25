use std::collections::HashMap;

use super::is_made_entirely_alphanumeric_or_empty;

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
