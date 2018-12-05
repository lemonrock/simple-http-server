// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


use ::errno::errno;
use ::libc::c_int;
use ::libc::close;
use ::libc::EBADF;
use ::libc::EFAULT;
use ::libc::EINVAL;
use ::libc::EMFILE;
use ::libc::ENFILE;
use ::libc::ENODEV;
use ::libc::ENOMEM;
use ::libc::O_CLOEXEC;
use ::libc::O_NONBLOCK;
use ::libc::timespec;
use ::std::error;
use ::std::fmt;
use ::std::fmt::Debug;
use ::std::fmt::Display;
use ::std::fmt::Formatter;
use ::std::mem::uninitialized;
use ::std::os::unix::io::RawFd;


include!("itimerspec.rs");
include!("TimerClock.rs");
include!("TimerCreationError.rs");
include!("TimerFileDescriptor.rs");
include!("TimerSetChoices.rs");
include!("timerfd_create.rs");
include!("timerfd_settime.rs");
include!("timerfd_gettime.rs");
