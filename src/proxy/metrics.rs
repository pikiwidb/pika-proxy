use crate::proxy::proxy::RawProxy;

pub trait Metrics {
    fn start_metrics_reporter(&self);
    fn start_metrics_json(&self);
    fn start_metrics_influxdb(&self);
    fn start_metrics_statsd(&self);
}

impl Metrics for RawProxy {
    fn start_metrics_reporter(&self) {
        todo!()
    }

    fn start_metrics_json(&self) {
        todo!()
    }

    fn start_metrics_influxdb(&self) {
        todo!()
    }

    fn start_metrics_statsd(&self) {
        todo!()
    }
}

mod tests {}
