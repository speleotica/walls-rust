use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::types::{ParseIssue, SourceLoc};

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct InvalidValue {
  #[serde(rename = "INVALID")]
  invalid: String,
  issues: Option<Vec<u32>>,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub enum ShotType {
  CompassAndTape,
  Rectilinear,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub enum LengthUnit {
  Meters,
  Feet,
  Inches,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct Length {
  value: f64,
  unit: LengthUnit,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
#[serde(untagged)]
pub enum MaybeValidLength {
  Valid(Length),
  Invalid(InvalidValue),
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub enum AngleUnit {
  Degrees,
  Mils,
  Grads,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct Angle {
  value: f64,
  unit: AngleUnit,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
#[serde(untagged)]
pub enum MaybeValidAngle {
  Valid(Angle),
  Invalid(InvalidValue),
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub enum InclinationUnit {
  Degrees,
  Mils,
  Grads,
  Percent,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct Inclination {
  value: f64,
  unit: InclinationUnit,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
#[serde(untagged)]
pub enum MaybeValidInclination {
  Valid(Inclination),
  Invalid(InvalidValue),
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub enum StationNameCaseConversion {
  Upper,
  Lower,
  Mixed,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
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

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
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

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub enum LrudStyle {
  FromStationPerpendicular,
  ToStationPerpendicular,
  FromStationBisector,
  ToStationBisector,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
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

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub enum TapingMethod {
  InstrumentToTarget,
  StationToStation,
  InstrumentToStation,
  StationToTarget,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct SrvSettings {
  shot_type: ShotType,
  compass_and_tape_order: Vec<CompassAndTapeItem>,
  rectilinear_order: Vec<RectilinearItem>,
  primary_distance_unit: LengthUnit,
  secondary_distance_unit: LengthUnit,
  frontsight_azimuth_unit: AngleUnit,
  backsight_azimuth_unit: AngleUnit,
  frontsight_inclination_unit: AngleUnit,
  backsight_inclination_unit: AngleUnit,
  magnetic_declination: Angle,
  grid_north_correction: Angle,
  rectilinear_north_correction: Angle,
  distance_correction: Length,
  frontsight_azimuth_correction: Angle,
  backsight_azimuth_correction: Angle,
  frontsight_inclination_correction: Angle,
  backsight_inclination_correction: Angle,
  height_adjustment: Length,
  backsight_azimuth_options: BacksightOptions,
  backsight_inclination_options: BacksightOptions,
  station_name_case_conversion: StationNameCaseConversion,
  lrud_style: LrudStyle,
  lrud_order: [LrudItem; 4],
  prefix: [String; 3],
  taping_method: TapingMethod,
  horizontal_unit_variance: f64,
  vertical_unit_variance: f64,
  flag: Option<String>,
  segment: Option<String>,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct BacksightOptions {
  is_corrected: bool,
  tolerance: Angle,
  do_not_average: bool,
  locs: Option<BacksightOptionsLocs>
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct BacksightOptionsLocs {
  is_corrected: Option<SourceLoc>,
  tolerance: Option<SourceLoc>,
  do_not_average: Option<SourceLoc>,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct InvalidBacksightOptions {
  is_corrected: bool,
  tolerance: Option<Angle>,
  do_not_average: bool,
  locs: Option<BacksightOptionsLocs>
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub enum MaybeValidBacksightOptions {
  Valid(BacksightOptions),
  Invalid { 
    #[serde(rename = "INVALID")]
    invalid: InvalidBacksightOptions,
    issues: Option<Vec<u32>>,
  },
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub enum PrefixLevel {
  Prefix1,
  Prefix2,
  Prefix3,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[serde(tag = "option")]
#[schemars(deny_unknown_fields)]
pub enum UnitsOption {
  CompassAndTape { loc: Option<SourceLoc> },
  Rectilinear { loc: Option<SourceLoc> },
  CompassAndTapeOrder { order: Vec<CompassAndTapeItem>, loc: Option<SourceLoc>, locs: Option<OrderOptionLocs> },
  RectilinearOrder { order: Vec<RectilinearItem>, loc: Option<SourceLoc>, locs: Option<OrderOptionLocs> },
  FrontsightAzimuthUnit { unit: AngleUnit, loc: Option<SourceLoc>, locs: Option<UnitOptionLocs> },
  BacksightAzimuthUnit { unit: AngleUnit, loc: Option<SourceLoc>, locs: Option<UnitOptionLocs> },
  PrimaryDistanceUnit { unit: LengthUnit, loc: Option<SourceLoc>, locs: Option<UnitOptionLocs> },
  SecondaryDistanceUnit { unit: LengthUnit, loc: Option<SourceLoc>, locs: Option<UnitOptionLocs> },
  DistanceUnit { unit: LengthUnit, loc: Option<SourceLoc>, locs: Option<UnitOptionLocs> },
  FrontsightInclinationUnit { unit: InclinationUnit, loc: Option<SourceLoc>, locs: Option<UnitOptionLocs> },
  BacksightInclinationUnit { unit: InclinationUnit, loc: Option<SourceLoc>, locs: Option<UnitOptionLocs> },
  MagneticDeclination { declination: Angle, loc: Option<SourceLoc>, locs: Option<DeclinationOptionLocs> },
  GridNorthCorrection { correction: Angle, loc: Option<SourceLoc>, locs: Option<CorrectionOptionLocs> },
  RectilinearNorthCorrection {correction: Angle, loc: Option<SourceLoc>, locs: Option<CorrectionOptionLocs> },
  DistanceCorrection { correction: Length, loc: Option<SourceLoc>, locs: Option<CorrectionOptionLocs> },
  FrontsightAzimuthCorrection { correction: Angle, loc: Option<SourceLoc>, locs: Option<CorrectionOptionLocs> },
  BacksightAzimuthCorrection { correction: Angle, loc: Option<SourceLoc>, locs: Option<CorrectionOptionLocs> },
  FrontsightInclinationCorrection { correction: Angle, loc: Option<SourceLoc>, locs: Option<CorrectionOptionLocs> },
  BacksightInclinationCorrection { correction: Angle, loc: Option<SourceLoc>, locs: Option<CorrectionOptionLocs> },
  HeightAdjustment { correction: Length, loc: Option<SourceLoc>, locs: Option<CorrectionOptionLocs> },
  BacksightAzimuthType(BacksightOptions),
  BacksightInclinationType(BacksightOptions),
  Reset { loc: Option<SourceLoc> },
  Save { loc: Option<SourceLoc> },
  Restore { loc: Option<SourceLoc> },
  StationNameCase { conversion: StationNameCaseConversion, loc: Option<SourceLoc>, locs: Option<StationNameCaseOptionLocs> },
  LrudStyle { style: LrudStyle, loc: Option<SourceLoc>, locs: Option<LrudStyleOptionLocs> },
  Prefix { level: PrefixLevel, prefix: Option<String>, loc: Option<SourceLoc>, locs: Option<PrefixOptionLocs> },
  TapingMethod { method: TapingMethod, loc: Option<SourceLoc>, locs: Option<TapingMethodOptionLocs> },
  UnitVariance { variance: f64, loc: Option<SourceLoc>, locs: Option<VarianceOptionLocs> },
  HorizontalUnitVariance { variance: f64, loc: Option<SourceLoc>, locs: Option<VarianceOptionLocs> },
  VerticalUnitVariance { variance: f64, loc: Option<SourceLoc>, locs: Option<VarianceOptionLocs> },
  Flag { flag: Option<String>, loc: Option<SourceLoc> , locs: Option<FlagOptionLocs> },
  Macro { name: String, value: Option<String>, loc: Option<SourceLoc>, locs: Option<MacroOptionLocs> },
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct OrderOptionLocs {
  option: SourceLoc,
  order: Option<SourceLoc>,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct UnitOptionLocs {
  option: SourceLoc,
  unit: Option<SourceLoc>,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct CorrectionOptionLocs {
  option: SourceLoc,
  correction: Option<SourceLoc>,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct DeclinationOptionLocs {
  option: SourceLoc,
  declination: Option<SourceLoc>,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct StationNameCaseOptionLocs {
  option: SourceLoc,
  conversion: Option<SourceLoc>,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct LrudStyleOptionLocs {
  option: SourceLoc,
  style: Option<SourceLoc>,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct PrefixOptionLocs {
  option: SourceLoc,
  prefix: Option<SourceLoc>,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct TapingMethodOptionLocs {
  option: SourceLoc,
  method: Option<SourceLoc>,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct VarianceOptionLocs {
  option: SourceLoc,
  variance: Option<SourceLoc>,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct FlagOptionLocs {
  option: SourceLoc,
  flag: Option<SourceLoc>,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct MacroOptionLocs {
  option: SourceLoc,
  name: Option<SourceLoc>,
  value: Option<SourceLoc>,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
#[serde(tag = "option")]
pub enum InvalidUnitsOption {
  CompassAndTapeOrder { order: Vec<MaybeValidCompassAndTapeItem>, loc: Option<SourceLoc>, locs: Option<OrderOptionLocs> },
  RectilinearOrder { order: Vec<MaybeValidRectilinearItem>, loc: Option<SourceLoc>, locs: Option<OrderOptionLocs> },
  FrontsightAzimuthUnit { unit: Option<String>, loc: Option<SourceLoc>, locs: Option<UnitOptionLocs> },
  BacksightAzimuthUnit { unit: Option<String>, loc: Option<SourceLoc>, locs: Option<UnitOptionLocs> },
  PrimaryDistanceUnit { unit: Option<String>, loc: Option<SourceLoc>, locs: Option<UnitOptionLocs> },
  SecondaryDistanceUnit { unit: Option<String>, loc: Option<SourceLoc>, locs: Option<UnitOptionLocs> },
  FrontsightInclinationUnit { unit: Option<String>, loc: Option<SourceLoc>, locs: Option<UnitOptionLocs> },
  BacksightInclinationUnit { unit: Option<String>, loc: Option<SourceLoc>, locs: Option<UnitOptionLocs> },
  MagneticDeclination { declination: Option<String>, loc: Option<SourceLoc>, locs: Option<DeclinationOptionLocs> },
  GridNorthCorrection { correction: Option<String>, loc: Option<SourceLoc>, locs: Option<CorrectionOptionLocs> },
  RectilinearNorthCorrection {correction: Option<String>, loc: Option<SourceLoc>, locs: Option<CorrectionOptionLocs> },
  DistanceCorrection { correction: Option<String>, loc: Option<SourceLoc>, locs: Option<CorrectionOptionLocs> },
  FrontsightAzimuthCorrection { correction: Option<String>, loc: Option<SourceLoc>, locs: Option<CorrectionOptionLocs> },
  BacksightAzimuthCorrection { correction: Option<String>, loc: Option<SourceLoc>, locs: Option<CorrectionOptionLocs> },
  FrontsightInclinationCorrection { correction: Option<String>, loc: Option<SourceLoc>, locs: Option<CorrectionOptionLocs> },
  BacksightInclinationCorrection { correction: Option<String>, loc: Option<SourceLoc>, locs: Option<CorrectionOptionLocs> },
  HeightAdjustment { correction: Option<String>, loc: Option<SourceLoc>, locs: Option<CorrectionOptionLocs> },
  BacksightAzimuthType(InvalidBacksightOptions),
  BacksightInclinationType(InvalidBacksightOptions),
  StationNameCase { conversion: Option<String>, loc: Option<SourceLoc>, locs: Option<StationNameCaseOptionLocs> },
  LrudStyle { style: Option<String>, loc: Option<SourceLoc>, locs: Option<LrudStyleOptionLocs> },
  TapingMethod { method: Option<String>, loc: Option<SourceLoc>, locs: Option<TapingMethodOptionLocs> },
  UnitVariance { variance: Option<String>, loc: Option<SourceLoc>, locs: Option<VarianceOptionLocs> },
  HorizontalUnitVariance { variance: Option<String>, loc: Option<SourceLoc>, locs: Option<VarianceOptionLocs> },
  VerticalUnitVariance { variance: Option<String>, loc: Option<SourceLoc>, locs: Option<VarianceOptionLocs> },
  Macro { name: Option<String>, value: Option<String>, loc: Option<SourceLoc>, locs: Option<MacroOptionLocs> },
  Unknown { value: Option<String>, loc: Option<SourceLoc> },
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[serde(untagged)]
#[schemars(deny_unknown_fields)]
pub enum MaybeValidUnitsOption {
  Valid(UnitsOption),
  Invalid { 
    #[serde(rename = "INVALID")]
    invalid: InvalidUnitsOption,
    issues: Option<Vec<u32>>,
  },
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub enum FixLocation {
  LatLong { latitude: Angle, longitude: Angle, elevation: Length, loc: Option<SourceLoc> },
  Grid { easting: Length, northing: Length, elevation: Length, loc: Option<SourceLoc> }
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub enum InvalidFixLocation {
  LatLong { latitude: Option<MaybeValidAngle>, longitude: Option<MaybeValidAngle>, elevation: Option<MaybeValidLength>, loc: Option<SourceLoc> },
  Grid { easting: Option<MaybeValidLength>, northing: Option<MaybeValidLength>, elevation: Option<MaybeValidLength>, loc: Option<SourceLoc> }
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub enum MaybeValidFixLocation {
  Valid(FixLocation),
  Invalid { 
    #[serde(rename = "INVALID")]
    invalid: InvalidFixLocation,
    issues: Option<Vec<u32>>,
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
    issues: Option<Vec<u32>>,
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
    issues: Option<Vec<u32>>,
  }
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
    issues: Option<Vec<u32>>,
  }
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct Color {
  red: u8,
  green: u8,
  blue: u8,
  loc: Option<SourceLoc>,
  locs: Option<ColorLocs>,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct ColorLocs {
  red: Option<SourceLoc>,
  green: Option<SourceLoc>,
  blue: Option<SourceLoc>,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
#[serde(untagged)]
pub enum MaybeValidU8 {
  Valid(u8),
  Invalid {
    #[serde(rename = "INVALID")]
    invalid: String,
    issues: Option<Vec<u32>>,
  }
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
#[serde(untagged)]
pub enum MaybeValidU32 {
  Valid(u32),
  Invalid {
    #[serde(rename = "INVALID")]
    invalid: String,
    issues: Option<Vec<u32>>,
  }
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct InvalidColor {
  red: Option<MaybeValidU8>,
  green: Option<MaybeValidU8>,
  blue: Option<MaybeValidU8>,
  loc: Option<SourceLoc>,
  locs: Option<ColorLocs>,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
#[serde(untagged)]
pub enum MaybeValidColor {
  Valid(Color),
  Invalid {
    #[serde(rename = "INVALID")]
    invalid: InvalidColor,
    issues: Option<Vec<u32>>,
  }
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
  distance: Option<SourceLoc>,
  frontsight_azimuth: Option<SourceLoc>,
  backsight_azimuth: Option<SourceLoc>,
  frontsight_inclination: Option<SourceLoc>,
  backsight_inclination: Option<SourceLoc>,
  instrument_height: Option<SourceLoc>,
  target_height: Option<SourceLoc>,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct RectilinearLocs {
  easting: Option<SourceLoc>,
  northing: Option<SourceLoc>,
  elevation: Option<SourceLoc>,
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
    issues: Option<Vec<u32>>,
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
  Comment { comment: String, inline: bool, loc: Option<SourceLoc> },
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct UnitsDirectiveLocs {
  directive: SourceLoc,
  comment: Option<SourceLoc>,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct PrefixDirectiveLocs {
  directive: SourceLoc,
  prefix: Option<SourceLoc>,
  comment: Option<SourceLoc>,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct NoteDirectiveLocs {
  directive: SourceLoc,
  station: Option<SourceLoc>,
  note: Option<SourceLoc>,
  comment: Option<SourceLoc>,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct DateDirectiveLocs {
  directive: SourceLoc,
  year: Option<SourceLoc>,
  month: Option<SourceLoc>,
  day: Option<SourceLoc>,
  comment: Option<SourceLoc>,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct FixDirectiveLocs {
  directive: SourceLoc,
  station: Option<SourceLoc>,
  location: Option<SourceLoc>,
  horizontal_variance: Option<SourceLoc>,
  vertical_variance: Option<SourceLoc>,
  note: Option<SourceLoc>,
  segment: Option<SourceLoc>,
  comment: Option<SourceLoc>,
}


#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct FlagDirectiveLocs {
  directive: SourceLoc,
  stations: Option<Vec<SourceLoc>>,
  flag: Option<SourceLoc>,
  comment: Option<SourceLoc>,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct SymbolDirectiveLocs {
  directive: SourceLoc,
  opacity: Option<SourceLoc>,
  shape: Option<SourceLoc>,
  point_size: Option<SourceLoc>,
  color: Option<SourceLoc>,
  flag: Option<SourceLoc>,
  comment: Option<SourceLoc>,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct ShotLocs {
  from: Option<SourceLoc>,
  to: Option<SourceLoc>,
  measurements: Option<SourceLoc>,
  horizontal_variance: Option<SourceLoc>,
  vertical_variance: Option<SourceLoc>,
  left: Option<SourceLoc>,
  right: Option<SourceLoc>,
  up: Option<SourceLoc>,
  down: Option<SourceLoc>,
  lrud_facing_azimuth: Option<SourceLoc>,
  left_azimuth: Option<SourceLoc>,
  right_azimuth: Option<SourceLoc>,
  c_flag: Option<SourceLoc>,
  segment: Option<SourceLoc>,
  comment: Option<SourceLoc>,
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
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[serde(untagged)]
#[schemars(deny_unknown_fields)]
pub enum MaybeValidSrvItem {
  Valid(SrvItem),
  Invalid { 
    #[serde(rename = "INVALID")]
    invalid: InvalidSrvItem,
    issues: Option<Vec<u32>>,
  },
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct WallsSrvFile {
  items: Vec<SrvItem>,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct InvalidWallsSrvFile {
  items: Vec<MaybeValidSrvItem>
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
  }
}
