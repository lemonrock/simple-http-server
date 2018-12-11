// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


use super::*;
use ::libc::c_char;
use ::std::ffi::CStr;
use ::std::ffi::CString;
use ::std::rc::Rc;
use ::std::rc::Weak;


include!("inotify_add_watch.rs");
include!("inotify_event.rs");
include!("inotify_init1.rs");
include!("inotify_rm_watch.rs");
include!("InotifyAddError.rs");
include!("InotifyAddWatchFlags.rs");
include!("InotifyEventFlags.rs");
include!("InotifyFileDescriptor.rs");
include!("InotifyWatchDescriptor.rs");
