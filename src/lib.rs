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


extern crate mio;
extern crate rustls;
extern crate thhp;
extern crate untrusted;
extern crate url;
extern crate vecio;
pub extern crate webpki;


use ::mio::Events;
use ::mio::Poll;
use ::mio::PollOpt;
use ::mio::Ready;
use ::mio::Token;
use ::mio::tcp::TcpListener;
use ::mio::tcp::TcpStream;
use ::mio::unix::UnixReady;
use ::rustls::Certificate;
use ::rustls::ClientCertVerified;
use ::rustls::ClientCertVerifier;
use ::rustls::DistinguishedNames;
use ::rustls::NoServerSessionStorage;
use ::rustls::ProtocolVersion;
use ::rustls::RootCertStore;
use ::rustls::ServerConfig;
use ::rustls::ServerSession;
use ::rustls::ServerSessionMemoryCache;
use ::rustls::TLSError;
use ::rustls::TLSError::FailedToGetCurrentTime;
use ::rustls::TLSError::NoCertificatesPresented;
use ::rustls::TLSError::WebPKIError;
use ::rustls::WriteV;
use ::rustls::internal::pemfile::certs;
use ::rustls::internal::pemfile::pkcs8_private_keys;
use ::rustls::internal::pemfile::rsa_private_keys;
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
use ::std::net::AddrParseError;
use ::std::net::Shutdown::Both;
use ::std::net::SocketAddr;
use ::std::sync::Arc;
use ::std::time::Duration;
use ::std::time::SystemTime;
use ::thhp::HeaderField;
use ::thhp::Request;
use ::thhp::Status::*;
use ::untrusted::Input;
use ::url::Url;
use ::url::ParseError;
use ::vecio::Rawv;
use ::webpki::*;


include!("CertificateExt.rs");
include!("ClientAuthenticationConfiguration.rs");
include!("Constraints.rs");
include!("HttpServerReadError.rs");
include!("MainLoopError.rs");
include!("NewServerClientConnectionError.rs");
include!("RequestExt.rs");
include!("ServedClientConnection.rs");
include!("ServedClientConnections.rs");
include!("ServerConfigurationError.rs");
include!("ServerSessionExt.rs");
include!("SimpleHttpsServer.rs");
include!("SignatureAlgorithms.rs");
include!("SupportedTlsVersions.rs");
include!("TimeExt.rs");
include!("TlsConfiguration.rs");
include!("TlsReadError.rs");
include!("TlsWriteError.rs");
include!("TokenStore.rs");
include!("WriteVAdapter.rs");
