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
extern crate mio;
extern crate rustls;
extern crate thhp;
extern crate time;
extern crate untrusted;
extern crate url;
extern crate vecio;
pub extern crate webpki;


use self::configuration::*;
use self::extensions::*;
use self::support::*;
use ::mio::*;
use ::mio::tcp::*;
use ::mio::unix::UnixReady;
use ::rustls::*;
use ::rustls::internal::pemfile::*;
use ::rustls::TLSError::FailedToGetCurrentTime;
use ::rustls::TLSError::NoCertificatesPresented;
use ::rustls::TLSError::WebPKIError;
use ::std::borrow::Borrow;
use ::std::borrow::Cow;
use ::std::cell::RefCell;
use ::std::error;
use ::std::fmt;
use ::std::fmt::Display;
use ::std::fmt::Formatter;
use ::std::fs::File;
use ::std::io;
use ::std::io::BufRead;
use ::std::io::BufReader;
use ::std::io::Read;
use ::std::io::Write;
use ::std::io::ErrorKind::WouldBlock;
use ::std::sync::atomic::AtomicBool;
use ::std::sync::atomic::AtomicUsize;
use ::std::sync::atomic::Ordering::Relaxed;
use ::std::collections::HashMap;
use ::std::mem::uninitialized;
use ::std::net::AddrParseError;
use ::std::net::Shutdown::Both;
use ::std::net::SocketAddr;
use ::std::ptr::NonNull;
use ::std::rc::Rc;
use ::std::sync::Arc;
use ::std::time::Duration;
use ::std::time::SystemTime;
use ::thhp::HeaderField;
use ::thhp::Request;
use ::thhp::Status::*;
use ::time::now_utc;
use ::time::Tm;
use ::untrusted::Input;
use ::url::Url;
use ::url::ParseError;
use ::vecio::Rawv;
use ::webpki::*;


/// API.
pub mod api;


/// Configuration.
pub mod configuration;


pub(crate) mod extensions;


pub(crate) mod support;


include!("HttpGetUser.rs");
include!("HttpReadBufferUser.rs");
include!("MainLoopError.rs");
include!("NewServerClientConnectionError.rs");
include!("ReadBufferUser.rs");
include!("ServedClientConnection.rs");
include!("ServedClientConnections.rs");
include!("SimpleHttpsServer.rs");
