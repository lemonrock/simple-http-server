// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![deny(missing_docs)]
#![deny(unreachable_patterns)]
#![feature(core_intrinsics)]


//! #simple-http-server-epoll
//! 
//! A wrapper around epoll for a simple HTTPS server in Rust which supports client authentication.
//!
//! Fully functional on Android and Linux.
//!
//! Mostly functional supported by Fuschia.
//!
//! Mostly functional supported by Illumos, a Solaris fork.
//!
//! Mostly functional supported by uclibc and emscripten.
//!
//!
//! ## Supported File Descriptors
//!
//! * Sockets.
//! * eventfd.
//! * signalfd.
//! * timerfd.
//!
//!
//! ## Unsupported for now
//!
//! * userfaultd.
//! * memfd.
//! * POSIX message queues (<(https://linux.die.net/man/7/mq_overview>).


#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] extern crate errno;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] extern crate libc;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] #[macro_use] extern crate likely;


#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] pub use epoll::*;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] pub use eventfd::*;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] pub use signalfd::*;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] pub use timerfd::*;


#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] mod epoll;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] mod eventfd;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] mod signalfd;
#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux", target_os = "solaris", target_env = "uclibc"))] mod timerfd;
