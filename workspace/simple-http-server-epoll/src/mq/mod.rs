// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


use super::*;
use ::libc::ENAMETOOLONG;
use ::libc::EEXIST;
use ::libc::mode_t;
use ::libc::O_CREAT;
use ::libc::O_EXCL;
use ::libc::S_IRUSR;
use ::libc::S_IWUSR;


include!("mq_attr.rs");
include!("mqd_t.rs");
include!("mq_open.rs");
include!("MessageQueueCreateReadOrWrite.rs");
include!("MessageQueueCreateSettings.rs");
include!("MessageQueueFileDescriptor.rs");
include!("OpenOrCreateMessageQueue.rs");
include!("OptionalMessageQueueCreateSettings.rs");

// TODO: Insist of one of RDONLY, WRONLY or RDWR for fanotify.
