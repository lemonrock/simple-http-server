// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux", not(target_arch = "powerpc")))] pub(crate) const ECHOKE: tcflag_t = 0o004000;
#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux", target_arch = "powerpc"))] pub(crate) const ECHOKE: tcflag_t = 0x00000001;
#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const ECHOKE: tcflag_t = 0x00000001;

#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux", not(target_arch = "powerpc")))] pub(crate) const ECHOE: tcflag_t = 0o000020;
#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux", target_arch = "powerpc"))] pub(crate) const ECHOE: tcflag_t = 0x00000002;
#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const ECHOE: tcflag_t = 0x00000002;

#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux", not(target_arch = "powerpc")))] pub(crate) const ECHOK: tcflag_t = 0o000040;
#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux", target_arch = "powerpc"))] pub(crate) const ECHOK: tcflag_t = 0x00000004;
#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const ECHOK: tcflag_t = 0x00000004;

#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux", not(target_arch = "powerpc")))] pub(crate) const ECHO: tcflag_t = 0o000010;
#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux", target_arch = "powerpc"))] pub(crate) const ECHO: tcflag_t = 0x00000008;
#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const ECHO: tcflag_t = 0x00000008;

#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux", not(target_arch = "powerpc")))] pub(crate) const ECHONL: tcflag_t = 0o000100;
#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux", target_arch = "powerpc"))] pub(crate) const ECHONL: tcflag_t = 0x00000010;
#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const ECHONL: tcflag_t = 0x00000010;

#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux", not(target_arch = "powerpc")))] pub(crate) const ECHOPRT: tcflag_t = 0o002000;
#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux", target_arch = "powerpc"))] pub(crate) const ECHOPRT: tcflag_t = 0x00000020;
#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const ECHOPRT: tcflag_t = 0x00000020;

#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux", not(target_arch = "powerpc")))] pub(crate) const ECHOCTL: tcflag_t = 0o001000;
#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux", target_arch = "powerpc"))] pub(crate) const ECHOCTL: tcflag_t = 0x00000040;
#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const ECHOCTL: tcflag_t = 0x00000040;

#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux", not(target_arch = "powerpc")))] pub(crate) const ISIG: tcflag_t = 0o000001;
#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux", target_arch = "powerpc"))] pub(crate) const ISIG: tcflag_t = 0x00000080;
#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const ISIG: tcflag_t = 0x00000080;

#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux", not(target_arch = "powerpc")))] pub(crate) const ICANON: tcflag_t = 0o000002;
#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux", target_arch = "powerpc"))] pub(crate) const ICANON: tcflag_t = 0x00000100;
#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const ICANON: tcflag_t = 0x00000100;

#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux", not(target_arch = "powerpc")))] pub(crate) const IEXTEN: tcflag_t = 0o100000;
#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux", target_arch = "powerpc"))] pub(crate) const IEXTEN: tcflag_t = 0x00000400;
#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const IEXTEN: tcflag_t = 0x00000400;

#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux", not(target_arch = "powerpc")))] pub(crate) const TOSTOP: tcflag_t = 0o000400;
#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux", target_arch = "powerpc"))] pub(crate) const TOSTOP: tcflag_t = 0x00400000;
#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const TOSTOP: tcflag_t = 0x00400000;

#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux", not(target_arch = "powerpc")))] pub(crate) const NOFLSH: tcflag_t = 0o000200;
#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux", target_arch = "powerpc"))] pub(crate) const NOFLSH: tcflag_t = 0x80000000;
#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const NOFLSH: tcflag_t = 0x80000000;

#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux", not(target_arch = "powerpc")))] pub(crate) const EXTPROC: tcflag_t = 0o200000;
#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux", target_arch = "powerpc"))] pub(crate) const EXTPROC: tcflag_t = 0x10000000;
#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const EXTPROC: tcflag_t = 0x00000800;

#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux", not(target_arch = "powerpc")))] pub(crate) const FLUSHO: tcflag_t = 0o010000;
#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux", target_arch = "powerpc"))] pub(crate) const FLUSHO: tcflag_t = 0x00800000;
#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const FLUSHO: tcflag_t = 0x00800000;

#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux", not(target_arch = "powerpc")))] pub(crate) const PENDIN: tcflag_t = 0o040000;
#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux", target_arch = "powerpc"))] pub(crate) const PENDIN: tcflag_t = 0x20000000;
#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const PENDIN: tcflag_t = 0x20000000;

#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux", not(target_arch = "powerpc")))] pub(crate) const XCASE: tcflag_t = 0o000004;
#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux", target_arch = "powerpc"))] pub(crate) const XCASE: tcflag_t = 0x00004000;

#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const NOKERNINFO: tcflag_t = 0x02000000;

#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const ALTWERASE: tcflag_t = 0x00000200;
