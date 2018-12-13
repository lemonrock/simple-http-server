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
//! * eventfd.
//! * fanotify.
//! * inotify.
//! * signalfd.
//! * sockets (TCP, UDP and the equivalent over Unix domain sockets).
//! * timerfd.
//!
//!
//! ## Unsupported for now
//!
//! * userfaultd.
//! * memfd.
//! * POSIX message queues (<(https://linux.die.net/man/7/mq_overview>).


#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] #[macro_use] extern crate bitflags;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] extern crate errno;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] extern crate libc;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] #[macro_use] extern crate likely;


#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use self::epoll::*;
#[cfg(unix)] use ::libc::close;
#[cfg(unix)] use ::std::os::unix::io::RawFd;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use ::errno::errno;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use ::libc::c_char;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use ::libc::c_int;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use ::libc::c_void;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use ::std::ffi::CStr;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use ::libc::EACCES;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use ::libc::EAGAIN;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use ::libc::EBADF;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use ::libc::ECANCELED;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use ::libc::EDESTADDRREQ;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use ::libc::EDQUOT;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use ::libc::EEXIST;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use ::libc::EFAULT;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use ::libc::EFBIG;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use ::libc::EINTR;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use ::libc::EINVAL;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use ::libc::EIO;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use ::libc::EISDIR;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use ::libc::ELOOP;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use ::libc::EMFILE;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use ::libc::ENFILE;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use ::libc::ENODEV;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use ::libc::ENOENT;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use ::libc::ENOMEM;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use ::libc::ENOSPC;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use ::libc::ENOSYS;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use ::libc::ENOTDIR;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use ::libc::EPERM;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use ::libc::EPIPE;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use ::libc::O_CLOEXEC;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use ::libc::O_NONBLOCK;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use ::libc::O_RDONLY;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use ::libc::O_RDWR;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use ::libc::O_WRONLY;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use ::libc::int32_t;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use ::libc::pid_t;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use ::libc::read;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use ::libc::sigset_t;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use ::libc::size_t;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use ::libc::uint8_t;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use ::libc::uint16_t;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use ::libc::uint32_t;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use ::libc::uint64_t;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use ::libc::write;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use ::std::cmp::Ordering;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use ::std::hash::Hash;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use ::std::hash::Hasher;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use ::std::error;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use ::std::fmt;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use ::std::fmt::Debug;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use ::std::fmt::Display;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use ::std::fmt::Formatter;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use ::std::mem::size_of;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use ::std::mem::transmute;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use ::std::mem::uninitialized;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use ::std::mem::zeroed;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use ::std::os::unix::ffi::OsStrExt;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use ::std::os::unix::io::AsRawFd;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] #[allow(unused_imports)] use ::std::os::unix::io::FromRawFd;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use ::std::os::unix::io::IntoRawFd;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use ::std::path::Path;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use ::std::ptr::null;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] use ::std::ptr::null_mut;


#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))]
/// EPoll file descriptors.
pub mod epoll;


#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))]
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


#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))]
/// Signal file descriptors.
pub mod signalfd;


#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux"))]
/// Socket file descriptors.
pub mod socket;


#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))]
/// Timer file descriptors.
pub mod timerfd;


#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] include!("CreationError.rs");
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] include!("path_bytes_without_trailing_nul.rs");
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] include!("RawFdExt.rs");
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] include!("StructReadError.rs");
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] include!("StructWriteError.rs");
