// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"), not(target_arch = "powerpc"))] pub(crate) const NCCS: usize = 32;
#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"), target_arch = "powerpc")] pub(crate) const NCCS: usize = 19;
#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const NCCS: usize = 20;
