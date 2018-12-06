// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// One of three possible types.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AcceptedConnectionEnum
{
	/// An Internet Protocol (IP) version 4 accepted connection.
	InternetProtocolVersion4(AcceptedConnection<sockaddr_in>),

	/// An Internet Protocol (IP) version 6 accepted connection.
	InternetProtocolVersion6(AcceptedConnection<sockaddr_in6>),

	/// An Unix Domain connection.
	UnixDomain(AcceptedConnection<sockaddr_un>),
}
