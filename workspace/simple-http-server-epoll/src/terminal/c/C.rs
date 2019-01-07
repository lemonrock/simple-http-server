// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.



#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), not(target_arch = "powerpc")))] pub(crate) const CSIZE: tcflag_t = 0o000060;
#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), target_arch = "powerpc"))] pub(crate) const CSIZE: tcflag_t = 00001400;
#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const CSIZE: tcflag_t = 0x00000300;

#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const CS5: tcflag_t = 0o000000;
#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const CS5: tcflag_t = 0x00000000;

#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), not(target_arch = "powerpc")))] pub(crate) const CS6: tcflag_t = 0o000020;
#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), target_arch = "powerpc"))] pub(crate) const CS6: tcflag_t = 00000400;
#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const CS6: tcflag_t = 0x00000100;

#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), not(target_arch = "powerpc")))] pub(crate) const CS7: tcflag_t = 0o000040;
#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), target_arch = "powerpc"))] pub(crate) const CS7: tcflag_t = 00001000;
#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const CS7: tcflag_t = 0x00000200;

#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), not(target_arch = "powerpc")))] pub(crate) const CS8: tcflag_t = 0o000060;
#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), target_arch = "powerpc"))] pub(crate) const CS8: tcflag_t = 00001400;
#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const CS8: tcflag_t = 0x00000300;

#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), not(target_arch = "powerpc")))] pub(crate) const CSTOPB: tcflag_t = 0o000100;
#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), target_arch = "powerpc"))] pub(crate) const CSTOPB: tcflag_t = 00002000;
#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const CSTOPB: tcflag_t = 0x00000400;

#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), not(target_arch = "powerpc")))] pub(crate) const CREAD: tcflag_t = 0o000200;
#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), target_arch = "powerpc"))] pub(crate) const CREAD: tcflag_t = 00004000;
#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const CREAD: tcflag_t = 0x00000800;

#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), not(target_arch = "powerpc")))] pub(crate) const PARENB: tcflag_t = 0o000400;
#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), target_arch = "powerpc"))] pub(crate) const PARENB: tcflag_t = 00010000;
#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const PARENB: tcflag_t = 0x00001000;

#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), not(target_arch = "powerpc")))] pub(crate) const PARODD: tcflag_t = 0o001000;
#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), target_arch = "powerpc"))] pub(crate) const PARODD: tcflag_t = 00020000;
#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const PARODD: tcflag_t = 0x00002000;

#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), not(target_arch = "powerpc")))] pub(crate) const HUPCL: tcflag_t = 0o002000;
#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), target_arch = "powerpc"))] pub(crate) const HUPCL: tcflag_t = 00040000;
#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const PARODD: tcflag_t = 0x00004000;

#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), not(target_arch = "powerpc")))] pub(crate) const CLOCAL: tcflag_t = 0o004000;
#[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), target_arch = "powerpc"))] pub(crate) const CLOCAL: tcflag_t = 0o0100000;
#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const CLOCAL: tcflag_t = 0x00008000;

#[allow(dead_code)] #[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), not(target_arch = "powerpc")))] pub(crate) const CBAUD: tcflag_t = 0o010017;
#[allow(dead_code)] #[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), target_arch = "powerpc"))] pub(crate) const CBAUD: tcflag_t = 0o0377;

#[allow(dead_code)] #[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), not(target_arch = "powerpc")))] pub(crate) const CBAUDEX: tcflag_t = 0o010000;
#[allow(dead_code)] #[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), target_arch = "powerpc"))] pub(crate) const CBAUDEX: tcflag_t = 0o000020;

#[allow(dead_code)] #[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), not(target_arch = "powerpc")))] pub(crate) const CIBAUD: tcflag_t = 0o02003600000;
#[allow(dead_code)] #[cfg(all(any(target_os = "android", target_os = "fuschia", target_os = "linux"), target_arch = "powerpc"))] pub(crate) const CIBAUD: tcflag_t = 0o77600000;

#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const CMSPAR: tcflag_t = 0o10000000000;

#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub(crate) const CRTSCTS: tcflag_t = 0o20000000000;
#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos"))] pub(crate) const CRTSCTS: tcflag_t = CCTS_OFLOW | CRTS_IFLOW;
#[cfg(target_os = "openbsd")] pub(crate) const CRTSCTS: tcflag_t = 0x00010000;

#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const CIGNORE: tcflag_t = 0x00000001;

#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos"))] pub(crate) const CCTS_OFLOW: tcflag_t = 0x00010000;
#[cfg(target_os = "openbsd")] pub(crate) const CCTS_OFLOW: tcflag_t = CRTSCTS;

#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos"))] pub(crate) const CRTS_IFLOW: tcflag_t = 0x00020000;
#[cfg(target_os = "openbsd")] pub(crate) const CRTS_IFLOW: tcflag_t = CRTSCTS;

#[cfg(any(target_os = "dragonfly", target_os = "ios", target_os = "macos", target_os = "openbsd"))] pub(crate) const MDMBUF: tcflag_t = 0x00100000;
#[cfg(target_os = "freebsd")] pub(crate) const MDMBUF: tcflag_t = CCAR_OFLOW;

#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos"))] pub(crate) const CDTR_IFLOW: tcflag_t = 0x00040000;

#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos"))] pub(crate) const CDSR_OFLOW: tcflag_t = 0x00080000;

#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos"))] pub(crate) const CCAR_OFLOW: tcflag_t = 0x00100000;
