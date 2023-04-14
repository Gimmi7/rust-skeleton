use crate::config::app_config::RESOURCES_DIR;
use serde_derive::Deserialize;
use std::env;
use std::sync::OnceLock;

#[derive(Debug, Deserialize, Default)]
pub struct AppConfig {
    pub aptos_node_url: Option<&'static str>,
    pub aptos_faucet_url: Option<&'static str>,
}

static CELL: OnceLock<AppConfig> = OnceLock::new();

pub fn get_app_config() -> &'static AppConfig {
    println!("run get_app_config");
    let value = CELL
        .get_or_try_init(|| -> Result<AppConfig, _> {
            let env = env::var("ENV").unwrap_or(String::from("test"));
            let env_path = format!("application-{env}.yml");
            if let Some(env_file) = RESOURCES_DIR.get_file(env_path) {
                return if let Some(env_yaml) = env_file.contents_utf8() {
                    serde_yaml::from_str::<AppConfig>(env_yaml)
                } else {
                    Ok(AppConfig::default())
                };
            } else {
                panic!("env {env} not supported")
            }
        })
        .unwrap();
    value
}

#[cfg(test)]
mod test {
    use crate::config::once_config::get_app_config;

    #[test]
    fn test_std_once() {
        let config = get_app_config();
        println!("{}", config.aptos_faucet_url.unwrap());
    }
}
