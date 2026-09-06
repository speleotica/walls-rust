use std::{
    ops::{Add, AddAssign},
    range::Range,
    sync::LazyLock,
};

use regex::Regex;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(JsonSchema, Serialize, Deserialize, Eq, PartialEq, Debug, Copy, Clone)]
#[schemars(deny_unknown_fields)]
pub struct SourcePos {
    pub line: usize,
    #[serde(rename = "col")]
    pub column: usize,
    #[serde(rename = "idx")]
    pub index: usize,
    #[serde(skip)]
    pub byte_pos: usize,
}

impl Add<&str> for SourcePos {
    type Output = SourcePos;

    fn add(self, rhs: &str) -> SourcePos {
        let mut end = self;
        end.byte_pos += rhs.len();
        let char_count = rhs.chars().count();
        end.index += char_count;
        end.line += rhs.bytes().filter(|&b| b == b'\n').count();
        if end.line == self.line {
            end.column = self.column + char_count;
        } else {
            match rhs.rfind('\n') {
                Some(index) => end.column = rhs[index..].chars().count(),
                None => end.column = 1,
            }
        }
        end
    }
}

impl AddAssign<&str> for SourcePos {
    fn add_assign(&mut self, rhs: &str) {
        let char_count = rhs.chars().count();
        self.byte_pos += rhs.len();
        self.index += char_count;
        let newline_count = rhs.bytes().filter(|&b| b == b'\n').count();
        self.line += newline_count;
        if newline_count == 0 {
            self.column += char_count;
        } else {
            match rhs.rfind('\n') {
                Some(index) => self.column = rhs[index..].chars().count(),
                None => self.column = 1,
            }
        }
    }
}

impl SourcePos {
    pub fn origin() -> SourcePos {
        SourcePos {
            line: 1,
            column: 1,
            index: 0,
            byte_pos: 0,
        }
    }

    pub fn span_of(self, slice: &str) -> SourceLoc {
        self.up_to(self + slice)
    }

    pub fn up_to(self, end: SourcePos) -> SourceLoc {
        SourceLoc::new(self, end)
    }
}

#[derive(JsonSchema, Serialize, Deserialize, Eq, PartialEq, Debug, Copy, Clone)]
#[schemars(deny_unknown_fields)]
pub struct SourceLoc {
    pub start: SourcePos,
    pub end: SourcePos,
}

impl SourceLoc {
    pub fn new(start: SourcePos, end: SourcePos) -> SourceLoc {
        assert!(start.index <= end.index);
        assert!(start.byte_pos <= end.byte_pos);
        assert!(start.line <= end.line);
        assert!(start.line < end.line || start.column <= end.column);
        SourceLoc { start, end }
    }
    pub fn range_of_str(str: &str, range: &Range<usize>) -> SourceLoc {
        let start = SourcePos::origin() + &str[0..range.start];
        let end = start + &str[range.start..range.end];
        start.up_to(end)
    }
    pub fn str_from_to(str: &str, from: usize, to: usize) -> SourceLoc {
        let start = SourcePos::origin() + &str[0..from];
        let end = start + &str[from..to];
        start.up_to(end)
    }
    pub fn find_range(text: &str, pat: &str) -> Option<SourceLoc> {
        text.find(pat)
            .map(|start| SourceLoc::str_from_to(text, start, start + pat.len()))
    }
}

impl From<SourcePos> for SourceLoc {
    fn from(pos: SourcePos) -> Self {
        pos.up_to(pos)
    }
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub enum ParseIssueSeverity {
    Error,
    Warning,
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct ParseIssue {
    pub severity: ParseIssueSeverity,
    pub code: String,
    pub message: Option<String>,
    pub loc: Option<SourceLoc>,
}

impl ParseIssue {
    pub fn new(
        severity: ParseIssueSeverity,
        code: &str,
        message: Option<String>,
        loc: Option<SourceLoc>,
    ) -> ParseIssue {
        ParseIssue {
            severity,
            code: code.into(),
            message,
            loc,
        }
    }
    pub fn is_error(&self) -> bool {
        self.severity == ParseIssueSeverity::Error
    }
    pub fn error(code: &str, message: Option<String>, loc: Option<SourceLoc>) -> ParseIssue {
        ParseIssue {
            severity: ParseIssueSeverity::Error,
            code: code.into(),
            message,
            loc,
        }
    }
    pub fn warning(code: &str, message: Option<String>, loc: Option<SourceLoc>) -> ParseIssue {
        ParseIssue {
            severity: ParseIssueSeverity::Warning,
            code: code.into(),
            message,
            loc,
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct ParseMatch<'h> {
    str: &'h str,
    start: SourcePos,
}

impl<'h> ParseMatch<'h> {
    pub fn slice(&self, start: usize, end: usize) -> ParseMatch<'h> {
        ParseMatch {
            str: &self.str[start..end],
            start: self.start + &self.str[0..start],
        }
    }

    /// Returns the byte offset of the start of the match in the haystack. The
    /// start of the match corresponds to the position where the match begins
    /// and includes the first byte in the match.
    ///
    /// It is guaranteed that `Match::start() <= Match::end()`.
    ///
    /// This is guaranteed to fall on a valid UTF-8 codepoint boundary. That
    /// is, it will never be an offset that appears between the UTF-8 code
    /// units of a UTF-8 encoded Unicode scalar value. Consequently, it is
    /// always safe to slice the corresponding haystack using this offset.
    #[inline]
    pub fn start(&self) -> usize {
        self.start.byte_pos
    }

    /// Returns the `SourcePos` of the start of the match.
    #[inline]
    pub fn start_pos(&self) -> SourcePos {
        self.start
    }

    /// Returns the byte offset of the end of the match in the haystack. The
    /// end of the match corresponds to the byte immediately following the last
    /// byte in the match. This means that `&slice[start..end]` works as one
    /// would expect.
    ///
    /// It is guaranteed that `Match::start() <= Match::end()`.
    ///
    /// This is guaranteed to fall on a valid UTF-8 codepoint boundary. That
    /// is, it will never be an offset that appears between the UTF-8 code
    /// units of a UTF-8 encoded Unicode scalar value. Consequently, it is
    /// always safe to slice the corresponding haystack using this offset.
    #[inline]
    pub fn end(&self) -> usize {
        self.start.byte_pos + self.str.len()
    }

    /// Returns the `SourcePos` of the start of the match.
    #[inline]
    pub fn end_pos(&self) -> SourcePos {
        self.start + self.str
    }

    /// Returns the `SourceLoc` of the match.
    #[inline]
    pub fn loc(&self) -> SourceLoc {
        self.start.span_of(self.str)
    }

    /// Returns true if and only if this match has a length of zero.
    ///
    /// Note that an empty match can only occur when the regex itself can
    /// match the empty string. Here are some examples of regexes that can
    /// all match the empty string: `^`, `^$`, `\b`, `a?`, `a*`, `a{0}`,
    /// `(foo|\d+|quux)?`.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.str.is_empty()
    }

    /// Returns the length, in bytes, of this match.
    #[inline]
    pub fn len(&self) -> usize {
        self.str.len()
    }

    /// Returns the range over the starting and ending byte offsets of the
    /// match in the haystack.
    ///
    /// It is always correct to slice the original haystack searched with this
    /// range. That is, because the offsets are guaranteed to fall on valid
    /// UTF-8 boundaries, the range returned is always valid.
    #[inline]
    pub fn range(&self) -> core::ops::Range<usize> {
        self.start()..self.end()
    }

    /// Returns the substring of the haystack that matched.
    #[inline]
    pub fn as_str(&self) -> &'h str {
        &self.str
    }

    /// Creates a new `ParseState` with this match as input
    #[inline]
    pub fn reparse(&self) -> ParseState<'h> {
        ParseState::new(&self.str, self.start)
    }

    /// Creates a new match from the given haystack and byte offsets.
    #[inline]
    fn new(str: &'h str, start: SourcePos) -> ParseMatch<'h> {
        ParseMatch { str, start }
    }
}

impl<'h> core::fmt::Debug for ParseMatch<'h> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Match")
            .field("start", &self.start())
            .field("end", &self.end())
            .field("string", &self.as_str())
            .finish()
    }
}

impl<'h> From<ParseMatch<'h>> for &'h str {
    fn from(m: ParseMatch<'h>) -> &'h str {
        m.as_str()
    }
}

impl<'h> From<ParseMatch<'h>> for String {
    fn from(m: ParseMatch<'h>) -> String {
        m.as_str().into()
    }
}

impl<'h> From<ParseMatch<'h>> for Option<String> {
    fn from(m: ParseMatch<'h>) -> Option<String> {
        Some(m.as_str().into())
    }
}

impl<'h> From<ParseMatch<'h>> for SourceLoc {
    fn from(m: ParseMatch<'h>) -> SourceLoc {
        m.loc()
    }
}

impl<'h> From<ParseMatch<'h>> for Option<SourceLoc> {
    fn from(m: ParseMatch<'h>) -> Option<SourceLoc> {
        Some(m.loc())
    }
}

impl<'h> From<ParseMatch<'h>> for core::ops::Range<usize> {
    fn from(m: ParseMatch<'h>) -> core::ops::Range<usize> {
        m.range()
    }
}

pub struct ParseState<'i> {
    input: &'i str,
    index: usize,
    pos: SourcePos,
}

fn check_parse_regex(regex: &Regex) {
    let s = regex.as_str();
    if !s.starts_with("^") {
        panic!("Parse regex doesn't start with ^: {s}")
    }
}

const REST: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^.*").unwrap());

impl<'i> ParseState<'i> {
    pub fn new(input: &'i str, pos: SourcePos) -> ParseState<'i> {
        ParseState {
            input,
            index: 0,
            pos,
        }
    }

    /// Returns the current parse index.
    pub fn index(&self) -> usize {
        self.index
    }

    pub fn pos(&self) -> SourcePos {
        self.pos
    }

    /// Returns `true` if the current parse index has reached the end of
    /// the input.
    pub fn is_done(&self) -> bool {
        self.index >= self.input.len()
    }

    /// Returns a match if `regex` (which must start with ^) matches at
    /// the current parse index.
    pub fn peek(&self, regex: &Regex) -> Option<ParseMatch<'i>> {
        check_parse_regex(regex);
        match regex.find(&self.input[self.index..]) {
            Some(m) => Some(ParseMatch::new(m.as_str(), self.pos)),
            None => None,
        }
    }

    /// Returns `true` if `regex` (which must start with ^) matches at
    /// the current parse index, and advances the parse index to the end
    /// of the match.
    pub fn is_match(&mut self, regex: &Regex) -> bool {
        check_parse_regex(regex);
        match regex.find(&self.input[self.index..]) {
            Some(m) => {
                self.index += m.end();
                self.pos += m.as_str();
                true
            }
            None => false,
        }
    }

    /// Returns a match if `regex` (which must start with ^) matches at
    /// the current parse index, and advances the parse index to the end
    /// of the match.
    pub fn find(&mut self, regex: &Regex) -> Option<ParseMatch<'i>> {
        check_parse_regex(regex);
        match regex.find(&self.input[self.index..]) {
            Some(m) => {
                let result = Some(ParseMatch::new(m.as_str(), self.pos));
                self.index += m.end();
                self.pos += m.as_str();
                result
            }
            None => None,
        }
    }

    pub fn rest(&mut self) -> ParseMatch<'i> {
        self.find(&REST).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_source_pos_add() {
        assert_eq!(
            SourcePos::origin() + "\n",
            SourcePos {
                line: 2,
                column: 1,
                index: 1,
                byte_pos: 1,
            }
        );
        assert_eq!(
            SourcePos::origin() + "\r\n",
            SourcePos {
                line: 2,
                column: 1,
                index: 2,
                byte_pos: 2,
            }
        );

        assert_eq!(
            SourcePos::origin() + "foobar",
            SourcePos {
                line: 1,
                column: 7,
                index: 6,
                byte_pos: 6,
            }
        );

        assert_eq!(
            SourcePos::origin() + "foo\nbar",
            SourcePos {
                line: 2,
                column: 4,
                index: 7,
                byte_pos: 7,
            }
        );

        assert_eq!(
            SourcePos::origin() + "foo\n\nbar",
            SourcePos {
                line: 3,
                column: 4,
                index: 8,
                byte_pos: 8,
            }
        );

        assert_eq!(
            SourcePos::origin() + "foo\n\nba\nr",
            SourcePos {
                line: 4,
                column: 2,
                index: 9,
                byte_pos: 9
            }
        );

        assert_eq!(
            SourcePos {
                line: 10,
                column: 5,
                index: 8,
                byte_pos: 8,
            } + "foobar",
            SourcePos {
                line: 10,
                column: 11,
                index: 14,
                byte_pos: 14
            }
        );

        assert_eq!(
            SourcePos {
                line: 10,
                column: 5,
                index: 8,
                byte_pos: 8
            } + "foo\n\nba\nr",
            SourcePos {
                line: 13,
                column: 2,
                index: 17,
                byte_pos: 17
            }
        );
    }

    #[test]
    fn test_parse_state() {
        let mut p = ParseState {
            input: "foobar",
            index: 0,
            pos: SourcePos::origin(),
        };

        let foo = Regex::new(r"^foo").unwrap();
        let bar = Regex::new(r"^bar").unwrap();

        let m = p.find(&foo);
        assert_eq!(m.map(|m| m.as_str()), Some("foo"));

        let m = p.find(&foo);
        assert_eq!(m.is_some(), false);
        assert_eq!(p.pos().byte_pos, 3);

        let m = p.find(&foo);
        assert_eq!(m.is_some(), false);

        let m = p.find(&bar);
        assert_eq!(m.is_some(), true);
        assert_eq!(p.is_done(), true);
    }
}
