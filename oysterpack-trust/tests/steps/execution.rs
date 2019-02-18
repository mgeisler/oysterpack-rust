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

use futures::{prelude::*, task::SpawnExt};
use oysterpack_trust::concurrent::execution::{self, *};
use std::{num::NonZeroUsize, panic, thread, time::Duration};

steps!(TestContext => {

    given regex "01D3W3GDYVS4P2SR0SECVT0JJT-1" |world, _matches, _step| {
        world.init_with_new_executor(10, false);
    };

    when regex "01D3W3GDYVS4P2SR0SECVT0JJT-2" |world, _matches, _step| {
        check_exeutor_thread_pool_size(world, 10);
    };

    then regex "01D3W3GDYVS4P2SR0SECVT0JJT-3" |world, _matches, _step| {
        check_total_threads_count_inc(world, 10);
    };

    given regex "01D3Y1CYCKZHY675FKEPPX4JE4-1" |world, _matches, _step| {
        world.init_with_new_executor(1, false);
    };

    when regex "01D3Y1CYCKZHY675FKEPPX4JE4-2" |world, _matches, _step| {
        spawn_tasks(world, 10, 0);
        await_tasks_completed(world);
    };

    then regex "01D3Y1CYCKZHY675FKEPPX4JE4-3" |world, _matches, _step| {
        check_spawned_task_count(world, 10);
    };

    then regex "01D3Y1CYCKZHY675FKEPPX4JE4-4" |world, _matches, _step| {
        check_completed_task_count(world, 10);
    };

    then regex "01D3Y1CYCKZHY675FKEPPX4JE4-5" |world, _matches, _step| {
        check_active_task_count(world, 0);
    };

    given regex "01D3Y1D8SJZ8JWPGJKFK4BYHP0-1" |world, _matches, _step| {
        world.init_with_new_executor(1, true);
    };

    when regex "01D3Y1D8SJZ8JWPGJKFK4BYHP0-2" |world, _matches, _step| {
        spawn_tasks(world, 5, 5);
        await_tasks_completed_while_gt(world, 0);
    };

    then regex "01D3Y1D8SJZ8JWPGJKFK4BYHP0-3" |world, _matches, _step| {
        check_spawned_task_count(world, 10);
    };

    then regex "01D3Y1D8SJZ8JWPGJKFK4BYHP0-4" |world, _matches, _step| {
        check_completed_task_count(world, 10);
    };

    then regex "01D3Y1D8SJZ8JWPGJKFK4BYHP0-5" |world, _matches, _step| {
        check_active_task_count(world, 0);
    };

    then regex "01D3Y1D8SJZ8JWPGJKFK4BYHP0-6" |world, _matches, _step| {
        check_panicked_task_count(world, 5);
    };

    given regex "01D3YW91CYQRB0XVAKF580WX04-1" |world, _matches, _step| {
        world.init();
        spawn_tasks(world, 0, num_cpus::get() * 2);
        await_tasks_completed_while_gt(world, 0);
    };

    when regex "01D3YW91CYQRB0XVAKF580WX04-2" |world, _matches, _step| {
        spawn_tasks(world, 10, 0);
    };

    then regex "01D3YW91CYQRB0XVAKF580WX04-3" |world, _matches, _step| {
        await_tasks_completed_while_gt(world, 0);
        check_completed_task_count(world, (10 + (num_cpus::get() * 2)) as u64);
    };

    then regex "01D3YW91CYQRB0XVAKF580WX04-4" |world, _matches, _step| {
        check_panicked_task_count(world, (num_cpus::get() * 2) as u64);
    };

});

fn run_tasks(
    world: &mut TestContext,
    success_count: usize,
    panic_count: usize,
    catch_unwind: bool,
) {
    for _ in 0..success_count {
        world.executor.run(async {});
    }

    for i in 0..panic_count {
        let mut executor = world.executor.clone();
        let future = async move { panic!("BOOM #{} !!!", i) };
        if catch_unwind {
            if executor.run(future.catch_unwind()).is_err() {
                eprintln!("run_tasks(): task panicked");
            }
        } else {
            if panic::catch_unwind(move || executor.run(future)).is_err() {
                eprintln!("run_tasks(): task panicked");
            }
        }
    }
}

fn spawn_tasks(world: &mut TestContext, success_count: usize, panic_count: usize) {
    for _ in 0..success_count {
        world.executor.spawn(async {}).unwrap();
    }

    for i in 0..panic_count {
        let future = async move { panic!("BOOM #{} !!!", i) };
        world.executor.spawn(future).unwrap();
    }
}

fn check_completed_task_count(world: &mut TestContext, expected_inc: u64) {
    assert_eq!(
        world.executor.completed_task_count(),
        world.executor_completed_task_count + expected_inc,
        "check_completed_task_count failed"
    );
}

fn check_spawned_task_count(world: &mut TestContext, expected_inc: u64) {
    assert_eq!(
        world.executor.spawned_task_count(),
        world.executor_spawned_task_count + expected_inc,
        "check_spawned_task_count failed"
    );
}

fn check_active_task_count(world: &mut TestContext, expected: u64) {
    assert_eq!(
        world.executor.active_task_count(),
        expected,
        "check_active_task_count failed"
    );
}

fn check_panicked_task_count(world: &mut TestContext, expected_inc: u64) {
    assert_eq!(
        world
            .executor
            .panicked_task_count()
            .expect("Executor does not track panicked tasks"),
        world
            .executor_panicked_task_count
            .expect("Executor does not track panicked tasks")
            + expected_inc,
        "check_panicked_task_count failed"
    );
}

fn await_tasks_completed(world: &mut TestContext) {
    while world.executor.active_task_count() > 0 {
        println!(
            "await_tasks_completed(): {}",
            world.executor.active_task_count()
        );
        thread::yield_now();
    }
}

fn await_tasks_completed_while_gt(world: &mut TestContext, count: u64) {
    while world.executor.active_task_count() > count {
        println!(
            "await_tasks_completed_while_gt(): {}",
            world.executor.active_task_count()
        );
        thread::yield_now();
    }
}

fn check_threads_started_inc(world: &mut TestContext, expected_inc: u64) {
    println!("total_threads_started = {}", execution::total_threads());
    assert_eq!(
        execution::total_threads(),
        world.total_threads + expected_inc
    );
}

fn check_exeutor_thread_pool_size(world: &mut TestContext, expected_count: u64) {
    for _ in 0..10 {
        if world.executor.thread_pool_size() < expected_count {
            println!(
                "waiting for thread pool to initialize - size = {}",
                world.executor.thread_pool_size()
            );
            thread::sleep(Duration::from_millis(1));
        }
    }
    assert_eq!(world.executor.thread_pool_size(), expected_count);
}

fn check_total_threads_count_inc(world: &mut TestContext, expected_inc: u64) {
    let expected_count = world.total_threads + expected_inc;
    for _ in 0..10 {
        if total_threads() < expected_count {
            println!(
                "waiting for thread pool to initialize - size = {}",
                world.executor.thread_pool_size()
            );
            thread::sleep(Duration::from_millis(1));
        }
    }
    assert_eq!(total_threads(), expected_count);
}

fn check_thread_pool_size_unchanged(world: &mut TestContext) {
    assert_eq!(
        world.executor.thread_pool_size(),
        world.executor_thread_pool_size
    );
}

pub struct TestContext {
    pub executor: Executor,
    pub executor_spawned_task_count: u64,
    pub executor_completed_task_count: u64,
    pub executor_thread_pool_size: u64,
    pub executor_panicked_task_count: Option<u64>,
    pub total_threads: u64,
}

impl TestContext {
    pub fn init(&mut self) {
        self.executor = execution::global_executor();
        self.gather_metrics();
    }

    pub fn init_with_new_executor(&mut self, thread_pool_size: usize, catch_unwind: bool) {
        self.executor = ExecutorBuilder::new(ExecutorId::generate())
            .set_pool_size(NonZeroUsize::new(thread_pool_size).unwrap())
            .set_catch_unwind(catch_unwind)
            .register()
            .unwrap();
        self.gather_metrics();
    }

    pub fn gather_metrics(&mut self) {
        self.executor_spawned_task_count = self.executor.spawned_task_count();
        self.executor_completed_task_count = self.executor.completed_task_count();
        self.executor_thread_pool_size = self.executor.thread_pool_size();
        self.executor_panicked_task_count = self.executor.panicked_task_count();
        self.total_threads = total_threads();
    }
}

impl Default for TestContext {
    fn default() -> Self {
        Self {
            executor: execution::global_executor(),
            executor_spawned_task_count: 0,
            executor_completed_task_count: 0,
            executor_thread_pool_size: 0,
            total_threads: 0,
            executor_panicked_task_count: None,
        }
    }
}
