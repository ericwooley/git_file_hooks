use std::collections::HashMap;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Command {
    pub patterns: Vec<String>,
    pub commands: Vec<String>,
}
#[derive(Debug)]
pub struct Config {
    map: HashMap<String, Vec<Command>>,
}

impl Config {
    pub fn new(raw_yaml: &String) -> Config {
        Config {
            map: deserialize_config(raw_yaml),
        }
    }
    pub fn get_commands(&self, hook: &String) -> Option<&Vec<Command>> {
        self.map.get(hook)
    }
}

fn deserialize_config(raw: &String) -> HashMap<String, Vec<Command>> {
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
checkout:
  - patterns:
      - "test"
    commands:
      - "echo 'works'""#,
        );
        println!("Testing Valid config, {}", config);
        let parsed_config = deserialize_config(&config);
        assert!(parsed_config.get("checkout").unwrap().len() == 1);
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
        let config = Config::new(&String::from(
            r#"
checkout:
  - patterns:
      - "test"
    commands:
      - "echo 'works'""#,
        ));
        assert_eq!(
            config
                .get_commands(&String::from("checkout"))
                .unwrap()
                .len(),
            1
        );
    }

}
