use lines_inclusive::LinesInclusive;
use regex::Regex;
use std::{num::ParseFloatError, sync::LazyLock};

use crate::{
    srv::types::{
        Angle, AngleUnit, CompassAndTapeItem, EINCLINATIONOUTOFRANGE, EINVALIDANGLE,
        EINVALIDANGLEUNIT, EINVALIDAZIMUTH, EINVALIDAZIMUTHUNIT, EINVALIDCASECONVERSION,
        EINVALIDDIRECTIVE, EINVALIDINCLINATION, EINVALIDINCLINATIONUNIT, EINVALIDLENGTH,
        EINVALIDLENGTHUNIT, EINVALIDMEASUREMENTORDER, EINVALIDOPTION, EINVALIDORDERITEM,
        EINVALIDTAPINGMETHOD, EINVALIDUNITVARIANCE, EMISSINGINCHES, EMISSINGVALUE, EUNEXPECTED,
        Inclination, InclinationUnit, InvalidSrvItem, InvalidUnitsOption, InvalidValue,
        InvalidWallsSrvFile, Length, LengthUnit, MaybeValidOrderItem, MaybeValidSrvItem,
        MaybeValidUnitsOption, MaybeValidWallsSrvFile, OrderItem, OrderOptionLocs,
        PrefixDirectiveLocs, PrefixLevel, RectilinearItem, SrvItem, SrvSettings,
        StationNameCaseConversion, TapingMethod, UnitsDirectiveLocs, UnitsOption, WallsSrvFile,
    },
    types::{ParseIssue, ParseMatch, ParseState, SourceLoc, SourcePos},
};

pub struct WallsSrvParser<'i> {
    input: &'i str,
    line: ParseState<'i>,
    settings: SrvSettings,
    saved_settings: Vec<SrvSettings>,
    issues: Vec<ParseIssue>,
}

const WHITESPACE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\s+").unwrap());
const DIRECTIVE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(?i)#[a-z]+[0-9]*").unwrap());
const INLINE_COMMENT: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^;\s*(.+)").unwrap());
const CHARACTER: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\S").unwrap());
const LENGTH_UNIT_SUFFIX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(?i)[mfi]").unwrap());
const INCHES_SUFFIX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(?i)i").unwrap());
const AZIMUTH_UNIT_SUFFIX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(?i)[dgm]").unwrap());
const INCLINATION_UNIT_SUFFIX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(?i)[dgmp]").unwrap());
const NAME: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[^:;,#\s]+").unwrap());
const LETTER: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(?i)[a-z]").unwrap());
const UNITS_OPTION: LazyLock<Regex> = LazyLock::new(|| Regex::new("^[^:;,#=\"\\s]+").unwrap());
const UNEXPECTED_AFTER_UNITS_OPTION: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[^\s;]+").unwrap());
const UNITS_OPTION_EQUALS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\s*=\s*").unwrap());
const UNITS_OPTION_VALUE: LazyLock<Regex> = UNITS_OPTION;
const SIGN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[-+]").unwrap());
const SIGNED_INTEGER: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[-+]?\d+").unwrap());
const UNSIGNED_INTEGER: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\d+").unwrap());
const SIGNED_NUMBER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[-+]?(\d+(\.\d*)?|\.\d+)").unwrap());
const UNSIGNED_NUMBER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(\d+(\.\d*)?|\.\d+)").unwrap());

impl<'i> WallsSrvParser<'i> {
    pub fn parse(input: &'i str) -> MaybeValidWallsSrvFile {
        let mut parser = WallsSrvParser {
            input,
            line: ParseState::new("", SourcePos::origin()),
            settings: SrvSettings::default(),
            saved_settings: Vec::new(),
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
    fn parse_line(&mut self, line: ParseState<'i>) -> Option<MaybeValidSrvItem> {
        self.line = line;
        self.skip_whitespace();
        if let Some(m) = self.find(&DIRECTIVE) {
            return Some(self.directive(m));
        }
        // TODO: data lines
        self.comment_and_eol().and_then(|m| {
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
    fn directive(&mut self, directive_match: ParseMatch<'i>) -> MaybeValidSrvItem {
        let directive = directive_match.as_str();
        let loc = directive_match.loc();
        match directive.to_ascii_lowercase().as_str() {
            "#units" => self.units_directive(directive_match),
            "#prefix" | "#prefix1" => self.prefix_directive(directive_match, PrefixLevel::Prefix1),
            "#prefix2" => self.prefix_directive(directive_match, PrefixLevel::Prefix2),
            "#prefix3" => self.prefix_directive(directive_match, PrefixLevel::Prefix3),
            _ => InvalidSrvItem::Unknown {
                text: directive.into(),
                loc: Some(loc),
            }
            .with_issue(self.push_error(
                EINVALIDDIRECTIVE,
                Some("Invalid directive".into()),
                Some(loc),
            )),
        }
    }
    fn units_directive(&mut self, directive_match: ParseMatch<'i>) -> MaybeValidSrvItem {
        let mut options: Vec<MaybeValidUnitsOption> = Vec::new();
        let mut locs = UnitsDirectiveLocs {
            directive: directive_match.loc(),
            comment: None,
        };
        let mut issues: Vec<usize> = Vec::new();
        while self.skip_whitespace() {
            if let Some(m) = self.find(&UNITS_OPTION) {
                let option = match m.as_str().to_ascii_lowercase().as_str() {
                    "ct" => todo!(),
                    "rect" => todo!(),
                    "order" => self.order_option(m),
                    "f" | "feet" => {
                        UnitsOption::distance_unit(LengthUnit::Feet, Some(m.loc())).into()
                    }
                    "m" | "meters" => {
                        UnitsOption::distance_unit(LengthUnit::Meters, Some(m.loc())).into()
                    }
                    "d" => self.distance_unit(
                        m,
                        UnitsOption::primary_distance_unit,
                        InvalidUnitsOption::primary_distance_unit,
                    ),
                    "s" => self.distance_unit(
                        m,
                        UnitsOption::secondary_distance_unit,
                        InvalidUnitsOption::secondary_distance_unit,
                    ),
                    "a" => self.azimuth_unit(
                        m,
                        UnitsOption::frontsight_azimuth_unit,
                        InvalidUnitsOption::frontsight_azimuth_unit,
                    ),
                    "ab" => self.azimuth_unit(
                        m,
                        UnitsOption::backsight_azimuth_unit,
                        InvalidUnitsOption::backsight_azimuth_unit,
                    ),
                    "v" => self.inclination_unit(
                        m,
                        UnitsOption::frontsight_inclination_unit,
                        InvalidUnitsOption::frontsight_inclination_unit,
                    ),
                    "vb" => self.inclination_unit(
                        m,
                        UnitsOption::backsight_inclination_unit,
                        InvalidUnitsOption::backsight_inclination_unit,
                    ),
                    "decl" => self.signed_azimuth_option(
                        m,
                        AngleUnit::Degrees,
                        UnitsOption::magnetic_declination,
                        InvalidUnitsOption::magnetic_declination,
                    ),
                    "grid" => self.signed_azimuth_option(
                        m,
                        AngleUnit::Degrees,
                        UnitsOption::grid_north_correction,
                        InvalidUnitsOption::grid_north_correction,
                    ),
                    "incd" => self.length_option(
                        m,
                        self.settings.primary_distance_unit,
                        UnitsOption::primary_distance_correction,
                        InvalidUnitsOption::primary_distance_correction,
                    ),
                    "incs" => self.length_option(
                        m,
                        self.settings.secondary_distance_unit,
                        UnitsOption::secondary_distance_correction,
                        InvalidUnitsOption::secondary_distance_correction,
                    ),
                    "inca" => self.signed_angle_option(
                        m,
                        self.settings.frontsight_azimuth_unit,
                        UnitsOption::frontsight_azimuth_correction,
                        InvalidUnitsOption::frontsight_azimuth_correction,
                    ),
                    "incab" => self.signed_angle_option(
                        m,
                        self.settings.backsight_azimuth_unit,
                        UnitsOption::backsight_azimuth_correction,
                        InvalidUnitsOption::backsight_azimuth_correction,
                    ),
                    "incv" => self.signed_inclination_option(
                        m,
                        self.settings.frontsight_inclination_unit,
                        UnitsOption::frontsight_inclination_correction,
                        InvalidUnitsOption::frontsight_inclination_correction,
                    ),
                    "incvb" => self.signed_inclination_option(
                        m,
                        self.settings.backsight_inclination_unit,
                        UnitsOption::backsight_inclination_correction,
                        InvalidUnitsOption::backsight_inclination_correction,
                    ),
                    "inch" => self.length_option(
                        m,
                        self.settings.secondary_distance_unit,
                        UnitsOption::height_adjustment,
                        InvalidUnitsOption::height_adjustment,
                    ),
                    "typeab" => todo!(),
                    "typevb" => todo!(),
                    "reset" => UnitsOption::Reset { loc: Some(m.loc()) }.into(),
                    "save" => UnitsOption::Save { loc: Some(m.loc()) }.into(),
                    // TODO: error when saved_settings stack is empty?
                    "restore" => UnitsOption::Restore { loc: Some(m.loc()) }.into(),
                    "case" => self.station_name_case_option(
                        m,
                        UnitsOption::station_name_case,
                        InvalidUnitsOption::station_name_case,
                    ),
                    "lrud" => todo!(),
                    "prefix" => todo!(),
                    "prefix2" => todo!(),
                    "prefix3" => todo!(),
                    "tape" => self.taping_method_option(
                        m,
                        UnitsOption::taping_method,
                        InvalidUnitsOption::taping_method,
                    ),
                    "uvh" => self.variance_option(
                        m,
                        UnitsOption::horizontal_unit_variance,
                        InvalidUnitsOption::horizontal_unit_variance,
                    ),
                    "uvv" => self.variance_option(
                        m,
                        UnitsOption::vertical_unit_variance,
                        InvalidUnitsOption::vertical_unit_variance,
                    ),
                    "uv" => self.variance_option(
                        m,
                        UnitsOption::unit_variance,
                        InvalidUnitsOption::unit_variance,
                    ),
                    "flag" => todo!(),
                    _ => InvalidUnitsOption::Unknown {
                        value: m.as_str().into(),
                        loc: Some(m.loc()),
                    }
                    .with_issue(self.push_error(
                        EUNEXPECTED,
                        Some("Invalid units option".into()),
                        Some(m.loc()),
                    )),
                };
                if let MaybeValidUnitsOption::Valid(option) = &option {
                    self.apply_option(option);
                }
                options.push(option);
            }
            if let Some(m) = self.find(&UNEXPECTED_AFTER_UNITS_OPTION) {
                if &m.as_str()[0..1] == "=" {
                    issues.push(self.push_error(
                        EUNEXPECTED,
                        Some("Unexpected #UNITS option value".into()),
                        Some(m.loc()),
                    ));
                } else {
                    options.push(
                        InvalidUnitsOption::Unknown {
                            value: m.as_str().into(),
                            loc: Some(m.loc()),
                        }
                        .with_issue(self.push_error(
                            EINVALIDOPTION,
                            Some("Invalid #UNITS option".into()),
                            Some(m.loc()),
                        )),
                    );
                }
            }
        }
        let loc = Some(directive_match.start_pos().up_to(self.pos()));

        let comment = self.comment_and_eol().and_then(|m| {
            locs.comment = Some(m.loc());
            Some(m.content())
        });

        // TODO: apply changes
        // self.settings = self.settings.apply_options(options);

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
            .with_issues(vec![
                // TODO?
            ])
        }
    }
    fn apply_option(&mut self, option: &UnitsOption) {
        self.settings.apply_option(option);
        match option {
            UnitsOption::Save { loc: _ } => self.saved_settings.push(self.settings.clone()),
            UnitsOption::Restore { loc: _ } => {
                if let Some(settings) = self.saved_settings.pop() {
                    self.settings = settings;
                }
            }
            UnitsOption::Reset { loc: _ } => self.settings = SrvSettings::default(),
            _ => {}
        }
    }
    fn get_option_value(&mut self, what: &str) -> Result<ParseMatch<'i>, ParseIssue> {
        if !self.is_match(&UNITS_OPTION_EQUALS) {
            Err(ParseIssue::error(
                EMISSINGVALUE,
                Some("Missing =".into()),
                Some(self.pos().into()),
            ))
        } else if let Some(value) = self.find(&UNITS_OPTION_VALUE) {
            Ok(value)
        } else {
            Err(ParseIssue::error(
                EMISSINGVALUE,
                Some(format!("Missing {what}").into()),
                Some(self.pos().into()),
            ))
        }
    }
    fn expect_done(&mut self, parser: &mut ParseState<'i>) {
        if !parser.is_done() {
            self.push_error(
                EUNEXPECTED,
                Some("Unexpected text".into()),
                Some(parser.rest().loc()),
            );
        }
    }
    fn option_value<V>(
        &mut self,
        option: ParseMatch<'i>,
        what: &str,
        parse_value: impl Fn(&mut ParseState<'i>) -> Result<V, ParseIssue>,
        valid: impl Fn(V, SourceLoc, Option<SourceLoc>) -> UnitsOption,
        invalid: impl Fn(Option<String>, SourceLoc, Option<SourceLoc>) -> InvalidUnitsOption,
    ) -> MaybeValidUnitsOption {
        match self.get_option_value(what) {
            Ok(value) => {
                let mut parser = value.reparse();
                let result = parse_value(&mut parser)
                    .map_or_else(
                        |e| {
                            invalid(Some(value.as_str().into()), option.loc(), Some(value.loc()))
                                .with_issue(self.push_issue(e))
                        },
                        |parsed| valid(parsed, option.loc(), Some(value.loc())).into(),
                    )
                    .into();
                self.expect_done(&mut parser);
                result
            }
            Err(issue) => {
                invalid(None, option.loc(), None).with_issues(vec![self.push_issue(issue)])
            }
        }
    }
    fn order_option(&mut self, option: ParseMatch<'i>) -> MaybeValidUnitsOption {
        match self.get_option_value("measurement order") {
            Ok(value) => {
                let value_lower = value.as_str().to_ascii_lowercase();
                match value_lower.as_str() {
                    "dav" | "dva" | "adv" | "avd" | "vda" | "vad" | "da" | "ad" => {
                        UnitsOption::compass_and_tape_order(
                            value_lower
                                .chars()
                                .into_iter()
                                .map(|c| match c {
                                    'd' => CompassAndTapeItem::Distance,
                                    'a' => CompassAndTapeItem::Azimuth,
                                    'v' => CompassAndTapeItem::Inclination,
                                    _ => unreachable!(),
                                })
                                .collect(),
                            option.loc(),
                            Some(value.loc()),
                        )
                        .into()
                    }
                    "enu" | "eun" | "neu" | "nue" | "uen" | "une" | "en" | "ne" => {
                        UnitsOption::rectilinear_order(
                            value_lower
                                .chars()
                                .into_iter()
                                .map(|c| match c {
                                    'e' => RectilinearItem::Easting,
                                    'n' => RectilinearItem::Northing,
                                    'u' => RectilinearItem::Elevation,
                                    _ => unreachable!(),
                                })
                                .collect(),
                            option.loc(),
                            Some(value.loc()),
                        )
                        .into()
                    }
                    invalid => {
                        let order: Vec<MaybeValidOrderItem> =
                            invalid
                                .chars()
                                .enumerate()
                                .into_iter()
                                .map(|(index, c)| match c {
                                    'd' => OrderItem::Distance.into(),
                                    'a' => OrderItem::Azimuth.into(),
                                    'v' => OrderItem::Inclination.into(),
                                    'e' => OrderItem::Easting.into(),
                                    'n' => OrderItem::Northing.into(),
                                    'u' => OrderItem::Elevation.into(),
                                    invalid => MaybeValidOrderItem::Invalid(InvalidValue {
                                        invalid: invalid.into(),
                                        issues: Some(vec![self.push_error(
                                            EINVALIDORDERITEM,
                                            Some("Invalid measurement item".into()),
                                            Some(value.loc().start.up_to(
                                                value.loc().start + &value.as_str()[0..index],
                                            )),
                                        )]),
                                    }),
                                })
                                .collect();
                        let all_items_valid = order
                            .iter()
                            .all(|o| matches!(o, MaybeValidOrderItem::Valid(_)));

                        MaybeValidUnitsOption::Invalid {
                            invalid: InvalidUnitsOption::order(
                                Some(order),
                                option.loc(),
                                Some(value.loc()),
                            ),
                            issues: all_items_valid.then(|| {
                                vec![self.push_error(
                                    EINVALIDMEASUREMENTORDER,
                                    Some("Invalid measurement order".into()),
                                    Some(value.loc()),
                                )]
                            }),
                        }
                    }
                }
            }
            Err(issue) => MaybeValidUnitsOption::Invalid {
                invalid: InvalidUnitsOption::Order {
                    order: None,
                    loc: Some(option.loc()),
                    locs: Some(OrderOptionLocs {
                        option: option.loc(),
                        order: None,
                    }),
                },
                issues: Some(vec![self.push_issue(issue)]),
            },
        }
    }
    fn distance_unit(
        &mut self,
        option: ParseMatch<'i>,
        valid: impl Fn(LengthUnit, SourceLoc, Option<SourceLoc>) -> UnitsOption,
        invalid: impl Fn(Option<String>, SourceLoc, Option<SourceLoc>) -> InvalidUnitsOption,
    ) -> MaybeValidUnitsOption {
        return self.option_value(
            option,
            "length unit",
            |p| {
                let value = p.rest();
                match value.as_str().to_ascii_lowercase().as_str() {
                    "m" | "meters" => Ok(LengthUnit::Meters),
                    "f" | "feet" => Ok(LengthUnit::Feet),
                    _ => Err(ParseIssue::error(
                        EINVALIDLENGTHUNIT,
                        Some("Invalid length unit".into()),
                        Some(value.loc()),
                    )),
                }
            },
            valid,
            invalid,
        );
    }
    fn azimuth_unit(
        &mut self,
        option: ParseMatch<'i>,
        valid: impl Fn(AngleUnit, SourceLoc, Option<SourceLoc>) -> UnitsOption,
        invalid: impl Fn(Option<String>, SourceLoc, Option<SourceLoc>) -> InvalidUnitsOption,
    ) -> MaybeValidUnitsOption {
        return self.option_value(
            option,
            "azimuth unit",
            |p| {
                let value = p.rest();
                match value.as_str().to_ascii_lowercase().as_str() {
                    "d" | "deg" | "degree" | "degrees" => Ok(AngleUnit::Degrees),
                    "g" | "grad" | "grads" => Ok(AngleUnit::Grads),
                    "m" | "mil" | "mils" => Ok(AngleUnit::Mils),
                    _ => Err(ParseIssue::error(
                        EINVALIDAZIMUTHUNIT,
                        Some("Invalid azimuth unit".into()),
                        Some(value.loc()),
                    )),
                }
            },
            valid,
            invalid,
        );
    }
    fn inclination_unit(
        &mut self,
        option: ParseMatch<'i>,
        valid: impl Fn(InclinationUnit, SourceLoc, Option<SourceLoc>) -> UnitsOption,
        invalid: impl Fn(Option<String>, SourceLoc, Option<SourceLoc>) -> InvalidUnitsOption,
    ) -> MaybeValidUnitsOption {
        return self.option_value(
            option,
            "inclination unit",
            |p| {
                let value = p.rest();
                match value.as_str().to_ascii_lowercase().as_str() {
                    "d" | "deg" | "degree" | "degrees" => Ok(InclinationUnit::Degrees),
                    "g" | "grad" | "grads" => Ok(InclinationUnit::Grads),
                    "m" | "mil" | "mils" => Ok(InclinationUnit::Mils),
                    "p" | "percent" => Ok(InclinationUnit::Percent),
                    _ => Err(ParseIssue::error(
                        EINVALIDINCLINATIONUNIT,
                        Some("Invalid inclination unit".into()),
                        Some(value.loc()),
                    )),
                }
            },
            valid,
            invalid,
        );
    }
    fn length_option(
        &mut self,
        option: ParseMatch<'i>,
        default_unit: LengthUnit,
        valid: impl Fn(Length, SourceLoc, Option<SourceLoc>) -> UnitsOption,
        invalid: impl Fn(Option<String>, SourceLoc, Option<SourceLoc>) -> InvalidUnitsOption,
    ) -> MaybeValidUnitsOption {
        return self.option_value(
            option,
            "length",
            |p| match signed_length(p, default_unit) {
                Ok(Some((length, _))) => Ok(length),
                Ok(None) => Err(ParseIssue::error(
                    EMISSINGVALUE,
                    Some("Missing length".into()),
                    Some(p.rest().loc()),
                )),
                Err(e) => Err(e.into()),
            },
            valid,
            invalid,
        );
    }
    fn signed_angle_option(
        &mut self,
        option: ParseMatch<'i>,
        default_unit: AngleUnit,
        valid: impl Fn(Angle, SourceLoc, Option<SourceLoc>) -> UnitsOption,
        invalid: impl Fn(Option<String>, SourceLoc, Option<SourceLoc>) -> InvalidUnitsOption,
    ) -> MaybeValidUnitsOption {
        return self.option_value(
            option,
            "angle",
            |p| match signed_angle(p, default_unit) {
                Ok(Some((angle, _))) => Ok(angle),
                Ok(None) => Err(ParseIssue::error(
                    EMISSINGVALUE,
                    Some("Missing angle".into()),
                    Some(p.rest().loc()),
                )),
                Err(e) => Err(e.into()),
            },
            valid,
            invalid,
        );
    }
    fn signed_azimuth_option(
        &mut self,
        option: ParseMatch<'i>,
        default_unit: AngleUnit,
        valid: impl Fn(Angle, SourceLoc, Option<SourceLoc>) -> UnitsOption,
        invalid: impl Fn(Option<String>, SourceLoc, Option<SourceLoc>) -> InvalidUnitsOption,
    ) -> MaybeValidUnitsOption {
        return self.option_value(
            option,
            "azimuth",
            |p| match signed_azimuth(p, default_unit) {
                Ok(Some((azimuth, _))) => Ok(azimuth),
                Ok(None) => Err(ParseIssue::error(
                    EMISSINGVALUE,
                    Some("Missing azimuth".into()),
                    Some(p.rest().loc()),
                )),
                Err(e) => Err(e.into()),
            },
            valid,
            invalid,
        );
    }
    fn signed_inclination_option(
        &mut self,
        option: ParseMatch<'i>,
        default_unit: InclinationUnit,
        valid: impl Fn(Inclination, SourceLoc, Option<SourceLoc>) -> UnitsOption,
        invalid: impl Fn(Option<String>, SourceLoc, Option<SourceLoc>) -> InvalidUnitsOption,
    ) -> MaybeValidUnitsOption {
        return self.option_value(
            option,
            "inclination",
            |p| match signed_inclination(p, default_unit) {
                Ok(Some((inclination, _))) => Ok(inclination),
                Ok(None) => Err(ParseIssue::error(
                    EMISSINGVALUE,
                    Some("Missing inclination".into()),
                    Some(p.rest().loc()),
                )),
                Err(e) => Err(e.into()),
            },
            valid,
            invalid,
        );
    }
    fn station_name_case_option(
        &mut self,
        option: ParseMatch<'i>,
        valid: impl Fn(StationNameCaseConversion, SourceLoc, Option<SourceLoc>) -> UnitsOption,
        invalid: impl Fn(Option<String>, SourceLoc, Option<SourceLoc>) -> InvalidUnitsOption,
    ) -> MaybeValidUnitsOption {
        return self.option_value(
            option,
            "case conversion",
            |p| {
                let value = p.rest();
                match value.as_str().to_ascii_lowercase().as_str() {
                    "l" | "lower" => Ok(StationNameCaseConversion::Lower),
                    "u" | "upper" => Ok(StationNameCaseConversion::Upper),
                    "m" | "mixed" => Ok(StationNameCaseConversion::Mixed),
                    _ => Err(ParseIssue::error(
                        EINVALIDCASECONVERSION,
                        Some("Invalid case conversion".into()),
                        Some(value.loc()),
                    )),
                }
            },
            valid,
            invalid,
        );
    }
    fn taping_method_option(
        &mut self,
        option: ParseMatch<'i>,
        valid: impl Fn(TapingMethod, SourceLoc, Option<SourceLoc>) -> UnitsOption,
        invalid: impl Fn(Option<String>, SourceLoc, Option<SourceLoc>) -> InvalidUnitsOption,
    ) -> MaybeValidUnitsOption {
        return self.option_value(
            option,
            "taping method",
            |p| {
                let value = p.rest();
                match value.as_str().to_ascii_lowercase().as_str() {
                    "it" => Ok(TapingMethod::InstrumentToTarget),
                    "ss" => Ok(TapingMethod::StationToStation),
                    "is" => Ok(TapingMethod::InstrumentToStation),
                    "st" => Ok(TapingMethod::StationToTarget),
                    _ => Err(ParseIssue::error(
                        EINVALIDTAPINGMETHOD,
                        Some("Invalid taping method".into()),
                        Some(value.loc()),
                    )),
                }
            },
            valid,
            invalid,
        );
    }
    fn variance_option(
        &mut self,
        option: ParseMatch<'i>,
        valid: impl Fn(f64, SourceLoc, Option<SourceLoc>) -> UnitsOption,
        invalid: impl Fn(Option<String>, SourceLoc, Option<SourceLoc>) -> InvalidUnitsOption,
    ) -> MaybeValidUnitsOption {
        return self.option_value(
            option,
            "unit variance",
            |p| match signed_number(p) {
                Some(Ok(variance)) => Ok(variance),
                None => Err(ParseIssue::error(
                    EMISSINGVALUE,
                    Some("Missing unit variance".into()),
                    Some(p.rest().loc()),
                )),
                Some(Err(_)) => Err(ParseIssue::error(
                    EINVALIDUNITVARIANCE,
                    Some("Invalid unit variance".into()),
                    Some(p.rest().loc()),
                )),
            },
            valid,
            invalid,
        );
    }
    fn prefix_directive(
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

        let comment = self.comment_and_eol().and_then(|m| {
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
    fn comment_and_eol(&mut self) -> Option<CommentMatch<'i>> {
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

struct InvalidNumber<'i>(ParseFloatError, ParseMatch<'i>);

fn unsigned_number<'i>(p: &mut ParseState<'i>) -> Option<Result<f64, InvalidNumber<'i>>> {
    p.find(&UNSIGNED_NUMBER)
        .map(|m| m.as_str().parse().map_err(|e| InvalidNumber(e, m)))
}

fn signed_number<'i>(p: &mut ParseState<'i>) -> Option<Result<f64, InvalidNumber<'i>>> {
    p.find(&SIGNED_NUMBER)
        .map(|m| m.as_str().parse().map_err(|e| InvalidNumber(e, m)))
}

enum InvalidLength<'i> {
    InvalidNumber(ParseFloatError, ParseMatch<'i>),
    InvalidUnit(ParseMatch<'i>),
    MissingInches(SourcePos),
}

impl<'i> From<InvalidNumber<'i>> for InvalidLength<'i> {
    fn from(value: InvalidNumber<'i>) -> Self {
        InvalidLength::InvalidNumber(value.0, value.1)
    }
}

impl<'i> From<InvalidLength<'i>> for ParseIssue {
    fn from(value: InvalidLength<'i>) -> Self {
        match value {
            InvalidLength::InvalidNumber(e, m) => {
                ParseIssue::error(EINVALIDLENGTH, Some(e.to_string()), Some(m.loc()))
            }
            InvalidLength::InvalidUnit(m) => ParseIssue::error(
                EINVALIDLENGTHUNIT,
                Some("Invalid length unit".into()),
                Some(m.loc()),
            ),
            InvalidLength::MissingInches(p) => ParseIssue::error(
                EMISSINGINCHES,
                Some("Missing inches".into()),
                Some(p.into()),
            ),
        }
    }
}

fn length_unit_suffix<'i>(p: &mut ParseState<'i>) -> Option<Result<LengthUnit, InvalidLength<'i>>> {
    p.find(&LETTER).map(|m| match m.as_str() {
        "m" | "M" => Ok(LengthUnit::Meters),
        "f" | "F" => Ok(LengthUnit::Feet),
        "i" | "I" => Ok(LengthUnit::Inches),
        _ => Err(InvalidLength::InvalidUnit(m)),
    })
}
fn unsigned_length<'i>(
    p: &mut ParseState<'i>,
    default_unit: LengthUnit,
) -> Result<Option<(Length, SourceLoc)>, InvalidLength<'i>> {
    let start = p.pos();
    if let Some(mut value) = unsigned_number(p).transpose()? {
        let unit = length_unit_suffix(p).transpose()?.unwrap_or(default_unit);
        if unit == LengthUnit::Inches {
            let inches = match unsigned_number(p).transpose() {
                Ok(Some(inches)) => Ok(inches),
                Ok(None) => Err(InvalidLength::MissingInches(p.pos())),
                Err(e) => Err(e.into()),
            }?;
            value = value * 12.0 + inches;
        }
        Ok(Some((Length { value, unit }, start.up_to(p.pos()))))
    } else if p.is_match(&INCHES_SUFFIX) {
        let inches = match unsigned_number(p).transpose() {
            Ok(Some(inches)) => Ok(inches),
            Ok(None) => Err(InvalidLength::MissingInches(p.pos())),
            Err(e) => Err(e.into()),
        }?;
        Ok(Some((
            Length {
                value: inches,
                unit: LengthUnit::Inches,
            },
            start.up_to(p.pos()),
        )))
    } else {
        Ok(None)
    }
}
fn signed_length<'i>(
    p: &mut ParseState<'i>,
    default_unit: LengthUnit,
) -> Result<Option<(Length, SourceLoc)>, InvalidLength<'i>> {
    let start = p.pos();
    let sign = p.find(&SIGN);
    let negate = match sign {
        Some(m) => m.as_str() == "-",
        None => false,
    };
    if let Some((length, _)) = unsigned_length(p, default_unit)? {
        Ok(Some((
            Length {
                value: if negate { -length.value } else { length.value },
                unit: length.unit,
            },
            start.up_to(p.pos()),
        )))
    } else {
        Ok(None)
    }
}

enum InvalidAngle<'i> {
    InvalidNumber(ParseFloatError, ParseMatch<'i>),
    InvalidUnit(ParseMatch<'i>),
}

impl<'i> From<InvalidNumber<'i>> for InvalidAngle<'i> {
    fn from(value: InvalidNumber<'i>) -> Self {
        InvalidAngle::InvalidNumber(value.0, value.1)
    }
}

impl<'i> From<InvalidAngle<'i>> for ParseIssue {
    fn from(value: InvalidAngle<'i>) -> Self {
        match value {
            InvalidAngle::InvalidNumber(e, m) => {
                ParseIssue::error(EINVALIDANGLE, Some(e.to_string()), Some(m.loc()))
            }
            InvalidAngle::InvalidUnit(m) => ParseIssue::error(
                EINVALIDANGLEUNIT,
                Some("Invalid angle unit".into()),
                Some(m.loc()),
            ),
        }
    }
}

fn angle_unit_suffix<'i>(p: &mut ParseState<'i>) -> Option<Result<AngleUnit, InvalidAngle<'i>>> {
    p.find(&LETTER).map(|m| match m.as_str() {
        "d" | "D" => Ok(AngleUnit::Degrees),
        "g" | "G" => Ok(AngleUnit::Grads),
        "m" | "M" => Ok(AngleUnit::Mils),
        _ => Err(InvalidAngle::InvalidUnit(m)),
    })
}
fn unsigned_angle<'i>(
    p: &mut ParseState<'i>,
    default_unit: AngleUnit,
) -> Result<Option<(Angle, SourceLoc)>, InvalidAngle<'i>> {
    let start = p.pos();
    // TODO: ddd:mm:ss format
    if let Some(value) = unsigned_number(p).transpose()? {
        let unit = angle_unit_suffix(p).transpose()?.unwrap_or(default_unit);
        Ok(Some((Angle { value, unit }, start.up_to(p.pos()))))
    } else {
        Ok(None)
    }
}
fn signed_angle<'i>(
    p: &mut ParseState<'i>,
    default_unit: AngleUnit,
) -> Result<Option<(Angle, SourceLoc)>, InvalidAngle<'i>> {
    let start = p.pos();
    let sign = p.find(&SIGN);
    let negate = match sign {
        Some(m) => m.as_str() == "-",
        None => false,
    };
    if let Some((angle, _)) = unsigned_angle(p, default_unit)? {
        Ok(Some((
            Angle {
                value: if negate { -angle.value } else { angle.value },
                unit: angle.unit,
            },
            start.up_to(p.pos()),
        )))
    } else {
        Ok(None)
    }
}

enum InvalidAzimuth<'i> {
    InvalidNumber(ParseFloatError, ParseMatch<'i>),
    InvalidUnit(ParseMatch<'i>),
}

impl<'i> From<InvalidNumber<'i>> for InvalidAzimuth<'i> {
    fn from(value: InvalidNumber<'i>) -> Self {
        InvalidAzimuth::InvalidNumber(value.0, value.1)
    }
}

impl<'i> From<InvalidAngle<'i>> for InvalidAzimuth<'i> {
    fn from(value: InvalidAngle<'i>) -> Self {
        match value {
            InvalidAngle::InvalidNumber(err, m) => InvalidAzimuth::InvalidNumber(err, m),
            InvalidAngle::InvalidUnit(m) => InvalidAzimuth::InvalidUnit(m),
        }
    }
}

impl<'i> From<InvalidAzimuth<'i>> for ParseIssue {
    fn from(value: InvalidAzimuth<'i>) -> Self {
        match value {
            InvalidAzimuth::InvalidNumber(e, m) => {
                ParseIssue::error(EINVALIDAZIMUTH, Some(e.to_string()), Some(m.loc()))
            }
            InvalidAzimuth::InvalidUnit(m) => ParseIssue::error(
                EINVALIDAZIMUTHUNIT,
                Some("Invalid azimuth unit".into()),
                Some(m.loc()),
            ),
        }
    }
}

fn signed_azimuth<'i>(
    p: &mut ParseState<'i>,
    default_unit: AngleUnit,
) -> Result<Option<(Angle, SourceLoc)>, InvalidAzimuth<'i>> {
    // TODO: N50.5W style format
    signed_angle(p, default_unit).map_err(|err| err.into())
}

enum InvalidInclination<'i> {
    InvalidNumber(ParseFloatError, ParseMatch<'i>),
    OutOfRange(SourceLoc),
    InvalidUnit(ParseMatch<'i>),
}

impl<'i> From<InvalidNumber<'i>> for InvalidInclination<'i> {
    fn from(value: InvalidNumber<'i>) -> Self {
        InvalidInclination::InvalidNumber(value.0, value.1)
    }
}

impl<'i> From<InvalidInclination<'i>> for ParseIssue {
    fn from(value: InvalidInclination<'i>) -> Self {
        match value {
            InvalidInclination::InvalidNumber(e, m) => {
                ParseIssue::error(EINVALIDINCLINATION, Some(e.to_string()), Some(m.loc()))
            }
            InvalidInclination::InvalidUnit(m) => ParseIssue::error(
                EINVALIDINCLINATIONUNIT,
                Some("Invalid inclination unit".into()),
                Some(m.loc()),
            ),
            InvalidInclination::OutOfRange(loc) => ParseIssue::error(
                EINCLINATIONOUTOFRANGE,
                Some("Inclination out of range".into()),
                Some(loc),
            ),
        }
    }
}
fn inclination_unit_suffix<'i>(
    p: &mut ParseState<'i>,
) -> Option<Result<InclinationUnit, InvalidInclination<'i>>> {
    p.find(&LETTER).map(|m| match m.as_str() {
        "d" | "D" => Ok(InclinationUnit::Degrees),
        "g" | "G" => Ok(InclinationUnit::Grads),
        "m" | "M" => Ok(InclinationUnit::Mils),
        "p" | "P" => Ok(InclinationUnit::Percent),
        _ => Err(InvalidInclination::InvalidUnit(m)),
    })
}
fn unsigned_inclination<'i>(
    p: &mut ParseState<'i>,
    default_unit: InclinationUnit,
) -> Result<Option<(Inclination, SourceLoc)>, InvalidInclination<'i>> {
    let start = p.pos();
    // TODO: ddd:mm:ss format
    if let Some(value) = unsigned_number(p).transpose()? {
        let unit = inclination_unit_suffix(p)
            .transpose()?
            .unwrap_or(default_unit);
        if match unit {
            InclinationUnit::Degrees => value > 90.0,
            InclinationUnit::Grads => value > 100.0,
            InclinationUnit::Mils => value > 1600.0,
            InclinationUnit::Percent => false,
        } {
            Err(InvalidInclination::OutOfRange(start.up_to(p.pos())))
        } else {
            Ok(Some((Inclination { value, unit }, start.up_to(p.pos()))))
        }
    } else {
        Ok(None)
    }
}
fn signed_inclination<'i>(
    p: &mut ParseState<'i>,
    default_unit: InclinationUnit,
) -> Result<Option<(Inclination, SourceLoc)>, InvalidInclination<'i>> {
    let start = p.pos();
    let sign = p.find(&SIGN);
    let negate = match sign {
        Some(m) => m.as_str() == "-",
        None => false,
    };
    if let Some((inclination, _)) = unsigned_inclination(p, default_unit)? {
        Ok(Some((
            Inclination {
                value: if negate {
                    -inclination.value
                } else {
                    inclination.value
                },
                unit: inclination.unit,
            },
            start.up_to(p.pos()),
        )))
    } else {
        Ok(None)
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
#[path = "parser_test.rs"]
mod tests;
