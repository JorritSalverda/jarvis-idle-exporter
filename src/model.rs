use jarvis_lib::config_client::SetDefaults;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub location: String,
    pub interval_seconds: f64,
    #[serde(default)]
    pub sample_configs: Vec<ConfigSample>,
}

impl SetDefaults for Config {
    fn set_defaults(&mut self) {}
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConfigSample {
    // default jarvis config for sample
    pub sample_name: String,
    pub value_watt: f64,
    pub instance_count: Option<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use jarvis_lib::config_client::{ConfigClient, ConfigClientConfig};

    #[test]
    fn read_config_from_file_returns_deserialized_test_file() {
        let config_client =
            ConfigClient::new(ConfigClientConfig::new("test-config.yaml".to_string()).unwrap());

        let config: Config = config_client.read_config_from_file().unwrap();

        assert_eq!(config.location, "My Home".to_string());
        assert_eq!(config.interval_seconds, 300.0);
        assert_eq!(config.sample_configs.len(), 2);
        assert_eq!(
            config.sample_configs[0].sample_name,
            "AmpliFi AFi-P-HD antenna".to_string()
        );
        assert_eq!(config.sample_configs[0].value_watt, 3.7f64);
        assert_eq!(config.sample_configs[0].instance_count, None);

        assert_eq!(
            config.sample_configs[1].sample_name,
            "TP-Link HS110".to_string()
        );
        assert_eq!(config.sample_configs[1].value_watt, 1.32f64);
        assert_eq!(config.sample_configs[1].instance_count, Some(20));
    }
}
