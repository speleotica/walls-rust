use super::*;
use insta::assert_yaml_snapshot;
use serde_json::Value;

#[test]
fn test_snapshots() {
    assert_yaml_snapshot!(&parse("#units order=dav"));
    assert_yaml_snapshot!(&parse("#units order=dva"));
    assert_yaml_snapshot!(&parse("#units order=adv"));
    assert_yaml_snapshot!(&parse("#units order=avd"));
    assert_yaml_snapshot!(&parse("#units order=vda"));
    assert_yaml_snapshot!(&parse("#units order=vad"));
    assert_yaml_snapshot!(&parse("#units order=da"));
    assert_yaml_snapshot!(&parse("#units order=ad"));
    assert_yaml_snapshot!(&parse("#units order=ddav"));
    assert_yaml_snapshot!(&parse("#units order=dv"));
    assert_yaml_snapshot!(&parse("#units order=daqv"));
    assert_yaml_snapshot!(&parse("#units order=enu"));
    assert_yaml_snapshot!(&parse("#units order=eun"));
    assert_yaml_snapshot!(&parse("#units order=neu"));
    assert_yaml_snapshot!(&parse("#units order=nue"));
    assert_yaml_snapshot!(&parse("#units order=uen"));
    assert_yaml_snapshot!(&parse("#units order=une"));
    assert_yaml_snapshot!(&parse("#units order=en"));
    assert_yaml_snapshot!(&parse("#units order=ne"));
    assert_yaml_snapshot!(&parse("#units order=nu"));
    assert_yaml_snapshot!(&parse("#units order=nuqe"));
    assert_yaml_snapshot!(&parse("#units order=nuda"));
    assert_yaml_snapshot!(&parse("#units order="));
    assert_yaml_snapshot!(&parse("#units order"));
    assert_yaml_snapshot!(&parse("#units order=uen;test"));

    assert_yaml_snapshot!(&parse("#units f"));
    assert_yaml_snapshot!(&parse("#units feet"));
    assert_yaml_snapshot!(&parse("#units m"));
    assert_yaml_snapshot!(&parse("#units meters"));

    assert_yaml_snapshot!(&parse("#units d"));
    assert_yaml_snapshot!(&parse("#units d="));
    assert_yaml_snapshot!(&parse("#units d=m"));
    assert_yaml_snapshot!(&parse("#units d=meters"));
    assert_yaml_snapshot!(&parse("#units d=f"));
    assert_yaml_snapshot!(&parse("#units d=feet"));
    assert_yaml_snapshot!(&parse("#units d=g"));

    assert_yaml_snapshot!(&parse("#units s"));
    assert_yaml_snapshot!(&parse("#units s="));
    assert_yaml_snapshot!(&parse("#units s=m"));
    assert_yaml_snapshot!(&parse("#units s=meters"));
    assert_yaml_snapshot!(&parse("#units s=f"));
    assert_yaml_snapshot!(&parse("#units s=feet"));
    assert_yaml_snapshot!(&parse("#units s=g"));

    assert_yaml_snapshot!(&parse("#units a"));
    assert_yaml_snapshot!(&parse("#units a="));
    assert_yaml_snapshot!(&parse("#units a=d"));
    assert_yaml_snapshot!(&parse("#units a=degrees"));
    assert_yaml_snapshot!(&parse("#units a=g"));
    assert_yaml_snapshot!(&parse("#units a=grads"));
    assert_yaml_snapshot!(&parse("#units a=m"));
    assert_yaml_snapshot!(&parse("#units a=mils"));
    assert_yaml_snapshot!(&parse("#units a=blargh"));

    assert_yaml_snapshot!(&parse("#units ab"));
    assert_yaml_snapshot!(&parse("#units ab="));
    assert_yaml_snapshot!(&parse("#units ab=d"));
    assert_yaml_snapshot!(&parse("#units ab=degrees"));
    assert_yaml_snapshot!(&parse("#units ab=g"));
    assert_yaml_snapshot!(&parse("#units ab=grads"));
    assert_yaml_snapshot!(&parse("#units ab=m"));
    assert_yaml_snapshot!(&parse("#units ab=mils"));
    assert_yaml_snapshot!(&parse("#units ab=blargh"));
}

fn parse(input: &str) -> Value {
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
                            if let Ok(parsed) = serde_json::from_value::<SourceLoc>(v.clone()) {
                                *v = input[parsed.start.index..parsed.end.index].into();
                            }
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
