// pulse - bitdrift's observability proxy
// Copyright Bitdrift, Inc. All rights reserved.
//
// Use of this source code is governed by a source available license that can be found in the
// LICENSE file or at:
// https://polyformproject.org/wp-content/uploads/2020/06/PolyForm-Shield-1.0.0.txt

use crate::pipeline::metric_cache::MetricKey;
use crate::protos::metric::{
  default_timestamp,
  CounterType,
  DownstreamId,
  Metric,
  MetricSource,
  MetricType,
  MetricValue,
  ParsedMetric,
};
use crate::reservoir_timer::ReservoirTimer;
use ahash::AHashMap;
use bd_log::warn_every;
use std::collections::hash_map::Entry;
use std::time::Instant;
use time::ext::NumericalDuration;

//
// PreBufferMetric
//

enum PreBufferMetric {
  Counter(f64),
  Gauge(f64),
  Timer(ReservoirTimer),
}

//
// PreBuffer
//

// This is a simple aggregation system designed for pre-buffering statsd metrics when enabled via
// the pre_buffer_window setting. See the documentation of that setting for more information.
// This will only realistically work for statsd but could be expanded to other protocols in the
// future if needed.
#[derive(Default)]
pub struct PreBuffer {
  metrics: AHashMap<MetricKey, PreBufferMetric>,
}

impl PreBuffer {
  pub fn buffer(&mut self, metrics: Vec<ParsedMetric>) {
    for metric in metrics {
      let (metric_id, sample_rate, _, value) = metric.into_metric().into_parts();
      let mtype = metric_id.mtype();
      // TODO(mattklein123): Use hashbrown to avoid creating the metric key unless we actually need
      // it.
      let metric = match self.metrics.entry(MetricKey::new(&metric_id)) {
        Entry::Occupied(entry) => entry.into_mut(),
        Entry::Vacant(entry) => {
          let new_metric = match mtype {
            Some(MetricType::Counter(CounterType::Delta)) => PreBufferMetric::Counter(0.0),
            Some(MetricType::Gauge | MetricType::DeltaGauge | MetricType::DirectGauge) => {
              PreBufferMetric::Gauge(0.0)
            },
            // TODO(mattklein123): Potentially make the reservoir size configurable.
            Some(MetricType::Timer) => PreBufferMetric::Timer(ReservoirTimer::new(10)),
            _ => {
              warn_every!(
                15.seconds(),
                "Unsupported pre-buffer metric type {:?} for metric {}",
                mtype,
                metric_id
              );
              continue;
            },
          };
          entry.insert(new_metric)
        },
      };

      match (metric, mtype) {
        (
          PreBufferMetric::Counter(ref mut counter),
          Some(MetricType::Counter(CounterType::Delta)),
        ) => {
          *counter += value.to_simple() * (1.0 / sample_rate.unwrap_or(1.0));
        },
        (
          PreBufferMetric::Gauge(ref mut gauge),
          Some(MetricType::Gauge | MetricType::DirectGauge),
        ) => {
          *gauge = value.to_simple();
        },
        (PreBufferMetric::Gauge(ref mut gauge), Some(MetricType::DeltaGauge)) => {
          *gauge += value.to_simple();
        },
        (PreBufferMetric::Timer(timer), Some(MetricType::Timer)) => {
          timer.aggregate(value.to_simple(), sample_rate.unwrap_or(1.0));
        },
        _ => {
          warn_every!(15.seconds(), "Pre-buffer metric changed type {:?}", mtype);
        },
      }
    }
  }

  pub fn flush(self, downstream_id: &DownstreamId) -> Vec<ParsedMetric> {
    let now_instant = Instant::now();
    let now_unix = default_timestamp();

    let mut ret = Vec::new();
    for (id, metric) in self.metrics {
      match metric {
        PreBufferMetric::Counter(counter) => {
          ret.push(ParsedMetric::new(
            Metric::new(
              id.to_metric_id(),
              None,
              now_unix,
              MetricValue::Simple(counter),
            ),
            MetricSource::Aggregation { prom_source: false },
            now_instant,
            downstream_id.clone(),
          ));
        },
        PreBufferMetric::Gauge(gauge) => {
          ret.push(ParsedMetric::new(
            Metric::new(
              id.to_metric_id(),
              None,
              now_unix,
              MetricValue::Simple(gauge),
            ),
            MetricSource::Aggregation { prom_source: false },
            now_instant,
            downstream_id.clone(),
          ));
        },
        PreBufferMetric::Timer(mut timer) => {
          let (reservoir, count) = timer.drain();
          let sample_rate = reservoir.len() as f64 / count;
          // TODO(mattklein123): Create new intermediate type which carries along all of the
          // samples in a single wrapper metric.
          for value in reservoir {
            ret.push(ParsedMetric::new(
              Metric::new(
                id.to_metric_id(),
                Some(sample_rate),
                now_unix,
                MetricValue::Simple(value),
              ),
              MetricSource::Aggregation { prom_source: false },
              now_instant,
              downstream_id.clone(),
            ));
          }
        },
      }
    }
    ret
  }
}
