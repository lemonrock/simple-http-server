// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


use super::*;
use ::libc::c_char;
use ::libc::c_uchar;
use ::libc::c_uint;
use ::libc::in_addr_t;
use ::libc::in_port_t;
use ::libc::ssize_t;
use ::libc::timespec;
use ::libc::unlink;


include!("accept4.rs");
include!("bind.rs");
include!("cmsghdr.rs");
include!("connect.rs");
include!("getsockname.rs");
include!("getsockopt.rs");
include!("in_addr.rs");
include!("in6_addr.rs");
include!("listen.rs");
include!("MSG_.rs");
include!("mmsghdr.rs");
include!("msghdr.rs");
include!("recvfrom.rs");
include!("recvmmsg.rs");
include!("recvmsg.rs");
include!("SCM_.rs");
include!("sendmsg.rs");
include!("setsockopt.rs");
include!("socketpair.rs");
include!("shutdown.rs");
include!("SO_.rs");
include!("sockaddr_in.rs");
include!("sockaddr_in6.rs");
include!("sockaddr_un.rs");
include!("sockaddr_storage.rs");
include!("socket.rs");
include!("SOL_.rs");
include!("TCP_.rs");
include!("ucred.rs");
