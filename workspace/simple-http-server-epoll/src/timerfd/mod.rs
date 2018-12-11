// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


use super::*;
use ::libc::timespec;


include!("itimerspec.rs");
include!("TimerClock.rs");
include!("TimerFileDescriptor.rs");
include!("TimerSetChoices.rs");
include!("timerfd_create.rs");
include!("timerfd_settime.rs");
include!("timerfd_gettime.rs");
