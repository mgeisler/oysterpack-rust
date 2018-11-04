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

//! OysterPack error standards:
//!
//! - Errors are assigned a unique ErrorId
//! - Errors are assigned a severity
//! - Errors are documented
//! - Errors have context
//! - Errors are timestamped
//! - Errors are tracked against crates in 2 ways :
//!   1. the binary crate - within which app the error occurred
//!   2. the library crate - the error was produced by which library

use chrono::{DateTime, Utc};
use devops::SourceCodeLocation;
use failure::Fail;
use oysterpack_uid::TypedULID;
use std::{collections::HashSet, fmt, sync::Arc};

#[macro_use]
mod macros;

#[allow(warnings)]
#[cfg(test)]
mod tests;

/// Decorates the failure cause with an ErrorId.
/// - errors are cloneable which enables errors to be sent on multiple channels, e.g., async error logging and tracking
#[derive(Debug, Clone)]
pub struct Error {
    id: ErrorId,
    timestamp: DateTime<Utc>,
    loc: SourceCodeLocation,
    instance: InstanceId,
    failure: ArcFailure,
}

impl Fail for Error {
    /// The failure that caused the Error is returned, i.e., the failure that is mapped to the ErrorId.
    /// Thus, this will always return Some(&Fail).
    fn cause(&self) -> Option<&dyn Fail> {
        Some(self.failure.failure())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ErrorId({})({}): {}", *self.id, self.loc, self.failure())
    }
}

impl Error {
    /// Error constructor.
    pub fn new(id: ErrorId, failure: impl Fail, loc: SourceCodeLocation) -> Error {
        Error {
            id,
            instance: InstanceId::generate(),
            failure: ArcFailure::new(failure),
            timestamp: Utc::now(),
            loc,
        }
    }

    /// ErrorId getter
    pub fn id(&self) -> ErrorId {
        self.id
    }

    /// Returns the error cause
    pub fn failure(&self) -> &dyn Fail {
        &self.failure
    }

    /// When the error occurred.
    pub fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }

    pub fn loc(&self) -> &SourceCodeLocation {
        &self.loc
    }

    /// Returns the chain of ErrorId(s) from all chained failures that themselves are an Error.
    /// The first ErrorId will be this Error's ErrorId.
    pub fn error_ids(&self) -> Vec<ErrorId> {
        fn collect_error_ids(error_ids: &mut Vec<ErrorId>, failure: &dyn Fail) {
            if let Some(cause) = failure.cause() {
                if let Some(e) = cause.downcast_ref::<Error>() {
                    error_ids.push(e.id());
                }
                collect_error_ids(error_ids, cause);
            }
        }

        let mut err_ds = vec![self.id];
        collect_error_ids(&mut err_ds, self);
        err_ds
    }

    /// Returns all distinct ErrorId(s) that are referenced by the error chain.
    /// It includes this Error's ErrorId. Thus, the returned HashSet will never be empty.
    pub fn distinct_error_ids(&self) -> HashSet<ErrorId> {
        fn collect_error_ids(error_ids: &mut HashSet<ErrorId>, failure: &dyn Fail) {
            for cause in failure.iter_chain() {
                if let Some(e) = cause.downcast_ref::<Error>() {
                    error_ids.insert(e.id());
                }
                collect_error_ids(error_ids, cause);
            }
        }

        let mut error_ids = HashSet::new();
        collect_error_ids(&mut error_ids, self);
        error_ids
    }
}

op_newtype! {
    /// Unique Error ID
    #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
    pub ErrorId(pub u128)
}

/// Represents an Error instance
#[derive(Debug)]
pub struct Instance;

/// Error instance ULID
/// This enables a specific error to be searched for withing another context, e.g., searching log events.
pub type InstanceId = TypedULID<Instance>;

/// ArcFailure is a thread-safe reference-counting pointer to an instance of Fail.
/// It provides shared ownership to a Fail instance.
///
/// Invoking clone on ArcFailure produces a new pointer to the same Fail instance in the heap.
/// When the last ArcFailure pointer to a given Fail instance is destroyed, the pointed-to Fail
/// instance is also destroyed.
#[derive(Clone, Debug)]
pub struct ArcFailure(Arc<Fail>);

impl ArcFailure {
    /// Wraps the provided error into an `ArcFailure`.
    /// If the failure already is an ArcFailure, then it will be cloned and returned.
    pub fn new(failure: impl Fail) -> ArcFailure {
        let failure = ArcFailure(Arc::new(failure));
        {
            if let Some(failure) = failure.0.downcast_ref::<ArcFailure>() {
                return failure.clone();
            }
        }
        failure
    }

    /// Returns a reference to the underlying failure
    pub fn failure(&self) -> &dyn Fail {
        &*self.0
    }
}

impl Fail for ArcFailure {
    fn cause(&self) -> Option<&dyn Fail> {
        self.0.cause()
    }
}

impl fmt::Display for ArcFailure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}
