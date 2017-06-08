// test-json/src/main.rs
extern crate json;

fn main() {
    let mut doc = json::parse(r#"
    {
        "code": 200,
        "success": true,
        "payload": {
            "features": [
                "awesome",
                "easyAPI",
                "lowLearningCurve"
            ]
        }
    }
    "#).expect("parse failed");
    
    let code = doc["code"].as_u32().unwrap_or(0);
    let success = doc["success"].as_bool().unwrap_or(false);
    let features = &mut doc["payload"]["features"];
    features.push("cargo!").expect("couldn't push");
    for v in features.members() {
        println!("{}",v.as_str().unwrap());
    }
    
    assert_eq!(code, 200);
    assert_eq!(success, true);
    
    //~ println!("debug {:?}",doc);
    //~ println!("display {}",doc);
}
