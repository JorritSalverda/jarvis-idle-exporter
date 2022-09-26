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
        last_measurement: Option<Measurement>,
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
            let instance_count = if let Some(instance_count) = sample_config.instance_count {
                instance_count as f64
            } else {
                1_f64
            };

            let counter_increment =
                sample_config.value_watt * config.interval_seconds * instance_count;

            // ensure counter increases from previous value
            let counter_value = if let Some(last_measurement) = last_measurement.as_ref() {
                if let Some(sample) = last_measurement
                    .samples
                    .iter()
                    .find(|s| s.sample_name == sample_config.sample_name)
                {
                    sample.value + counter_increment
                } else {
                    counter_increment
                }
            } else {
                counter_increment
            };

            // store as gauge for timeline graphs
            measurement.samples.push(Sample {
                entity_type: EntityType::Device,
                entity_name: "jarvis-idle-exporter".into(),
                sample_type: SampleType::ElectricityConsumption,
                sample_name: sample_config.sample_name.clone(),
                metric_type: MetricType::Gauge,
                value: sample_config.value_watt * instance_count,
            });

            // store as counter for totals
            measurement.samples.push(Sample {
                entity_type: EntityType::Device,
                entity_name: "jarvis-idle-exporter".into(),
                sample_type: SampleType::ElectricityConsumption,
                sample_name: sample_config.sample_name,
                metric_type: MetricType::Counter,
                value: counter_value,
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
