// Copyright (c) 2024 Contributors to the Eclipse Foundation
//
// See the NOTICE file(s) distributed with this work for additional
// information regarding copyright ownership.
//
// This program and the accompanying materials are made available under the
// terms of the Apache Software License 2.0 which is available at
// https://www.apache.org/licenses/LICENSE-2.0, or the MIT license
// which is available at https://opensource.org/licenses/MIT.
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use iceoryx2_bb_container::semantic_string::SemanticString;
use iceoryx2_bb_posix::config::*;
use iceoryx2_bb_posix::file::{File, FileBuilder};
use iceoryx2_bb_posix::file_descriptor::FileDescriptorManagement;
use iceoryx2_bb_posix::shared_memory::Permission;
use iceoryx2_bb_posix::unix_datagram_socket::CreationMode;
use iceoryx2_bb_posix::{process_state::*, unique_system_id::UniqueSystemId};
use iceoryx2_bb_system_types::{file_name::FileName, file_path::FilePath};
use iceoryx2_bb_testing::assert_that;

fn generate_file_path() -> FilePath {
    let mut file = FileName::new(b"process_state_tests").unwrap();
    file.push_bytes(
        UniqueSystemId::new()
            .unwrap()
            .value()
            .to_string()
            .as_bytes(),
    )
    .unwrap();

    FilePath::from_path_and_file(&test_directory(), &file).unwrap()
}

#[test]
pub fn process_state_guard_can_be_created() {
    let path = generate_file_path();

    let guard = ProcessGuard::new(&path).unwrap();

    assert_that!(*guard.path(), eq path);
    assert_that!(File::does_exist(&path).unwrap(), eq true);
}

#[test]
pub fn process_state_guard_removes_file_when_dropped() {
    let path = generate_file_path();

    let guard = ProcessGuard::new(&path).unwrap();
    assert_that!(File::does_exist(&path).unwrap(), eq true);
    drop(guard);
    assert_that!(File::does_exist(&path).unwrap(), eq false);
}

#[test]
pub fn process_state_guard_cannot_use_already_existing_file() {
    let path = generate_file_path();

    let file = FileBuilder::new(&path)
        .creation_mode(CreationMode::PurgeAndCreate)
        .create()
        .unwrap();

    let guard = ProcessGuard::new(&path);
    assert_that!(guard.is_err(), eq true);
    assert_that!(guard.err().unwrap(), eq ProcessGuardCreateError::AlreadyExists);

    file.remove_self().unwrap();
}

#[test]
pub fn process_state_guard_can_remove_already_existing_file() {
    let path = generate_file_path();

    FileBuilder::new(&path)
        .creation_mode(CreationMode::PurgeAndCreate)
        .create()
        .unwrap();

    let guard = unsafe { ProcessGuard::remove(&path) };
    assert_that!(guard.is_ok(), eq true);
    assert_that!(guard.ok().unwrap(), eq true);
}

// the lock detection does work on some OS only in the inter process context.
// In the process local context the lock is not detected when the fcntl GETLK call is originating
// from the same thread os the fcntl SETLK call. If it is called from a different thread GETLK
// blocks despite it should be non-blocking.
#[test]
#[cfg(not(any(target_os = "linux", target_os = "freebsd", target_os = "macos")))]
pub fn process_state_watcher_detects_alive_state_from_existing_process() {
    let path = generate_file_path();

    let guard = ProcessGuard::new(&path).unwrap();
    let watcher = ProcessMonitor::new(&path).unwrap();

    assert_that!(watcher.state().unwrap(), eq ProcessState::Alive);
    drop(guard);
    assert_that!(watcher.state().unwrap(), eq ProcessState::DoesNotExist);
}

#[test]
pub fn process_state_watcher_detects_dead_state() {
    let path = generate_file_path();

    let file = FileBuilder::new(&path)
        .creation_mode(CreationMode::PurgeAndCreate)
        .create()
        .unwrap();
    let watcher = ProcessMonitor::new(&path).unwrap();

    assert_that!(watcher.state().unwrap(), eq ProcessState::Dead);
    file.remove_self().unwrap();
    assert_that!(watcher.state().unwrap(), eq ProcessState::DoesNotExist);
}

#[test]
pub fn process_state_watcher_detects_non_existing_state() {
    let path = generate_file_path();

    let watcher = ProcessMonitor::new(&path).unwrap();
    assert_that!(watcher.state().unwrap(), eq ProcessState::DoesNotExist);
}

#[test]
pub fn process_state_watcher_transitions_work_starting_from_non_existing_process() {
    let path = generate_file_path();

    let watcher = ProcessMonitor::new(&path).unwrap();
    assert_that!(watcher.state().unwrap(), eq ProcessState::DoesNotExist);
    let file = FileBuilder::new(&path)
        .creation_mode(CreationMode::PurgeAndCreate)
        .create()
        .unwrap();
    assert_that!(watcher.state().unwrap(), eq ProcessState::Dead);
    file.remove_self().unwrap();
    assert_that!(watcher.state().unwrap(), eq ProcessState::DoesNotExist);
}

#[test]
pub fn process_state_watcher_transitions_work_starting_from_existing_process() {
    let path = generate_file_path();

    let file = FileBuilder::new(&path)
        .creation_mode(CreationMode::PurgeAndCreate)
        .create()
        .unwrap();

    let watcher = ProcessMonitor::new(&path).unwrap();
    assert_that!(watcher.state().unwrap(), eq ProcessState::Dead);
    file.remove_self().unwrap();
    assert_that!(watcher.state().unwrap(), eq ProcessState::DoesNotExist);

    let file = FileBuilder::new(&path)
        .creation_mode(CreationMode::PurgeAndCreate)
        .create()
        .unwrap();
    assert_that!(watcher.state().unwrap(), eq ProcessState::Dead);

    file.remove_self().unwrap();
}

#[test]
pub fn process_state_watcher_detects_initialized_state() {
    let path = generate_file_path();

    let mut file = FileBuilder::new(&path)
        .creation_mode(CreationMode::PurgeAndCreate)
        .permission(Permission::OWNER_WRITE)
        .create()
        .unwrap();

    let watcher = ProcessMonitor::new(&path).unwrap();
    assert_that!(watcher.state().unwrap(), eq ProcessState::InInitialization);
    file.set_permission(Permission::OWNER_ALL).unwrap();
    file.remove_self().unwrap();
    assert_that!(watcher.state().unwrap(), eq ProcessState::DoesNotExist);
}
