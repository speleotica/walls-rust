use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::types::{ParseIssue, SourceLoc};

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct InvalidValue {
    #[serde(rename = "INVALID")]
    pub invalid: String,
    pub issues: Option<Vec<usize>>,
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
#[schemars(deny_unknown_fields)]
pub enum ShotType {
    CompassAndTape,
    Rectilinear,
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
#[schemars(deny_unknown_fields)]
pub enum LengthUnit {
    Meters,
    Feet,
    Inches,
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
#[schemars(deny_unknown_fields)]
pub struct Length {
    pub value: f64,
    pub unit: LengthUnit,
}

impl Length {
    pub fn meters(value: f64) -> Length {
        Length {
            value,
            unit: LengthUnit::Meters,
        }
    }
    pub fn feet(value: f64) -> Length {
        Length {
            value,
            unit: LengthUnit::Feet,
        }
    }
    pub fn inches(value: f64) -> Length {
        Length {
            value,
            unit: LengthUnit::Inches,
        }
    }
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
#[serde(untagged)]
pub enum MaybeValidLength {
    Valid(Length),
    Invalid(InvalidValue),
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
#[schemars(deny_unknown_fields)]
pub enum AngleUnit {
    Degrees,
    Mils,
    Grads,
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
#[schemars(deny_unknown_fields)]
pub struct Angle {
    pub value: f64,
    pub unit: AngleUnit,
}

impl Angle {
    pub fn degrees(value: f64) -> Angle {
        Angle {
            value,
            unit: AngleUnit::Degrees,
        }
    }
    pub fn mils(value: f64) -> Angle {
        Angle {
            value,
            unit: AngleUnit::Mils,
        }
    }
    pub fn grads(value: f64) -> Angle {
        Angle {
            value,
            unit: AngleUnit::Grads,
        }
    }
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
#[serde(untagged)]
pub enum MaybeValidAngle {
    Valid(Angle),
    Invalid(InvalidValue),
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
#[schemars(deny_unknown_fields)]
pub enum InclinationUnit {
    Degrees,
    Mils,
    Grads,
    Percent,
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
#[schemars(deny_unknown_fields)]
pub struct Inclination {
    pub value: f64,
    pub unit: InclinationUnit,
}

impl Inclination {
    pub fn degrees(value: f64) -> Inclination {
        Inclination {
            value,
            unit: InclinationUnit::Degrees,
        }
    }
    pub fn mils(value: f64) -> Inclination {
        Inclination {
            value,
            unit: InclinationUnit::Mils,
        }
    }
    pub fn grads(value: f64) -> Inclination {
        Inclination {
            value,
            unit: InclinationUnit::Grads,
        }
    }
    pub fn percent(value: f64) -> Inclination {
        Inclination {
            value,
            unit: InclinationUnit::Percent,
        }
    }
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
#[serde(untagged)]
pub enum MaybeValidInclination {
    Valid(Inclination),
    Invalid(InvalidValue),
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
#[schemars(deny_unknown_fields)]
pub enum StationNameCaseConversion {
    Upper,
    Lower,
    Mixed,
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
#[schemars(deny_unknown_fields)]
pub enum CompassAndTapeItem {
    Distance,
    Azimuth,
    Inclination,
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
#[schemars(deny_unknown_fields)]
pub enum RectilinearItem {
    Easting,
    Northing,
    Elevation,
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
#[schemars(deny_unknown_fields)]
pub enum OrderItem {
    Distance,
    Azimuth,
    Inclination,
    Easting,
    Northing,
    Elevation,
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
#[serde(untagged)]
pub enum MaybeValidOrderItem {
    Valid(OrderItem),
    Invalid(InvalidValue),
}

impl From<OrderItem> for MaybeValidOrderItem {
    fn from(item: OrderItem) -> Self {
        MaybeValidOrderItem::Valid(item)
    }
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
#[schemars(deny_unknown_fields)]
pub enum LrudStyle {
    FromStationPerpendicular,
    ToStationPerpendicular,
    FromStationBisector,
    ToStationBisector,
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
#[schemars(deny_unknown_fields)]
pub enum LrudItem {
    Left,
    Right,
    Up,
    Down,
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
#[serde(untagged)]
pub enum MaybeValidLrudItem {
    Valid(LrudItem),
    Invalid(InvalidValue),
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
#[schemars(deny_unknown_fields)]
pub enum TapingMethod {
    InstrumentToTarget,
    StationToStation,
    InstrumentToStation,
    StationToTarget,
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[schemars(deny_unknown_fields)]
pub struct SrvSettings {
    pub shot_type: ShotType,
    pub compass_and_tape_order: Vec<CompassAndTapeItem>,
    pub rectilinear_order: Vec<RectilinearItem>,
    pub primary_distance_unit: LengthUnit,
    pub secondary_distance_unit: LengthUnit,
    pub frontsight_azimuth_unit: AngleUnit,
    pub backsight_azimuth_unit: AngleUnit,
    pub frontsight_inclination_unit: InclinationUnit,
    pub backsight_inclination_unit: InclinationUnit,
    pub magnetic_declination: Angle,
    pub grid_north_correction: Angle,
    pub rectilinear_north_correction: Angle,
    pub primary_distance_correction: Length,
    pub secondary_distance_correction: Length,
    pub frontsight_azimuth_correction: Angle,
    pub backsight_azimuth_correction: Angle,
    pub frontsight_inclination_correction: Inclination,
    pub backsight_inclination_correction: Inclination,
    pub height_adjustment: Length,
    pub backsight_azimuth_options: BacksightOptions,
    pub backsight_inclination_options: BacksightOptions,
    pub station_name_case_conversion: StationNameCaseConversion,
    pub lrud_style: LrudStyle,
    pub lrud_order: [LrudItem; 4],
    /// [ PREFIX1, PREFIX2, PREFIX3 ]
    pub prefix: [String; 3],
    pub taping_method: TapingMethod,
    pub horizontal_unit_variance: f64,
    pub vertical_unit_variance: f64,
    pub flag: Option<String>,
    pub segment: Option<String>,
}

impl SrvSettings {
    pub fn default() -> SrvSettings {
        SrvSettings {
            shot_type: ShotType::CompassAndTape,
            compass_and_tape_order: vec![
                CompassAndTapeItem::Distance,
                CompassAndTapeItem::Azimuth,
                CompassAndTapeItem::Inclination,
            ],
            rectilinear_order: vec![
                RectilinearItem::Easting,
                RectilinearItem::Northing,
                RectilinearItem::Elevation,
            ],
            primary_distance_unit: LengthUnit::Meters,
            secondary_distance_unit: LengthUnit::Meters,
            frontsight_azimuth_unit: AngleUnit::Degrees,
            backsight_azimuth_unit: AngleUnit::Degrees,
            frontsight_inclination_unit: InclinationUnit::Degrees,
            backsight_inclination_unit: InclinationUnit::Degrees,
            magnetic_declination: Angle::degrees(0.0),
            grid_north_correction: Angle::degrees(0.0),
            rectilinear_north_correction: Angle::degrees(0.0),
            primary_distance_correction: Length::meters(0.0),
            secondary_distance_correction: Length::meters(0.0),
            frontsight_azimuth_correction: Angle::degrees(0.0),
            backsight_azimuth_correction: Angle::degrees(0.0),
            frontsight_inclination_correction: Inclination::degrees(0.0),
            backsight_inclination_correction: Inclination::degrees(0.0),
            height_adjustment: Length::meters(0.0),
            backsight_azimuth_options: BacksightOptions::default(),
            backsight_inclination_options: BacksightOptions::default(),
            station_name_case_conversion: StationNameCaseConversion::Mixed,
            lrud_style: LrudStyle::FromStationPerpendicular,
            lrud_order: [
                LrudItem::Left,
                LrudItem::Right,
                LrudItem::Up,
                LrudItem::Down,
            ],
            prefix: ["".into(), "".into(), "".into()],
            taping_method: TapingMethod::InstrumentToTarget,
            horizontal_unit_variance: 0.0,
            vertical_unit_variance: 0.0,
            flag: None,
            segment: None,
        }
    }

    pub fn apply_option(&mut self, option: &UnitsOption) {
        match option {
            UnitsOption::FrontsightAzimuthCorrection {
                correction,
                loc: _,
                locs: _,
            } => self.frontsight_azimuth_correction = *correction,
            UnitsOption::FrontsightAzimuthUnit {
                unit,
                loc: _,
                locs: _,
            } => self.frontsight_azimuth_unit = *unit,
            UnitsOption::BacksightAzimuthCorrection {
                correction,
                loc: _,
                locs: _,
            } => self.backsight_azimuth_correction = *correction,
            UnitsOption::BacksightAzimuthType(options) => {
                self.backsight_azimuth_options = options.clone();
            }
            UnitsOption::BacksightAzimuthUnit {
                unit,
                loc: _,
                locs: _,
            } => self.backsight_azimuth_unit = *unit,
            UnitsOption::FrontsightInclinationCorrection {
                correction,
                loc: _,
                locs: _,
            } => self.frontsight_inclination_correction = *correction,
            UnitsOption::FrontsightInclinationUnit {
                unit,
                loc: _,
                locs: _,
            } => self.frontsight_inclination_unit = *unit,
            UnitsOption::BacksightInclinationCorrection {
                correction,
                loc: _,
                locs: _,
            } => self.backsight_inclination_correction = *correction,
            UnitsOption::BacksightInclinationType(options) => {
                self.backsight_inclination_options = options.clone()
            }
            UnitsOption::BacksightInclinationUnit {
                unit,
                loc: _,
                locs: _,
            } => self.backsight_inclination_unit = *unit,
            UnitsOption::CompassAndTape { loc: _ } => self.shot_type = ShotType::CompassAndTape,
            UnitsOption::CompassAndTapeOrder {
                order,
                loc: _,
                locs: _,
            } => self.compass_and_tape_order = order.clone(),
            UnitsOption::DistanceUnit { unit, loc: _ } => {
                self.primary_distance_unit = *unit;
                self.secondary_distance_unit = *unit;
            }
            UnitsOption::Flag {
                flag,
                loc: _,
                locs: _,
            } => self.flag = flag.clone(),
            UnitsOption::GridNorthCorrection {
                correction,
                loc: _,
                locs: _,
            } => self.grid_north_correction = *correction,
            UnitsOption::HeightAdjustment {
                correction,
                loc: _,
                locs: _,
            } => self.height_adjustment = *correction,
            UnitsOption::HorizontalUnitVariance {
                variance,
                loc: _,
                locs: _,
            } => self.horizontal_unit_variance = *variance,
            UnitsOption::VerticalUnitVariance {
                variance,
                loc: _,
                locs: _,
            } => self.vertical_unit_variance = *variance,
            UnitsOption::UnitVariance {
                variance,
                loc: _,
                locs: _,
            } => {
                self.horizontal_unit_variance = *variance;
                self.vertical_unit_variance = *variance;
            }
            UnitsOption::Lrud {
                style,
                order,
                loc: _,
                locs: _,
            } => {
                self.lrud_style = *style;
                if let Some(order) = order {
                    self.lrud_order = *order;
                }
            }
            UnitsOption::Macro {
                name: _,
                value: _,
                loc: _,
                locs: _,
            } => todo!(),
            UnitsOption::MagneticDeclination {
                declination,
                loc: _,
                locs: _,
            } => self.magnetic_declination = *declination,
            UnitsOption::Prefix {
                level,
                prefix,
                loc: _,
                locs: _,
            } => self.prefix[usize::from(*level)] = prefix.clone().unwrap_or("".into()),
            UnitsOption::PrimaryDistanceCorrection {
                correction,
                loc: _,
                locs: _,
            } => self.primary_distance_correction = *correction,
            UnitsOption::PrimaryDistanceUnit {
                unit,
                loc: _,
                locs: _,
            } => self.primary_distance_unit = *unit,
            UnitsOption::Rectilinear { loc: _ } => self.shot_type = ShotType::Rectilinear,
            UnitsOption::RectilinearNorthCorrection {
                correction,
                loc: _,
                locs: _,
            } => self.rectilinear_north_correction = *correction,
            UnitsOption::RectilinearOrder {
                order,
                loc: _,
                locs: _,
            } => self.rectilinear_order = order.clone(),
            UnitsOption::Reset { loc: _ } => {} // handled by `WallsSrvParser::apply_option`
            UnitsOption::Restore { loc: _ } => {} // handled by `WallsSrvParser::apply_option`
            UnitsOption::Save { loc: _ } => {}  // handled by `WallsSrvParser::apply_option`
            UnitsOption::SecondaryDistanceCorrection {
                correction,
                loc: _,
                locs: _,
            } => self.secondary_distance_correction = *correction,
            UnitsOption::SecondaryDistanceUnit {
                unit,
                loc: _,
                locs: _,
            } => self.secondary_distance_unit = *unit,
            UnitsOption::StationNameCase {
                conversion,
                loc: _,
                locs: _,
            } => self.station_name_case_conversion = *conversion,
            UnitsOption::TapingMethod {
                method,
                loc: _,
                locs: _,
            } => self.taping_method = *method,
        };
    }
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[schemars(deny_unknown_fields)]
pub struct BacksightOptions {
    pub is_corrected: bool,
    pub tolerance: Angle,
    pub do_not_average: bool,
    pub locs: Option<BacksightOptionsLocs>,
}

impl BacksightOptions {
    pub fn default() -> BacksightOptions {
        BacksightOptions {
            is_corrected: false,
            tolerance: Angle::degrees(5.0),
            do_not_average: false,
            locs: None,
        }
    }
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[schemars(deny_unknown_fields)]
pub struct BacksightOptionsLocs {
    pub is_corrected: Option<SourceLoc>,
    pub tolerance: Option<SourceLoc>,
    pub do_not_average: Option<SourceLoc>,
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct InvalidBacksightOptions {
    pub is_corrected: bool,
    pub tolerance: Option<Angle>,
    pub do_not_average: bool,
    pub locs: Option<BacksightOptionsLocs>,
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub enum MaybeValidBacksightOptions {
    Valid(BacksightOptions),
    Invalid {
        #[serde(rename = "INVALID")]
        invalid: InvalidBacksightOptions,
        issues: Option<Vec<usize>>,
    },
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
#[schemars(deny_unknown_fields)]
pub enum PrefixLevel {
    Prefix1,
    Prefix2,
    Prefix3,
}

impl From<PrefixLevel> for usize {
    fn from(level: PrefixLevel) -> Self {
        match level {
            PrefixLevel::Prefix1 => 0,
            PrefixLevel::Prefix2 => 1,
            PrefixLevel::Prefix3 => 2,
        }
    }
}

impl From<usize> for PrefixLevel {
    fn from(level: usize) -> Self {
        match level {
            0 => PrefixLevel::Prefix1,
            1 => PrefixLevel::Prefix2,
            2 => PrefixLevel::Prefix3,
            _ => panic!("Invalid prefix level index: {level}"),
        }
    }
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[serde(tag = "option")]
#[schemars(deny_unknown_fields)]
pub enum UnitsOption {
    #[schemars(title = "CompassAndTapeOption")]
    CompassAndTape { loc: Option<SourceLoc> },
    #[schemars(title = "RectilinearOption")]
    Rectilinear { loc: Option<SourceLoc> },
    #[schemars(title = "CompassAndTapeOrderOption")]
    CompassAndTapeOrder {
        order: Vec<CompassAndTapeItem>,
        loc: Option<SourceLoc>,
        locs: Option<OrderOptionLocs>,
    },
    #[schemars(title = "RectilinearOrderOption")]
    RectilinearOrder {
        order: Vec<RectilinearItem>,
        loc: Option<SourceLoc>,
        locs: Option<OrderOptionLocs>,
    },
    #[schemars(title = "FrontsightAzimuthUnitOption")]
    FrontsightAzimuthUnit {
        unit: AngleUnit,
        loc: Option<SourceLoc>,
        locs: Option<UnitOptionLocs>,
    },
    #[schemars(title = "BacksightAzimuthUnitOption")]
    BacksightAzimuthUnit {
        unit: AngleUnit,
        loc: Option<SourceLoc>,
        locs: Option<UnitOptionLocs>,
    },
    #[schemars(title = "PrimaryDistanceUnitOption")]
    PrimaryDistanceUnit {
        unit: LengthUnit,
        loc: Option<SourceLoc>,
        locs: Option<UnitOptionLocs>,
    },
    #[schemars(title = "SecondaryDistanceUnitOption")]
    SecondaryDistanceUnit {
        unit: LengthUnit,
        loc: Option<SourceLoc>,
        locs: Option<UnitOptionLocs>,
    },
    #[schemars(title = "DistanceUnitOption")]
    DistanceUnit {
        unit: LengthUnit,
        loc: Option<SourceLoc>,
    },
    #[schemars(title = "FrontsightInclinationUnitOption")]
    FrontsightInclinationUnit {
        unit: InclinationUnit,
        loc: Option<SourceLoc>,
        locs: Option<UnitOptionLocs>,
    },
    #[schemars(title = "BacksightInclinationUnitOption")]
    BacksightInclinationUnit {
        unit: InclinationUnit,
        loc: Option<SourceLoc>,
        locs: Option<UnitOptionLocs>,
    },
    #[schemars(title = "MagneticDeclinationOption")]
    MagneticDeclination {
        declination: Angle,
        loc: Option<SourceLoc>,
        locs: Option<DeclinationOptionLocs>,
    },
    #[schemars(title = "GridNorthCorrectionOption")]
    GridNorthCorrection {
        correction: Angle,
        loc: Option<SourceLoc>,
        locs: Option<CorrectionOptionLocs>,
    },
    #[schemars(title = "RectilinearNorthCorrectionOption")]
    RectilinearNorthCorrection {
        correction: Angle,
        loc: Option<SourceLoc>,
        locs: Option<CorrectionOptionLocs>,
    },
    #[schemars(title = "PrimaryDistanceCorrectionOption")]
    PrimaryDistanceCorrection {
        correction: Length,
        loc: Option<SourceLoc>,
        locs: Option<CorrectionOptionLocs>,
    },
    #[schemars(title = "SecondaryDistanceCorrectionOption")]
    SecondaryDistanceCorrection {
        correction: Length,
        loc: Option<SourceLoc>,
        locs: Option<CorrectionOptionLocs>,
    },
    #[schemars(title = "FrontsightAzimuthCorrectionOption")]
    FrontsightAzimuthCorrection {
        correction: Angle,
        loc: Option<SourceLoc>,
        locs: Option<CorrectionOptionLocs>,
    },
    #[schemars(title = "BacksightAzimuthCorrectionOption")]
    BacksightAzimuthCorrection {
        correction: Angle,
        loc: Option<SourceLoc>,
        locs: Option<CorrectionOptionLocs>,
    },
    #[schemars(title = "FrontsightInclinationCorrectionOption")]
    FrontsightInclinationCorrection {
        correction: Inclination,
        loc: Option<SourceLoc>,
        locs: Option<CorrectionOptionLocs>,
    },
    #[schemars(title = "FrontsightInclinationCorrectionOption")]
    BacksightInclinationCorrection {
        correction: Inclination,
        loc: Option<SourceLoc>,
        locs: Option<CorrectionOptionLocs>,
    },
    #[schemars(title = "HeightAdjustmentOption")]
    HeightAdjustment {
        correction: Length,
        loc: Option<SourceLoc>,
        locs: Option<CorrectionOptionLocs>,
    },
    #[schemars(title = "BacksightAzimuthType")]
    BacksightAzimuthType(BacksightOptions),
    #[schemars(title = "BacksightInclinationType")]
    BacksightInclinationType(BacksightOptions),
    #[schemars(title = "ResetOption")]
    Reset { loc: Option<SourceLoc> },
    #[schemars(title = "SaveOption")]
    Save { loc: Option<SourceLoc> },
    #[schemars(title = "RestoreOption")]
    Restore { loc: Option<SourceLoc> },
    #[schemars(title = "StationNameCaseOption")]
    StationNameCase {
        conversion: StationNameCaseConversion,
        loc: Option<SourceLoc>,
        locs: Option<StationNameCaseOptionLocs>,
    },
    #[schemars(title = "LrudOption")]
    Lrud {
        style: LrudStyle,
        order: Option<[LrudItem; 4]>,
        loc: Option<SourceLoc>,
        locs: Option<LrudOptionLocs>,
    },
    #[schemars(title = "PrefixOption")]
    Prefix {
        level: PrefixLevel,
        prefix: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<PrefixOptionLocs>,
    },
    #[schemars(title = "TapingMethodOption")]
    TapingMethod {
        method: TapingMethod,
        loc: Option<SourceLoc>,
        locs: Option<TapingMethodOptionLocs>,
    },
    #[schemars(title = "UnitVarianceOption")]
    UnitVariance {
        variance: f64,
        loc: Option<SourceLoc>,
        locs: Option<VarianceOptionLocs>,
    },
    #[schemars(title = "HorizontalUnitVarianceOption")]
    HorizontalUnitVariance {
        variance: f64,
        loc: Option<SourceLoc>,
        locs: Option<VarianceOptionLocs>,
    },
    #[schemars(title = "VerticalUnitVarianceOption")]
    VerticalUnitVariance {
        variance: f64,
        loc: Option<SourceLoc>,
        locs: Option<VarianceOptionLocs>,
    },
    #[schemars(title = "FlagOption")]
    Flag {
        flag: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<FlagOptionLocs>,
    },
    #[schemars(title = "MacroOption")]
    Macro {
        name: String,
        value: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<MacroOptionLocs>,
    },
}

impl UnitsOption {
    pub fn compass_and_tape_order(
        order: Vec<CompassAndTapeItem>,
        option_loc: SourceLoc,
        order_loc: Option<SourceLoc>,
    ) -> UnitsOption {
        UnitsOption::CompassAndTapeOrder {
            order,
            loc: order_loc.and_then(|u| Some(option_loc.start.up_to(u.end))),
            locs: Some(OrderOptionLocs {
                option: option_loc,
                order: order_loc,
            }),
        }
    }
    pub fn rectilinear_order(
        order: Vec<RectilinearItem>,
        option_loc: SourceLoc,
        order_loc: Option<SourceLoc>,
    ) -> UnitsOption {
        UnitsOption::RectilinearOrder {
            order,
            loc: order_loc.and_then(|u| Some(option_loc.start.up_to(u.end))),
            locs: Some(OrderOptionLocs {
                option: option_loc,
                order: order_loc,
            }),
        }
    }
    pub fn distance_unit(unit: LengthUnit, loc: Option<SourceLoc>) -> UnitsOption {
        UnitsOption::DistanceUnit { unit, loc }
    }
    pub fn primary_distance_unit(
        unit: LengthUnit,
        option_loc: SourceLoc,
        unit_loc: Option<SourceLoc>,
    ) -> UnitsOption {
        UnitsOption::PrimaryDistanceUnit {
            unit,
            loc: unit_loc.and_then(|u| Some(option_loc.start.up_to(u.end))),
            locs: Some(UnitOptionLocs {
                option: option_loc,
                unit: unit_loc,
            }),
        }
    }
    pub fn secondary_distance_unit(
        unit: LengthUnit,
        option_loc: SourceLoc,
        unit_loc: Option<SourceLoc>,
    ) -> UnitsOption {
        UnitsOption::SecondaryDistanceUnit {
            unit,
            loc: unit_loc.and_then(|u| Some(option_loc.start.up_to(u.end))),
            locs: Some(UnitOptionLocs {
                option: option_loc,
                unit: unit_loc,
            }),
        }
    }
    pub fn frontsight_azimuth_unit(
        unit: AngleUnit,
        option_loc: SourceLoc,
        unit_loc: Option<SourceLoc>,
    ) -> UnitsOption {
        UnitsOption::FrontsightAzimuthUnit {
            unit,
            loc: unit_loc.and_then(|u| Some(option_loc.start.up_to(u.end))),
            locs: Some(UnitOptionLocs {
                option: option_loc,
                unit: unit_loc,
            }),
        }
    }
    pub fn backsight_azimuth_unit(
        unit: AngleUnit,
        option_loc: SourceLoc,
        unit_loc: Option<SourceLoc>,
    ) -> UnitsOption {
        UnitsOption::BacksightAzimuthUnit {
            unit,
            loc: unit_loc.and_then(|u| Some(option_loc.start.up_to(u.end))),
            locs: Some(UnitOptionLocs {
                option: option_loc,
                unit: unit_loc,
            }),
        }
    }
    pub fn frontsight_inclination_unit(
        unit: InclinationUnit,
        option_loc: SourceLoc,
        unit_loc: Option<SourceLoc>,
    ) -> UnitsOption {
        UnitsOption::FrontsightInclinationUnit {
            unit,
            loc: unit_loc.and_then(|u| Some(option_loc.start.up_to(u.end))),
            locs: Some(UnitOptionLocs {
                option: option_loc,
                unit: unit_loc,
            }),
        }
    }
    pub fn backsight_inclination_unit(
        unit: InclinationUnit,
        option_loc: SourceLoc,
        unit_loc: Option<SourceLoc>,
    ) -> UnitsOption {
        UnitsOption::BacksightInclinationUnit {
            unit,
            loc: unit_loc.and_then(|u| Some(option_loc.start.up_to(u.end))),
            locs: Some(UnitOptionLocs {
                option: option_loc,
                unit: unit_loc,
            }),
        }
    }
    pub fn primary_distance_correction(
        correction: Length,
        option_loc: SourceLoc,
        correction_loc: Option<SourceLoc>,
    ) -> UnitsOption {
        UnitsOption::PrimaryDistanceCorrection {
            correction,
            loc: correction_loc.and_then(|u| Some(option_loc.start.up_to(u.end))),
            locs: Some(CorrectionOptionLocs {
                option: option_loc,
                correction: correction_loc,
            }),
        }
    }
    pub fn secondary_distance_correction(
        correction: Length,
        option_loc: SourceLoc,
        correction_loc: Option<SourceLoc>,
    ) -> UnitsOption {
        UnitsOption::SecondaryDistanceCorrection {
            correction,
            loc: correction_loc.and_then(|u| Some(option_loc.start.up_to(u.end))),
            locs: Some(CorrectionOptionLocs {
                option: option_loc,
                correction: correction_loc,
            }),
        }
    }
    pub fn height_adjustment(
        correction: Length,
        option_loc: SourceLoc,
        correction_loc: Option<SourceLoc>,
    ) -> UnitsOption {
        UnitsOption::HeightAdjustment {
            correction,
            loc: correction_loc.and_then(|u| Some(option_loc.start.up_to(u.end))),
            locs: Some(CorrectionOptionLocs {
                option: option_loc,
                correction: correction_loc,
            }),
        }
    }
    pub fn frontsight_azimuth_correction(
        correction: Angle,
        option_loc: SourceLoc,
        correction_loc: Option<SourceLoc>,
    ) -> UnitsOption {
        UnitsOption::FrontsightAzimuthCorrection {
            correction,
            loc: correction_loc.and_then(|u| Some(option_loc.start.up_to(u.end))),
            locs: Some(CorrectionOptionLocs {
                option: option_loc,
                correction: correction_loc,
            }),
        }
    }
    pub fn backsight_azimuth_correction(
        correction: Angle,
        option_loc: SourceLoc,
        correction_loc: Option<SourceLoc>,
    ) -> UnitsOption {
        UnitsOption::BacksightAzimuthCorrection {
            correction,
            loc: correction_loc.and_then(|u| Some(option_loc.start.up_to(u.end))),
            locs: Some(CorrectionOptionLocs {
                option: option_loc,
                correction: correction_loc,
            }),
        }
    }
    pub fn frontsight_inclination_correction(
        correction: Inclination,
        option_loc: SourceLoc,
        correction_loc: Option<SourceLoc>,
    ) -> UnitsOption {
        UnitsOption::FrontsightInclinationCorrection {
            correction,
            loc: correction_loc.and_then(|u| Some(option_loc.start.up_to(u.end))),
            locs: Some(CorrectionOptionLocs {
                option: option_loc,
                correction: correction_loc,
            }),
        }
    }
    pub fn backsight_inclination_correction(
        correction: Inclination,
        option_loc: SourceLoc,
        correction_loc: Option<SourceLoc>,
    ) -> UnitsOption {
        UnitsOption::BacksightInclinationCorrection {
            correction,
            loc: correction_loc.and_then(|u| Some(option_loc.start.up_to(u.end))),
            locs: Some(CorrectionOptionLocs {
                option: option_loc,
                correction: correction_loc,
            }),
        }
    }
    pub fn magnetic_declination(
        declination: Angle,
        option_loc: SourceLoc,
        declination_loc: Option<SourceLoc>,
    ) -> UnitsOption {
        UnitsOption::MagneticDeclination {
            declination,
            loc: declination_loc.and_then(|u| Some(option_loc.start.up_to(u.end))),
            locs: Some(DeclinationOptionLocs {
                option: option_loc,
                declination: declination_loc,
            }),
        }
    }
    pub fn grid_north_correction(
        correction: Angle,
        option_loc: SourceLoc,
        correction_loc: Option<SourceLoc>,
    ) -> UnitsOption {
        UnitsOption::GridNorthCorrection {
            correction,
            loc: correction_loc.and_then(|u| Some(option_loc.start.up_to(u.end))),
            locs: Some(CorrectionOptionLocs {
                option: option_loc,
                correction: correction_loc,
            }),
        }
    }
    pub fn rect_north_correction(
        correction: Angle,
        option_loc: SourceLoc,
        correction_loc: Option<SourceLoc>,
    ) -> UnitsOption {
        UnitsOption::RectilinearNorthCorrection {
            correction,
            loc: correction_loc.and_then(|u| Some(option_loc.start.up_to(u.end))),
            locs: Some(CorrectionOptionLocs {
                option: option_loc,
                correction: correction_loc,
            }),
        }
    }
    pub fn station_name_case(
        conversion: StationNameCaseConversion,
        option_loc: SourceLoc,
        conversion_loc: Option<SourceLoc>,
    ) -> UnitsOption {
        UnitsOption::StationNameCase {
            conversion,
            loc: conversion_loc.and_then(|u| Some(option_loc.start.up_to(u.end))),
            locs: Some(StationNameCaseOptionLocs {
                option: option_loc,
                conversion: conversion_loc,
            }),
        }
    }
    pub fn taping_method(
        method: TapingMethod,
        option_loc: SourceLoc,
        method_loc: Option<SourceLoc>,
    ) -> UnitsOption {
        UnitsOption::TapingMethod {
            method,
            loc: method_loc.and_then(|u| Some(option_loc.start.up_to(u.end))),
            locs: Some(TapingMethodOptionLocs {
                option: option_loc,
                method: method_loc,
            }),
        }
    }
    pub fn horizontal_unit_variance(
        variance: f64,
        option_loc: SourceLoc,
        variance_loc: Option<SourceLoc>,
    ) -> UnitsOption {
        UnitsOption::HorizontalUnitVariance {
            variance,
            loc: variance_loc.and_then(|u| Some(option_loc.start.up_to(u.end))),
            locs: Some(VarianceOptionLocs {
                option: option_loc,
                variance: variance_loc,
            }),
        }
    }
    pub fn vertical_unit_variance(
        variance: f64,
        option_loc: SourceLoc,
        variance_loc: Option<SourceLoc>,
    ) -> UnitsOption {
        UnitsOption::VerticalUnitVariance {
            variance,
            loc: variance_loc.and_then(|u| Some(option_loc.start.up_to(u.end))),
            locs: Some(VarianceOptionLocs {
                option: option_loc,
                variance: variance_loc,
            }),
        }
    }
    pub fn unit_variance(
        variance: f64,
        option_loc: SourceLoc,
        variance_loc: Option<SourceLoc>,
    ) -> UnitsOption {
        UnitsOption::UnitVariance {
            variance,
            loc: variance_loc.and_then(|u| Some(option_loc.start.up_to(u.end))),
            locs: Some(VarianceOptionLocs {
                option: option_loc,
                variance: variance_loc,
            }),
        }
    }
    pub fn flag(
        flag: Option<String>,
        option_loc: SourceLoc,
        flag_loc: Option<SourceLoc>,
    ) -> UnitsOption {
        UnitsOption::Flag {
            flag,
            loc: flag_loc.and_then(|u| Some(option_loc.start.up_to(u.end))),
            locs: Some(FlagOptionLocs {
                option: option_loc,
                flag: flag_loc,
            }),
        }
    }
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct OrderOptionLocs {
    pub option: SourceLoc,
    pub order: Option<SourceLoc>,
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct UnitOptionLocs {
    pub option: SourceLoc,
    pub unit: Option<SourceLoc>,
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct CorrectionOptionLocs {
    pub option: SourceLoc,
    pub correction: Option<SourceLoc>,
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct DeclinationOptionLocs {
    pub option: SourceLoc,
    pub declination: Option<SourceLoc>,
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct StationNameCaseOptionLocs {
    pub option: SourceLoc,
    pub conversion: Option<SourceLoc>,
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct LrudOptionLocs {
    pub option: SourceLoc,
    pub style: Option<SourceLoc>,
    pub order: Option<SourceLoc>,
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct PrefixOptionLocs {
    pub option: SourceLoc,
    pub prefix: Option<SourceLoc>,
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct TapingMethodOptionLocs {
    pub option: SourceLoc,
    pub method: Option<SourceLoc>,
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct VarianceOptionLocs {
    pub option: SourceLoc,
    pub variance: Option<SourceLoc>,
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct FlagOptionLocs {
    pub option: SourceLoc,
    pub flag: Option<SourceLoc>,
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct MacroOptionLocs {
    pub option: SourceLoc,
    pub name: Option<SourceLoc>,
    pub value: Option<SourceLoc>,
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
#[serde(tag = "option")]
pub enum InvalidUnitsOption {
    #[schemars(title = "InvalidOrderOption")]
    Order {
        order: Option<Vec<MaybeValidOrderItem>>,
        loc: Option<SourceLoc>,
        locs: Option<OrderOptionLocs>,
    },
    #[schemars(title = "InvalidFrontsightAzimuthUnitOption")]
    FrontsightAzimuthUnit {
        unit: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<UnitOptionLocs>,
    },
    #[schemars(title = "InvalidBacksightAzimuthUnitOption")]
    BacksightAzimuthUnit {
        unit: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<UnitOptionLocs>,
    },
    #[schemars(title = "InvalidPrimaryDistanceUnitOption")]
    PrimaryDistanceUnit {
        unit: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<UnitOptionLocs>,
    },
    #[schemars(title = "InvalidSecondaryDistanceUnitOption")]
    SecondaryDistanceUnit {
        unit: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<UnitOptionLocs>,
    },
    #[schemars(title = "InvalidFrontsightInclinationUnitOption")]
    FrontsightInclinationUnit {
        unit: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<UnitOptionLocs>,
    },
    #[schemars(title = "InvalidBacksightInclinationUnitOption")]
    BacksightInclinationUnit {
        unit: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<UnitOptionLocs>,
    },
    #[schemars(title = "InvalidMagneticDeclinationOption")]
    MagneticDeclination {
        declination: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<DeclinationOptionLocs>,
    },
    #[schemars(title = "InvalidGridNorthCorrectionOption")]
    GridNorthCorrection {
        correction: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<CorrectionOptionLocs>,
    },
    #[schemars(title = "InvalidRectilinearNorthCorrectionOption")]
    RectilinearNorthCorrection {
        correction: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<CorrectionOptionLocs>,
    },
    #[schemars(title = "InvalidPrimaryDistanceCorrectionOption")]
    PrimaryDistanceCorrection {
        correction: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<CorrectionOptionLocs>,
    },
    #[schemars(title = "InvalidSecondaryDistanceCorrectionOption")]
    SecondaryDistanceCorrection {
        correction: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<CorrectionOptionLocs>,
    },
    #[schemars(title = "InvalidFrontsightAzimuthCorrectionOption")]
    FrontsightAzimuthCorrection {
        correction: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<CorrectionOptionLocs>,
    },
    #[schemars(title = "InvalidBacksightAzimuthCorrectionOption")]
    BacksightAzimuthCorrection {
        correction: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<CorrectionOptionLocs>,
    },
    #[schemars(title = "InvalidFrontsightInclinationCorrectionOption")]
    FrontsightInclinationCorrection {
        correction: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<CorrectionOptionLocs>,
    },
    #[schemars(title = "InvalidBacksightInclinationCorrectionOption")]
    BacksightInclinationCorrection {
        correction: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<CorrectionOptionLocs>,
    },
    #[schemars(title = "InvalidHeightAdjustmentOption")]
    HeightAdjustment {
        correction: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<CorrectionOptionLocs>,
    },
    #[schemars(title = "InvalidBacksightAzimuthType")]
    BacksightAzimuthType(InvalidBacksightOptions),
    #[schemars(title = "InvalidBacksightInclinationType")]
    BacksightInclinationType(InvalidBacksightOptions),
    #[schemars(title = "InvalidStationNameCaseOption")]
    StationNameCase {
        conversion: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<StationNameCaseOptionLocs>,
    },
    #[schemars(title = "InvalidLrudOption")]
    Lrud {
        // TODO: make maybe valid type
        style: Option<String>,
        // TODO: make maybe valid type
        order: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<LrudOptionLocs>,
    },
    #[schemars(title = "InvalidTapingMethodOption")]
    TapingMethod {
        method: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<TapingMethodOptionLocs>,
    },
    #[schemars(title = "InvalidUnitVarianceOption")]
    UnitVariance {
        variance: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<VarianceOptionLocs>,
    },
    #[schemars(title = "InvalidHorizontalUnitVarianceOption")]
    HorizontalUnitVariance {
        variance: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<VarianceOptionLocs>,
    },
    #[schemars(title = "InvalidVerticalUnitVarianceOption")]
    VerticalUnitVariance {
        variance: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<VarianceOptionLocs>,
    },
    #[schemars(title = "InvalidMacroOption")]
    Macro {
        name: Option<String>,
        value: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<MacroOptionLocs>,
    },
    #[schemars(title = "InvalidUnknownOption")]
    Unknown {
        value: String,
        loc: Option<SourceLoc>,
    },
}

impl InvalidUnitsOption {
    pub fn order(
        order: Option<Vec<MaybeValidOrderItem>>,
        option_loc: SourceLoc,
        order_loc: Option<SourceLoc>,
    ) -> InvalidUnitsOption {
        InvalidUnitsOption::Order {
            order,
            loc: order_loc.and_then(|u| Some(option_loc.start.up_to(u.end))),
            locs: Some(OrderOptionLocs {
                option: option_loc,
                order: order_loc,
            }),
        }
    }
    pub fn primary_distance_unit(
        unit: Option<String>,
        option_loc: SourceLoc,
        unit_loc: Option<SourceLoc>,
    ) -> InvalidUnitsOption {
        InvalidUnitsOption::PrimaryDistanceUnit {
            unit,
            loc: unit_loc.and_then(|u| Some(option_loc.start.up_to(u.end))),
            locs: Some(UnitOptionLocs {
                option: option_loc,
                unit: unit_loc,
            }),
        }
    }
    pub fn secondary_distance_unit(
        unit: Option<String>,
        option_loc: SourceLoc,
        unit_loc: Option<SourceLoc>,
    ) -> InvalidUnitsOption {
        InvalidUnitsOption::SecondaryDistanceUnit {
            unit,
            loc: unit_loc.and_then(|u| Some(option_loc.start.up_to(u.end))),
            locs: Some(UnitOptionLocs {
                option: option_loc,
                unit: unit_loc,
            }),
        }
    }
    pub fn frontsight_azimuth_unit(
        unit: Option<String>,
        option_loc: SourceLoc,
        unit_loc: Option<SourceLoc>,
    ) -> InvalidUnitsOption {
        InvalidUnitsOption::FrontsightAzimuthUnit {
            unit,
            loc: unit_loc.and_then(|u| Some(option_loc.start.up_to(u.end))),
            locs: Some(UnitOptionLocs {
                option: option_loc,
                unit: unit_loc,
            }),
        }
    }
    pub fn backsight_azimuth_unit(
        unit: Option<String>,
        option_loc: SourceLoc,
        unit_loc: Option<SourceLoc>,
    ) -> InvalidUnitsOption {
        InvalidUnitsOption::BacksightAzimuthUnit {
            unit,
            loc: unit_loc.and_then(|u| Some(option_loc.start.up_to(u.end))),
            locs: Some(UnitOptionLocs {
                option: option_loc,
                unit: unit_loc,
            }),
        }
    }
    pub fn frontsight_inclination_unit(
        unit: Option<String>,
        option_loc: SourceLoc,
        unit_loc: Option<SourceLoc>,
    ) -> InvalidUnitsOption {
        InvalidUnitsOption::FrontsightInclinationUnit {
            unit,
            loc: unit_loc.and_then(|u| Some(option_loc.start.up_to(u.end))),
            locs: Some(UnitOptionLocs {
                option: option_loc,
                unit: unit_loc,
            }),
        }
    }
    pub fn backsight_inclination_unit(
        unit: Option<String>,
        option_loc: SourceLoc,
        unit_loc: Option<SourceLoc>,
    ) -> InvalidUnitsOption {
        InvalidUnitsOption::BacksightInclinationUnit {
            unit,
            loc: unit_loc.and_then(|u| Some(option_loc.start.up_to(u.end))),
            locs: Some(UnitOptionLocs {
                option: option_loc,
                unit: unit_loc,
            }),
        }
    }
    pub fn primary_distance_correction(
        correction: Option<String>,
        option_loc: SourceLoc,
        correction_loc: Option<SourceLoc>,
    ) -> InvalidUnitsOption {
        InvalidUnitsOption::PrimaryDistanceCorrection {
            correction,
            loc: correction_loc.and_then(|u| Some(option_loc.start.up_to(u.end))),
            locs: Some(CorrectionOptionLocs {
                option: option_loc,
                correction: correction_loc,
            }),
        }
    }
    pub fn secondary_distance_correction(
        correction: Option<String>,
        option_loc: SourceLoc,
        correction_loc: Option<SourceLoc>,
    ) -> InvalidUnitsOption {
        InvalidUnitsOption::SecondaryDistanceCorrection {
            correction,
            loc: correction_loc.and_then(|u| Some(option_loc.start.up_to(u.end))),
            locs: Some(CorrectionOptionLocs {
                option: option_loc,
                correction: correction_loc,
            }),
        }
    }
    pub fn height_adjustment(
        correction: Option<String>,
        option_loc: SourceLoc,
        correction_loc: Option<SourceLoc>,
    ) -> InvalidUnitsOption {
        InvalidUnitsOption::HeightAdjustment {
            correction,
            loc: correction_loc.and_then(|u| Some(option_loc.start.up_to(u.end))),
            locs: Some(CorrectionOptionLocs {
                option: option_loc,
                correction: correction_loc,
            }),
        }
    }
    pub fn frontsight_azimuth_correction(
        correction: Option<String>,
        option_loc: SourceLoc,
        correction_loc: Option<SourceLoc>,
    ) -> InvalidUnitsOption {
        InvalidUnitsOption::FrontsightAzimuthCorrection {
            correction,
            loc: correction_loc.and_then(|u| Some(option_loc.start.up_to(u.end))),
            locs: Some(CorrectionOptionLocs {
                option: option_loc,
                correction: correction_loc,
            }),
        }
    }
    pub fn backsight_azimuth_correction(
        correction: Option<String>,
        option_loc: SourceLoc,
        correction_loc: Option<SourceLoc>,
    ) -> InvalidUnitsOption {
        InvalidUnitsOption::BacksightAzimuthCorrection {
            correction,
            loc: correction_loc.and_then(|u| Some(option_loc.start.up_to(u.end))),
            locs: Some(CorrectionOptionLocs {
                option: option_loc,
                correction: correction_loc,
            }),
        }
    }
    pub fn frontsight_inclination_correction(
        correction: Option<String>,
        option_loc: SourceLoc,
        correction_loc: Option<SourceLoc>,
    ) -> InvalidUnitsOption {
        InvalidUnitsOption::FrontsightInclinationCorrection {
            correction,
            loc: correction_loc.and_then(|u| Some(option_loc.start.up_to(u.end))),
            locs: Some(CorrectionOptionLocs {
                option: option_loc,
                correction: correction_loc,
            }),
        }
    }
    pub fn backsight_inclination_correction(
        correction: Option<String>,
        option_loc: SourceLoc,
        correction_loc: Option<SourceLoc>,
    ) -> InvalidUnitsOption {
        InvalidUnitsOption::BacksightInclinationCorrection {
            correction,
            loc: correction_loc.and_then(|u| Some(option_loc.start.up_to(u.end))),
            locs: Some(CorrectionOptionLocs {
                option: option_loc,
                correction: correction_loc,
            }),
        }
    }
    pub fn magnetic_declination(
        declination: Option<String>,
        option_loc: SourceLoc,
        declination_loc: Option<SourceLoc>,
    ) -> InvalidUnitsOption {
        InvalidUnitsOption::MagneticDeclination {
            declination,
            loc: declination_loc.and_then(|u| Some(option_loc.start.up_to(u.end))),
            locs: Some(DeclinationOptionLocs {
                option: option_loc,
                declination: declination_loc,
            }),
        }
    }
    pub fn grid_north_correction(
        correction: Option<String>,
        option_loc: SourceLoc,
        correction_loc: Option<SourceLoc>,
    ) -> InvalidUnitsOption {
        InvalidUnitsOption::GridNorthCorrection {
            correction,
            loc: correction_loc.and_then(|u| Some(option_loc.start.up_to(u.end))),
            locs: Some(CorrectionOptionLocs {
                option: option_loc,
                correction: correction_loc,
            }),
        }
    }
    pub fn rect_north_correction(
        correction: Option<String>,
        option_loc: SourceLoc,
        correction_loc: Option<SourceLoc>,
    ) -> InvalidUnitsOption {
        InvalidUnitsOption::RectilinearNorthCorrection {
            correction,
            loc: correction_loc.and_then(|u| Some(option_loc.start.up_to(u.end))),
            locs: Some(CorrectionOptionLocs {
                option: option_loc,
                correction: correction_loc,
            }),
        }
    }
    pub fn station_name_case(
        conversion: Option<String>,
        option_loc: SourceLoc,
        conversion_loc: Option<SourceLoc>,
    ) -> InvalidUnitsOption {
        InvalidUnitsOption::StationNameCase {
            conversion,
            loc: conversion_loc.and_then(|u| Some(option_loc.start.up_to(u.end))),
            locs: Some(StationNameCaseOptionLocs {
                option: option_loc,
                conversion: conversion_loc,
            }),
        }
    }
    pub fn taping_method(
        method: Option<String>,
        option_loc: SourceLoc,
        method_loc: Option<SourceLoc>,
    ) -> InvalidUnitsOption {
        InvalidUnitsOption::TapingMethod {
            method,
            loc: method_loc.and_then(|u| Some(option_loc.start.up_to(u.end))),
            locs: Some(TapingMethodOptionLocs {
                option: option_loc,
                method: method_loc,
            }),
        }
    }
    pub fn horizontal_unit_variance(
        variance: Option<String>,
        option_loc: SourceLoc,
        variance_loc: Option<SourceLoc>,
    ) -> InvalidUnitsOption {
        InvalidUnitsOption::HorizontalUnitVariance {
            variance,
            loc: variance_loc.and_then(|u| Some(option_loc.start.up_to(u.end))),
            locs: Some(VarianceOptionLocs {
                option: option_loc,
                variance: variance_loc,
            }),
        }
    }
    pub fn vertical_unit_variance(
        variance: Option<String>,
        option_loc: SourceLoc,
        variance_loc: Option<SourceLoc>,
    ) -> InvalidUnitsOption {
        InvalidUnitsOption::VerticalUnitVariance {
            variance,
            loc: variance_loc.and_then(|u| Some(option_loc.start.up_to(u.end))),
            locs: Some(VarianceOptionLocs {
                option: option_loc,
                variance: variance_loc,
            }),
        }
    }
    pub fn unit_variance(
        variance: Option<String>,
        option_loc: SourceLoc,
        variance_loc: Option<SourceLoc>,
    ) -> InvalidUnitsOption {
        InvalidUnitsOption::UnitVariance {
            variance,
            loc: variance_loc.and_then(|u| Some(option_loc.start.up_to(u.end))),
            locs: Some(VarianceOptionLocs {
                option: option_loc,
                variance: variance_loc,
            }),
        }
    }
    pub fn with_issue(self, issue: usize) -> MaybeValidUnitsOption {
        self.with_issues(vec![issue])
    }
    pub fn with_issues(self, issues: Vec<usize>) -> MaybeValidUnitsOption {
        MaybeValidUnitsOption::Invalid {
            invalid: self,
            issues: Some(issues),
        }
    }
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[serde(untagged)]
#[schemars(deny_unknown_fields)]
pub enum MaybeValidUnitsOption {
    Valid(UnitsOption),
    Invalid {
        #[serde(rename = "INVALID")]
        invalid: InvalidUnitsOption,
        issues: Option<Vec<usize>>,
    },
}

impl From<UnitsOption> for MaybeValidUnitsOption {
    fn from(option: UnitsOption) -> MaybeValidUnitsOption {
        MaybeValidUnitsOption::Valid(option)
    }
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub enum FixLocation {
    LatLong {
        latitude: Angle,
        longitude: Angle,
        elevation: Length,
        loc: Option<SourceLoc>,
    },
    Grid {
        easting: Length,
        northing: Length,
        elevation: Length,
        loc: Option<SourceLoc>,
    },
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub enum InvalidFixLocation {
    LatLong {
        latitude: Option<MaybeValidAngle>,
        longitude: Option<MaybeValidAngle>,
        elevation: Option<MaybeValidLength>,
        loc: Option<SourceLoc>,
    },
    Grid {
        easting: Option<MaybeValidLength>,
        northing: Option<MaybeValidLength>,
        elevation: Option<MaybeValidLength>,
        loc: Option<SourceLoc>,
    },
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[serde(untagged)]
#[schemars(deny_unknown_fields)]
pub enum MaybeValidFixLocation {
    Valid(FixLocation),
    Invalid {
        #[serde(rename = "INVALID")]
        invalid: InvalidFixLocation,
        issues: Option<Vec<usize>>,
    },
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[serde(tag = "type")]
#[schemars(deny_unknown_fields)]
pub enum VarianceAssignment {
    #[schemars(title = "LengthVarianceAssignment")]
    Length { length: Length },
    #[schemars(title = "RMSErrorVarianceAssignment")]
    RMSError { length: Length },
    #[schemars(title = "FloatShotVarianceAssignment")]
    FloatShot,
    #[schemars(title = "FloatTraverseVarianceAssignment")]
    FloatTraverse,
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[serde(tag = "type")]
#[schemars(deny_unknown_fields)]
pub enum InvalidVarianceAssignment {
    #[schemars(title = "InvalidLengthVarianceAssignment")]
    Length { length: String },
    #[schemars(title = "InvalidVarianceAssignment")]
    RMSError { length: String },
    #[schemars(title = "UnknownVarianceAssignment")]
    Unknown { value: String },
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[serde(untagged)]
#[schemars(deny_unknown_fields)]
pub enum MaybeValidVarianceAssignment {
    Valid(VarianceAssignment),
    Invalid {
        #[serde(rename = "INVALID")]
        invalid: InvalidVarianceAssignment,
        issues: Option<Vec<usize>>,
    },
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub enum SymbolOpacity {
    Solid,
    Opaque,
    Clear,
    Transparent,
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
#[serde(untagged)]
pub enum MaybeValidSymbolOpacity {
    Valid(SymbolOpacity),
    Invalid {
        #[serde(rename = "INVALID")]
        invalid: String,
        issues: Option<Vec<usize>>,
    },
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub enum SymbolShape {
    Square,
    Circle,
    Triangle,
    PlusSign,
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
#[serde(untagged)]
pub enum MaybeValidSymbolShape {
    Valid(SymbolShape),
    Invalid {
        #[serde(rename = "INVALID")]
        invalid: String,
        issues: Option<Vec<usize>>,
    },
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub loc: Option<SourceLoc>,
    pub locs: Option<ColorLocs>,
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct ColorLocs {
    pub red: Option<SourceLoc>,
    pub green: Option<SourceLoc>,
    pub blue: Option<SourceLoc>,
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
#[serde(untagged)]
pub enum MaybeValidU8 {
    Valid(u8),
    Invalid {
        #[serde(rename = "INVALID")]
        invalid: String,
        issues: Option<Vec<usize>>,
    },
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
#[serde(untagged)]
pub enum MaybeValidU32 {
    Valid(u32),
    Invalid {
        #[serde(rename = "INVALID")]
        invalid: String,
        issues: Option<Vec<usize>>,
    },
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct InvalidColor {
    pub red: Option<MaybeValidU8>,
    pub green: Option<MaybeValidU8>,
    pub blue: Option<MaybeValidU8>,
    pub loc: Option<SourceLoc>,
    pub locs: Option<ColorLocs>,
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
#[serde(untagged)]
pub enum MaybeValidColor {
    Valid(Color),
    Invalid {
        #[serde(rename = "INVALID")]
        invalid: InvalidColor,
        issues: Option<Vec<usize>>,
    },
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
#[serde(tag = "type")]
pub enum ShotMeasurements {
    CompassAndTape {
        distance: Length,
        frontsight_azimuth: Option<Angle>,
        backsight_azimuth: Option<Angle>,
        frontsight_inclination: Option<Inclination>,
        backsight_inclination: Option<Inclination>,
        instrument_height: Option<Length>,
        target_height: Option<Length>,
        locs: Option<CompassAndTapeLocs>,
    },
    Rectilinear {
        easting: Length,
        northing: Length,
        elevation: Option<Length>,
        locs: Option<RectilinearLocs>,
    },
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct CompassAndTapeLocs {
    pub distance: Option<SourceLoc>,
    pub frontsight_azimuth: Option<SourceLoc>,
    pub backsight_azimuth: Option<SourceLoc>,
    pub frontsight_inclination: Option<SourceLoc>,
    pub backsight_inclination: Option<SourceLoc>,
    pub instrument_height: Option<SourceLoc>,
    pub target_height: Option<SourceLoc>,
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct RectilinearLocs {
    pub easting: Option<SourceLoc>,
    pub northing: Option<SourceLoc>,
    pub elevation: Option<SourceLoc>,
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
#[serde(tag = "type")]
pub enum InvalidShotMeasurements {
    CompassAndTape {
        distance: MaybeValidLength,
        frontsight_azimuth: Option<MaybeValidAngle>,
        backsight_azimuth: Option<MaybeValidAngle>,
        frontsight_inclination: Option<MaybeValidInclination>,
        backsight_inclination: Option<MaybeValidInclination>,
        instrument_height: Option<MaybeValidLength>,
        target_height: Option<MaybeValidLength>,
        locs: Option<CompassAndTapeLocs>,
    },
    Rectilinear {
        easting: MaybeValidLength,
        northing: MaybeValidLength,
        elevation: Option<MaybeValidLength>,
        locs: Option<RectilinearLocs>,
    },
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[serde(untagged)]
#[schemars(deny_unknown_fields)]
pub enum MaybeValidShotMeasurements {
    Valid(ShotMeasurements),
    Invalid {
        #[serde(rename = "INVALID")]
        invalid: InvalidShotMeasurements,
        issues: Option<Vec<usize>>,
    },
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields, _unstable_ref_variants)]
#[serde(tag = "type")]
pub enum SrvItem {
    #[schemars(title = "UnitsDirective")]
    UnitsDirective {
        options: Vec<UnitsOption>,
        comment: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<UnitsDirectiveLocs>,
    },
    #[schemars(title = "SegmentDirective")]
    SegmentDirective {
        segment: String,
        comment: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<SymbolDirectiveLocs>,
    },
    #[schemars(title = "FixDirective")]
    FixDirective {
        station: String,
        location: FixLocation,
        horizontal_variance: Option<VarianceAssignment>,
        vertical_variance: Option<VarianceAssignment>,
        note: Option<String>,
        segment: Option<String>,
        comment: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<FixDirectiveLocs>,
    },
    #[schemars(title = "PrefixDirective")]
    PrefixDirective {
        level: PrefixLevel,
        prefix: Option<String>,
        comment: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<PrefixDirectiveLocs>,
    },
    #[schemars(title = "NoteDirective")]
    NoteDirective {
        station: String,
        note: String,
        comment: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<NoteDirectiveLocs>,
    },
    #[schemars(title = "FlagDirective")]
    FlagDirective {
        stations: Vec<String>,
        flag: String,
        comment: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<FlagDirectiveLocs>,
    },
    #[schemars(title = "SymbolDirective")]
    SymbolDirective {
        opacity: Option<SymbolOpacity>,
        shape: Option<SymbolShape>,
        point_size: Option<u32>,
        color: Option<Color>,
        flag: Option<String>,
        comment: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<SymbolDirectiveLocs>,
    },
    #[schemars(title = "DateDirective")]
    DateDirective {
        year: u32,
        month: u8,
        day: u8,
        comment: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<DateDirectiveLocs>,
    },
    #[schemars(title = "Shot")]
    Shot {
        from: Option<String>,
        to: Option<String>,
        measurements: Option<ShotMeasurements>,
        horizontal_variance: Option<VarianceAssignment>,
        vertical_variance: Option<VarianceAssignment>,
        left: Option<Length>,
        right: Option<Length>,
        up: Option<Length>,
        down: Option<Length>,
        lrud_facing_azimuth: Option<Angle>,
        left_azimuth: Option<Angle>,
        right_azimuth: Option<Angle>,
        c_flag: bool,
        segment: Option<String>,
        comment: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<ShotLocs>,
    },
    #[schemars(title = "Comment")]
    Comment {
        comment: String,
        inline: bool,
        loc: Option<SourceLoc>,
    },
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct UnitsDirectiveLocs {
    pub directive: SourceLoc,
    pub comment: Option<SourceLoc>,
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct PrefixDirectiveLocs {
    pub directive: SourceLoc,
    pub prefix: Option<SourceLoc>,
    pub comment: Option<SourceLoc>,
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct NoteDirectiveLocs {
    pub directive: SourceLoc,
    pub station: Option<SourceLoc>,
    pub note: Option<SourceLoc>,
    pub comment: Option<SourceLoc>,
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct DateDirectiveLocs {
    pub directive: SourceLoc,
    pub year: Option<SourceLoc>,
    pub month: Option<SourceLoc>,
    pub day: Option<SourceLoc>,
    pub comment: Option<SourceLoc>,
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct FixDirectiveLocs {
    pub directive: SourceLoc,
    pub station: Option<SourceLoc>,
    pub location: Option<SourceLoc>,
    pub horizontal_variance: Option<SourceLoc>,
    pub vertical_variance: Option<SourceLoc>,
    pub note: Option<SourceLoc>,
    pub segment: Option<SourceLoc>,
    pub comment: Option<SourceLoc>,
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct FlagDirectiveLocs {
    pub directive: SourceLoc,
    pub stations: Option<Vec<SourceLoc>>,
    pub flag: Option<SourceLoc>,
    pub comment: Option<SourceLoc>,
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct SymbolDirectiveLocs {
    pub directive: SourceLoc,
    pub opacity: Option<SourceLoc>,
    pub shape: Option<SourceLoc>,
    pub point_size: Option<SourceLoc>,
    pub color: Option<SourceLoc>,
    pub flag: Option<SourceLoc>,
    pub comment: Option<SourceLoc>,
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct ShotLocs {
    pub from: Option<SourceLoc>,
    pub to: Option<SourceLoc>,
    pub measurements: Option<SourceLoc>,
    pub horizontal_variance: Option<SourceLoc>,
    pub vertical_variance: Option<SourceLoc>,
    pub left: Option<SourceLoc>,
    pub right: Option<SourceLoc>,
    pub up: Option<SourceLoc>,
    pub down: Option<SourceLoc>,
    pub lrud_facing_azimuth: Option<SourceLoc>,
    pub left_azimuth: Option<SourceLoc>,
    pub right_azimuth: Option<SourceLoc>,
    pub c_flag: Option<SourceLoc>,
    pub segment: Option<SourceLoc>,
    pub comment: Option<SourceLoc>,
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
#[serde(tag = "type")]
pub enum InvalidSrvItem {
    #[schemars(title = "InvalidUnitsDirective")]
    UnitsDirective {
        options: Vec<MaybeValidUnitsOption>,
        comment: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<UnitsDirectiveLocs>,
    },
    #[schemars(title = "InvalidFixDirective")]
    FixDirective {
        station: Option<String>,
        location: Option<MaybeValidFixLocation>,
        horizontal_variance: Option<MaybeValidVarianceAssignment>,
        vertical_variance: Option<MaybeValidVarianceAssignment>,
        note: Option<String>,
        segment: Option<String>,
        comment: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<FixDirectiveLocs>,
    },
    #[schemars(title = "InvalidFlagDirective")]
    FlagDirective {
        stations: Vec<String>,
        flag: Option<String>,
        comment: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<FlagDirectiveLocs>,
    },
    #[schemars(title = "InvalidSymbolDirective")]
    SymbolDirective {
        opacity: Option<MaybeValidSymbolOpacity>,
        shape: Option<MaybeValidSymbolShape>,
        point_size: Option<MaybeValidU32>,
        color: Option<MaybeValidColor>,
        flag: Option<String>,
        comment: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<SymbolDirectiveLocs>,
    },
    #[schemars(title = "InvalidDateDirective")]
    DateDirective {
        year: Option<MaybeValidU32>,
        month: Option<MaybeValidU8>,
        day: Option<MaybeValidU8>,
        comment: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<DateDirectiveLocs>,
    },
    #[schemars(title = "InvalidShot")]
    Shot {
        from: Option<String>,
        to: Option<String>,
        measurements: Option<MaybeValidShotMeasurements>,
        horizontal_variance: Option<MaybeValidVarianceAssignment>,
        vertical_variance: Option<MaybeValidVarianceAssignment>,
        left: Option<MaybeValidLength>,
        right: Option<MaybeValidLength>,
        up: Option<MaybeValidLength>,
        down: Option<MaybeValidLength>,
        lrud_facing_azimuth: Option<MaybeValidAngle>,
        left_azimuth: Option<MaybeValidAngle>,
        right_azimuth: Option<MaybeValidAngle>,
        c_flag: bool,
        segment: Option<String>,
        comment: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<ShotLocs>,
    },
    #[schemars(title = "UnknownItem")]
    Unknown {
        text: String,
        loc: Option<SourceLoc>,
    },
}

impl InvalidSrvItem {
    pub fn with_issue(self, issue: usize) -> MaybeValidSrvItem {
        self.with_issues(vec![issue])
    }
    pub fn with_issues(self, issues: Vec<usize>) -> MaybeValidSrvItem {
        MaybeValidSrvItem::Invalid {
            invalid: self,
            issues: (!issues.is_empty()).then_some(issues),
        }
    }
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[serde(untagged)]
#[schemars(deny_unknown_fields)]
pub enum MaybeValidSrvItem {
    Valid(SrvItem),
    Invalid {
        #[serde(rename = "INVALID")]
        invalid: InvalidSrvItem,
        issues: Option<Vec<usize>>,
    },
}

impl From<SrvItem> for MaybeValidSrvItem {
    fn from(value: SrvItem) -> Self {
        MaybeValidSrvItem::Valid(value)
    }
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct WallsSrvFile {
    pub items: Vec<SrvItem>,
    pub issues: Option<Vec<ParseIssue>>,
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct InvalidWallsSrvFile {
    pub items: Vec<MaybeValidSrvItem>,
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
#[serde(untagged)]
pub enum MaybeValidWallsSrvFile {
    Valid(WallsSrvFile),
    Invalid {
        #[serde(rename = "INVALID")]
        invalid: InvalidWallsSrvFile,
        issues: Vec<ParseIssue>,
    },
}

impl From<WallsSrvFile> for MaybeValidWallsSrvFile {
    fn from(value: WallsSrvFile) -> Self {
        MaybeValidWallsSrvFile::Valid(value)
    }
}

pub const EINVALIDDIRECTIVE: &str = "EINVALIDDIRECTIVE";
pub const EUNEXPECTED: &str = "EUNEXPECTED";
pub const EINVALIDOPTION: &str = "EINVALIDOPTION";
pub const EMISSINGVALUE: &str = "EMISSINGVALUE";
pub const EINVALIDOPTIONVALUE: &str = "EINVALIDOPTIONVALUE";
pub const EINVALIDMEASUREMENTORDER: &str = "EINVALIDMEASUREMENTORDER";
pub const EINVALIDORDERITEM: &str = "EINVALIDORDERITEM";
pub const EINVALIDLENGTH: &str = "EINVALIDLENGTH";
pub const EINVALIDLENGTHUNIT: &str = "EINVALIDLENGTHUNIT";
pub const EINVALIDANGLE: &str = "EINVALIDANGLEUNIT";
pub const EINVALIDANGLEUNIT: &str = "EINVALIDANGLEUNIT";
pub const EINVALIDAZIMUTH: &str = "EINVALIDAZIMUTH";
pub const EAZIMUTHOUTOFRANGE: &str = "EAZIMUTHOUTOFRANGE";
pub const EINVALIDAZIMUTHUNIT: &str = "EINVALIDAZIMUTHUNIT";
pub const EINVALIDINCLINATION: &str = "EINVALIDINCLINATION";
pub const EINCLINATIONOUTOFRANGE: &str = "EINCLINATIONOUTOFRANGE";
pub const EINVALIDINCLINATIONUNIT: &str = "EINVALIDINCLINATIONUNIT";
pub const EINVALIDCASECONVERSION: &str = "EINVALIDCASECONVERSION";
pub const EINVALIDTAPINGMETHOD: &str = "EINVALIDTAPINGMETHOD";
pub const EINVALIDUNITVARIANCE: &str = "EINVALIDUNITVARIANCE";
pub const EMISSINGINCHES: &str = "EMISSINGINCHES";
pub const EMISSINGWHITESPACE: &str = "EMISSINGWHITESPACE";
