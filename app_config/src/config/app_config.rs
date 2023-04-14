use std::env;


use include_dir::{include_dir, Dir};
use once_cell::sync::Lazy;
use serde_derive::Deserialize;

pub static RESOURCES_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/src/resources");

#[derive(Debug, Deserialize, Default)]
pub struct AppConfig {
    pub aptos_node_url: Option<&'static str>,
    pub aptos_faucet_url: Option<&'static str>,
}

pub static APP_CONFIG: Lazy<AppConfig> = Lazy::new(|| {
    let env = env::var("ENV").unwrap_or(String::from("test"));
    let env_path = format!("application-{env}.yml");
    if let Some(env_file) = RESOURCES_DIR.get_file(env_path) {
        return if let Some(env_yaml) = env_file.contents_utf8() {
            serde_yaml::from_str::<AppConfig>(env_yaml).unwrap()
        } else {
            AppConfig::default()
        };
    } else {
        panic!("env {env} not supported")
    }
});


#[cfg(test)]
mod test {
    use crate::config::app_config::APP_CONFIG;

    #[test]
    fn test_once_cell() {
        dbg!(&APP_CONFIG);
        println!("{}", &APP_CONFIG.aptos_faucet_url.unwrap());
    }
}