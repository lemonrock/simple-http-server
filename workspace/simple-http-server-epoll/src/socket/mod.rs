// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


use super::*;
use ::libc::AF_INET;
use ::libc::AF_INET6;
use ::libc::AF_UNIX;
use ::libc::c_char;
use ::libc::EACCES;
use ::libc::EADDRINUSE;
use ::libc::EADDRNOTAVAIL;
use ::libc::EALREADY;
use ::libc::EAFNOSUPPORT;
use ::libc::ECONNABORTED;
use ::libc::ECONNREFUSED;
use ::libc::EINPROGRESS;
use ::libc::EISCONN;
use ::libc::ENETUNREACH;
use ::libc::ENOBUFS;
use ::libc::ENOSR;
use ::libc::ENOTDIR;
use ::libc::ENOTSOCK;
use ::libc::EOPNOTSUPP;
use ::libc::EPROTO;
use ::libc::EPROTONOSUPPORT;
use ::libc::EROFS;
use ::libc::ESOCKTNOSUPPORT;
use ::libc::ETIMEDOUT;
use ::libc::in_addr_t;
use ::libc::in_port_t;
use ::libc::IPPROTO_TCP;
use ::libc::IPPROTO_UDP;
use ::libc::sa_family_t; // Typically u16.
use ::libc::SOCK_DGRAM;
use ::libc::SOCK_STREAM;
use ::libc::socklen_t; // Typically u32.
use ::std::borrow::Borrow;
use ::std::borrow::BorrowMut;
use ::std::cmp::Ordering;
use ::std::hash::Hash;
use ::std::hash::Hasher;
use ::std::marker::PhantomData;
use ::std::net::SocketAddr;
use ::std::net::SocketAddrV4;
use ::std::net::SocketAddrV6;
use ::std::ops::Deref;
use ::std::ops::DerefMut;
use ::std::os::unix::ffi::OsStrExt;


include!("accept4.rs");
include!("AcceptedConnection.rs");
include!("AcceptedConnectionEnum.rs");
include!("bind.rs");
include!("connect.rs");
include!("ConnectionFailedReason.rs");
include!("FilePathInvalidReason.rs");
include!("in_addr.rs");
include!("in6_addr.rs");
include!("listen.rs");
include!("NewSocketClientError.rs");
include!("NewSocketServerListenerError.rs");
include!("ServerListenerSocketFileDescriptor.rs");
include!("ServerListenerSocketFileDescriptorEnum.rs");
include!("socket.rs");
include!("sockaddr_in.rs");
include!("sockaddr_in6.rs");
include!("sockaddr_un.rs");
include!("sockaddr_storage.rs");
include!("SocketAcceptError.rs");
include!("SocketAddress.rs");
include!("SocketBindError.rs");
include!("SocketConnectError.rs");
include!("SocketCreationError.rs");
include!("SocketData.rs");
include!("SocketFileDescriptor.rs");
include!("SocketListenError.rs");
include!("StreamingSocketFileDescriptor.rs");
include!("StreamingSocketFileDescriptorEnum.rs");


/// MIPS and SPARC were early ports of Linux and so often differ in details that they shouldn't.
#[cfg(all(any(target_arch = "android", target_arch = "linux"), any(target_arch = "mips", target_arch = "mips64", target_arch = "sparc", target_arch = "sparc64")))] pub(crate) const SO_REUSEADDR: c_int = 0x0004;
#[cfg(all(any(target_arch = "android", target_arch = "linux"), any(target_arch = "mips", target_arch = "mips64", target_arch = "sparc", target_arch = "sparc64")))] pub(crate) const SO_REUSEPORT: c_int = 0x0200;

/// Everything except MIPS.
#[cfg(not(all(any(target_arch = "android", target_arch = "linux"), any(target_arch = "mips", target_arch = "mips64", target_arch = "sparc", target_arch = "sparc64"))))] pub(crate) const SO_REUSEADDR: c_int = 2;
#[cfg(not(all(any(target_arch = "android", target_arch = "linux"), any(target_arch = "mips", target_arch = "mips64", target_arch = "sparc", target_arch = "sparc64"))))] pub(crate) const SO_REUSEPORT: c_int = 15;

