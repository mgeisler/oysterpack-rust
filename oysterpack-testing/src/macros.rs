// Copyright 2018 OysterPack Inc.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! provides support for initializing logging for testing purposes

// TODO: add support to configure logger levels
/// Generates a module named `tests`, which provides test support functionality.
/// - configures logging
#[macro_export]
macro_rules! op_tests_mod {
    () => {
        #[cfg(test)]
        pub(crate) mod tests {

            /// Used to track logging initialization
            #[derive(Eq, PartialEq)]
            pub enum LogInitState {
                NotInitialized,
                Initializing,
                Initialized,
            }

            pub static mut _FERN_INITIALIZED: LogInitState = LogInitState::NotInitialized;

            fn init_log() {
                unsafe {
                    if _FERN_INITIALIZED == LogInitState::NotInitialized {
                        _FERN_INITIALIZED = LogInitState::Initializing;
                        if _FERN_INITIALIZED == LogInitState::Initializing {
                            const CARGO_PKG_NAME: &str = env!("CARGO_PKG_NAME");
                            $crate::fern::Dispatch::new()
                                .format(|out, message, record| {
                                    out.finish(format_args!(
                                        "{}[{}][{}][{}:{}] {}",
                                        $crate::chrono::Local::now().format("[%H:%M:%S%.3f]"),
                                        record.level(),
                                        record.target(),
                                        record.file().unwrap(),
                                        record.line().unwrap(),
                                        message
                                    ))
                                }).level($crate::log::LevelFilter::Warn)
                                .level_for(CARGO_PKG_NAME, $crate::log::LevelFilter::Debug)
                                .chain(::std::io::stdout())
                                .apply()
                                .unwrap();
                            _FERN_INITIALIZED = LogInitState::Initialized;
                            info!("logging has been initialized for {}", CARGO_PKG_NAME);
                        }
                    }
                    // There may be a race condition because tests may run in parallel.
                    // Thus, wait until logging has been initialized before running the test.
                    while _FERN_INITIALIZED != LogInitState::Initialized {
                        ::std::thread::yield_now();
                    }
                }
            }

            /// - ensures logging is configured and initialized
            /// - collects test execution time and logs it
            pub fn run_test<F: FnOnce() -> ()>(name: &str, test: F) {
                init_log();
                let before = ::std::time::Instant::now();
                test();
                let after = ::std::time::Instant::now();
                info!(
                    "{}: test run time: {:?}",
                    name,
                    after.duration_since(before)
                );
            }

            #[test]
            fn compiles() {
                run_test("compiles", || info!("it compiles :)"));
            }
        }
    };
}

/// Creates a test function, which executes the specified expression block.
/// Metadata attributes can be specified on the test function, e.g., `#[ignore]`
#[macro_export]
macro_rules! op_test {
    (
        $(#[$outer:meta])*
        $Name:ident $Fn:block
    ) => {
        #[test]
        fn $Name() {
            ::tests::run_test(stringify!($Name), || $Fn);
        }
    };
}

#[cfg(all(test, feature = "tests"))]
mod tests {

    use tests::run_test;

    #[test]
    fn tests_op_test() {
        run_test("tests_op_test", || info!("tests_op_test passed !!!"));
    }

    #[test]
    fn test_op_test_fn() {
        run_test("test_op_test_fn", test_bar);
    }

    fn test_bar() {
        info!("bar passed !!!")
    }

    op_test!(bar {test_bar()});
}
