/*
 * Copyright 2019 OysterPack Inc.
 *
 *    Licensed under the Apache License, Version 2.0 (the "License");
 *    you may not use this file except in compliance with the License.
 *    You may obtain a copy of the License at
 *
 *        http://www.apache.org/licenses/LICENSE-2.0
 *
 *    Unless required by applicable law or agreed to in writing, software
 *    distributed under the License is distributed on an "AS IS" BASIS,
 *    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *    See the License for the specific language governing permissions and
 *    limitations under the License.
 */

use cucumber_rust::*;

mod steps;

use oysterpack_trust::metrics;
use std::{collections::HashMap, sync::Arc, thread};

#[derive(Default)]
pub struct TestContext {
    pub metric_id: Option<metrics::MetricId>,
    pub metrics: Option<HashMap<metrics::MetricId, Arc<dyn prometheus::core::Collector>>>,
    pub command_sender: Option<crossbeam::Sender<Command>>
}

impl TestContext {
    fn init(&mut self) {
        self.metric_id = None;
        self.metrics = None;
        self.command_sender = None
    }

    fn spawn_command_handlers(&mut self)  {
        let (tx, rx) = crossbeam::channel::bounded(0);
        self.command_sender = Some(tx.clone());
        for _ in 0..2 {
            let rx = rx.clone();
            thread::spawn(move || {
                for command in rx {
                    match command {
                        Command::RegisterMetrics(reply_chan) => {
                            let metric_id = metrics::MetricId::generate();
                            metrics::registry().register_counter(metric_id, "counter", None).unwrap();
                            reply_chan.send(metric_id).unwrap();
                        },
                        Command::CheckMetric(metric_id, reply_chan) => {
                            if metrics::registry().gather_metrics_by_name(&[metric_id.name().as_str()]).is_empty() {
                                reply_chan.send(Err("no metrics gathered")).unwrap();
                                break;
                            }
                            if metrics::registry().descs_for_metric_id(metric_id).is_empty() {
                                reply_chan.send(Err("no Desc(s) found")).unwrap();
                                break;
                            }
                            if metrics::registry().collectors_for_metric_id(metric_id).is_empty() {
                                reply_chan.send(Err("no Collector(s) found")).unwrap();
                                break;
                            }

                            reply_chan.send(Ok(())).unwrap();
                        }
                        Command::Stop => break
                    }
                }
            });
        }
    }

    fn stop_command_handlers(&mut self) {
        loop {
            for sender in self.command_sender.iter() {
                if sender.send(Command::Stop).is_err() {
                    return
                }
            }
        }
    }
}



pub enum Command {
    RegisterMetrics(crossbeam::channel::Sender<metrics::MetricId>),
    CheckMetric(metrics::MetricId, crossbeam::channel::Sender<Result<(),&'static str>>),
    Stop
}


impl cucumber_rust::World for TestContext {}

cucumber! {
    features: "./features/metrics",
    world: crate::TestContext,
    steps: &[
        steps::metrics::steps
    ]
}
