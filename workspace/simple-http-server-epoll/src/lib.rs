// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![deny(missing_docs)]
#![deny(unreachable_patterns)]
#![feature(core_intrinsics)]
#![feature(read_initializer)]
#![feature(try_from)]


//! #simple-http-server-epoll
//! 
//! A wrapper around epoll ('Event Poll') for a simple HTTPS server in Rust which supports client authentication.
//!
//! Fully functional on Android and Linux.
//!
//! Mostly functional on Fuschia.
//!
//! Mostly functional on Illumos, a Solaris fork.
//!
//! Mostly functional on uclibc and emscripten.
//!
//!
//! ## Supported File Descriptors
//!
//! * epoll.
//! * eventfd.
//! * fanotify.
//! * inotify.
//! * POSIX message queues (<(https://linux.die.net/man/7/mq_overview>).
//! * signalfd.
//! * sockets (TCP, UDP and the equivalent over Unix Domain Sockets).
//! * timerfd.
//!
//!
//! ## Unix Domain Sockets
//!
//!
//! ### When using file paths
//!
//! * Every effort is made to create the socket file path cleanly;
//! * To make sure all parent folders exist;
//! * To make sure parent folder permissions are correctly set;
//! * To remove any stale files;
//! * To remove socket file paths on drop (close).
//!
//! The above features may not work correctly after the use of `seccomp` system calls (particularly the attempt to delete a socket file path on close).
//!
//!
//! ## Unsupported for now
//!
//! * `pipe2()`.
//! * `socketpair()`.
//! * Linux zero copy send (`MSG_ZEROCOPY`) and receive (`SO_ZEROCOPY`), mostly because they have a horrible, hacky API.
//! * `SO_BUSY_POLL`.
//! * Anonymous Unix Domain Sockets and `autobind`; setting of the `SO_PASSCRED` socket option.
//! * Receiving credentials over Unix Domain Sockets.
//! * Linux abstract Unix Domain Sockets.


#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] extern crate arrayvec;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] #[macro_use] extern crate bitflags;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] #[macro_use] extern crate cfg_if;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] extern crate errno;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] extern crate libc;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] #[macro_use] extern crate likely;

cfg_if!
{
	if #[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))]
	{
		use self::epoll::*;
		use ::arrayvec::ArrayVec;
		use ::errno::errno;
		use ::libc::c_char;
		use ::libc::c_int;
		use ::libc::c_void;
		use ::std::ffi::CStr;
		use ::libc::EACCES;
		use ::libc::EAGAIN;
		use ::libc::EBADF;
		use ::libc::ECANCELED;
		use ::libc::EDESTADDRREQ;
		use ::libc::EDQUOT;
		use ::libc::EEXIST;
		use ::libc::EFAULT;
		use ::libc::EFBIG;
		use ::libc::EINTR;
		use ::libc::EINVAL;
		use ::libc::EIO;
		use ::libc::EISDIR;
		use ::libc::ELOOP;
		use ::libc::EMFILE;
		use ::libc::ENFILE;
		use ::libc::ENODEV;
		use ::libc::ENOENT;
		use ::libc::ENOMEM;
		use ::libc::ENOSPC;
		use ::libc::ENOSYS;
		use ::libc::ENOTDIR;
		use ::libc::EPERM;
		use ::libc::EPIPE;
		use ::libc::O_CLOEXEC;
		use ::libc::O_NONBLOCK;
		use ::libc::O_RDONLY;
		use ::libc::O_RDWR;
		use ::libc::O_WRONLY;
		use ::libc::int32_t;
		use ::libc::pid_t;
		use ::libc::read;
		use ::libc::sigset_t;
		use ::libc::size_t;
		use ::libc::uint8_t;
		use ::libc::uint16_t;
		use ::libc::uint32_t;
		use ::libc::uint64_t;
		use ::libc::write;
		use ::std::cmp::Ordering;
		use ::std::hash::Hash;
		use ::std::hash::Hasher;
		use ::std::error;
		use ::std::fmt;
		use ::std::fmt::Debug;
		use ::std::fmt::Display;
		use ::std::fmt::Formatter;
		use ::std::mem::size_of;
		use ::std::mem::transmute;
		use ::std::mem::uninitialized;
		use ::std::mem::zeroed;
		use ::std::os::unix::ffi::OsStrExt;
		use ::std::os::unix::io::AsRawFd;
		#[allow(unused_imports)] use ::std::os::unix::io::FromRawFd;
		use ::std::os::unix::io::IntoRawFd;
		use ::std::path::Path;
		use ::std::ptr::null;
		use ::std::ptr::null_mut;


		#[cfg(unix)] use ::libc::close;
		#[cfg(unix)] use ::std::os::unix::io::RawFd;


		/// EPoll file descriptors.
		pub mod epoll;


		/// Event file descriptors.
		pub mod eventfd;


		#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux"))]
		/// inotify file descriptors.
		pub mod inotify;


		#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux"))]
		/// POSIX message queue file descriptors.
		pub mod mq;


		#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux"))]
		/// fanotify file descriptors.
		pub mod fanotify;


		/// Signal file descriptors.
		pub mod signalfd;


		#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux"))]
		/// Socket file descriptors.
		pub mod socket;


		/// Timer file descriptors.
		pub mod timerfd;


		include!("CreationError.rs");
		include!("path_bytes_without_trailing_nul.rs");
		include!("RawFdExt.rs");
		include!("StructReadError.rs");
		include!("StructWriteError.rs");
	}
}

