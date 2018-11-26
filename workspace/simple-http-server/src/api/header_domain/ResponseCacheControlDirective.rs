// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// A valid value for a `Cache-Control` header when used in a HTTP response.
#[derive(Debug, Copy, Clone, PartialOrd, Ord, Hash)]
pub enum ResponseCacheControlDirective
{
	/// `must-revalidate`.
	must_revalidate,

	/// `no-cache`.
	no_cache,

	/// `no-store`.
	no_store,

	/// `no-transform`.
	no_transform,

	public,

	private,

	/// `proxy-revalidate`.
	proxy_revalidate,

	/// `max-age`.
	max_age
	{
		seconds: usize,
	},

	/// `s-maxage`.
	s_maxage
	{
		seconds: usize,
	},

	/// This is an extension to the core specification.
	immutable,

	/// This is an extension to the core specification and is not widely supported.
	///
	/// `stale-while-revalidate'
	stale_while_revalidate
	{
		seconds: usize,
	},

	/// This is an extension to the core specification and is not widely supported.
	///
	/// `stale-if-error'
	stale_if_error
	{
		seconds: usize,
	},
}

impl PartialEq for ResponseCacheControlDirective
{
	fn eq(&self, other: &Self) -> bool
	{
		use self::RobotTag::*;

		match (self, other)
		{
			(must_revalidate, must_revalidate) => true,

			(no_cache, no_cache) => true,

			(no_store, no_store) => true,

			(no_transform, no_transform) => true,

			(public, public) => true,

			(private, private) => true,

			(proxy_revalidate, proxy_revalidate) => true,

			(max_age { .. }, max_age { .. }) => true,

			(s_maxage { .. }, s_maxage { .. }) => true,

			(immutable, immutable) => true,

			(stale_while_revalidate { .. }, stale_while_revalidate) => true,

			(stale_if_error { .. }, stale_if_error) => true,

			(_, _) => false,
		}
	}
}

impl Eq for ResponseCacheControlDirective
{
}

impl ResponseCacheControlDirective
{
	#[inline(always)]
	fn buffer(&self) -> Cow<'static, [u8]>
	{
		use self::ResponseCacheControlDirective::*;
		use self::Cow::Borrowed;

		match *self
		{
			must_revalidate => Borrowed(b"must-revalidate"),

			no_cache => Borrowed(b"no-cache"),

			no_store => Borrowed(b"no-store"),

			no_transform => Borrowed(b"no-transform"),

			public => Borrowed(b"public"),

			private => Borrowed(b"private"),

			proxy_revalidate => Borrowed(b"proxy-revalidate"),

			max_age { seconds } => Self::with_seconds(b"max-age", seconds),

			s_maxage { seconds } => Self::with_seconds(b"s-maxage=", seconds),

			immutable => Borrowed(b"immutable"),

			stale_while_revalidate { seconds } => Self::with_seconds(b"stale-while-revalidate=", seconds),

			stale_if_error { seconds } => Self::with_seconds("stale-if-error", seconds),
		}
	}

	#[inline(always)]
	fn with_leading_comma_buffer(&self) -> Cow<'static, [u8]>
	{
		use self::ResponseCacheControlDirective::*;
		use self::Cow::*;

		match *self
		{
			must_revalidate => Borrowed(b",must-revalidate"),

			no_cache => Borrowed(b",no-cache"),

			no_store => Borrowed(b",no-store"),

			no_transform => Borrowed(b",no-transform"),

			public => Borrowed(b",public"),

			private => Borrowed(b",private"),

			proxy_revalidate => Borrowed(b",proxy-revalidate"),

			max_age { seconds } => Self::with_seconds(b",max-age", seconds),

			s_maxage { seconds } => Self::with_seconds(b",s-maxage=", seconds),

			immutable => Borrowed(b",immutable"),

			stale_while_revalidate { seconds } => Self::with_seconds(b",stale-while-revalidate=", seconds),

			stale_if_error { seconds } => Self::with_seconds(",stale-if-error", seconds),
		}
	}

	fn with_seconds(preamble: &[u8], seconds: usize) -> Cow<'static, [u8]>
	{
		let mut array = Vec::with_capacity_for_usize(preamble.len(), 0);
		array.extend_from_slice(preamble);
		array.write_usize(seconds);
		Cow::Owned(array)
	}
}
