// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


use ::errno::errno;
use ::libc::c_int;
use ::libc::c_void;
use ::libc::close;
use ::libc::EBADF;
use ::libc::EEXIST;
use ::libc::EFAULT;
use ::libc::EINVAL;
use ::libc::EINTR;
use ::libc::ELOOP;
use ::libc::EMFILE;
use ::libc::ENFILE;
use ::libc::ENOENT;
use ::libc::ENOMEM;
use ::libc::ENOSPC;
use ::libc::EPERM;
use ::libc::O_CLOEXEC;
use ::libc::sigset_t;
use ::libc::uint32_t;
use ::libc::uint64_t;
use ::std::cmp::Ordering;
use ::std::error;
use ::std::fmt;
use ::std::fmt::Debug;
use ::std::fmt::Display;
use ::std::fmt::Formatter;
use ::std::hash::Hash;
use ::std::hash::Hasher;
use ::std::mem::zeroed;
use ::std::os::unix::io::RawFd;
use ::std::ptr::null_mut;


include!("epoll_create1.rs");
include!("epoll_ctl.rs");
include!("epoll_data_t.rs");
include!("epoll_event.rs");
include!("epoll_pwait.rs");
include!("epoll_wait.rs");
include!("EPoll.rs");
include!("EPollAddError.rs");
include!("EPollCreationError.rs");
include!("EPollDeleteError.rs");
include!("EPollModifyError.rs");
include!("EPollTimeOut.rs");
include!("EPollWaitError.rs");
