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

//! This crate represents the OysterPack core. It follows an expansion / contraction approach.
//! New features will be introduced into the core as new packages, i.e., core expansion. Once features
//! have stabilized, then they will be extracted out into their separate crates, i.e., core contraction.
//!
//!

#![deny(missing_docs, missing_debug_implementations)]
#![doc(html_root_url = "https://docs.rs/oysterpack_core/0.1.0")]

#[macro_use]
extern crate oysterpack_macros;
extern crate oysterpack_app_metadata;
extern crate oysterpack_uid;

#[macro_use]
extern crate log;

#[cfg(test)]
extern crate fern;
#[macro_use]
#[cfg(test)]
extern crate lazy_static;
#[cfg(test)]
extern crate chrono;

#[cfg(test)]
mod tests;
