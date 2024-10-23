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

use std::os::raw::c_char;

use iceoryx2::service::builder::{
    event::{EventCreateError, EventOpenError, EventOpenOrCreateError},
    publish_subscribe::{
        PublishSubscribeCreateError, PublishSubscribeOpenError, PublishSubscribeOpenOrCreateError,
    },
};

pub trait ErrorAsString {
    fn as_str(&self) -> &'static str;
    fn as_cstr(&self) -> *const c_char {
        self.as_str().as_ptr() as *const c_char
    }
}

impl ErrorAsString for PublishSubscribeCreateError {
    fn as_str(&self) -> &'static str {
        match self {
            PublishSubscribeCreateError::ServiceInCorruptedState => "ServiceInCorruptedState",
            PublishSubscribeCreateError::SubscriberBufferMustBeLargerThanHistorySize => {
                "SubscriberBufferMustBeLargerThanHistorySize"
            }
            PublishSubscribeCreateError::AlreadyExists => "AlreadyExists",
            PublishSubscribeCreateError::InsufficientPermissions => "InsufficientPermissions",
            PublishSubscribeCreateError::InternalFailure => "InternalFailure",
            PublishSubscribeCreateError::IsBeingCreatedByAnotherInstance => {
                "IsBeingCreatedByAnotherInstance"
            }
            PublishSubscribeCreateError::HangsInCreation => "HangsInCreation",
        }
    }
}

impl ErrorAsString for PublishSubscribeOpenError {
    fn as_str(&self) -> &'static str {
        match self {
            PublishSubscribeOpenError::DoesNotExist => "DoesNotExist",
            PublishSubscribeOpenError::InternalFailure => "InternalFailure",
            PublishSubscribeOpenError::IncompatibleTypes => "IncompatibleTypes",
            PublishSubscribeOpenError::IncompatibleMessagingPattern => {
                "IncompatibleMessagingPattern"
            }
            PublishSubscribeOpenError::IncompatibleAttributes => "IncompatibleAttributes",
            PublishSubscribeOpenError::DoesNotSupportRequestedMinBufferSize => {
                "DoesNotSupportRequestedMinBufferSize"
            }
            PublishSubscribeOpenError::DoesNotSupportRequestedMinHistorySize => {
                "DoesNotSupportRequestedMinHistorySize"
            }
            PublishSubscribeOpenError::DoesNotSupportRequestedMinSubscriberBorrowedSamples => {
                "DoesNotSupportRequestedMinSubscriberBorrowedSamples"
            }
            PublishSubscribeOpenError::DoesNotSupportRequestedAmountOfPublishers => {
                "DoesNotSupportRequestedAmountOfPublishers"
            }
            PublishSubscribeOpenError::DoesNotSupportRequestedAmountOfSubscribers => {
                "DoesNotSupportRequestedAmountOfSubscribers"
            }
            PublishSubscribeOpenError::DoesNotSupportRequestedAmountOfNodes => {
                "DoesNotSupportRequestedAmountOfNodes"
            }
            PublishSubscribeOpenError::IncompatibleOverflowBehavior => {
                "IncompatibleOverflowBehavior"
            }
            PublishSubscribeOpenError::InsufficientPermissions => "InsufficientPermissions",
            PublishSubscribeOpenError::ServiceInCorruptedState => "ServiceInCorruptedState",
            PublishSubscribeOpenError::HangsInCreation => "HangsInCreation",
            PublishSubscribeOpenError::ExceedsMaxNumberOfNodes => "ExceedsMaxNumberOfNodes",
            PublishSubscribeOpenError::IsMarkedForDestruction => "IsMarkedForDestruction",
        }
    }
}

impl ErrorAsString for PublishSubscribeOpenOrCreateError {
    fn as_str(&self) -> &'static str {
        match self {
            PublishSubscribeOpenOrCreateError::PublishSubscribeOpenError(_) => todo!(),
            PublishSubscribeOpenOrCreateError::PublishSubscribeCreateError(_) => todo!(),
        }
    }
}

impl ErrorAsString for EventOpenError {
    fn as_str(&self) -> &'static str {
        match self {
            EventOpenError::DoesNotExist => "DoesNotExist",
            EventOpenError::InsufficientPermissions => "InsufficientPermissions",
            EventOpenError::ServiceInCorruptedState => "ServiceInCorruptedState",
            EventOpenError::IncompatibleMessagingPattern => "IncompatibleMessagingPattern",
            EventOpenError::IncompatibleAttributes => "IncompatibleAttributes",
            EventOpenError::InternalFailure => "InternalFailure",
            EventOpenError::HangsInCreation => "HangsInCreation",
            EventOpenError::DoesNotSupportRequestedAmountOfNotifiers => {
                "DoesNotSupportRequestedAmountOfNotifiers"
            }
            EventOpenError::DoesNotSupportRequestedAmountOfListeners => {
                "DoesNotSupportRequestedAmountOfListeners"
            }
            EventOpenError::DoesNotSupportRequestedMaxEventId => {
                "DoesNotSupportRequestedMaxEventId"
            }
            EventOpenError::DoesNotSupportRequestedAmountOfNodes => {
                "DoesNotSupportRequestedAmountOfNodes"
            }
            EventOpenError::ExceedsMaxNumberOfNodes => "ExceedsMaxNumberOfNodes",
            EventOpenError::IsMarkedForDestruction => "IsMarkedForDestruction",
        }
    }
}

impl ErrorAsString for EventCreateError {
    fn as_str(&self) -> &'static str {
        match self {
            EventCreateError::ServiceInCorruptedState => "ServiceInCorruptedState",
            EventCreateError::InternalFailure => "InternalFailure",
            EventCreateError::IsBeingCreatedByAnotherInstance => "IsBeingCreatedByAnotherInstance",
            EventCreateError::AlreadyExists => "AlreadyExists",
            EventCreateError::HangsInCreation => "HangsInCreation",
            EventCreateError::InsufficientPermissions => "InsufficientPermissions",
        }
    }
}

impl ErrorAsString for EventOpenOrCreateError {
    fn as_str(&self) -> &'static str {
        match self {
            EventOpenOrCreateError::EventOpenError(e) => e.as_str(),
            EventOpenOrCreateError::EventCreateError(e) => e.as_str(),
        }
    }
}
