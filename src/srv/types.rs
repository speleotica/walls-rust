use std::ops::{Add, Sub};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::skip_serializing_none;

use crate::types::{ParseIssue, SourceLoc};

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
pub enum SrvErrorCode {
    InvalidDirective,
    Unexpected,
    InvalidOption,
    MissingValue,
    InvalidOptionValue,
    InvalidMeasurementOrder,
    InvalidOrderItem,
    InvalidNumber,
    InvalidLength,
    InvalidLengthUnit,
    MissingInches,
    InvalidAngle,
    InvalidAngleUnit,
    AngleOutOfRange,
    MissingMinutes,
    MissingSeconds,
    DegreesOutOfRange,
    MinutesOutOfRange,
    SecondsOutOfRange,
    InvalidAzimuth,
    AzimuthOutOfRange,
    MissingEndingDir,
    InvalidEndingDir,
    InvalidAzimuthUnit,
    InvalidInclination,
    InclinationOutOfRange,
    InvalidInclinationUnit,
    InvalidCaseConversion,
    InvalidTapingMethod,
    InvalidUnitVariance,
    InvalidLrudStyle,
    MissingLrudStyle,
    InvalidLrudOrderItem,
    InvalidLrudOrder,
    MissingLrudOrder,
    MissingWhitespace,
    InvalidBacksightCorrected,
    MissingBacksightCorrected,
    MissingTolerance,
    InvalidTolerance,
    InvalidDoNotAverage,
    MissingDoNotAverage,
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
#[serde(untagged)]
pub enum MaybeValidBool {
    Valid(bool),
    Invalid(InvalidValue),
}

impl From<bool> for MaybeValidBool {
    fn from(value: bool) -> Self {
        MaybeValidBool::Valid(value)
    }
}

impl From<InvalidValue> for MaybeValidBool {
    fn from(value: InvalidValue) -> Self {
        MaybeValidBool::Invalid(value)
    }
}

impl From<Result<bool, InvalidValue>> for MaybeValidBool {
    fn from(value: Result<bool, InvalidValue>) -> Self {
        match value {
            Ok(value) => MaybeValidBool::Valid(value),
            Err(invalid) => MaybeValidBool::Invalid(invalid),
        }
    }
}

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
/// The type of shot vector: compass and tape (distance, azimuth, inclination)
/// or rectilinear (easting, northing, elevation)
pub enum ShotType {
    /// Indicates shot vectors are defined by distance, azimuth, and inclination
    /// measurements
    CompassAndTape,
    /// Indicates shot vectors are defined by easting, northing, and elevation
    /// measurements
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
    /// 1 revolution = 360 degrees
    Degrees,
    /// 1 revolution = 6400 mils
    Mils,
    /// 1 revolution = 400 grads
    Grads,
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
#[schemars(deny_unknown_fields)]
pub struct Angle {
    /// The magnitude of the angle, in units of `unit`.  For a
    /// degrees-minutes-seconds angle, this will be the converted decimal degrees
    /// magnitude.
    pub value: f64,
    /// The unit of the `value`
    pub unit: AngleUnit,
    /// The original degrees portion of a degrees-minutes-seconds angle,
    /// if given.  This will be present for literals like `3:05`, `3:05:40`,
    /// but omitted for literals like `:05`, `::40`.
    pub degrees: Option<f64>,
    /// The original minutes portion of a degrees-minutes-seconds angle,
    /// if given.  This will be present for literals like `:05, `3:05`, `3:05:40`,
    /// but omitted for literals like `3::40`, `::40`.
    pub minutes: Option<f64>,
    /// The original seconds portion of a degrees-minutes-seconds angle,
    /// if given.  This will be present for literals like `::40`, `:05:40`, `3:05:40`,
    /// but omitted for literals like `3:05`, `:05`.
    pub seconds: Option<f64>,
    pub locs: Option<AngleLocs>,
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
#[schemars(deny_unknown_fields)]
pub struct AngleLocs {
    /// The location of the degrees portion of a degrees-minutes-seconds angle,
    /// if given.
    pub degrees: Option<SourceLoc>,
    /// The location of the minutes portion of a degrees-minutes-seconds angle,
    /// if given.
    pub minutes: Option<SourceLoc>,
    /// The location of the seconds portion of a degrees-minutes-seconds angle,
    /// if given.
    pub seconds: Option<SourceLoc>,
}

impl Angle {
    pub fn new(value: f64, unit: AngleUnit) -> Angle {
        Angle {
            value,
            unit,
            degrees: None,
            minutes: None,
            seconds: None,
            locs: None,
        }
    }

    pub fn negate(&self) -> Angle {
        Angle {
            value: -self.value,
            unit: self.unit,
            degrees: self.degrees.map(|degrees| -degrees),
            minutes: self.degrees.map(|minutes| -minutes),
            seconds: self.degrees.map(|seconds| -seconds),
            locs: self.locs,
        }
    }

    pub fn dms(
        degrees: Option<f64>,
        minutes: Option<f64>,
        seconds: Option<f64>,
        locs: Option<AngleLocs>,
    ) -> Angle {
        Angle {
            value: degrees.unwrap_or(0.0)
                + (minutes.unwrap_or(0.0) + seconds.unwrap_or(0.0) / 60.0) / 60.0,
            unit: AngleUnit::Degrees,
            degrees,
            minutes,
            seconds,
            locs,
        }
    }

    pub fn degrees(value: f64) -> Angle {
        Angle::new(value, AngleUnit::Degrees)
    }
    pub fn mils(value: f64) -> Angle {
        Angle::new(value, AngleUnit::Mils)
    }
    pub fn grads(value: f64) -> Angle {
        Angle::new(value, AngleUnit::Grads)
    }
    pub fn to_unit(self, unit: AngleUnit) -> Angle {
        if unit == self.unit {
            self
        } else {
            let degrees = match self.unit {
                AngleUnit::Degrees => self.value,
                AngleUnit::Grads => self.value * 90.0 / 100.0,
                AngleUnit::Mils => self.value * 90.0 / 1600.0,
            };
            Angle::new(
                match unit {
                    AngleUnit::Degrees => degrees,
                    AngleUnit::Grads => degrees * 100.0 / 90.0,
                    AngleUnit::Mils => self.value * 1600.0 / 90.0,
                },
                unit,
            )
        }
    }
    pub fn normalize(self) -> Angle {
        Angle::new(
            self.value
                % match self.unit {
                    AngleUnit::Degrees => 360.0,
                    AngleUnit::Grads => 400.0,
                    AngleUnit::Mils => 6400.0,
                },
            self.unit,
        )
    }
}

impl Add for Angle {
    type Output = Angle;

    fn add(self, rhs: Self) -> Self::Output {
        if rhs.value == 0.0 {
            self
        } else {
            Angle::new(self.value + rhs.to_unit(self.unit).value, self.unit).normalize()
        }
    }
}

impl Sub for Angle {
    type Output = Angle;

    fn sub(self, rhs: Self) -> Self::Output {
        if rhs.value == 0.0 {
            self
        } else {
            Angle::new(self.value - rhs.to_unit(self.unit).value, self.unit).normalize()
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

impl From<Angle> for MaybeValidAngle {
    fn from(value: Angle) -> Self {
        MaybeValidAngle::Valid(value)
    }
}

impl From<InvalidValue> for MaybeValidAngle {
    fn from(value: InvalidValue) -> Self {
        MaybeValidAngle::Invalid(value)
    }
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
#[schemars(deny_unknown_fields)]
pub enum InclinationUnit {
    /// 1 revolution = 360 degrees
    Degrees,
    /// 1 revolution = 6400 mils
    Mils,
    /// 1 revolution = 400 grads
    Grads,
    /// Percent grade (rise / run * 100): 100% = 45 degrees
    Percent,
}

impl From<AngleUnit> for InclinationUnit {
    fn from(value: AngleUnit) -> Self {
        match value {
            AngleUnit::Degrees => InclinationUnit::Degrees,
            AngleUnit::Grads => InclinationUnit::Grads,
            AngleUnit::Mils => InclinationUnit::Mils,
        }
    }
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
#[schemars(deny_unknown_fields)]
pub struct Inclination {
    pub value: f64,
    pub unit: InclinationUnit,
    pub degrees: Option<f64>,
    pub minutes: Option<f64>,
    pub seconds: Option<f64>,
    pub locs: Option<AngleLocs>,
}

impl Inclination {
    pub fn new(value: f64, unit: InclinationUnit) -> Inclination {
        Inclination {
            value,
            unit,
            degrees: None,
            minutes: None,
            seconds: None,
            locs: None,
        }
    }

    pub fn negate(&self) -> Inclination {
        Inclination {
            value: -self.value,
            unit: self.unit,
            degrees: self.degrees.map(|degrees| -degrees),
            minutes: self.degrees.map(|minutes| -minutes),
            seconds: self.degrees.map(|seconds| -seconds),
            locs: self.locs,
        }
    }

    pub fn dms(
        degrees: f64,
        minutes: f64,
        seconds: Option<f64>,
        locs: Option<AngleLocs>,
    ) -> Inclination {
        Inclination {
            value: degrees + (minutes + seconds.unwrap_or(0.0) / 60.0) / 60.0,
            unit: InclinationUnit::Degrees,
            degrees: Some(degrees),
            minutes: Some(minutes),
            seconds,
            locs,
        }
    }

    pub fn degrees(value: f64) -> Inclination {
        Inclination::new(value, InclinationUnit::Degrees)
    }
    pub fn mils(value: f64) -> Inclination {
        Inclination::new(value, InclinationUnit::Mils)
    }
    pub fn grads(value: f64) -> Inclination {
        Inclination::new(value, InclinationUnit::Grads)
    }
    pub fn percent(value: f64) -> Inclination {
        Inclination::new(value, InclinationUnit::Percent)
    }
}

impl From<Angle> for Inclination {
    fn from(angle: Angle) -> Self {
        Inclination {
            value: angle.value,
            unit: angle.unit.into(),
            degrees: angle.degrees,
            minutes: angle.minutes,
            seconds: angle.seconds,
            locs: angle.locs,
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
    /// Convert station names to uppercase
    Upper,
    /// Convert station names to lowercase
    Lower,
    /// Leave station name case as is
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
    /// LRUDs of a shot are associated with the from station and perpendicular
    /// to the shot (unless a facing azimuth or left/right azimuths are specified)
    FromStationPerpendicular,
    /// LRUDs of a shot are associated with the to station and perpendicular
    /// to the shot (unless a facing azimuth or left/right azimuths are specified)
    ToStationPerpendicular,
    /// LRUDs of a shot are associated with the from station and bisect
    /// the angle between the shot and the preceeding shot
    /// (unless a facing azimuth or left/right azimuths are specified)
    FromStationBisector,
    /// LRUDs of a shot are associated with the to station and bisect
    /// the angle between the shot and the succeeding shot
    /// (unless a facing azimuth or left/right azimuths are specified)
    ToStationBisector,
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
#[serde(untagged)]
pub enum MaybeValidLrudStyle {
    Valid(LrudStyle),
    Invalid(InvalidValue),
}

impl From<Result<LrudStyle, InvalidValue>> for MaybeValidLrudStyle {
    fn from(value: Result<LrudStyle, InvalidValue>) -> Self {
        match value {
            Ok(valid) => valid.into(),
            Err(invalid) => invalid.into(),
        }
    }
}

impl Into<Result<LrudStyle, InvalidValue>> for MaybeValidLrudStyle {
    fn into(self) -> Result<LrudStyle, InvalidValue> {
        match self {
            MaybeValidLrudStyle::Valid(valid) => Ok(valid),
            MaybeValidLrudStyle::Invalid(invalid) => Err(invalid),
        }
    }
}

impl Into<MaybeValidLrudStyle> for LrudStyle {
    fn into(self) -> MaybeValidLrudStyle {
        MaybeValidLrudStyle::Valid(self)
    }
}

impl Into<MaybeValidLrudStyle> for InvalidValue {
    fn into(self) -> MaybeValidLrudStyle {
        MaybeValidLrudStyle::Invalid(self)
    }
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

impl Into<MaybeValidLrudItem> for LrudItem {
    fn into(self) -> MaybeValidLrudItem {
        MaybeValidLrudItem::Valid(self)
    }
}

impl Into<MaybeValidLrudItem> for InvalidValue {
    fn into(self) -> MaybeValidLrudItem {
        MaybeValidLrudItem::Invalid(self)
    }
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
#[serde(untagged)]
pub enum MaybeValidLrudOrder {
    /// # LrudOrder
    /// The order in which LRUD measurements are specified
    Valid([LrudItem; 4]),
    Invalid {
        #[serde(rename = "INVALID")]
        invalid: Vec<MaybeValidLrudItem>,
        issues: Option<Vec<usize>>,
    },
}

impl Into<MaybeValidLrudOrder> for [LrudItem; 4] {
    fn into(self) -> MaybeValidLrudOrder {
        MaybeValidLrudOrder::Valid(self)
    }
}

impl From<Result<[LrudItem; 4], MaybeValidLrudOrder>> for MaybeValidLrudOrder {
    fn from(result: Result<[LrudItem; 4], MaybeValidLrudOrder>) -> MaybeValidLrudOrder {
        match result {
            Ok(valid) => valid.into(),
            Err(invalid) => invalid,
        }
    }
}

impl Into<Result<[LrudItem; 4], MaybeValidLrudOrder>> for MaybeValidLrudOrder {
    fn into(self) -> Result<[LrudItem; 4], MaybeValidLrudOrder> {
        match self {
            MaybeValidLrudOrder::Valid(valid) => Ok(valid),
            invalid => Err(invalid),
        }
    }
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
#[schemars(deny_unknown_fields)]
pub enum TapingMethod {
    /// Indicates the distance of a shot is measured from the instrument to
    /// the target (either of which may be above or below the FROm/TO station,
    /// if `instrument_height`/`target_height` are given)
    InstrumentToTarget,
    /// Indicates the distance of a shot is measured from station to station,
    /// even if the inclination instrument and/or target positions are above
    /// or below the FROm/TO station, if `instrument_height`/`target_height`
    /// are given)
    StationToStation,
    /// Indicates the distance of a shot is measured from the inclination
    /// instrument to the TO station, even if the inclination target position
    /// is above or below the TO station, if `target_height` is given)
    InstrumentToStation,
    /// Indicates the distance of a shot is measured from the FROM
    /// station to the inclination target, even if the inclination instrument
    /// position is above or below the FROM station, if `instrument_height` is given)
    StationToTarget,
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[schemars(deny_unknown_fields)]
pub struct SrvSettings {
    /// The type of shot vector: compass and tape (distance, azimuth, inclination)
    /// or rectilinear (easting, northing, elevation)
    pub shot_type: ShotType,
    /// The order of distance, azimuth, and (optionally) inclination measurements
    /// when the `shot_type` is `CompassAndTape`
    pub compass_and_tape_order: Vec<CompassAndTapeItem>,
    /// The order of easting, northing, and (optionally) elevation measurements
    /// when the `shot_type` is `Rectilinear`
    pub rectilinear_order: Vec<RectilinearItem>,
    /// The default unit for all distances except LRUDs and instrument/target heights
    pub primary_distance_unit: LengthUnit,
    /// The default unit LRUDs and instrument/target heights
    pub secondary_distance_unit: LengthUnit,
    /// The default unit for frontsight azimuth measurements
    pub frontsight_azimuth_unit: AngleUnit,
    /// The default unit for backsight azimuth measurements
    pub backsight_azimuth_unit: AngleUnit,
    /// The default unit for frontsight inclination measurements
    pub frontsight_inclination_unit: InclinationUnit,
    /// The default unit for backsight inclination measurements
    pub backsight_inclination_unit: InclinationUnit,
    /// The magnetic declination.  If omitted, it may be derived from the `#DATE`,
    /// otherwise it defaults to 0
    pub magnetic_declination: Option<Angle>,
    pub grid_north_correction: Option<Angle>,
    pub rectilinear_north_correction: Angle,
    /// Correction that should be applied to the distance measurements of
    /// `CompassAndTape` shots, except when the distance is zero.
    pub primary_distance_correction: Length,
    /// Correction that should be applied to LRUDs and instrument/target heights
    pub secondary_distance_correction: Length,
    /// Correction that should be applied to frontsight azimuth measurements
    pub frontsight_azimuth_correction: Angle,
    /// Correction that should be applied to backsight azimuth measurements
    pub backsight_azimuth_correction: Angle,
    /// Correction that should be applied to frontsight inclination measurements
    /// (except for pure vertical shots)
    pub frontsight_inclination_correction: Inclination,
    /// Correction that should be applied to backsight inclination measurements
    /// (except for pure vertical shots)
    pub backsight_inclination_correction: Inclination,
    /// An additional vertical adjustment that should be added to the computed
    /// elevation of the TO station with respect to the FROM station
    /// (except for pure vertical shots)
    pub height_adjustment: Length,
    /// Options for backsight azimuth measurements
    pub backsight_azimuth_options: BacksightOptions,
    /// Options for backsight inclination measurements
    pub backsight_inclination_options: BacksightOptions,
    /// Case conversion that should be applied to station names
    pub station_name_case_conversion: StationNameCaseConversion,
    /// The style of LRUD measurements (whether they're associated with the
    /// FROM or TO station, and how they're angled)
    pub lrud_style: LrudStyle,
    /// The order of LRUD measurements
    pub lrud_order: [LrudItem; 4],
    /// Default prefixes that should be applied to station names (`[ PREFIX1, PREFIX2, PREFIX3 ]`)
    pub prefix: [String; 3],
    /// How distance measurements are defined for shots with instrument and/or target heights
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
            magnetic_declination: None,
            grid_north_correction: None,
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
            UnitsOption::BacksightAzimuthType {
                options,
                loc: _,
                locs: _,
            } => {
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
            UnitsOption::BacksightInclinationType {
                options,
                loc: _,
                locs: _,
            } => self.backsight_inclination_options = options.clone(),
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
            } => self.grid_north_correction = Some(*correction),
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
            } => self.magnetic_declination = Some(*declination),
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
    /// If true, backsight measurements are "corrected" (inverted to match
    /// the facing direction of the frontsight measurements)
    pub is_corrected: bool,
    /// The maximum difference between frontsight and backsight measurements
    /// to tolerate without logging and error
    pub tolerance: Angle,
    /// If true, only the frontsight measurement should be used for calculating
    /// station positions.  Otherwise, the average of the frontsight and
    /// backsight measurements should be used
    pub do_not_average: bool,
}

impl BacksightOptions {
    pub fn default() -> BacksightOptions {
        BacksightOptions {
            is_corrected: false,
            tolerance: Angle::degrees(5.0),
            do_not_average: false,
        }
    }
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
#[schemars(deny_unknown_fields)]
pub struct BacksightOptionsLocs {
    /// The location of the `TYPEAB` or `TYPEVB` text
    pub option: Option<SourceLoc>,
    /// The location of the corrected flag (`N` or `C`)
    pub is_corrected: Option<SourceLoc>,
    /// The location of the tolerance
    pub tolerance: Option<SourceLoc>,
    /// The location of the do not average flag (`X`)
    pub do_not_average: Option<SourceLoc>,
}

impl BacksightOptionsLocs {
    pub fn loc(self) -> Option<SourceLoc> {
        match self {
            BacksightOptionsLocs {
                option: Some(option),
                is_corrected,
                tolerance,
                do_not_average,
            } => Some(
                option.start.up_to(
                    do_not_average
                        .or(tolerance)
                        .or(is_corrected)
                        .unwrap_or(option)
                        .end,
                ),
            ),
            _ => None,
        }
    }
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct InvalidBacksightOptions {
    pub is_corrected: Option<MaybeValidBool>,
    pub tolerance: Option<MaybeValidAngle>,
    pub do_not_average: Option<MaybeValidBool>,
}

#[derive(JsonSchema, Serialize_repr, Deserialize_repr, PartialEq, Debug, Copy, Clone)]
#[repr(u8)]
#[schemars(deny_unknown_fields)]
pub enum PrefixLevel {
    Prefix1 = 1,
    Prefix2 = 2,
    Prefix3 = 3,
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
    /// # CompassAndTapeOption
    /// A `CT` option, which indicates shot vectors are defined with
    /// distance, azimuth, and inclination measurements
    CompassAndTape { loc: Option<SourceLoc> },
    /// # RectilinearOption
    /// A `RECT` option, which indicates shot vectors are defined with
    /// easting, northing, and elevation measurements
    Rectilinear { loc: Option<SourceLoc> },
    /// # CompassAndTapeOrderOption
    /// An `ORDER` option for `CompassAndTape` measurements
    CompassAndTapeOrder {
        /// The order of the distance, azimuth, and (optionally) inclination
        /// measurements
        order: Vec<CompassAndTapeItem>,
        loc: Option<SourceLoc>,
        locs: Option<OrderOptionLocs>,
    },
    /// # RectilinearOrderOption
    /// An `ORDER` option for `Rectilinear` measurements
    RectilinearOrder {
        /// The order of the easting, northing, and (optionally) elevation
        /// measurements
        order: Vec<RectilinearItem>,
        loc: Option<SourceLoc>,
        locs: Option<OrderOptionLocs>,
    },
    /// # FrontsightAzimuthUnitOption
    /// An `A=` option, the default unit for frontsight azimuth measurements
    FrontsightAzimuthUnit {
        /// the default unit for frontsight azimuth measurements
        unit: AngleUnit,
        loc: Option<SourceLoc>,
        locs: Option<UnitOptionLocs>,
    },
    /// # BacksightAzimuthUnitOption
    /// An `AB=` option, the default unit for backsight azimuth measurements
    BacksightAzimuthUnit {
        /// the default unit for backsight azimuth measurements
        unit: AngleUnit,
        loc: Option<SourceLoc>,
        locs: Option<UnitOptionLocs>,
    },
    /// # PrimaryDistanceUnitOption
    /// An `D=` option, the default unit for all distances except LRUDs,
    /// instrument and target heights
    PrimaryDistanceUnit {
        /// the default unit for all distances except LRUDs,
        /// instrument and target heights
        unit: LengthUnit,
        loc: Option<SourceLoc>,
        locs: Option<UnitOptionLocs>,
    },
    /// # SecondaryDistanceUnitOption
    /// An `S=` option, the default unit for LRUDs, instrument and target heights
    SecondaryDistanceUnit {
        /// the default unit for LRUDs, instrument and target heights
        unit: LengthUnit,
        loc: Option<SourceLoc>,
        locs: Option<UnitOptionLocs>,
    },
    /// # DistanceUnitOption
    /// A `Feet` or `Meters` option, the default unit for all distances
    DistanceUnit {
        unit: LengthUnit,
        loc: Option<SourceLoc>,
    },
    /// # FrontsightInclinationUnitOption
    /// A `V=` option, the default unit for frontsight inclination measurements
    FrontsightInclinationUnit {
        /// the default unit for frontsight inclination measurements
        unit: InclinationUnit,
        loc: Option<SourceLoc>,
        locs: Option<UnitOptionLocs>,
    },
    /// # BacksightInclinationUnitOption
    /// A `VB=` option, the default unit for backsight inclination measurements
    BacksightInclinationUnit {
        /// the default unit for backsight inclination measurements
        unit: InclinationUnit,
        loc: Option<SourceLoc>,
        locs: Option<UnitOptionLocs>,
    },
    /// # MagneticDeclinationOption
    /// A `DECL=` option, the magnetic declination angle
    /// (true north minus magnetic north)
    MagneticDeclination {
        /// the magnetic declination angle
        /// (true north minus magnetic north)
        declination: Angle,
        loc: Option<SourceLoc>,
        locs: Option<DeclinationOptionLocs>,
    },
    /// # GridNorthCorrectionOption
    /// A `GRID=` option,
    GridNorthCorrection {
        correction: Angle,
        loc: Option<SourceLoc>,
        locs: Option<CorrectionOptionLocs>,
    },
    /// # RectilinearNorthCorrectionOption
    RectilinearNorthCorrection {
        correction: Angle,
        loc: Option<SourceLoc>,
        locs: Option<CorrectionOptionLocs>,
    },
    /// # PrimaryDistanceCorrectionOption
    PrimaryDistanceCorrection {
        correction: Length,
        loc: Option<SourceLoc>,
        locs: Option<CorrectionOptionLocs>,
    },
    /// # SecondaryDistanceCorrectionOption
    SecondaryDistanceCorrection {
        correction: Length,
        loc: Option<SourceLoc>,
        locs: Option<CorrectionOptionLocs>,
    },
    /// # FrontsightAzimuthCorrectionOption
    FrontsightAzimuthCorrection {
        correction: Angle,
        loc: Option<SourceLoc>,
        locs: Option<CorrectionOptionLocs>,
    },
    /// # BacksightAzimuthCorrectionOption
    BacksightAzimuthCorrection {
        correction: Angle,
        loc: Option<SourceLoc>,
        locs: Option<CorrectionOptionLocs>,
    },
    /// # FrontsightInclinationCorrectionOption
    FrontsightInclinationCorrection {
        correction: Inclination,
        loc: Option<SourceLoc>,
        locs: Option<CorrectionOptionLocs>,
    },
    /// # FrontsightInclinationCorrectionOption
    BacksightInclinationCorrection {
        correction: Inclination,
        loc: Option<SourceLoc>,
        locs: Option<CorrectionOptionLocs>,
    },
    /// # HeightAdjustmentOption
    HeightAdjustment {
        correction: Length,
        loc: Option<SourceLoc>,
        locs: Option<CorrectionOptionLocs>,
    },
    /// # BacksightAzimuthType
    BacksightAzimuthType {
        #[serde(flatten)]
        options: BacksightOptions,
        loc: Option<SourceLoc>,
        locs: Option<BacksightOptionsLocs>,
    },
    /// # BacksightInclinationType
    BacksightInclinationType {
        #[serde(flatten)]
        options: BacksightOptions,
        loc: Option<SourceLoc>,
        locs: Option<BacksightOptionsLocs>,
    },
    /// # ResetOption
    Reset { loc: Option<SourceLoc> },
    /// # SaveOption
    Save { loc: Option<SourceLoc> },
    /// # RestoreOption
    Restore { loc: Option<SourceLoc> },
    /// # StationNameCaseOption
    StationNameCase {
        conversion: StationNameCaseConversion,
        loc: Option<SourceLoc>,
        locs: Option<StationNameCaseOptionLocs>,
    },
    /// # LrudOption
    Lrud {
        style: LrudStyle,
        order: Option<[LrudItem; 4]>,
        loc: Option<SourceLoc>,
        locs: Option<LrudOptionLocs>,
    },
    /// # PrefixOption
    Prefix {
        level: PrefixLevel,
        prefix: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<PrefixOptionLocs>,
    },
    /// # TapingMethodOption
    TapingMethod {
        method: TapingMethod,
        loc: Option<SourceLoc>,
        locs: Option<TapingMethodOptionLocs>,
    },
    /// # UnitVarianceOption
    UnitVariance {
        variance: f64,
        loc: Option<SourceLoc>,
        locs: Option<VarianceOptionLocs>,
    },
    /// # HorizontalUnitVarianceOption
    HorizontalUnitVariance {
        variance: f64,
        loc: Option<SourceLoc>,
        locs: Option<VarianceOptionLocs>,
    },
    /// # VerticalUnitVarianceOption
    VerticalUnitVariance {
        variance: f64,
        loc: Option<SourceLoc>,
        locs: Option<VarianceOptionLocs>,
    },
    /// # FlagOption
    Flag {
        flag: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<FlagOptionLocs>,
    },
    /// # MacroOption
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
    pub fn backsight_azimuth_type(
        options: BacksightOptions,
        locs: Option<BacksightOptionsLocs>,
    ) -> UnitsOption {
        UnitsOption::BacksightAzimuthType {
            options,
            locs,
            loc: locs.and_then(|l| l.loc()),
        }
    }
    pub fn backsight_inclination_type(
        options: BacksightOptions,
        locs: Option<BacksightOptionsLocs>,
    ) -> UnitsOption {
        UnitsOption::BacksightInclinationType {
            options,
            locs,
            loc: locs.and_then(|l| l.loc()),
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
    /// # InvalidOrderOption
    Order {
        order: Option<Vec<MaybeValidOrderItem>>,
        loc: Option<SourceLoc>,
        locs: Option<OrderOptionLocs>,
    },
    /// # InvalidFrontsightAzimuthUnitOption
    FrontsightAzimuthUnit {
        unit: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<UnitOptionLocs>,
    },
    /// # InvalidBacksightAzimuthUnitOption
    BacksightAzimuthUnit {
        unit: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<UnitOptionLocs>,
    },
    /// # InvalidPrimaryDistanceUnitOption
    PrimaryDistanceUnit {
        unit: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<UnitOptionLocs>,
    },
    /// # InvalidSecondaryDistanceUnitOption
    SecondaryDistanceUnit {
        unit: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<UnitOptionLocs>,
    },
    /// # InvalidFrontsightInclinationUnitOption
    FrontsightInclinationUnit {
        unit: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<UnitOptionLocs>,
    },
    /// # InvalidBacksightInclinationUnitOption
    BacksightInclinationUnit {
        unit: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<UnitOptionLocs>,
    },
    /// # InvalidMagneticDeclinationOption
    MagneticDeclination {
        declination: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<DeclinationOptionLocs>,
    },
    /// # InvalidGridNorthCorrectionOption
    GridNorthCorrection {
        correction: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<CorrectionOptionLocs>,
    },
    /// # InvalidRectilinearNorthCorrectionOption
    RectilinearNorthCorrection {
        correction: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<CorrectionOptionLocs>,
    },
    /// # InvalidPrimaryDistanceCorrectionOption
    PrimaryDistanceCorrection {
        correction: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<CorrectionOptionLocs>,
    },
    /// # InvalidSecondaryDistanceCorrectionOption
    SecondaryDistanceCorrection {
        correction: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<CorrectionOptionLocs>,
    },
    /// # InvalidFrontsightAzimuthCorrectionOption
    FrontsightAzimuthCorrection {
        correction: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<CorrectionOptionLocs>,
    },
    /// # InvalidBacksightAzimuthCorrectionOption
    BacksightAzimuthCorrection {
        correction: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<CorrectionOptionLocs>,
    },
    /// # InvalidFrontsightInclinationCorrectionOption
    FrontsightInclinationCorrection {
        correction: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<CorrectionOptionLocs>,
    },
    /// # InvalidBacksightInclinationCorrectionOption
    BacksightInclinationCorrection {
        correction: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<CorrectionOptionLocs>,
    },
    /// # InvalidHeightAdjustmentOption
    HeightAdjustment {
        correction: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<CorrectionOptionLocs>,
    },
    /// # InvalidBacksightAzimuthType
    BacksightAzimuthType {
        #[serde(flatten)]
        options: InvalidBacksightOptions,
        loc: Option<SourceLoc>,
        locs: Option<BacksightOptionsLocs>,
    },
    /// # InvalidBacksightInclinationType
    BacksightInclinationType {
        #[serde(flatten)]
        options: InvalidBacksightOptions,
        loc: Option<SourceLoc>,
        locs: Option<BacksightOptionsLocs>,
    },
    /// # InvalidStationNameCaseOption
    StationNameCase {
        conversion: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<StationNameCaseOptionLocs>,
    },
    /// # InvalidLrudOption
    Lrud {
        style: Option<MaybeValidLrudStyle>,
        order: Option<MaybeValidLrudOrder>,
        loc: Option<SourceLoc>,
        locs: Option<LrudOptionLocs>,
    },
    /// # InvalidTapingMethodOption
    TapingMethod {
        method: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<TapingMethodOptionLocs>,
    },
    /// # InvalidUnitVarianceOption
    UnitVariance {
        variance: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<VarianceOptionLocs>,
    },
    /// # InvalidHorizontalUnitVarianceOption
    HorizontalUnitVariance {
        variance: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<VarianceOptionLocs>,
    },
    /// # InvalidVerticalUnitVarianceOption
    VerticalUnitVariance {
        variance: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<VarianceOptionLocs>,
    },
    /// # InvalidMacroOption
    Macro {
        name: Option<String>,
        value: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<MacroOptionLocs>,
    },
    /// # InvalidUnknownOption
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
    pub fn backsight_azimuth_type(
        options: InvalidBacksightOptions,
        locs: Option<BacksightOptionsLocs>,
    ) -> InvalidUnitsOption {
        InvalidUnitsOption::BacksightAzimuthType {
            options,
            locs,
            loc: locs.and_then(|l| l.loc()),
        }
    }
    pub fn backsight_inclination_type(
        options: InvalidBacksightOptions,
        locs: Option<BacksightOptionsLocs>,
    ) -> InvalidUnitsOption {
        InvalidUnitsOption::BacksightInclinationType {
            options,
            locs,
            loc: locs.and_then(|l| l.loc()),
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
pub enum MaybeValidLrud {
    Valid(LrudStyle),
    Invalid {
        #[serde(rename = "INVALID")]
        invalid: String,
        issues: Option<Vec<usize>>,
    },
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
    /// # LatLongFixLocation
    LatLong {
        latitude: Angle,
        longitude: Angle,
        elevation: Length,
        loc: Option<SourceLoc>,
    },
    /// # GridFixLocation
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
    /// # InvalidLatLongFixLocation
    LatLong {
        latitude: Option<MaybeValidAngle>,
        longitude: Option<MaybeValidAngle>,
        elevation: Option<MaybeValidLength>,
        loc: Option<SourceLoc>,
    },
    /// # InvalidGridFixLocation
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
    /// # LengthVarianceAssignment
    Length { length: Length },
    /// # RMSErrorVarianceAssignment
    RMSError { length: Length },
    /// # FloatShotVarianceAssignment
    FloatShot,
    /// # FloatTraverseVarianceAssignment
    FloatTraverse,
}

#[skip_serializing_none]
#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[serde(tag = "type")]
#[schemars(deny_unknown_fields)]
pub enum InvalidVarianceAssignment {
    /// # InvalidLengthVarianceAssignment
    Length { length: String },
    /// # InvalidVarianceAssignment
    RMSError { length: String },
    /// # UnknownVarianceAssignment
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
    /// # CompassAndTapeShotMeasurements
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
    /// # RectilinearShotMeasurements
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
    /// # InvalidCompassAndTapeShotMeasurements
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
    /// # InvalidRectilinearShotMeasurements
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
    /// # UnitsDirective
    UnitsDirective {
        options: Vec<UnitsOption>,
        comment: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<UnitsDirectiveLocs>,
    },
    /// # SegmentDirective
    SegmentDirective {
        segment: String,
        comment: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<SymbolDirectiveLocs>,
    },
    /// # FixDirective
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
    /// # PrefixDirective
    PrefixDirective {
        level: PrefixLevel,
        prefix: Option<String>,
        comment: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<PrefixDirectiveLocs>,
    },
    /// # NoteDirective
    NoteDirective {
        station: String,
        note: String,
        comment: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<NoteDirectiveLocs>,
    },
    /// # FlagDirective
    FlagDirective {
        stations: Vec<String>,
        flag: String,
        comment: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<FlagDirectiveLocs>,
    },
    /// # SymbolDirective
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
    /// # DateDirective
    DateDirective {
        year: u32,
        month: u8,
        day: u8,
        comment: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<DateDirectiveLocs>,
    },
    /// # Shot
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
    /// # Comment
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
    /// # InvalidUnitsDirective
    UnitsDirective {
        options: Vec<MaybeValidUnitsOption>,
        comment: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<UnitsDirectiveLocs>,
    },
    /// # InvalidFixDirective
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
    /// # InvalidFlagDirective
    FlagDirective {
        stations: Vec<String>,
        flag: Option<String>,
        comment: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<FlagDirectiveLocs>,
    },
    /// # InvalidSymbolDirective
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
    /// # InvalidDateDirective
    DateDirective {
        year: Option<MaybeValidU32>,
        month: Option<MaybeValidU8>,
        day: Option<MaybeValidU8>,
        comment: Option<String>,
        loc: Option<SourceLoc>,
        locs: Option<DateDirectiveLocs>,
    },
    /// # InvalidShot
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
    /// # UnknownItem
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
    pub issues: Option<Vec<ParseIssue<SrvErrorCode>>>,
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
        issues: Vec<ParseIssue<SrvErrorCode>>,
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
pub const EINVALIDNUMBER: &str = "EINVALIDNUMBER";
pub const EINVALIDLENGTH: &str = "EINVALIDLENGTH";
pub const EINVALIDLENGTHUNIT: &str = "EINVALIDLENGTHUNIT";
pub const EINVALIDANGLE: &str = "EINVALIDANGLE";
pub const EINVALIDANGLEUNIT: &str = "EINVALIDANGLEUNIT";
pub const EANGLEOUTOFRANGE: &str = "EANGLEOUTOFRANGE";
pub const EMISSINGMINUTES: &str = "EMISSINGMINUTES";
pub const EMISSINGSECONDS: &str = "EMISSINGSECONDS";
pub const EDEGREESOUTOFRANGE: &str = "EDEGREESOUTOFRANGE";
pub const EMINUTESOUTOFRANGE: &str = "EMINUTESOUTOFRANGE";
pub const ESECONDSOUTOFRANGE: &str = "ESECONDSOUTOFRANGE";
pub const EINVALIDAZIMUTH: &str = "EINVALIDAZIMUTH";
pub const EAZIMUTHOUTOFRANGE: &str = "EAZIMUTHOUTOFRANGE";
pub const EMISSINGENDINGDIR: &str = "EMISSINGENDINGDIR";
pub const EINVALIDENDINGDIR: &str = "EINVALIDENDINGDIR";
pub const EINVALIDAZIMUTHUNIT: &str = "EINVALIDAZIMUTHUNIT";
pub const EINVALIDINCLINATION: &str = "EINVALIDINCLINATION";
pub const EINCLINATIONOUTOFRANGE: &str = "EINCLINATIONOUTOFRANGE";
pub const EINVALIDINCLINATIONUNIT: &str = "EINVALIDINCLINATIONUNIT";
pub const EINVALIDCASECONVERSION: &str = "EINVALIDCASECONVERSION";
pub const EINVALIDTAPINGMETHOD: &str = "EINVALIDTAPINGMETHOD";
pub const EINVALIDUNITVARIANCE: &str = "EINVALIDUNITVARIANCE";
pub const EINVALIDLRUDSTYLE: &str = "EINVALIDLRUDSTYLE";
pub const EMISSINGLRUDSTYLE: &str = "EMISSINGLRUDSTYLE";
pub const EINVALIDLRUDORDERITEM: &str = "EINVALIDLRUDORDERITEM";
pub const EINVALIDLRUDORDER: &str = "EINVALIDLRUDORDER";
pub const EMISSINGLRUDORDER: &str = "EMISSINGLRUDORDER";
pub const EMISSINGINCHES: &str = "EMISSINGINCHES";
pub const EMISSINGWHITESPACE: &str = "EMISSINGWHITESPACE";
pub const EINVALIDBACKSIGHTCORRECTED: &str = "EINVALIDBACKSIGHTCORRECTED";
pub const EMISSINGBACKSIGHTCORRECTED: &str = "EMISSINGBACKSIGHTCORRECTED";
pub const EMISSINGTOLERANCE: &str = "EMISSINGTOLERANCE";
pub const EINVALIDTOLERANCE: &str = "EINVALIDTOLERANCE";
pub const EINVALIDDONOTAVERAGE: &str = "EINVALIDDONOTAVERAGE";
pub const EMISSINGDONOTAVERAGE: &str = "EMISSINGDONOTAVERAGE";
