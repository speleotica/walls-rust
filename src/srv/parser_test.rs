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

    assert_yaml_snapshot!(&parse("#units v"));
    assert_yaml_snapshot!(&parse("#units v="));
    assert_yaml_snapshot!(&parse("#units v=d"));
    assert_yaml_snapshot!(&parse("#units v=degrees"));
    assert_yaml_snapshot!(&parse("#units v=g"));
    assert_yaml_snapshot!(&parse("#units v=grads"));
    assert_yaml_snapshot!(&parse("#units v=m"));
    assert_yaml_snapshot!(&parse("#units v=mils"));
    assert_yaml_snapshot!(&parse("#units v=p"));
    assert_yaml_snapshot!(&parse("#units v=percent"));
    assert_yaml_snapshot!(&parse("#units v=blargh"));

    assert_yaml_snapshot!(&parse("#units vb"));
    assert_yaml_snapshot!(&parse("#units vb="));
    assert_yaml_snapshot!(&parse("#units vb=d"));
    assert_yaml_snapshot!(&parse("#units vb=degrees"));
    assert_yaml_snapshot!(&parse("#units vb=g"));
    assert_yaml_snapshot!(&parse("#units vb=grads"));
    assert_yaml_snapshot!(&parse("#units vb=m"));
    assert_yaml_snapshot!(&parse("#units vb=mils"));
    assert_yaml_snapshot!(&parse("#units vb=p"));
    assert_yaml_snapshot!(&parse("#units vb=percent"));
    assert_yaml_snapshot!(&parse("#units vb=blargh"));

    assert_yaml_snapshot!(&parse("#units decl"));
    assert_yaml_snapshot!(&parse("#units decl="));
    assert_yaml_snapshot!(&parse("#units decl=2"));
    assert_yaml_snapshot!(&parse("#units decl=+2"));
    assert_yaml_snapshot!(&parse("#units decl=-2"));
    assert_yaml_snapshot!(&parse("#units decl=-2.3"));
    assert_yaml_snapshot!(&parse("#units decl=-2q"));
    assert_yaml_snapshot!(&parse("#units a=g decl=2"));
    assert_yaml_snapshot!(&parse("#units a=g decl=2m"));
    assert_yaml_snapshot!(&parse("#units a=g decl=2m"));
    assert_yaml_snapshot!(&parse("#units decl=-2q-"));
    assert_yaml_snapshot!(&parse("#units decl=-2m-"));
    assert_yaml_snapshot!(&parse("#units decl=a2m"));
    assert_yaml_snapshot!(&parse("#units decl=--2m"));
    // TODO: dms options

    assert_yaml_snapshot!(&parse("#units grid"));
    assert_yaml_snapshot!(&parse("#units grid="));
    assert_yaml_snapshot!(&parse("#units grid=2"));
    assert_yaml_snapshot!(&parse("#units grid=+2"));
    assert_yaml_snapshot!(&parse("#units grid=-2"));
    assert_yaml_snapshot!(&parse("#units grid=-2.3"));
    assert_yaml_snapshot!(&parse("#units grid=-2q"));
    assert_yaml_snapshot!(&parse("#units a=g grid=2"));
    assert_yaml_snapshot!(&parse("#units a=g grid=2m"));
    assert_yaml_snapshot!(&parse("#units a=g grid=2m"));
    assert_yaml_snapshot!(&parse("#units grid=-2q-"));
    assert_yaml_snapshot!(&parse("#units grid=-2m-"));
    // TODO: dms options

    assert_yaml_snapshot!(&parse("#units incd"));
    assert_yaml_snapshot!(&parse("#units incd="));
    assert_yaml_snapshot!(&parse("#units incd=2"));
    assert_yaml_snapshot!(&parse("#units incd=+2"));
    assert_yaml_snapshot!(&parse("#units incd=-2"));
    assert_yaml_snapshot!(&parse("#units incd=-2.3"));
    assert_yaml_snapshot!(&parse("#units incd=-2q"));
    assert_yaml_snapshot!(&parse("#units d=f incd=2"));
    assert_yaml_snapshot!(&parse("#units d=f incd=2m"));
    assert_yaml_snapshot!(&parse("#units s=f incd=2m"));
    assert_yaml_snapshot!(&parse("#units incd=-2q-"));
    assert_yaml_snapshot!(&parse("#units incd=-2f-"));

    assert_yaml_snapshot!(&parse("#units incs"));
    assert_yaml_snapshot!(&parse("#units incs="));
    assert_yaml_snapshot!(&parse("#units incs=2"));
    assert_yaml_snapshot!(&parse("#units incs=+2"));
    assert_yaml_snapshot!(&parse("#units incs=-2"));
    assert_yaml_snapshot!(&parse("#units incs=-2.3"));
    assert_yaml_snapshot!(&parse("#units incs=-2q"));
    assert_yaml_snapshot!(&parse("#units d=f incs=2"));
    assert_yaml_snapshot!(&parse("#units d=f incs=2m"));
    assert_yaml_snapshot!(&parse("#units s=f incs=2m"));
    assert_yaml_snapshot!(&parse("#units incs=-2q-"));
    assert_yaml_snapshot!(&parse("#units incs=-2f-"));

    assert_yaml_snapshot!(&parse("#units inca"));
    assert_yaml_snapshot!(&parse("#units inca="));
    assert_yaml_snapshot!(&parse("#units inca=2"));
    assert_yaml_snapshot!(&parse("#units inca=+2"));
    assert_yaml_snapshot!(&parse("#units inca=-2"));
    assert_yaml_snapshot!(&parse("#units inca=-2.3"));
    assert_yaml_snapshot!(&parse("#units inca=-2q"));
    assert_yaml_snapshot!(&parse("#units a=g inca=2"));
    assert_yaml_snapshot!(&parse("#units a=g inca=2m"));
    assert_yaml_snapshot!(&parse("#units a=g inca=2m"));
    assert_yaml_snapshot!(&parse("#units inca=-2q-"));
    assert_yaml_snapshot!(&parse("#units inca=-2m-"));
    // TODO: dms options

    assert_yaml_snapshot!(&parse("#units incab"));
    assert_yaml_snapshot!(&parse("#units incab="));
    assert_yaml_snapshot!(&parse("#units incab=2"));
    assert_yaml_snapshot!(&parse("#units incab=+2"));
    assert_yaml_snapshot!(&parse("#units incab=-2"));
    assert_yaml_snapshot!(&parse("#units incab=-2.3"));
    assert_yaml_snapshot!(&parse("#units incab=-2q"));
    assert_yaml_snapshot!(&parse("#units ab=g incab=2"));
    assert_yaml_snapshot!(&parse("#units ab=g incab=2m"));
    assert_yaml_snapshot!(&parse("#units ab=g incab=2m"));
    assert_yaml_snapshot!(&parse("#units incab=-2q-"));
    assert_yaml_snapshot!(&parse("#units incab=-2m-"));
    // TODO: dms options

    assert_yaml_snapshot!(&parse("#units incv"));
    assert_yaml_snapshot!(&parse("#units incv="));
    assert_yaml_snapshot!(&parse("#units incv=2"));
    assert_yaml_snapshot!(&parse("#units incv=+2"));
    assert_yaml_snapshot!(&parse("#units incv=-2"));
    assert_yaml_snapshot!(&parse("#units incv=-2.3"));
    assert_yaml_snapshot!(&parse("#units incv=-2q"));
    assert_yaml_snapshot!(&parse("#units incv=-2g"));
    assert_yaml_snapshot!(&parse("#units incv=-2p"));
    assert_yaml_snapshot!(&parse("#units v=g incv=2"));
    assert_yaml_snapshot!(&parse("#units v=g incv=2m"));
    assert_yaml_snapshot!(&parse("#units v=g incv=2m"));
    assert_yaml_snapshot!(&parse("#units incv=-2q-"));
    assert_yaml_snapshot!(&parse("#units incv=-2m-"));
    // TODO: dms options

    assert_yaml_snapshot!(&parse("#units incvb"));
    assert_yaml_snapshot!(&parse("#units incvb="));
    assert_yaml_snapshot!(&parse("#units incvb=2"));
    assert_yaml_snapshot!(&parse("#units incvb=+2"));
    assert_yaml_snapshot!(&parse("#units incvb=-2"));
    assert_yaml_snapshot!(&parse("#units incvb=-2.3"));
    assert_yaml_snapshot!(&parse("#units incvb=-2q"));
    assert_yaml_snapshot!(&parse("#units incvb=-2g"));
    assert_yaml_snapshot!(&parse("#units incvb=-2p"));
    assert_yaml_snapshot!(&parse("#units vb=g incvb=2"));
    assert_yaml_snapshot!(&parse("#units vb=g incvb=2m"));
    assert_yaml_snapshot!(&parse("#units vb=g incvb=2m"));
    assert_yaml_snapshot!(&parse("#units incvb=-2q-"));
    assert_yaml_snapshot!(&parse("#units incvb=-2m-"));
    // TODO: dms options

    assert_yaml_snapshot!(&parse("#units inch"));
    assert_yaml_snapshot!(&parse("#units inch="));
    assert_yaml_snapshot!(&parse("#units inch=2"));
    assert_yaml_snapshot!(&parse("#units inch=+2"));
    assert_yaml_snapshot!(&parse("#units inch=-2"));
    assert_yaml_snapshot!(&parse("#units inch=-2.3"));
    assert_yaml_snapshot!(&parse("#units inch=-2q"));
    assert_yaml_snapshot!(&parse("#units d=f inch=2"));
    assert_yaml_snapshot!(&parse("#units d=f inch=2m"));
    assert_yaml_snapshot!(&parse("#units s=f inch=2m"));
    assert_yaml_snapshot!(&parse("#units inch=-2q-"));
    assert_yaml_snapshot!(&parse("#units inch=-2f-"));

    assert_yaml_snapshot!(&parse("#units case"));
    assert_yaml_snapshot!(&parse("#units case="));
    assert_yaml_snapshot!(&parse("#units case=l"));
    assert_yaml_snapshot!(&parse("#units case=L"));
    assert_yaml_snapshot!(&parse("#units case=lower"));
    assert_yaml_snapshot!(&parse("#units case=u"));
    assert_yaml_snapshot!(&parse("#units case=U"));
    assert_yaml_snapshot!(&parse("#units case=upper"));
    assert_yaml_snapshot!(&parse("#units case=m"));
    assert_yaml_snapshot!(&parse("#units case=M"));
    assert_yaml_snapshot!(&parse("#units case=Mixed"));
    assert_yaml_snapshot!(&parse("#units case=q"));

    assert_yaml_snapshot!(&parse("#units tape"));
    assert_yaml_snapshot!(&parse("#units tape="));
    assert_yaml_snapshot!(&parse("#units tape=it"));
    assert_yaml_snapshot!(&parse("#units tape=ss"));
    assert_yaml_snapshot!(&parse("#units tape=is"));
    assert_yaml_snapshot!(&parse("#units tape=st"));
    assert_yaml_snapshot!(&parse("#units tape=sq"));
    assert_yaml_snapshot!(&parse("#units tape=itt"));

    assert_yaml_snapshot!(&parse("#units uvh"));
    assert_yaml_snapshot!(&parse("#units uvh="));
    assert_yaml_snapshot!(&parse("#units uvh=0"));
    assert_yaml_snapshot!(&parse("#units uvh=0.0"));
    assert_yaml_snapshot!(&parse("#units uvh=.0"));
    assert_yaml_snapshot!(&parse("#units uvh=1.2"));
    assert_yaml_snapshot!(&parse("#units uvh=-1.3"));
    assert_yaml_snapshot!(&parse("#units uvh=-1.3a"));

    assert_yaml_snapshot!(&parse("#units uvv"));
    assert_yaml_snapshot!(&parse("#units uvv="));
    assert_yaml_snapshot!(&parse("#units uvv=0"));
    assert_yaml_snapshot!(&parse("#units uvv=0.0"));
    assert_yaml_snapshot!(&parse("#units uvv=.0"));
    assert_yaml_snapshot!(&parse("#units uvv=1.2"));
    assert_yaml_snapshot!(&parse("#units uvv=-1.3"));
    assert_yaml_snapshot!(&parse("#units uvv=-1.3a"));

    assert_yaml_snapshot!(&parse("#units uv"));
    assert_yaml_snapshot!(&parse("#units uv="));
    assert_yaml_snapshot!(&parse("#units uv=0"));
    assert_yaml_snapshot!(&parse("#units uv=0.0"));
    assert_yaml_snapshot!(&parse("#units uv=.0"));
    assert_yaml_snapshot!(&parse("#units uv=1.2"));
    assert_yaml_snapshot!(&parse("#units uv=-1.3"));
    assert_yaml_snapshot!(&parse("#units uv=-1.3a"));
    assert_yaml_snapshot!(&parse("#units uv=a-1.3"));
    assert_yaml_snapshot!(&parse("#units uv=--1.3"));

    assert_yaml_snapshot!(&parse("#prefix"));
    assert_yaml_snapshot!(&parse("#prefix="));
    assert_yaml_snapshot!(&parse("#prefix a"));
    assert_yaml_snapshot!(&parse("#prefix\ta"));
    assert_yaml_snapshot!(&parse("#prefix ab!c"));
    assert_yaml_snapshot!(&parse("#prefix1"));
    assert_yaml_snapshot!(&parse("#prefix1 "));
    assert_yaml_snapshot!(&parse("#prefix1 a"));
    assert_yaml_snapshot!(&parse("#prefix1 ab!c"));
    assert_yaml_snapshot!(&parse("#prefix2"));
    assert_yaml_snapshot!(&parse("#prefix2 ab!c"));
    assert_yaml_snapshot!(&parse("#prefix3"));
    assert_yaml_snapshot!(&parse("#prefix3 ab!c"));

    assert_yaml_snapshot!(&parse("#grefix"));

    assert_yaml_snapshot!(&parse("#units ct"));
    assert_yaml_snapshot!(&parse("#Units CT;comment"));

    assert_yaml_snapshot!(&parse("#units rect"));
    assert_yaml_snapshot!(&parse("#units Rect;comment"));
    assert_yaml_snapshot!(&parse("#units Rect="));
    assert_yaml_snapshot!(&parse("#units Rect=3"));
    assert_yaml_snapshot!(&parse("#units Rect=3.8"));
    assert_yaml_snapshot!(&parse("#units Rect=-3.8g"));
    assert_yaml_snapshot!(&parse("#units Rect=+3.8m"));
    assert_yaml_snapshot!(&parse("#units Rect=+3.8q"));
    assert_yaml_snapshot!(&parse("#units Rect=--3.8"));

    assert_yaml_snapshot!(&parse("#units lrud"));
    assert_yaml_snapshot!(&parse("#units Lrud="));
    assert_yaml_snapshot!(&parse("#units Lrud=f"));
    assert_yaml_snapshot!(&parse("#units lrud=F"));
    assert_yaml_snapshot!(&parse("#units Lrud=t"));
    assert_yaml_snapshot!(&parse("#units lrud=T"));
    assert_yaml_snapshot!(&parse("#units Lrud=fb"));
    assert_yaml_snapshot!(&parse("#units lrud=FB"));
    assert_yaml_snapshot!(&parse("#units Lrud=tb"));
    assert_yaml_snapshot!(&parse("#units lrud=TB"));
    assert_yaml_snapshot!(&parse("#units lrud=f:"));
    assert_yaml_snapshot!(&parse("#units lrud=f:lrud"));
    assert_yaml_snapshot!(&parse("#units lrud=f:dlur"));
    assert_yaml_snapshot!(&parse("#units lrud=t:lru"));
    assert_yaml_snapshot!(&parse("#units lrud=t:lrudd"));
    assert_yaml_snapshot!(&parse("#units lrud=tb:lqrud"));
    assert_yaml_snapshot!(&parse("#units lrud=:ldur"));
    assert_yaml_snapshot!(&parse("#units lrud=t:ludr;comment"));
    assert_yaml_snapshot!(&parse("#units lrud=t:ludr#blah"));
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
