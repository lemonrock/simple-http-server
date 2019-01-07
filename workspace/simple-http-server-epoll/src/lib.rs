// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![deny(missing_docs)]
#![deny(unreachable_patterns)]
#![feature(core_intrinsics)]
#![feature(read_initializer)]
#![feature(self_struct_ctor)]
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
//! * character devices and serial ports.
//! * epoll.
//! * eventfd.
//! * fanotify.
//! * inotify.
//! * POSIX message queues (<(https://linux.die.net/man/7/mq_overview>).
//! * pipes_and_fifos (anonymous and named (FIFO)s).
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
//! The above features may not work correctly after the use of `seccomp` to lock down system calls (particularly the attempt to delete a socket file path on close).
//!
//!
//! ## Pipes
//!
//! * To be able to use epoll with standard in (`stdin`), use `pipes_and_fifos::ReceivePipeFileDescriptor::standard_in()`.
//! * To be able to use epoll with standard out (`stdout`), use `pipes_and_fifos::SendPipeFileDescriptor::standard_out()`.
//! * To be able to use epoll with standard error (`stderr`), use `pipes_and_fifos::SendPipeFileDescriptor::standard_error()`.
//!
//!
//! ## Unsupported for now
//!
//! * Linux zero copy send (`MSG_ZEROCOPY`) and receive (`SO_ZEROCOPY`), mostly because they have a horrible, hacky API.
//! * `SO_BUSY_POLL` and `SO_INCOMING_CPU`.
//! * Unix Domain Sockets using `autobind`; setting of the `SO_PASSCRED` socket option.
//! * Receiving credentials over Unix Domain Sockets using `recvmsg()`.
//! * `vmsplice()`, `tee()` and `splice()`.
//! * `mkfifo()`
//! * epoll and serial port / USB, eg https://www.cnblogs.com/darryo/p/selectpollepoll-on-serial-port.html; in effect, very similar to a pipe. Can we epoll on any character device, then? (and `mknod()`).
//! * infiniband sockets
//! * canbus sockets


#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] extern crate arrayvec;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] #[macro_use] extern crate bitflags;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] #[macro_use] extern crate cfg_if;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] extern crate errno;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] extern crate libc;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] #[macro_use] extern crate likely;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] extern crate strum;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] #[macro_use] extern crate strum_macros;

cfg_if!
{
	if #[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))]
	{
		use self::epoll::*;
		use self::character_devices_and_terminals::TerminalSettingsError;
		use ::arrayvec::ArrayVec;
		use ::errno::errno;
		use ::errno::Errno;
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


		/// Character device file descriptors, with support for terminals (and thus serial ports).
		pub mod character_devices_and_terminals;


		/// EPoll file descriptors.
		pub mod epoll;


		/// Event file descriptors.
		pub mod eventfd;


		#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux"))]
		/// inotify file descriptors.
		pub mod inotify;


		#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux"))]
		/// POSIX message queue file descriptors.
		pub mod posix_message_queues;


		/// Anonymous and named, connected unidirectional pipes_and_fifos (act like TCP connected sockets).
		///
		/// Since Linux 2.6.35, the default pipe capacity is 16 pages (which are 4096 bytes on x86-64), but the capacity can be queried and set using the `fcntl()` `F_GETPIPE_SZ` and `F_SETPIPE_SZ` operations.
		///
		/// The unread bytes in a pipe can be obtained using the `fcntl()` operation `FIONREAD`.
		///
		/// The maximum capacity that can be set for a non-privileged process (one without the `CAP_SYS_RESOURCE` capability) is specified in the file `/proc/sys/fs/pipe-max-size`; it defaults to 1Mb.
		///
		/// Writes of less than `PIPE_BUF` bytes are atomic; `PIPE_BUF` is 4096 bytes on Linux.
		///
		/// Atomic writes are significant when there are many writers sharing one named pipe (FIFO).
		pub mod pipes_and_fifos;


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
		include!("InvalidPathReason.rs");
		include!("path_bytes_without_trailing_nul.rs");
		include!("RawFdExt.rs");
		include!("SpecialFileOpenError.rs");
		include!("StructReadError.rs");
		include!("StructWriteError.rs");
	}
}

