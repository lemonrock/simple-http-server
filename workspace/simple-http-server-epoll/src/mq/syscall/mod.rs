// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


use super::*;
use ::libc::ssize_t;
use ::libc::timespec;


include!("mq_attr.rs");
include!("mqd_t.rs");
include!("mq_getattr.rs");
include!("mq_open.rs");
include!("mq_timedreceive.rs");
include!("mq_timedsend.rs");
include!("mq_unlink.rs");
