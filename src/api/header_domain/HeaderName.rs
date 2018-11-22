// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Represents a header name as used in a `Vary` or `Access-Control-Allow-Headers` header.
///
/// The field `leading_comma_then_name` is a convenience but there is no easy way to construct it using macros or constant logic.
///
/// A typical header might `HeaderName { name: b"Content-Encoding", b",Content-Encoding" }`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HeaderName
{
	pub name: &'static [u8],
	pub leading_comma_then_name: &'static [u8],
}

impl HeaderName
{
	/// Represents the wildcard header name `*`
	pub const Wildcard: Self = HeaderName
	{
		name: b"*",
		leading_comma_then_name: b",*",
	};

	/// Is the wildcard (asterisk) header name?
	#[inline(always)]
	pub fn is_wildcard(&self) -> bool
	{
		self.name == Self::Wildcard.name
	}
}
