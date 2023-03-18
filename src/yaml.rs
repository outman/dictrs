use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::path::{PathBuf};
use std::{fmt};


const YAML_CONFIG: &str = ".dictrs.yaml";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub(crate) openai_api_key: String,
    pub(crate) openai_api_org: String,
    pub(crate) openai_api_endpoint: String,
    pub(crate) openai_api_request_form: OpenAIRequestForm,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIRequestForm {
    pub(crate) model: String,
    pub(crate) temperature: f32,
    pub(crate) max_tokens: u32,
    pub(crate) top_p: f32,
    pub(crate) frequency_penalty: f32,
    pub(crate) presence_penalty: f32,
}

#[derive(Debug, Clone)]
pub struct ConfigError;

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("Please check $HOME/{} configure.", YAML_CONFIG))
    }
}

impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

pub fn load_yaml_config(config_path: PathBuf) -> Result<Config, Box<dyn Error>> {
    let mut file = File::open(config_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let config: Config = serde_yaml::from_str(&contents)?;
    return Ok(config);
}

pub fn init_yaml_config() -> Result<PathBuf, Box<dyn Error>> {
    let home_dir = dirs::home_dir();
    if let Some(home) = home_dir {
        let config_file = PathBuf::from(&home).join(YAML_CONFIG);
        if !config_file.exists() {
            if let Ok(mut file) = File::create(&config_file) {
                file.write_all(get_config_template().as_bytes())?;
                return Ok(config_file)
            }
        }
        return Ok(config_file);
    }
    Err(Box::try_from(ConfigError).unwrap())
}

fn get_config_template() -> String {
    return r#"openai_api_key: ""
openai_api_org: ""
openai_api_endpoint: "https://api.openai.com/v1/completions"
openai_api_request_form:
  model: text-davinci-003
  temperature: 0.3
  max_tokens: 100
  top_p: 1.00
  frequency_penalty: 0.0
  presence_penalty: 0.0
"#.to_string();
}