// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright ©2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// 
pub(crate) const TCP_NODELAY: c_int = 1;

/// 
pub(crate) const TCP_MAXSEG: c_int = 2;

/// 
pub(crate) const TCP_CORK: c_int = 3;

/// 
pub(crate) const TCP_KEEPIDLE: c_int = 4;

/// 
pub(crate) const TCP_KEEPINTVL: c_int = 5;

/// 
pub(crate) const TCP_KEEPCNT: c_int = 6;

/// 
pub(crate) const TCP_SYNCNT: c_int = 7;

/// 
pub(crate) const TCP_LINGER2: c_int = 8;

/// 
pub(crate) const TCP_DEFER_ACCEPT: c_int = 9;

/// 
pub(crate) const TCP_WINDOW_CLAMP: c_int = 10;

/// 
pub(crate) const TCP_INFO: c_int = 11;

/// 
pub(crate) const TCP_QUICKACK: c_int = 12;

/// 
pub(crate) const TCP_CONGESTION: c_int = 13;

/// 
pub(crate) const TCP_MD5SIG: c_int = 14;

/// 
pub(crate) const TCP_THIN_LINEAR_TIMEOUTS: c_int = 16;

/// 
pub(crate) const TCP_THIN_DUPACK: c_int = 17;

/// 
pub(crate) const TCP_USER_TIMEOUT: c_int = 18;

/// 
pub(crate) const TCP_REPAIR: c_int = 19;

/// 
pub(crate) const TCP_REPAIR_QUEUE: c_int = 20;

/// 
pub(crate) const TCP_QUEUE_SEQ: c_int = 21;

/// 
pub(crate) const TCP_REPAIR_OPTIONS: c_int = 22;

/// 
pub(crate) const TCP_FASTOPEN: c_int = 23;

/// 
pub(crate) const TCP_TIMESTAMP: c_int = 24;

/// 
pub(crate) const TCP_NOTSENT_LOWAT: c_int = 25;

/// 
pub(crate) const TCP_CC_INFO: c_int = 26;

/// 
pub(crate) const TCP_SAVE_SYN: c_int = 27;

/// 
pub(crate) const TCP_SAVED_SYN: c_int = 28;

/// 
pub(crate) const TCP_REPAIR_WINDOW: c_int = 29;

/// 
pub(crate) const TCP_FASTOPEN_CONNECT: c_int = 30;

/// 
pub(crate) const TCP_ULP: c_int = 31;

/// 
pub(crate) const TCP_MD5SIG_EXT: c_int = 32;

/// 
pub(crate) const TCP_FASTOPEN_KEY: c_int = 33;

/// 
pub(crate) const TCP_FASTOPEN_NO_COOKIE: c_int = 34;

/// 
pub(crate) const TCP_ZEROCOPY_RECEIVE: c_int = 35;

/// 
pub(crate) const TCP_INQ: c_int = 36;
