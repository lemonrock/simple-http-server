// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


use super::*;
use ::libc::AF_INET;
use ::libc::AF_INET6;
use ::libc::AF_UNIX;
use ::libc::EACCES;
use ::libc::EADDRINUSE;
use ::libc::EADDRNOTAVAIL;
use ::libc::EALREADY;
use ::libc::EAFNOSUPPORT;
use ::libc::ECONNREFUSED;
use ::libc::EINPROGRESS;
use ::libc::EISCONN;
use ::libc::ENETUNREACH;
use ::libc::ENOBUFS;
use ::libc::ENOTDIR;
use ::libc::ENOTSOCK;
use ::libc::EOPNOTSUPP;
use ::libc::EPROTONOSUPPORT;
use ::libc::EROFS;
use ::libc::ETIMEDOUT;
use ::libc::IPPROTO_TCP;
use ::libc::IPPROTO_UDP;
use ::libc::sa_family_t; // Typically u16.
use ::libc::SOCK_DGRAM;
use ::libc::SOCK_STREAM;
use ::libc::sockaddr_in;
use ::libc::sockaddr_in6;
use ::libc::sockaddr_un;
use ::libc::socklen_t; // Typically u32.
use ::std::net::SocketAddr;
use ::std::net::SocketAddrV4;
use ::std::net::SocketAddrV6;
use ::std::os::unix::ffi::OsStrExt;


include!("bind.rs");
include!("connect.rs");
include!("FilePathInvalidReason.rs");
include!("listen.rs");
include!("NewSocketClientError.rs");
include!("NewSocketServerListenerError.rs");
include!("socket.rs");
include!("sockaddr_storage.rs");
include!("SocketAddress.rs");
include!("SocketBindError.rs");
include!("SocketConnectError.rs");
include!("SocketCreationError.rs");
include!("SocketData.rs");
include!("SocketFileDescriptor.rs");
include!("SocketListenError.rs");


/// MIPS and SPARC were early ports of Linux and so often differ in details that they shouldn't.
#[cfg(all(any(target_arch = "android", target_arch = "linux"), any(target_arch = "mips", target_arch = "mips64", target_arch = "sparc", target_arch = "sparc64")))] pub(crate) const SO_REUSEADDR: c_int = 0x0004;
#[cfg(all(any(target_arch = "android", target_arch = "linux"), any(target_arch = "mips", target_arch = "mips64", target_arch = "sparc", target_arch = "sparc64")))] pub(crate) const SO_REUSEPORT: c_int = 0x0200;

/// Everything except MIPS.
#[cfg(not(all(any(target_arch = "android", target_arch = "linux"), any(target_arch = "mips", target_arch = "mips64", target_arch = "sparc", target_arch = "sparc64"))))] pub(crate) const SO_REUSEADDR: c_int = 2;
#[cfg(not(all(any(target_arch = "android", target_arch = "linux"), any(target_arch = "mips", target_arch = "mips64", target_arch = "sparc", target_arch = "sparc64"))))] pub(crate) const SO_REUSEPORT: c_int = 15;
