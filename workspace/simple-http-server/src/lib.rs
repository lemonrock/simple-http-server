// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![deny(missing_docs)]
#![deny(unreachable_patterns)]


//! #simple-http-server
//! 
//! A simple HTTPS server in Rust which supports client authentication.


extern crate arrayvec;
extern crate cpu_affinity;
#[cfg(unix)] extern crate libc;
#[macro_use] extern crate likely;
extern crate mio;
extern crate mio_extras;
extern crate num_cpus;
extern crate rustls;
extern crate simple_http_server_vectored_buffers;
extern crate time;
extern crate treebitmap;
extern crate untrusted;
extern crate vecio;
pub extern crate webpki;


use self::api::*;
use self::arena::*;
use self::configuration::*;
use self::extensions::*;
use self::support::*;
use self::tokens::*;
use self::workers::*;
use ::arrayvec::ArrayVec;
use ::cpu_affinity::LogicalCores;
#[cfg(unix)] use ::libc::pthread_sigmask;
#[cfg(unix)] use ::libc::SIG_SETMASK;
#[cfg(unix)] use ::libc::sigfillset;
use ::mio::*;
use ::mio::tcp::*;
use ::mio::unix::UnixReady;
use ::mio_extras::channel::*;
use ::rustls::*;
use ::rustls::internal::pemfile::*;
use ::rustls::TLSError::FailedToGetCurrentTime;
use ::rustls::TLSError::NoCertificatesPresented;
use ::rustls::TLSError::WebPKIError;
use ::simple_http_server_vectored_buffers::*;
use ::std::borrow::Borrow;
use ::std::borrow::Cow;
use ::std::cell::RefCell;
use ::std::cmp::max;
use ::std::cmp::min;
use ::std::collections::HashMap;
use ::std::convert::AsMut;
use ::std::convert::AsRef;
use ::std::error;
use ::std::fmt;
use ::std::fmt::Display;
use ::std::fmt::Formatter;
use ::std::fs::File;
use ::std::io;
use ::std::io::BufRead;
use ::std::io::BufReader;
use ::std::io::ErrorKind;
use ::std::io::Read;
use ::std::io::Write;
use ::std::io::ErrorKind::WouldBlock;
use ::std::mem::ManuallyDrop;
use ::std::mem::transmute;
use ::std::mem::uninitialized;
use ::std::net::AddrParseError;
use ::std::net::Ipv4Addr;
use ::std::net::Ipv6Addr;
use ::std::net::Shutdown::Both;
use ::std::net::SocketAddr;
use ::std::ops::Deref;
use ::std::ops::DerefMut;
use ::std::ops::Index;
use ::std::ops::IndexMut;
use ::std::panic::PanicInfo;
use ::std::panic::set_hook;
use ::std::ptr::drop_in_place;
use ::std::ptr::NonNull;
use ::std::ptr::null_mut;
use ::std::ptr::write;
use ::std::rc::Rc;
use ::std::slice::from_raw_parts;
use ::std::slice::from_raw_parts_mut;
use ::std::sync::Arc;
use ::std::sync::atomic::AtomicBool;
use ::std::sync::atomic::AtomicUsize;
use ::std::sync::atomic::Ordering::Relaxed;
use ::std::sync::atomic::Ordering::SeqCst;
use ::std::thread::*;
use ::std::time::Duration;
use ::std::time::SystemTime;
use ::time::now_utc;
use ::time::Tm;
use ::treebitmap::IpLookupTable;
use ::untrusted::Input;
use ::vecio::Rawv;
use ::webpki::*;


/// API.
pub mod api;


pub(crate) mod arena;


/// Configuration.
pub mod configuration;


pub(crate) mod extensions;


pub(crate) mod support;


pub(crate) mod tokens;


pub(crate) mod workers;


include!("ConnectionObserver.rs");
include!("ConnectionObserverConnectError.rs");
include!("MainLoopError.rs");
include!("ReadBufferUser.rs");
include!("SimpleHttpsServer.rs");
include!("Terminate.rs");
