// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


use super::*;
use self::syscall::*;
use ::libc::c_uint;
use ::libc::ENAMETOOLONG;
use ::libc::EEXIST;
use ::libc::EMSGSIZE;
use ::libc::ETIMEDOUT;
use ::libc::mode_t;
use ::libc::O_CREAT;
use ::libc::O_EXCL;
use ::libc::S_IRUSR;
use ::libc::S_IWUSR;
use ::std::convert::TryFrom;


mod syscall;


include!("MessagePriority.rs");
include!("MessageQueue.rs");
include!("MessageQueueCreateSendOrReceive.rs");
include!("MessageQueueCreateSettings.rs");
include!("MessageQueueFileDescriptor.rs");
include!("MessageQueueUnlinkError.rs");
include!("OpenOrCreateMessageQueue.rs");
include!("OptionalMessageQueueCreateSettings.rs");
include!("Receive.rs");
include!("ReceiveMessageQueueFileDescriptor.rs");
include!("Send.rs");
include!("SendAndReceiveMessageQueueFileDescriptor.rs");
include!("SendMessageQueueFileDescriptor.rs");
