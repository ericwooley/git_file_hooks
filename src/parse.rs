use std::collections::HashMap;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Command {
    pub patterns: Vec<String>,
    pub commands: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub hooks: HashMap<String, Vec<Command>>,
}

pub fn deserialize_config(raw: &String) -> Config {
    serde_yaml::from_str(&raw).unwrap_or_else(|err| {
        eprintln!("Could not parse input:\n {}", &raw.as_str());
        eprintln!("Error: {:?}", err);
        panic!("Error validating config file")
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn valid_deserialize_config() {
        let config = String::from(
            r#"
hooks:
  checkout:
    - patterns:
        - "test"
      commands:
        - "echo 'works'""#,
        );
        println!("Testing Valid config, {}", config);
        let parsed_config = deserialize_config(&config);
        assert!(parsed_config.hooks.get("checkout").unwrap().len() == 1);
    }
    #[test]
    #[should_panic]
    fn empty_deserialize_config() {
        let config = String::from(r#""#);
        println!("Testing Valid config, {}", config);
        deserialize_config(&config);
    }

    #[test]
    #[should_panic]
    fn invalid_deserialize_config() {
        let config = String::from(
            r#"
hooks:
checkout:
    patterns:
        - "test"
      commands:
        - "echo 'works'""#,
        );
        println!("Testing Valid config, {}", config);
        deserialize_config(&config);
    }
    #[test]
    fn get_checkout_config() {
        let config = deserialize_config(&String::from(
            r#"
hooks:
  checkout:
    - patterns:
        - "test"
      commands:
        - "echo 'works'""#,
        ));
        assert_eq!(
            config.hooks.get(&String::from("checkout")).unwrap().len(),
            1
        );
    }

}
