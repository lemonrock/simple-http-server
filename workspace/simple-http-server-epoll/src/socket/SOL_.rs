// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// MIPS just has to be different.
#[cfg(all(any(target_arch = "android", target_arch = "linux"), any(target_arch = "mips", target_arch = "mips64", target_arch = "sparc64")))] pub(crate) const SOL_SOCKET: c_int = 0xFFFF;
#[cfg(not(all(any(target_arch = "android", target_arch = "linux"), any(target_arch = "mips", target_arch = "mips64", target_arch = "sparc64"))))] pub(crate) const SOL_SOCKET: c_int = 1;

///
pub(crate) const SOL_TCP: c_int = 6;

///
#[allow(dead_code)]
pub(crate) const SOL_UDP: c_int = 17;
