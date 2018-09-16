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

// TODO: Document crate
//! # OysterPack XXX

// #![deny(missing_docs, missing_debug_implementations, warnings)]
#![deny(missing_docs, missing_debug_implementations)]
// TODO: update url
#![doc(html_root_url = "https://docs.rs/oysterpack_lib_template/0.1.0")]

extern crate built;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
extern crate chrono;

pub mod build;

#[cfg(test)]
mod tests;