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

use super::{Event, Eventful, Id, SeverityLevel};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
struct Foo(String);

impl Foo {}

// BOILERPLATE THAT CAN BE GENERATED //
impl Eventful for Foo {
    const EVENT_ID: Id = Id(1);

    const EVENT_SEVERITY_LEVEL: SeverityLevel = SeverityLevel::Info;

    fn new_event(data: Foo) -> Event<Foo> {
        Event::new(data)
    }
}

#[test]
fn foo_event() {
    let foo_event = Foo::new_event(Foo("foo data".into()));
    println!(
        "foo_event: {}",
        serde_json::to_string_pretty(&foo_event).unwrap()
    );
    assert_eq!(foo_event.id(), Foo::EVENT_ID);
    assert_eq!(*foo_event.data(), Foo("foo data".into()));
}
