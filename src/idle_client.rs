use crate::model::Config;
use jarvis_lib::measurement_client::MeasurementClient;
use jarvis_lib::model::{EntityType, Measurement, MetricType, Sample, SampleType};

use chrono::Utc;
use std::error::Error;
use tracing::info;
use uuid::Uuid;

pub struct IdleClient {}

impl MeasurementClient<Config> for IdleClient {
    fn get_measurements(
        &self,
        config: Config,
        last_measurements: Option<Vec<Measurement>>,
    ) -> Result<Vec<Measurement>, Box<dyn Error>> {
        info!("Writing measurements from idle config...");

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

            // get previous counter value to have a continuously increasing counter
            let last_counter_value: f64 =
                if let Some(last_measurements) = last_measurements.as_ref() {
                    if !last_measurements.is_empty() {
                        if let Some(sample) = last_measurements[last_measurements.len() - 1]
                            .samples
                            .iter()
                            .find(|s| {
                                s.sample_name == sample_config.sample_name
                                    && s.metric_type == MetricType::Counter
                            })
                        {
                            sample.value
                        } else {
                            0_f64
                        }
                    } else {
                        0_f64
                    }
                } else {
                    0_f64
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
                value: last_counter_value
                    + sample_config.value_watt * instance_count * config.interval_seconds,
            });
        }

        Ok(vec![measurement])
    }
}

impl IdleClient {
    pub fn new() -> Self {
        Self {}
    }
}
