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
        let mut p = pos;
        None
    }
    pub fn get_literal(&self, start: usize, end: usize) -> &str {
        &self.source[start..end]
    }
}
