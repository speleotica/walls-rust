use super::*;
use crate::srv::types::CorrectionOptionLocs;
use insta::assert_yaml_snapshot;
use serde_json::Value;

#[test]
fn test_snapshots() {
    assert_yaml_snapshot!(&parse_test_srv("#units order=dav"));
    assert_yaml_snapshot!(&parse_test_srv("#units order=dva"));
    assert_yaml_snapshot!(&parse_test_srv("#units order=adv"));
    assert_yaml_snapshot!(&parse_test_srv("#units order=avd"));
    assert_yaml_snapshot!(&parse_test_srv("#units order=vda"));
    assert_yaml_snapshot!(&parse_test_srv("#units order=vad"));
    assert_yaml_snapshot!(&parse_test_srv("#units order=da"));
    assert_yaml_snapshot!(&parse_test_srv("#units order=ad"));
    assert_yaml_snapshot!(&parse_test_srv("#units order=ddav"));
    assert_yaml_snapshot!(&parse_test_srv("#units order=dv"));
    assert_yaml_snapshot!(&parse_test_srv("#units order=daqv"));
    assert_yaml_snapshot!(&parse_test_srv("#units order=enu"));
    assert_yaml_snapshot!(&parse_test_srv("#units order=eun"));
    assert_yaml_snapshot!(&parse_test_srv("#units order=neu"));
    assert_yaml_snapshot!(&parse_test_srv("#units order=nue"));
    assert_yaml_snapshot!(&parse_test_srv("#units order=uen"));
    assert_yaml_snapshot!(&parse_test_srv("#units order=une"));
    assert_yaml_snapshot!(&parse_test_srv("#units order=en"));
    assert_yaml_snapshot!(&parse_test_srv("#units order=ne"));
    assert_yaml_snapshot!(&parse_test_srv("#units order=nu"));
    assert_yaml_snapshot!(&parse_test_srv("#units order=nuqe"));
    assert_yaml_snapshot!(&parse_test_srv("#units order=nuda"));
    assert_yaml_snapshot!(&parse_test_srv("#units order="));
    assert_yaml_snapshot!(&parse_test_srv("#units order"));
    assert_yaml_snapshot!(&parse_test_srv("#units order=uen;test"));
}

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
fn test_order_vad() {
    let input = "#units order=vad;test";
    assert_eq!(
        WallsSrvParser::parse(input),
        WallsSrvFile {
            items: vec![SrvItem::UnitsDirective {
                options: vec![UnitsOption::CompassAndTapeOrder {
                    order: vec![
                        CompassAndTapeItem::Inclination,
                        CompassAndTapeItem::Azimuth,
                        CompassAndTapeItem::Distance
                    ],
                    loc: SourceLoc::find_range(input, "order=vad"),
                    locs: Some(OrderOptionLocs {
                        option: SourceLoc::find_range(input, "order").unwrap(),
                        order: SourceLoc::find_range(input, "vad"),
                    })
                }],
                comment: Some("test".into()),
                loc: Some(SourceLoc::str_from_to(
                    input,
                    0,
                    input.find(";test").unwrap()
                )),
                locs: Some(UnitsDirectiveLocs {
                    directive: SourceLoc::find_range(input, "#units").unwrap(),
                    comment: SourceLoc::find_range(input, ";test")
                })
            }],
            issues: None,
        }
        .into()
    )
}

#[test]
fn test_order_nue() {
    let input = "#units order=nue";
    assert_eq!(
        WallsSrvParser::parse(input),
        WallsSrvFile {
            items: vec![SrvItem::UnitsDirective {
                options: vec![UnitsOption::RectilinearOrder {
                    order: vec![
                        RectilinearItem::Northing,
                        RectilinearItem::Elevation,
                        RectilinearItem::Easting
                    ],
                    loc: SourceLoc::find_range(input, "order=nue"),
                    locs: Some(OrderOptionLocs {
                        option: SourceLoc::find_range(input, "order").unwrap(),
                        order: SourceLoc::find_range(input, "nue"),
                    })
                }],
                comment: None,
                loc: Some(SourceLoc::str_from_to(input, 0, input.len())),
                locs: Some(UnitsDirectiveLocs {
                    directive: SourceLoc::find_range(input, "#units").unwrap(),
                    comment: None
                })
            }],
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

#[test]
fn test_units_feet_incd() {
    let input = "#units feet incd=3.4 ;foobus";
    assert_eq!(
        WallsSrvParser::parse(input),
        WallsSrvFile {
            items: vec![SrvItem::UnitsDirective {
                options: vec![
                    UnitsOption::distance_unit(
                        LengthUnit::Feet,
                        SourceLoc::find_range(input, "feet")
                    ),
                    UnitsOption::PrimaryDistanceCorrection {
                        correction: Length {
                            value: 3.4,
                            unit: LengthUnit::Feet
                        },
                        loc: SourceLoc::find_range(input, "incd=3.4"),
                        locs: Some(CorrectionOptionLocs {
                            option: SourceLoc::find_range(input, "incd").unwrap(),
                            correction: SourceLoc::find_range(input, "3.4")
                        })
                    }
                ],
                comment: Some("foobus".into()),
                loc: Some(SourceLoc::str_from_to(input, 0, input.find(";").unwrap())),
                locs: Some(UnitsDirectiveLocs {
                    directive: SourceLoc::find_range(input, "#units").unwrap(),
                    comment: SourceLoc::find_range(input, ";foobus")
                })
            }],
            issues: None
        }
        .into(),
    )
}

fn parse_test_srv(input: &str) -> Value {
    let parsed = WallsSrvParser::parse(input);
    let mut value = serde_json::to_value(parsed).unwrap();
    update_test_locs(&mut value, input);
    value
}

fn update_test_locs(val: &mut Value, input: &str) {
    match val {
        Value::Object(map) => {
            // Check each key-value pair in the object
            let keys: Vec<String> = map.keys().cloned().collect();
            for key in keys {
                match key.as_str() {
                    "loc" => {
                        if let Some(v) = map.get_mut(&key) {
                            let parsed = serde_json::from_value::<SourceLoc>(v.clone()).unwrap();
                            *v = input[parsed.start.index..parsed.end.index].into();
                        }
                    }
                    "locs" => {
                        if let Some(Value::Object(locs)) = map.get_mut(&key) {
                            let keys: Vec<String> = locs.keys().cloned().collect();
                            for key in keys {
                                if let Some(v) = locs.get_mut(&key) {
                                    if let Ok(parsed) =
                                        serde_json::from_value::<SourceLoc>(v.clone())
                                    {
                                        *v = input[parsed.start.index..parsed.end.index].into();
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }

            // Recurse into the values of the object
            for (_, v) in map.iter_mut() {
                update_test_locs(v, input);
            }
        }
        Value::Array(arr) => {
            // Recurse into array elements
            for item in arr {
                update_test_locs(item, input);
            }
        }
        _ => {} // Base case: primitive values
    }
}
