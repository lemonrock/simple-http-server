// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


use super::*;
use ::libc::AT_FDCWD;
use ::libc::c_uchar;
use ::libc::c_ushort;
use ::libc::c_ulonglong;
use ::libc::c_uint;
use ::libc::O_APPEND;
use ::libc::O_CLOEXEC;
use ::libc::O_DSYNC;
use ::libc::O_LARGEFILE;
use ::libc::O_NOATIME;
use ::libc::O_NONBLOCK;
use ::libc::O_RDONLY;
use ::libc::O_RDWR;
use ::libc::O_SYNC;
use ::libc::O_WRONLY;
use ::std::fs::File;


include!("fanotify_event_metadata.rs");
include!("fanotify_init.rs");
include!("fanotify_mark.rs");
include!("fanotify_response.rs");
include!("FanotifyFileDescriptor.rs");
include!("FanotifyMarkError.rs");
include!("FileStatusFlags.rs");
include!("MarkEventFlags.rs");
include!("MarkFlags.rs");
include!("MarkPath.rs");
