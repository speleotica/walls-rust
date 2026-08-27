use regex::{Captures, Match, Regex};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct SourcePos {
    line: u32,
    column: u32,
    index: u32,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct SourceLoc {
    start: SourcePos,
    end: SourcePos,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub enum ParseIssueSeverity {
    Error,
    Warning,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct ParseIssue {
    severity: ParseIssueSeverity,
    code: String,
    message: Option<String>,
    loc: Option<SourceLoc>,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ParseMatch<'h> {
    m: Match<'h>,
    offs: usize,
}

impl<'h> ParseMatch<'h> {
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
        self.m.start() + self.offs
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
        self.m.end() + self.offs
    }

    /// Returns true if and only if this match has a length of zero.
    ///
    /// Note that an empty match can only occur when the regex itself can
    /// match the empty string. Here are some examples of regexes that can
    /// all match the empty string: `^`, `^$`, `\b`, `a?`, `a*`, `a{0}`,
    /// `(foo|\d+|quux)?`.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.m.is_empty()
    }

    /// Returns the length, in bytes, of this match.
    #[inline]
    pub fn len(&self) -> usize {
        self.m.len()
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
        &self.m.as_str()
    }

    /// Creates a new match from the given haystack and byte offsets.
    #[inline]
    fn new(m: Match<'h>, offs: usize) -> ParseMatch<'h> {
        ParseMatch { m, offs }
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

impl<'h> From<ParseMatch<'h>> for core::ops::Range<usize> {
    fn from(m: ParseMatch<'h>) -> core::ops::Range<usize> {
        m.range()
    }
}

pub struct ParseCaptures<'h> {
    captures: Captures<'h>,
    offs: usize,
}

impl<'h> ParseCaptures<'h> {
    /// Returns the `Match` associated with the capture group at index `i`. If
    /// `i` does not correspond to a capture group, or if the capture group did
    /// not participate in the match, then `None` is returned.
    ///
    /// When `i == 0`, this is guaranteed to return a non-`None` value.
    ///
    /// # Examples
    ///
    /// Get the substring that matched with a default of an empty string if the
    /// group didn't participate in the match:
    ///
    /// ```
    /// use regex::Regex;
    ///
    /// let re = Regex::new(r"[a-z]+(?:([0-9]+)|([A-Z]+))").unwrap();
    /// let caps = re.captures("abc123").unwrap();
    ///
    /// let substr1 = caps.get(1).map_or("", |m| m.as_str());
    /// let substr2 = caps.get(2).map_or("", |m| m.as_str());
    /// assert_eq!(substr1, "123");
    /// assert_eq!(substr2, "");
    /// ```
    #[inline]
    pub fn get(&self, i: usize) -> Option<ParseMatch<'h>> {
        match self.captures.get(i) {
            Some(m) => Some(ParseMatch::new(m, self.offs)),
            None => None,
        }
    }

    /// Return the overall match for the capture.
    ///
    /// This returns the match for index `0`. That is it is equivalent to
    /// `m.get(0).unwrap()`
    ///
    /// # Example
    ///
    /// ```
    /// use regex::Regex;
    ///
    /// let re = Regex::new(r"[a-z]+([0-9]+)").unwrap();
    /// let caps = re.captures("   abc123-def").unwrap();
    ///
    /// assert_eq!(caps.get_match().as_str(), "abc123");
    ///
    /// ```
    #[inline]
    pub fn get_match(&self) -> ParseMatch<'h> {
        self.get(0).unwrap()
    }

    /// Returns the `Match` associated with the capture group named `name`. If
    /// `name` isn't a valid capture group or it refers to a group that didn't
    /// match, then `None` is returned.
    ///
    /// Note that unlike `caps["name"]`, this returns a `Match` whose lifetime
    /// matches the lifetime of the haystack in this `Captures` value.
    /// Conversely, the substring returned by `caps["name"]` has a lifetime
    /// of the `Captures` value, which is likely shorter than the lifetime of
    /// the haystack. In some cases, it may be necessary to use this method to
    /// access the matching substring instead of the `caps["name"]` notation.
    ///
    /// # Examples
    ///
    /// Get the substring that matched with a default of an empty string if the
    /// group didn't participate in the match:
    ///
    /// ```
    /// use regex::Regex;
    ///
    /// let re = Regex::new(
    ///     r"[a-z]+(?:(?<numbers>[0-9]+)|(?<letters>[A-Z]+))",
    /// ).unwrap();
    /// let caps = re.captures("abc123").unwrap();
    ///
    /// let numbers = caps.name("numbers").map_or("", |m| m.as_str());
    /// let letters = caps.name("letters").map_or("", |m| m.as_str());
    /// assert_eq!(numbers, "123");
    /// assert_eq!(letters, "");
    /// ```
    #[inline]
    pub fn name(&self, name: &str) -> Option<ParseMatch<'h>> {
        match self.captures.name(name) {
            Some(m) => Some(ParseMatch::new(m, self.offs)),
            None => None,
        }
    }

    /// This is a convenience routine for extracting the substrings
    /// corresponding to matching capture groups.
    ///
    /// This returns a tuple where the first element corresponds to the full
    /// substring of the haystack that matched the regex. The second element is
    /// an array of substrings, with each corresponding to the substring that
    /// matched for a particular capture group.
    ///
    /// # Panics
    ///
    /// This panics if the number of possible matching groups in this
    /// `Captures` value is not fixed to `N` in all circumstances.
    /// More precisely, this routine only works when `N` is equivalent to
    /// [`Regex::static_captures_len`].
    ///
    /// Stated more plainly, if the number of matching capture groups in a
    /// regex can vary from match to match, then this function always panics.
    ///
    /// For example, `(a)(b)|(c)` could produce two matching capture groups
    /// or one matching capture group for any given match. Therefore, one
    /// cannot use `extract` with such a pattern.
    ///
    /// But a pattern like `(a)(b)|(c)(d)` can be used with `extract` because
    /// the number of capture groups in every match is always equivalent,
    /// even if the capture _indices_ in each match are not.
    ///
    /// # Example
    ///
    /// ```
    /// use regex::Regex;
    ///
    /// let re = Regex::new(r"([0-9]{4})-([0-9]{2})-([0-9]{2})").unwrap();
    /// let hay = "On 2010-03-14, I became a Tennessee lamb.";
    /// let Some((full, [year, month, day])) =
    ///     re.captures(hay).map(|caps| caps.extract()) else { return };
    /// assert_eq!("2010-03-14", full);
    /// assert_eq!("2010", year);
    /// assert_eq!("03", month);
    /// assert_eq!("14", day);
    /// ```
    ///
    /// # Example: iteration
    ///
    /// This example shows how to use this method when iterating over all
    /// `Captures` matches in a haystack.
    ///
    /// ```
    /// use regex::Regex;
    ///
    /// let re = Regex::new(r"([0-9]{4})-([0-9]{2})-([0-9]{2})").unwrap();
    /// let hay = "1973-01-05, 1975-08-25 and 1980-10-18";
    ///
    /// let mut dates: Vec<(&str, &str, &str)> = vec![];
    /// for (_, [y, m, d]) in re.captures_iter(hay).map(|c| c.extract()) {
    ///     dates.push((y, m, d));
    /// }
    /// assert_eq!(dates, vec![
    ///     ("1973", "01", "05"),
    ///     ("1975", "08", "25"),
    ///     ("1980", "10", "18"),
    /// ]);
    /// ```
    ///
    /// # Example: parsing different formats
    ///
    /// This API is particularly useful when you need to extract a particular
    /// value that might occur in a different format. Consider, for example,
    /// an identifier that might be in double quotes or single quotes:
    ///
    /// ```
    /// use regex::Regex;
    ///
    /// let re = Regex::new(r#"id:(?:"([^"]+)"|'([^']+)')"#).unwrap();
    /// let hay = r#"The first is id:"foo" and the second is id:'bar'."#;
    /// let mut ids = vec![];
    /// for (_, [id]) in re.captures_iter(hay).map(|c| c.extract()) {
    ///     ids.push(id);
    /// }
    /// assert_eq!(ids, vec!["foo", "bar"]);
    /// ```
    pub fn extract<const N: usize>(&self) -> (&'h str, [&'h str; N]) {
        self.captures.extract()
    }

    /// Expands all instances of `$ref` in `replacement` to the corresponding
    /// capture group, and writes them to the `dst` buffer given. A `ref` can
    /// be a capture group index or a name. If `ref` doesn't refer to a capture
    /// group that participated in the match, then it is replaced with the
    /// empty string.
    ///
    /// # Format
    ///
    /// The format of the replacement string supports two different kinds of
    /// capture references: unbraced and braced.
    ///
    /// For the unbraced format, the format supported is `$ref` where `name`
    /// can be any character in the class `[0-9A-Za-z_]`. `ref` is always
    /// the longest possible parse. So for example, `$1a` corresponds to the
    /// capture group named `1a` and not the capture group at index `1`. If
    /// `ref` matches `^[0-9]+$`, then it is treated as a capture group index
    /// itself and not a name.
    ///
    /// For the braced format, the format supported is `${ref}` where `ref` can
    /// be any sequence of bytes except for `}`. If no closing brace occurs,
    /// then it is not considered a capture reference. As with the unbraced
    /// format, if `ref` matches `^[0-9]+$`, then it is treated as a capture
    /// group index and not a name.
    ///
    /// The braced format is useful for exerting precise control over the name
    /// of the capture reference. For example, `${1}a` corresponds to the
    /// capture group reference `1` followed by the letter `a`, where as `$1a`
    /// (as mentioned above) corresponds to the capture group reference `1a`.
    /// The braced format is also useful for expressing capture group names
    /// that use characters not supported by the unbraced format. For example,
    /// `${foo[bar].baz}` refers to the capture group named `foo[bar].baz`.
    ///
    /// If a capture group reference is found and it does not refer to a valid
    /// capture group, then it will be replaced with the empty string.
    ///
    /// To write a literal `$`, use `$$`.
    ///
    /// # Example
    ///
    /// ```
    /// use regex::Regex;
    ///
    /// let re = Regex::new(
    ///     r"(?<day>[0-9]{2})-(?<month>[0-9]{2})-(?<year>[0-9]{4})",
    /// ).unwrap();
    /// let hay = "On 14-03-2010, I became a Tennessee lamb.";
    /// let caps = re.captures(hay).unwrap();
    ///
    /// let mut dst = String::new();
    /// caps.expand("year=$year, month=$month, day=$day", &mut dst);
    /// assert_eq!(dst, "year=2010, month=03, day=14");
    /// ```
    #[inline]
    pub fn expand(&self, replacement: &str, dst: &mut String) {
        self.captures.expand(replacement, dst)
    }

    /// Returns the total number of capture groups. This includes both
    /// matching and non-matching groups.
    ///
    /// The length returned is always equivalent to the number of elements
    /// yielded by [`Captures::iter`]. Consequently, the length is always
    /// greater than zero since every `Captures` value always includes the
    /// match for the entire regex.
    ///
    /// # Example
    ///
    /// ```
    /// use regex::Regex;
    ///
    /// let re = Regex::new(r"(\w)(\d)?(\w)").unwrap();
    /// let caps = re.captures("AZ").unwrap();
    /// assert_eq!(caps.len(), 4);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.captures.len()
    }
}

pub struct ParseState<'i> {
    input: &'i String,
    index: usize,
}

fn check_parse_regex(regex: &Regex) {
    let s = regex.as_str();
    if !s.starts_with("^") {
        panic!("Parse regex doesn't start with ^: {s}")
    }
}

impl<'i> ParseState<'i> {
    /// Returns the current parse index.
    pub fn index(&self) -> usize {
        self.index
    }

    /// Returns `true` if the current parse index has reached the end of
    /// the input.
    pub fn is_done(&self) -> bool {
        self.index >= self.input.len()
    }

    /// Returns a match if `regex` (which must start with ^) matches at
    /// the current parse index.
    pub fn peek<'h>(&'h self, regex: &Regex) -> Option<ParseMatch<'h>> {
        check_parse_regex(regex);
        match regex.find(&self.input[self.index..]) {
            Some(m) => Some(ParseMatch::new(m, self.index)),
            None => None,
        }
    }

    /// Returns a match if `regex` (which must start with ^) matches at
    /// the current parse index, and advances the parse index to the end
    /// of the match.
    pub fn find<'h>(&'h mut self, regex: &Regex) -> Option<ParseMatch<'h>> {
        check_parse_regex(regex);
        match regex.find(&self.input[self.index..]) {
            Some(m) => {
                let result = Some(ParseMatch::new(m, self.index));
                self.index += m.end();
                result
            }
            None => None,
        }
    }

    /// Returns a capture if `regex` (which must start with ^) matches at
    /// the current parse index, and advances the parse index to the end
    /// of the match.
    pub fn captures<'h>(&'h mut self, regex: &Regex) -> Option<ParseCaptures<'h>> {
        check_parse_regex(regex);
        match regex.captures(&self.input[self.index..]) {
            Some(c) => {
                let end = c.get_match().end();
                let result = Some(ParseCaptures {
                    captures: c,
                    offs: self.index,
                });
                self.index += end;
                result
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_state() {
        let mut p = ParseState {
            input: &String::from("foobar"),
            index: 0,
        };

        let foo = Regex::new(r"^foo").unwrap();

        let m = p.find(&foo);
        assert_eq!(m.map(|m| m.as_str()), Some("foo"));

        let m = p.find(&foo);
        assert_eq!(m.is_some(), false);
        assert_eq!(p.index(), 3);

        let c = p.captures(&Regex::new(r"^b(..)").unwrap()).unwrap();
        assert_eq!(c.get_match().as_str(), "bar");
        assert_eq!(c.get(1).unwrap().as_str(), "ar");

        let m = p.find(&foo);
        assert_eq!(m.is_some(), false);

        assert_eq!(p.is_done(), true);
    }
}
