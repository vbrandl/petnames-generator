use anyhow::Result;
use lazy_static::lazy_static;
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder, PrometheusHandle};

lazy_static! {
    pub static ref PROMETHEUS_HANDLE: PrometheusHandle =
        setup_metrics_recorder().expect("cannot setup metrics recorder");
}

fn setup_metrics_recorder() -> Result<PrometheusHandle> {
    const EXPONENTIAL_SECONDS: &[f64] = &[
        0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
    ];

    Ok(PrometheusBuilder::new()
        .set_buckets_for_metric(
            Matcher::Full("http_requests_duration_seconds".to_string()),
            EXPONENTIAL_SECONDS,
        )?
        .install_recorder()?)
}
