// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// `A` is either an Internet Protocol Version 4 address (`Ipv4Addr`) or an Internet Protocol Version 6 address (`Ipv6Addr`).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PermittedInternetProtocolVersionAddresses<A>
{
	/// An Internet Protocol address.
	pub address: A,

	/// A bit mask from 0 to 32 for Internet Protocol Version 4 addresses and from 0 to 128 for Internet Protocol Version 6 addresses.
	pub mask: u32,
}
