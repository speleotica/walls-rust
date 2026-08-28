use std::sync::LazyLock;

use lines_inclusive::LinesInclusive;
use regex::Regex;

use crate::{
    srv::types::{
        EINVALIDDIRECTIVE, EINVALIDLINE, EUNEXPECTED, InvalidSrvItem, InvalidWallsSrvFile,
        MaybeValidSrvItem, MaybeValidWallsSrvFile, SrvItem, UnitsDirectiveLocs, WallsSrvFile,
    },
    types::{ParseCaptures, ParseIssue, ParseMatch, ParseState, SourceLoc, SourcePos},
};

pub struct WallsSrvParser<'i> {
    input: &'i str,
    line: ParseState<'i>,
    issues: Vec<ParseIssue>,
}

static WHITESPACE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\s+").unwrap());
static DIRECTIVE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(?i)#[a-z]+[0-9]*").unwrap());
static INLINE_COMMENT: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^;\s*(.+)").unwrap());
static CHARACTER: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\S").unwrap());

impl<'i> WallsSrvParser<'i> {
    pub fn parse(input: &'i str) -> MaybeValidWallsSrvFile {
        let mut parser = WallsSrvParser {
            input,
            line: ParseState::new("", SourcePos::origin()),
            issues: Vec::new(),
        };

        let mut items: Vec<MaybeValidSrvItem> = Vec::new();

        let mut pos = SourcePos::origin();

        for line in parser.input.lines_inclusive() {
            if let Some(item) = parser.parse_line(ParseState::new(line.trim_end(), pos)) {
                items.push(item);
            }
            pos += line;
        }

        if items
            .iter()
            .all(|i| matches!(i, MaybeValidSrvItem::Valid(_)))
        {
            MaybeValidWallsSrvFile::Valid(WallsSrvFile {
                items: (items
                    .into_iter()
                    .map(|i| match i {
                        MaybeValidSrvItem::Valid(i) => i,
                        _ => unreachable!(),
                    })
                    .collect()),
                issues: if parser.issues.is_empty() {
                    None
                } else {
                    Some(parser.issues)
                },
            })
        } else {
            MaybeValidWallsSrvFile::Invalid {
                invalid: InvalidWallsSrvFile { items },
                issues: parser.issues,
            }
        }
    }
    fn find(&mut self, regex: &Regex) -> Option<ParseMatch<'i>> {
        self.line.find(regex)
    }
    fn captures(&mut self, regex: &Regex) -> Option<ParseCaptures<'i>> {
        self.line.captures(regex)
    }
    fn is_line_done(&self) -> bool {
        self.line.is_done()
    }
    fn parse_line(&mut self, line: ParseState<'i>) -> Option<MaybeValidSrvItem> {
        self.line = line;
        self.find(&WHITESPACE);
        if let Some(m) = self.find(&DIRECTIVE) {
            return Some(self.parse_directive(m));
        }
        if let Some(c) = self.captures(&INLINE_COMMENT) {
            if let Some(m) = c.get(1) {
                return Some(
                    SrvItem::Comment {
                        comment: String::from(m.as_str()),
                        inline: true,
                        loc: Some(m.loc()),
                    }
                    .into(),
                );
            }
        }
        match self.find(&CHARACTER) {
            Some(m) => Some(MaybeValidSrvItem::Invalid {
                invalid: (InvalidSrvItem::Unknown {
                    text: String::from(m.as_str()),
                    loc: Some(m.loc()),
                }),
                issues: Some(vec![self.push_error(
                    EINVALIDLINE,
                    Some("Expected directive, from station or comment".into()),
                    Some(m.loc()),
                )]),
            }),
            None => None,
        }
    }
    fn parse_directive(&mut self, directive_match: ParseMatch<'i>) -> MaybeValidSrvItem {
        let directive = directive_match.as_str();
        let loc = directive_match.loc();
        match directive.to_ascii_lowercase().as_str() {
            "#units" => self.parse_units_directive(directive_match),
            _ => MaybeValidSrvItem::Invalid {
                invalid: (InvalidSrvItem::Unknown {
                    text: directive.into(),
                    loc: Some(loc),
                }),
                issues: Some(vec![self.push_error(
                    EINVALIDDIRECTIVE,
                    Some("Invalid directive".into()),
                    Some(loc),
                )]),
            },
        }
    }
    fn parse_units_directive(&mut self, directive_match: ParseMatch<'i>) -> MaybeValidSrvItem {
        let mut locs = UnitsDirectiveLocs {
            directive: directive_match.loc(),
            comment: None,
        };
        while let Some(_) = self.find(&WHITESPACE) {}
        let mut comment: Option<String> = None;
        if let Some(c) = self.captures(&INLINE_COMMENT)
            && let Some(m) = c.get(1)
        {
            comment = Some(m.as_str().into());
            locs.comment = Some(m.loc());
        }
        if let Some(m) = self.find(&CHARACTER) {
            self.push_error(
                EUNEXPECTED,
                Some("Unexpected text after units".into()),
                Some(m.loc()),
            );
        }
        SrvItem::UnitsDirective {
            options: vec![],
            comment,
            loc: Some(SourceLoc::new(directive_match.start_pos(), self.line.pos())),
            locs: Some(locs),
        }
        .into()
    }
    fn push_issue(&mut self, issue: ParseIssue) -> usize {
        let index = self.issues.len();
        self.issues.push(issue);
        index
    }
    fn push_error(&mut self, code: &str, message: Option<String>, loc: Option<SourceLoc>) -> usize {
        self.push_issue(ParseIssue::error(code, message, loc))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_comments() {
        let input = ";foo\n\n; bar";
        assert_eq!(
            WallsSrvParser::parse(input),
            WallsSrvFile {
                items: vec![
                    SrvItem::Comment {
                        comment: "foo".into(),
                        inline: true,
                        loc: Some(SourceLoc::str_from_to(input, 1, 4))
                    },
                    SrvItem::Comment {
                        comment: "bar".into(),
                        inline: true,
                        loc: Some(SourceLoc::str_from_to(input, 8, 11))
                    }
                ],
                issues: None,
            }
            .into()
        )
    }

    #[test]
    fn test_comments_and_invalid_line() {
        let input = ";foo\n\n; bar\n   #";

        assert_eq!(
            WallsSrvParser::parse(input),
            MaybeValidWallsSrvFile::Invalid {
                invalid: InvalidWallsSrvFile {
                    items: vec![
                        SrvItem::Comment {
                            comment: "foo".into(),
                            inline: true,
                            loc: Some(SourceLoc::str_from_to(input, 1, 4))
                        }
                        .into(),
                        SrvItem::Comment {
                            comment: "bar".into(),
                            inline: true,
                            loc: Some(SourceLoc::str_from_to(input, 8, 11))
                        }
                        .into(),
                        MaybeValidSrvItem::Invalid {
                            invalid: InvalidSrvItem::Unknown {
                                text: "#".into(),
                                loc: Some(SourceLoc::str_from_to(input, 15, 16)),
                            },
                            issues: Some(vec![0usize]),
                        }
                    ],
                },
                issues: vec![ParseIssue::error(
                    &EINVALIDLINE,
                    Some("Expected directive, from station or comment".into()),
                    Some(SourceLoc::str_from_to(input, 15, 16))
                )],
            }
        )
    }
}
