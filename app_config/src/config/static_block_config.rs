

use serde_derive::Deserialize;



#[derive(Debug, Deserialize, Default)]
pub struct AppConfig {
    pub aptos_node_url: Option<&'static str>,
    pub aptos_faucet_url: Option<&'static str>,
}

pub static STATIC_CONFIG: &AppConfig = {
    &AppConfig {
        aptos_node_url: None,
        aptos_faucet_url: None,
    }
};


#[cfg(test)]
mod test{
    use crate::config::static_block_config::STATIC_CONFIG;

    #[test]
    fn test_static_block(){
        dbg!(STATIC_CONFIG);
    }
}