// Copyright 2018 OysterPack Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::command::*;
use chrono::prelude::*;
use crossbeam_channel as channel;
use std::time::SystemTime;
use tokio::{self, prelude::*};

use tests::*;

#[test]
fn command_success_with_no_progress_subscriber() {
    struct Foo;

    impl Future for Foo {
        type Item = SystemTime;
        type Error = ();

        fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
            Ok(Async::Ready(SystemTime::now()))
        }
    }

    run_test(|| {
        let (s, r) = channel::unbounded();

        let foo_cmd = Command::new(Foo)
            .and_then(move |result| {
                s.send(result);
                future::finished(result)
            })
            .map(|ts| {
                info!("{:?}", <DateTime<Utc> as From<SystemTime>>::from(ts));
                ()
            });
        tokio::run(foo_cmd);

        let result = r.try_recv();
        assert!(result.is_some());
        info!("Received result: {:?}", result);
    });
}

#[test]
fn command_success_with_progress_subscriber() {
    struct Foo;

    impl Future for Foo {
        type Item = SystemTime;
        type Error = ();

        fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
            Ok(Async::Ready(SystemTime::now()))
        }
    }

    run_test(|| {
        let (s, r) = channel::unbounded();

        let (progress_sender, progress_receiver) = channel::unbounded();

        let foo_cmd = Builder::new(Foo).progress_subscriber_chan(progress_sender).build();
        let foo_cmd = foo_cmd
            .and_then(move |result| {
                s.send(result);
                future::finished(result)
            })
            .map(|ts| {
                info!("{:?}", <DateTime<Utc> as From<SystemTime>>::from(ts));
                ()
            });
        tokio::run(foo_cmd);

        let result = r.try_recv();
        assert!(result.is_some());
        info!("Received result: {:?}", result);

        let progress_events: Vec<_> = progress_receiver.collect();
        info!("Progress events: {:?}", progress_events);
        assert_eq!(progress_events.len(), 1);
    });
}