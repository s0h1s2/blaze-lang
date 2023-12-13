use std::cmp;

pub struct SourceText<'a> {
    lines: Vec<String>,
    source: &'a str,
}

impl<'a> SourceText<'a> {
    pub fn new(input: &'a str) -> Self {
        return SourceText {
            source: input,
            lines: input.lines().map(|l| l.to_string()).collect(),
        };
    }
    pub fn get_line_by_pos(&self, pos: usize) -> Option<(usize, usize, String)> {
        let mut col_acc = 0;
        for line in 0..self.lines.len() {
            let line_len = self.lines[line].len();
            col_acc += line_len;
            if pos <= col_acc {
                let col = cmp::max(pos - (col_acc - line_len + 1), 1);
                return Some((line + 1, col, self.lines[line].clone()));
            }
        }
        None
    }
    pub fn get_literal(&self, start: usize, end: usize) -> &str {
        &self.source[start..end]
    }
}
