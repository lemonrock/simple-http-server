// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


pub trait TargetUriUser
{
	/// Will be called at least once.
	///
	/// Will be followed by either a call to `use_query()` or to `finished()`.
	///
	/// No percent-decoding is performed but values are checked for byte validity.
	///
	/// A decoder of the segment could assume that it is %-encoded UTF-8 string.
	fn use_segment(&mut self, segment_starts_at_inclusive: NonNull<u8>, segment_ends_at_exclusive: NonNull<u8>) -> Result<(), InvalidReason>;

	/// Will not be called if there is no query,
	///
	/// Will be called if there is an empty query (a query of zero length).
	///
	/// Will only be called once.
	///
	/// No percent-decoding is performed but values are checked for byte validity.
	///
	/// A decoder of the query could assume that it is %-encoded UTF-8 string.
	fn use_query(&mut self, query_starts_at_inclusive: NonNull<u8>, query_ends_at_exclusive: NonNull<u8>) -> Result<(), InvalidReason>;

	/// Called either after the final call to `use_segment()` or after the call to `use_query()`.
	fn finished(&mut self) -> Result<(), InvalidReason>;
}
