// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


use ::libc::c_int;
use ::libc::c_void;
use ::libc::uint32_t;
use ::libc::uint64_t;
use ::std::cmp::Ordering;
use ::std::fmt;
use ::std::fmt::Debug;
use ::std::fmt::Formatter;
use ::std::hash::Hash;
use ::std::hash::Hasher;
use ::std::mem::zeroed;


/// epoll data associated with an event and registered with a file descriptor (FD).
#[derive(Copy, Clone, Eq)]
pub union epoll_data_t
{
	/// Data as a pointer.
	pub ptr: *mut c_void,

	/// Data as a file descriptor.
	pub fd: c_int,

	/// Data as an arbitrary 32-bit unsigned integer.
	pub u32: uint32_t,

	/// Data as an arbitrary 64-bit unsigned integer.
	pub u64: uint64_t,
}

impl Default for epoll_data_t
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for epoll_data_t
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "{}", unsafe { self.u64 })
	}
}

impl PartialEq for epoll_data_t
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		unsafe { self.u64 == other.u64 }
	}
}

impl Hash for epoll_data_t
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, hasher: &mut H)
	{
		unsafe { self.u64.hash(hasher) }
	}
}

impl PartialOrd for epoll_data_t
{
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		unsafe { self.u64.partial_cmp(&other.u64) }
	}
}

impl Ord for epoll_data_t
{
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering
	{
		unsafe { self.u64.cmp(&other.u64) }
	}
}

/// Represents an event that occurs after waiting on an epoll file descriptor.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct epoll_event
{
	/// A bitfield of events.
	pub events: uint32_t,

	/// An union containing the data associated when epoll_ctl was called.
	pub data: epoll_data_t,
}
