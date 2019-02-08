#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;

use checkout_for_file::parser;

#[derive(Deserialize)]
struct Test {
    test: String,
}

#[test]
fn parse_json_test() {
    let json_string = String::from(
        r#"
    {"test": "value"}
    "#,
    );
    let result: Test = parser::parse_json(&json_string);
    println!("result: {:?}", result.test);
    assert_eq!(result.test, "value");
}
