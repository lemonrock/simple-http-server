// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


use super::*;
use ::libc::AF_INET;
use ::libc::AF_INET6;
use ::libc::AF_UNIX;
use ::libc::c_char;
use ::libc::c_uchar;
use ::libc::c_uint;
use ::libc::EADDRINUSE;
use ::libc::EADDRNOTAVAIL;
use ::libc::EALREADY;
use ::libc::EAFNOSUPPORT;
use ::libc::ECONNABORTED;
use ::libc::ECONNREFUSED;
use ::libc::ECONNRESET;
use ::libc::EINPROGRESS;
use ::libc::EISCONN;
use ::libc::EMSGSIZE;
use ::libc::ENETUNREACH;
use ::libc::ENOBUFS;
use ::libc::ENOPROTOOPT;
use ::libc::ENOSR;
use ::libc::ENOTCONN;
use ::libc::ENOTDIR;
use ::libc::ENOTSOCK;
use ::libc::EOPNOTSUPP;
use ::libc::EPROTO;
use ::libc::EPROTONOSUPPORT;
use ::libc::EROFS;
use ::libc::ESOCKTNOSUPPORT;
use ::libc::ETIMEDOUT;
use ::libc::gid_t;
use ::libc::in_addr_t;
use ::libc::in_port_t;
use ::libc::iovec;
use ::libc::IPPROTO_TCP;
use ::libc::IPPROTO_UDP;
use ::libc::sa_family_t; // Typically u16.
use ::libc::send;
use ::libc::SOCK_DGRAM;
use ::libc::SOCK_STREAM;
use ::libc::socklen_t; // Typically u32.
use ::libc::ssize_t;
use ::libc::uid_t;
use ::libc::unlink;
use ::std::borrow::Borrow;
use ::std::borrow::BorrowMut;
use ::std::cmp::Ordering;
use ::std::hash::Hash;
use ::std::hash::Hasher;
use ::std::io;
use ::std::io::ErrorKind;
use ::std::io::Initializer;
use ::std::io::Read;
use ::std::io::Write;
use ::std::marker::PhantomData;
use ::std::net::SocketAddr;
use ::std::net::SocketAddrV4;
use ::std::net::SocketAddrV6;
use ::std::ops::Deref;
use ::std::ops::DerefMut;
use ::std::slice::from_raw_parts;
use ::std::slice::from_raw_parts_mut;
use ::std::fs::DirBuilder;
use ::std::fs::remove_dir;
use ::std::fs::remove_file;
use ::std::fs::set_permissions;
use ::std::os::unix::fs::DirBuilderExt;
use ::std::os::unix::fs::PermissionsExt;


include!("accept4.rs");
include!("AcceptedConnection.rs");
include!("AcceptedConnectionEnum.rs");
include!("bind.rs");
include!("cmsghdr.rs");
include!("connect.rs");
include!("ConnectionFailedReason.rs");
include!("ErrorFlags.rs");
include!("FilePathInvalidReason.rs");
include!("getsockname.rs");
include!("in_addr.rs");
include!("in6_addr.rs");
include!("listen.rs");
include!("MessageHeadersIterator.rs");
include!("MSG_.rs");
include!("msghdr.rs");
include!("NewSocketClientError.rs");
include!("NewSocketServerListenerError.rs");
include!("ReceiveFlags.rs");
include!("ReceiveFileDescriptorsError.rs");
include!("recvfrom.rs");
include!("recvmsg.rs");
include!("SCM_.rs");
include!("SendFlags.rs");
include!("sendmsg.rs");
include!("ServerListenerSocketFileDescriptor.rs");
include!("ServerListenerSocketFileDescriptorEnum.rs");
include!("setsockopt.rs");
include!("socketpair.rs");
include!("shutdown.rs");
include!("SO_.rs");
include!("sockaddr_in.rs");
include!("sockaddr_in6.rs");
include!("sockaddr_un.rs");
include!("sockaddr_storage.rs");
include!("socket.rs");
include!("SocketAcceptError.rs");
include!("SocketAddress.rs");
include!("SocketBindError.rs");
include!("SocketConnectError.rs");
include!("SocketData.rs");
include!("SocketFileDescriptor.rs");
include!("SocketListenError.rs");
include!("SOL_.rs");
include!("StreamingSocketFileDescriptor.rs");
include!("StreamingSocketFileDescriptorEnum.rs");
include!("TCP_.rs");
include!("ucred.rs");
include!("UnixSocketAddress.rs");
