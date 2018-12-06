// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


use ::errno::errno;
use ::libc::c_int;
use ::libc::close;
use ::libc::AF_INET;
use ::libc::AF_INET6;
use ::libc::EACCES;
use ::libc::EAFNOSUPPORT;
use ::libc::EINTR;
use ::libc::EINVAL;
use ::libc::EMFILE;
use ::libc::ENFILE;
use ::libc::ENOBUFS;
use ::libc::ENOMEM;
use ::libc::EPROTONOSUPPORT;
use ::libc::O_CLOEXEC;
use ::libc::O_NONBLOCK;
use ::libc::SOCK_DGRAM;
use ::libc::SOCK_STREAM;
use ::libc::read;
use ::libc::write;
use ::std::error;
use ::std::fmt;
use ::std::fmt::Debug;
use ::std::fmt::Display;
use ::std::fmt::Formatter;
use ::std::mem::size_of;
use ::std::os::unix::io::RawFd;


include!("socket.rs");
include!("SocketCreationError.rs");
include!("SocketFileDescriptor.rs");

/// MIPS and SPARC were early ports of Linux and so often differ in details that they shouldn't.
#[cfg(all(any(target_arch = "android", target_arch = "linux"), any(target_arch = "mips", target_arch = "mips64", target_arch = "sparc", target_arch = "sparc64")))] pub(crate) const SO_REUSEADDR: c_int = 0x0004;
#[cfg(all(any(target_arch = "android", target_arch = "linux"), any(target_arch = "mips", target_arch = "mips64", target_arch = "sparc", target_arch = "sparc64")))] pub(crate) const SO_REUSEPORT: c_int = 0x0200;

/// Everything except MIPS.
#[cfg(not(all(any(target_arch = "android", target_arch = "linux"), any(target_arch = "mips", target_arch = "mips64", target_arch = "sparc", target_arch = "sparc64"))))] pub(crate) const SO_REUSEADDR: c_int = 2;
#[cfg(not(all(any(target_arch = "android", target_arch = "linux"), any(target_arch = "mips", target_arch = "mips64", target_arch = "sparc", target_arch = "sparc64"))))] pub(crate) const SO_REUSEPORT: c_int = 15;
