use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::types::{ParseIssue, SourceLoc};

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct InvalidValue {
    #[serde(rename = "INVALID")]
    pub invalid: String,
    pub issues: Option<Vec<usize>>,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[schemars(deny_unknown_fields)]
pub enum ShotType {
    CompassAndTape,
    Rectilinear,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[schemars(deny_unknown_fields)]
pub enum LengthUnit {
    Meters,
    Feet,
    Inches,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug, Clone)]
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

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
#[serde(untagged)]
pub enum MaybeValidLength {
    Valid(Length),
    Invalid(InvalidValue),
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[schemars(deny_unknown_fields)]
pub enum AngleUnit {
    Degrees,
    Mils,
    Grads,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug, Clone)]
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

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
#[serde(untagged)]
pub enum MaybeValidAngle {
    Valid(Angle),
    Invalid(InvalidValue),
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[schemars(deny_unknown_fields)]
pub enum InclinationUnit {
    Degrees,
    Mils,
    Grads,
    Percent,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug, Clone)]
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

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
#[serde(untagged)]
pub enum MaybeValidInclination {
    Valid(Inclination),
    Invalid(InvalidValue),
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[schemars(deny_unknown_fields)]
pub enum StationNameCaseConversion {
    Upper,
    Lower,
    Mixed,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[schemars(deny_unknown_fields)]
pub enum CompassAndTapeItem {
    Distance,
    Azimuth,
    Inclination,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
#[serde(untagged)]
pub enum MaybeValidCompassAndTapeItem {
    Valid(CompassAndTapeItem),
    Invalid(InvalidValue),
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[schemars(deny_unknown_fields)]
pub enum RectilinearItem {
    Easting,
    Northing,
    Elevation,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
#[serde(untagged)]
pub enum MaybeValidRectilinearItem {
    Valid(RectilinearItem),
    Invalid(InvalidValue),
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[schemars(deny_unknown_fields)]
pub enum LrudStyle {
    FromStationPerpendicular,
    ToStationPerpendicular,
    FromStationBisector,
    ToStationBisector,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[schemars(deny_unknown_fields)]
pub enum LrudItem {
    Left,
    Right,
    Up,
    Down,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
#[serde(untagged)]
pub enum MaybeValidLrudItem {
    Valid(LrudItem),
    Invalid(InvalidValue),
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[schemars(deny_unknown_fields)]
pub enum TapingMethod {
    InstrumentToTarget,
    StationToStation,
    InstrumentToStation,
    StationToTarget,
}

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
    pub distance_correction: Length,
    pub frontsight_azimuth_correction: Angle,
    pub backsight_azimuth_correction: Angle,
    pub frontsight_inclination_correction: Angle,
    pub backsight_inclination_correction: Angle,
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
            distance_correction: Length::meters(0.0),
            frontsight_azimuth_correction: Angle::degrees(0.0),
            backsight_azimuth_correction: Angle::degrees(0.0),
            frontsight_inclination_correction: Angle::degrees(0.0),
            backsight_inclination_correction: Angle::degrees(0.0),
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
}

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

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[schemars(deny_unknown_fields)]
pub struct BacksightOptionsLocs {
    pub is_corrected: Option<SourceLoc>,
    pub tolerance: Option<SourceLoc>,
    pub do_not_average: Option<SourceLoc>,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct InvalidBacksightOptions {
    pub is_corrected: bool,
    pub tolerance: Option<Angle>,
    pub do_not_average: bool,
    pub locs: Option<BacksightOptionsLocs>,
}

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

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[serde(tag = "option")]
#[schemars(deny_unknown_fields)]
pub enum UnitsOption {
    CompassAndTape {
        loc: Option<SourceLoc>,
    },
    Rectilinear {
        loc: Option<SourceLoc>,
    },
    CompassAndTapeOrder {
        order: Vec<CompassAndTapeItem>,
        loc: Option<SourceLoc>,
        locs: Option<OrderOptionLocs>,
    },
    RectilinearOrder {
        order: Vec<RectilinearItem>,
        loc: Option<SourceLoc>,
        locs: Option<OrderOptionLocs>,
    },
    FrontsightAzimuthUnit {
        unit: AngleUnit,
        loc: Option<SourceLoc>,
        locs: Option<UnitOptionLocs>,
    },
    BacksightAzimuthUnit {
        unit: AngleUnit,
        loc: Option<SourceLoc>,
        locs: Option<UnitOptionLocs>,
    },
    PrimaryDistanceUnit {
        unit: LengthUnit,
        loc: Option<SourceLoc>,
        locs: Option<UnitOptionLocs>,
    },
    SecondaryDistanceUnit {
        unit: LengthUnit,
        loc: Option<SourceLoc>,
        locs: Option<UnitOptionLocs>,
    },
    DistanceUnit {
        unit: LengthUnit,
        loc: Option<SourceLoc>,
        locs: Option<UnitOptionLocs>,
    },
    FrontsightInclinationUnit {
        unit: InclinationUnit,
        loc: Option<SourceLoc>,
        locs: Option<UnitOptionLocs>,
    },
    BacksightInclinationUnit {
        unit: InclinationUnit,
        loc: Option<SourceLoc>,
        locs: Option<UnitOptionLocs>,
    },
    MagneticDeclination {
        declination: Angle,
        loc: Option<SourceLoc>,
        locs: Option<DeclinationOptionLocs>,
    },
    GridNorthCorrection {
        correction: Angle,
        loc: Option<SourceLoc>,
        locs: Option<CorrectionOptionLocs>,
    },
    RectilinearNorthCorrection {
        correction: Angle,
        loc: Option<SourceLoc>,
        locs: Option<CorrectionOptionLocs>,
    },
    DistanceCorrection {
        correction: Length,
        loc: Option<SourceLoc>,
        locs: Option<CorrectionOptionLocs>,
    },
    FrontsightAzimuthCorrection {
        correction: Angle,
        loc: Option<SourceLoc>,
        locs: Option<CorrectionOptionLocs>,
    },
    BacksightAzimuthCorrection {
        correction: Angle,
        loc: Option<SourceLoc>,
        locs: Option<CorrectionOptionLocs>,
    },
    FrontsightInclinationCorrection {
        correction: Angle,
        loc: Option<SourceLoc>,
        locs: Option<CorrectionOptionLocs>,
    },
    BacksightInclinationCorrection {
        correction: Angle,
        loc: Option<SourceLoc>,
        locs: Option<CorrectionOptionLocs>,
    },
    HeightAdjustment {
        correction: Length,
        loc: Option<SourceLoc>,
        locs: Option<CorrectionOptionLocs>,
    },
    BacksightAzimuthType(BacksightOptions),
    BacksightInclinationType(BacksightOptions),
    Reset {
        loc: Option<SourceLoc>,
    },
    Save {
        loc: Option<SourceLoc>,
    },
    Restore {
        loc: Option<SourceLoc>,
    },
    StationNameCase {
        conversion: StationNameCaseConversion,
        loc: Option<SourceLoc>,
        locs: Option<StationNameCaseOptionLocs>,
    },
    LrudStyle {
        style: LrudStyle,
        loc: Option<SourceLoc>,
        locs: Option<LrudStyleOptionLocs>,
    },
    Prefix {
        level: PrefixLevel,
        prefix: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<PrefixOptionLocs>,
    },
    TapingMethod {
        method: TapingMethod,
        loc: Option<SourceLoc>,
        locs: Option<TapingMethodOptionLocs>,
    },
    UnitVariance {
        variance: f64,
        loc: Option<SourceLoc>,
        locs: Option<VarianceOptionLocs>,
    },
    HorizontalUnitVariance {
        variance: f64,
        loc: Option<SourceLoc>,
        locs: Option<VarianceOptionLocs>,
    },
    VerticalUnitVariance {
        variance: f64,
        loc: Option<SourceLoc>,
        locs: Option<VarianceOptionLocs>,
    },
    Flag {
        flag: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<FlagOptionLocs>,
    },
    Macro {
        name: String,
        value: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<MacroOptionLocs>,
    },
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct OrderOptionLocs {
    pub option: SourceLoc,
    pub order: Option<SourceLoc>,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct UnitOptionLocs {
    pub option: SourceLoc,
    pub unit: Option<SourceLoc>,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct CorrectionOptionLocs {
    pub option: SourceLoc,
    pub correction: Option<SourceLoc>,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct DeclinationOptionLocs {
    pub option: SourceLoc,
    pub declination: Option<SourceLoc>,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct StationNameCaseOptionLocs {
    pub option: SourceLoc,
    pub conversion: Option<SourceLoc>,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct LrudStyleOptionLocs {
    pub option: SourceLoc,
    pub style: Option<SourceLoc>,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct PrefixOptionLocs {
    pub option: SourceLoc,
    pub prefix: Option<SourceLoc>,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct TapingMethodOptionLocs {
    pub option: SourceLoc,
    pub method: Option<SourceLoc>,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct VarianceOptionLocs {
    pub option: SourceLoc,
    pub variance: Option<SourceLoc>,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct FlagOptionLocs {
    pub option: SourceLoc,
    pub flag: Option<SourceLoc>,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct MacroOptionLocs {
    pub option: SourceLoc,
    pub name: Option<SourceLoc>,
    pub value: Option<SourceLoc>,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
#[serde(tag = "option")]
pub enum InvalidUnitsOption {
    CompassAndTapeOrder {
        order: Vec<MaybeValidCompassAndTapeItem>,
        loc: Option<SourceLoc>,
        locs: Option<OrderOptionLocs>,
    },
    RectilinearOrder {
        order: Vec<MaybeValidRectilinearItem>,
        loc: Option<SourceLoc>,
        locs: Option<OrderOptionLocs>,
    },
    FrontsightAzimuthUnit {
        unit: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<UnitOptionLocs>,
    },
    BacksightAzimuthUnit {
        unit: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<UnitOptionLocs>,
    },
    PrimaryDistanceUnit {
        unit: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<UnitOptionLocs>,
    },
    SecondaryDistanceUnit {
        unit: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<UnitOptionLocs>,
    },
    FrontsightInclinationUnit {
        unit: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<UnitOptionLocs>,
    },
    BacksightInclinationUnit {
        unit: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<UnitOptionLocs>,
    },
    MagneticDeclination {
        declination: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<DeclinationOptionLocs>,
    },
    GridNorthCorrection {
        correction: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<CorrectionOptionLocs>,
    },
    RectilinearNorthCorrection {
        correction: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<CorrectionOptionLocs>,
    },
    DistanceCorrection {
        correction: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<CorrectionOptionLocs>,
    },
    FrontsightAzimuthCorrection {
        correction: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<CorrectionOptionLocs>,
    },
    BacksightAzimuthCorrection {
        correction: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<CorrectionOptionLocs>,
    },
    FrontsightInclinationCorrection {
        correction: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<CorrectionOptionLocs>,
    },
    BacksightInclinationCorrection {
        correction: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<CorrectionOptionLocs>,
    },
    HeightAdjustment {
        correction: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<CorrectionOptionLocs>,
    },
    BacksightAzimuthType(InvalidBacksightOptions),
    BacksightInclinationType(InvalidBacksightOptions),
    StationNameCase {
        conversion: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<StationNameCaseOptionLocs>,
    },
    LrudStyle {
        style: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<LrudStyleOptionLocs>,
    },
    TapingMethod {
        method: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<TapingMethodOptionLocs>,
    },
    UnitVariance {
        variance: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<VarianceOptionLocs>,
    },
    HorizontalUnitVariance {
        variance: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<VarianceOptionLocs>,
    },
    VerticalUnitVariance {
        variance: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<VarianceOptionLocs>,
    },
    Macro {
        name: Option<String>,
        value: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<MacroOptionLocs>,
    },
    Unknown {
        value: Option<String>,
        loc: Option<SourceLoc>,
    },
}

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

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub enum MaybeValidFixLocation {
    Valid(FixLocation),
    Invalid {
        #[serde(rename = "INVALID")]
        invalid: InvalidFixLocation,
        issues: Option<Vec<usize>>,
    },
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[serde(tag = "type")]
#[schemars(deny_unknown_fields)]
pub enum VarianceAssignment {
    Length { length: Length },
    RMSError { length: Length },
    FloatShot,
    FloatTraverse,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[serde(tag = "type")]
#[schemars(deny_unknown_fields)]
pub enum InvalidVarianceAssignment {
    Length { length: String },
    RMSError { length: String },
    Unknown { value: String },
}

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

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub enum SymbolOpacity {
    Solid,
    Opaque,
    Clear,
    Transparent,
}

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

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub enum SymbolShape {
    Square,
    Circle,
    Triangle,
    PlusSign,
}

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

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub loc: Option<SourceLoc>,
    pub locs: Option<ColorLocs>,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct ColorLocs {
    pub red: Option<SourceLoc>,
    pub green: Option<SourceLoc>,
    pub blue: Option<SourceLoc>,
}

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

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct InvalidColor {
    pub red: Option<MaybeValidU8>,
    pub green: Option<MaybeValidU8>,
    pub blue: Option<MaybeValidU8>,
    pub loc: Option<SourceLoc>,
    pub locs: Option<ColorLocs>,
}

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

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct RectilinearLocs {
    pub easting: Option<SourceLoc>,
    pub northing: Option<SourceLoc>,
    pub elevation: Option<SourceLoc>,
}

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

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct UnitsDirectiveLocs {
    pub directive: SourceLoc,
    pub comment: Option<SourceLoc>,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct PrefixDirectiveLocs {
    pub directive: SourceLoc,
    pub prefix: Option<SourceLoc>,
    pub comment: Option<SourceLoc>,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct NoteDirectiveLocs {
    pub directive: SourceLoc,
    pub station: Option<SourceLoc>,
    pub note: Option<SourceLoc>,
    pub comment: Option<SourceLoc>,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct DateDirectiveLocs {
    pub directive: SourceLoc,
    pub year: Option<SourceLoc>,
    pub month: Option<SourceLoc>,
    pub day: Option<SourceLoc>,
    pub comment: Option<SourceLoc>,
}

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

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct FlagDirectiveLocs {
    pub directive: SourceLoc,
    pub stations: Option<Vec<SourceLoc>>,
    pub flag: Option<SourceLoc>,
    pub comment: Option<SourceLoc>,
}

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
    pub fn with_issues(self, issues: Vec<usize>) -> MaybeValidSrvItem {
        MaybeValidSrvItem::Invalid {
            invalid: self,
            issues: (!issues.is_empty()).then_some(issues),
        }
    }
}

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

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct WallsSrvFile {
    pub items: Vec<SrvItem>,
    pub issues: Option<Vec<ParseIssue>>,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct InvalidWallsSrvFile {
    pub items: Vec<MaybeValidSrvItem>,
}

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
