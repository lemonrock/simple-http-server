// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


cfg_if!
{
	if #[cfg(all(any(target_arch = "android", target_arch = "linux"), any(target_arch = "mips", target_arch = "mips64", target_arch = "sparc64")))]
	{
		pub(crate) const SO_DEBUG: c_int = 1;

		pub(crate) const SO_REUSEADDR: c_int = 0x0004;

		pub(crate) const SO_KEEPALIVE: c_int = 0x0008;

		pub(crate) const SO_DONTROUTE: c_int = 0x0010;

		pub(crate) const SO_BROADCAST: c_int = 0x0020;

		pub(crate) const SO_LINGER: c_int = 0x0080;

		pub(crate) const SO_OOBINLINE: c_int = 0x0100;

		pub(crate) const SO_REUSEPORT: c_int = 0x0200;

		pub(crate) const SO_SNDBUF: c_int = 0x1001;

		pub(crate) const SO_RCVBUF: c_int = 0x1002;

		pub(crate) const SO_SNDLOWAT: c_int = 0x1003;

		pub(crate) const SO_RCVLOWAT: c_int = 0x1004;

		pub(crate) const SO_RCVTIMEO: c_int = 0x1006;

		pub(crate) const SO_SNDTIMEO: c_int = 0x1005;

		pub(crate) const SO_ERROR: c_int = 0x1007;

		pub(crate) const SO_TYPE: c_int = 0x1008;

		pub(crate) const SO_ACCEPTCONN: c_int = 0x1009;

		pub(crate) const SO_PROTOCOL: c_int = 0x1028;

		pub(crate) const SO_DOMAIN: c_int = 0x1029;

		pub(crate) const SO_NO_CHECK: c_int = 11;

		pub(crate) const SO_PRIORITY: c_int = 12;

		pub(crate) const SO_BSDCOMPAT: c_int = 14;

		pub(crate) const SO_PASSCRED: c_int = 17;

		pub(crate) const SO_PEERCRED: c_int = 18;

		pub(crate) const SO_PEERSEC: c_int = 30;

		pub(crate) const SO_SNDBUFFORCE: c_int = 31;

		pub(crate) const SO_RCVBUFFORCE: c_int = 33;
	}
	else if #[cfg(all(any(target_arch = "android", target_arch = "linux"), any(target_arch = "powerpc", target_arch = "powerpc64")))]
	{
		pub(crate) const SO_DEBUG: c_int = 1;

		pub(crate) const SO_REUSEADDR: c_int = 2;

		pub(crate) const SO_TYPE: c_int = 3;

		pub(crate) const SO_ERROR: c_int = 4;

		pub(crate) const SO_DONTROUTE: c_int = 5;

		pub(crate) const SO_BROADCAST: c_int = 6;

		pub(crate) const SO_SNDBUF: c_int = 7;

		pub(crate) const SO_RCVBUF: c_int = 8;

		pub(crate) const SO_KEEPALIVE: c_int = 9;

		pub(crate) const SO_OOBINLINE: c_int = 10;

		pub(crate) const SO_NO_CHECK: c_int = 11;

		pub(crate) const SO_PRIORITY: c_int = 12;

		pub(crate) const SO_LINGER: c_int = 13;

		pub(crate) const SO_BSDCOMPAT: c_int = 14;

		pub(crate) const SO_REUSEPORT: c_int = 15;


		// For some odd reason, these values differ for the PowerPC architecture from the generic values (below).

		pub(crate) const SO_RCVLOWAT: c_int = 16;

		pub(crate) const SO_SNDLOWAT: c_int = 17;

		pub(crate) const SO_RCVTIMEO: c_int = 18;

		pub(crate) const SO_SNDTIMEO: c_int = 19;

		pub(crate) const SO_PASSCRED: c_int = 20;

		pub(crate) const SO_PEERCRED: c_int = 21;


		pub(crate) const SO_ACCEPTCONN: c_int = 30;

		pub(crate) const SO_PEERSEC: c_int = 31;

		pub(crate) const SO_SNDBUFFORCE: c_int = 32;

		pub(crate) const SO_RCVBUFFORCE: c_int = 33;

		pub(crate) const SO_PROTOCOL: c_int = 38;

		pub(crate) const SO_DOMAIN: c_int = 39;
	}
	else
	{
		pub(crate) const SO_DEBUG: c_int = 1;

		pub(crate) const SO_REUSEADDR: c_int = 2;

		pub(crate) const SO_TYPE: c_int = 3;

		pub(crate) const SO_ERROR: c_int = 4;

		pub(crate) const SO_DONTROUTE: c_int = 5;

		pub(crate) const SO_BROADCAST: c_int = 6;

		pub(crate) const SO_SNDBUF: c_int = 7;

		pub(crate) const SO_RCVBUF: c_int = 8;

		pub(crate) const SO_KEEPALIVE: c_int = 9;

		pub(crate) const SO_OOBINLINE: c_int = 10;

		pub(crate) const SO_NO_CHECK: c_int = 11;

		pub(crate) const SO_PRIORITY: c_int = 12;

		pub(crate) const SO_LINGER: c_int = 13;

		pub(crate) const SO_BSDCOMPAT: c_int = 14;

		pub(crate) const SO_REUSEPORT: c_int = 15;


		pub(crate) const SO_PASSCRED: c_int = 16;

		pub(crate) const SO_PEERCRED: c_int = 17;

		pub(crate) const SO_RCVLOWAT: c_int = 18;

		pub(crate) const SO_SNDLOWAT: c_int = 19;

		pub(crate) const SO_RCVTIMEO: c_int = 20;

		pub(crate) const SO_SNDTIMEO: c_int = 21;


		pub(crate) const SO_ACCEPTCONN: c_int = 30;

		pub(crate) const SO_PEERSEC: c_int = 31;

		pub(crate) const SO_SNDBUFFORCE: c_int = 32;

		pub(crate) const SO_RCVBUFFORCE: c_int = 33;

		pub(crate) const SO_PROTOCOL: c_int = 38;

		pub(crate) const SO_DOMAIN: c_int = 39;
	}
}

pub(crate) const SO_SECURITY_AUTHENTICATION: c_int = 22;

pub(crate) const SO_SECURITY_ENCRYPTION_TRANSPORT: c_int = 23;

pub(crate) const SO_SECURITY_ENCRYPTION_NETWORK: c_int = 24;

pub(crate) const SO_BINDTODEVICE: c_int = 25;

pub(crate) const SO_ATTACH_FILTER: c_int = 26;

pub(crate) const SO_DETACH_FILTER: c_int = 27;

pub(crate) const SO_GET_FILTER: c_int = SO_ATTACH_FILTER;

pub(crate) const SO_PEERNAME: c_int = 28;

pub(crate) const SO_TIMESTAMP: c_int = 29;

pub(crate) const SO_PASSSEC: c_int = 34;

pub(crate) const SO_TIMESTAMPNS: c_int = 35;

pub(crate) const SO_MARK: c_int = 36;

pub(crate) const SO_TIMESTAMPING: c_int = 37;

pub(crate) const SO_RXQ_OVFL: c_int = 40;

pub(crate) const SO_WIFI_STATUS: c_int = 41;

pub(crate) const SO_PEEK_OFF: c_int = 42;

pub(crate) const SO_NOFCS: c_int = 43;

pub(crate) const SO_LOCK_FILTER: c_int = 44;

pub(crate) const SO_SELECT_ERR_QUEUE: c_int = 45;

pub(crate) const SO_BUSY_POLL: c_int = 46;

pub(crate) const SO_MAX_PACING_RATE: c_int = 47;

pub(crate) const SO_BPF_EXTENSIONS: c_int = 48;

pub(crate) const SO_INCOMING_CPU: c_int = 49;

pub(crate) const SO_ATTACH_BPF: c_int = 50;

pub(crate) const SO_DETACH_BPF: c_int = SO_DETACH_FILTER;

pub(crate) const SO_ATTACH_REUSEPORT_CBPF: c_int = 51;

pub(crate) const SO_ATTACH_REUSEPORT_EBPF: c_int = 52;

pub(crate) const SO_CNX_ADVICE: c_int = 53;

pub(crate) const SO_MEMINFO: c_int = 55;

pub(crate) const SO_INCOMING_NAPI_ID: c_int = 56;

pub(crate) const SO_COOKIE: c_int = 57;

pub(crate) const SO_PEERGROUPS: c_int = 59;

pub(crate) const SO_ZEROCOPY: c_int = 60;
