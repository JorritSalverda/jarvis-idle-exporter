use crate::model::Config;
use jarvis_lib::measurement_client::MeasurementClient;
use jarvis_lib::model::{EntityType, Measurement, MetricType, Sample, SampleType};

use chrono::Utc;
use log::info;
use std::error::Error;
use uuid::Uuid;

pub struct IdleClient {}

impl MeasurementClient<Config> for IdleClient {
    fn get_measurement(
        &self,
        config: Config,
        _last_measurement: Option<Measurement>,
    ) -> Result<Measurement, Box<dyn Error>> {
        info!("Writing measurement from idle config...");

        let mut measurement = Measurement {
            id: Uuid::new_v4().to_string(),
            source: String::from("jarvis-idle-exporter"),
            location: config.location.clone(),
            samples: Vec::new(),
            measured_at_time: Utc::now(),
        };

        for sample_config in config.sample_configs {
            measurement.samples.push(Sample {
                entity_type: EntityType::Device,
                entity_name: "jarvis-idle-exporter device".into(),
                sample_type: SampleType::ElectricityConsumption,
                sample_name: sample_config.sample_name.clone(),
                metric_type: MetricType::Counter,
                value: sample_config.value_watt,
            });
            measurement.samples.push(Sample {
                entity_type: EntityType::Device,
                entity_name: "jarvis-idle-exporter device".into(),
                sample_type: SampleType::ElectricityConsumption,
                sample_name: sample_config.sample_name,
                metric_type: MetricType::Counter,
                value: sample_config.value_watt * config.interval_seconds,
            });
        }

        Ok(measurement)
    }
}

impl IdleClient {
    pub fn new() -> Self {
        Self {}
    }
}
