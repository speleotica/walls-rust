use std::sync::LazyLock;

use lines_inclusive::LinesInclusive;
use regex::Regex;

use crate::{
    srv::types::{
        EINVALIDDIRECTIVE, EUNEXPECTED, InvalidSrvItem, InvalidWallsSrvFile, MaybeValidSrvItem,
        MaybeValidUnitsOption, MaybeValidWallsSrvFile, PrefixDirectiveLocs, PrefixLevel, SrvItem,
        SrvSettings, UnitsDirectiveLocs, WallsSrvFile,
    },
    types::{
        ParseCaptures, ParseIssue, ParseIssueSeverity, ParseMatch, ParseState, SourceLoc, SourcePos,
    },
};

pub struct WallsSrvParser<'i> {
    input: &'i str,
    line: ParseState<'i>,
    settings: SrvSettings,
    issues: Vec<ParseIssue>,
}

static WHITESPACE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\s+").unwrap());
static DIRECTIVE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(?i)#[a-z]+[0-9]*").unwrap());
static INLINE_COMMENT: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^;\s*(.+)").unwrap());
static CHARACTER: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\S").unwrap());
static NAME: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\S+").unwrap());

impl<'i> WallsSrvParser<'i> {
    pub fn parse(input: &'i str) -> MaybeValidWallsSrvFile {
        let mut parser = WallsSrvParser {
            input,
            line: ParseState::new("", SourcePos::origin()),
            settings: SrvSettings::default(),
            issues: Vec::new(),
        };

        let mut items: Vec<MaybeValidSrvItem> = Vec::new();

        let mut pos = SourcePos::origin();

        for line in parser.input.lines_inclusive() {
            // TODO: block comments
            if let Some(item) = parser.parse_line(ParseState::new(line.trim_end(), pos)) {
                items.push(item);
            }
            pos += line;
        }

        if !parser.issues.iter().any(|i| i.is_error())
            && items
                .iter()
                .all(|i| matches!(i, MaybeValidSrvItem::Valid(_)))
        {
            WallsSrvFile {
                items: (items
                    .into_iter()
                    .map(|i| match i {
                        MaybeValidSrvItem::Valid(i) => i,
                        _ => unreachable!(),
                    })
                    .collect()),
                issues: (!parser.issues.is_empty()).then_some(parser.issues),
            }
            .into()
        } else {
            MaybeValidWallsSrvFile::Invalid {
                invalid: InvalidWallsSrvFile { items },
                issues: parser.issues,
            }
        }
    }
    #[inline]
    fn pos(&self) -> SourcePos {
        self.line.pos()
    }
    #[inline]
    fn is_match(&mut self, regex: &Regex) -> bool {
        self.line.is_match(regex)
    }
    #[inline]
    fn find(&mut self, regex: &Regex) -> Option<ParseMatch<'i>> {
        self.line.find(regex)
    }
    #[inline]
    fn captures(&mut self, regex: &Regex) -> Option<ParseCaptures<'i>> {
        self.line.captures(regex)
    }
    fn parse_line(&mut self, line: ParseState<'i>) -> Option<MaybeValidSrvItem> {
        self.line = line;
        self.skip_whitespace();
        if let Some(m) = self.find(&DIRECTIVE) {
            return Some(self.parse_directive(m));
        }
        // TODO: data lines
        self.parse_comment_and_eol().and_then(|m| {
            Some(
                SrvItem::Comment {
                    comment: m.content(),
                    inline: true,
                    loc: Some(m.loc()),
                }
                .into(),
            )
        })
    }
    fn parse_directive(&mut self, directive_match: ParseMatch<'i>) -> MaybeValidSrvItem {
        let directive = directive_match.as_str();
        let loc = directive_match.loc();
        match directive.to_ascii_lowercase().as_str() {
            "#units" => self.parse_units_directive(directive_match),
            "#prefix" => self.parse_prefix_directive(directive_match, PrefixLevel::Prefix1),
            "#prefix1" => self.parse_prefix_directive(directive_match, PrefixLevel::Prefix1),
            "#prefix2" => self.parse_prefix_directive(directive_match, PrefixLevel::Prefix2),
            "#prefix3" => self.parse_prefix_directive(directive_match, PrefixLevel::Prefix3),
            _ => InvalidSrvItem::Unknown {
                text: directive.into(),
                loc: Some(loc),
            }
            .with_issues(vec![self.push_error(
                EINVALIDDIRECTIVE,
                Some("Invalid directive".into()),
                Some(loc),
            )]),
        }
    }
    fn parse_units_directive(&mut self, directive_match: ParseMatch<'i>) -> MaybeValidSrvItem {
        let mut options: Vec<MaybeValidUnitsOption> = Vec::new();
        let mut locs = UnitsDirectiveLocs {
            directive: directive_match.loc(),
            comment: None,
        };
        let mut issues: Vec<usize> = Vec::new();
        while self.skip_whitespace() {
            todo!("implement options parsing")
        }
        let loc = Some(directive_match.start_pos().up_to(self.pos()));

        let comment = self.parse_comment_and_eol().and_then(|m| {
            locs.comment = Some(m.loc());
            Some(m.content())
        });

        if options
            .iter()
            .all(|o| matches!(o, MaybeValidUnitsOption::Valid(_)))
        {
            SrvItem::UnitsDirective {
                options: options
                    .into_iter()
                    .map(|o| match o {
                        MaybeValidUnitsOption::Valid(o) => o,
                        _ => unreachable!(),
                    })
                    .collect(),
                comment,
                loc,
                locs: Some(locs),
            }
            .into()
        } else {
            InvalidSrvItem::UnitsDirective {
                options,
                comment,
                loc,
                locs: Some(locs),
            }
            .with_issues(issues)
        }
    }
    fn parse_prefix_directive(
        &mut self,
        directive_match: ParseMatch<'i>,
        level: PrefixLevel,
    ) -> MaybeValidSrvItem {
        let mut locs = PrefixDirectiveLocs {
            directive: directive_match.loc(),
            prefix: None,
            comment: None,
        };

        self.skip_whitespace();
        let prefix = self.find(&NAME).and_then(|m| {
            locs.prefix = Some(m.loc());
            Some(m.as_str().into())
        });
        let prefix_or_empty = prefix.clone().unwrap_or("".into());
        let index: usize = level.into();
        if self.settings.prefix[index] != prefix_or_empty {
            self.settings = self.settings.clone();
            self.settings.prefix[index] = prefix_or_empty;
        }

        let loc = Some(directive_match.start_pos().up_to(self.pos()));

        let comment = self.parse_comment_and_eol().and_then(|m| {
            locs.comment = Some(m.loc());
            Some(m.content())
        });
        SrvItem::PrefixDirective {
            level,
            prefix,
            comment,
            loc,
            locs: Some(locs),
        }
        .into()
    }
    fn skip_whitespace(&mut self) -> bool {
        self.is_match(&WHITESPACE)
    }
    fn parse_comment_and_eol(&mut self) -> Option<CommentMatch<'i>> {
        self.skip_whitespace();
        let comment = self.find(&INLINE_COMMENT);
        if let Some(m) = self.find(&CHARACTER) {
            self.push_error(
                EUNEXPECTED,
                Some("Unexpected character".into()),
                Some(m.loc()),
            );
        }
        comment.and_then(|m| Some(CommentMatch(m)))
    }
    fn push_issue(&mut self, issue: ParseIssue) -> usize {
        let index = self.issues.len();
        self.issues.push(issue);
        index
    }
    #[inline]
    fn push_error(&mut self, code: &str, message: Option<String>, loc: Option<SourceLoc>) -> usize {
        self.push_issue(ParseIssue::error(code, message, loc))
    }
}

struct CommentMatch<'i>(ParseMatch<'i>);

impl<'i> CommentMatch<'i> {
    fn loc(&self) -> SourceLoc {
        self.0.loc()
    }
    fn content(&self) -> String {
        self.0.as_str()[1..].into()
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
                        loc: Some(SourceLoc::str_from_to(input, 0, 4))
                    },
                    SrvItem::Comment {
                        comment: " bar".into(),
                        inline: true,
                        loc: Some(SourceLoc::str_from_to(input, 6, 11))
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
                            loc: Some(SourceLoc::str_from_to(input, 0, 4))
                        }
                        .into(),
                        SrvItem::Comment {
                            comment: " bar".into(),
                            inline: true,
                            loc: Some(SourceLoc::str_from_to(input, 6, 11))
                        }
                        .into(),
                    ],
                },
                issues: vec![ParseIssue::error(
                    &EUNEXPECTED,
                    Some("Unexpected character".into()),
                    Some(SourceLoc::str_from_to(input, 15, 16))
                )],
            }
        )
    }
}
