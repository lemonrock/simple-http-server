// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// An inotify_event.
///
/// Is of variable length.
pub struct inotify_event
{
	wd: c_int,

	mask: uint32_t,

	/// `cookie` is a unique integer that connects related events.
	///
	/// Currently this is only used for rename events, and allows the resulting pair of `IN_MOVED_FROM` and `IN_MOVED_TO` events to be connected by the application.
	///
	/// For all other event types, cookie is set to 0.
	cookie: uint32_t,

	/// The `len` field counts all of the bytes in `name`, including the null bytes.
	len: uint32_t,

	/// The name field is present only when an event is returned for a file inside a watched directory; it identifies the file pathname relative to the watched directory.
	///
	/// This pathname is null-terminated, and may include further null bytes ('\0') to align subsequent reads to a suitable address boundary.
	name: [c_char; inotify_event::NAME_MAX + 1],
}

impl inotify_event
{
	/// Defined in `limits.h`.
	const NAME_MAX: usize = 255;

	#[inline(always)]
	pub(crate) fn unpopulated() -> Self
	{
		Self
		{
			wd: unsafe { uninitialized() },
			mask: unsafe { uninitialized() },
			cookie: 0,
			len: 0,
			name: unsafe { zeroed() },
		}
	}

	/// Underlying watch descriptor.
	///
	/// `-1` if invalid.
	#[inline(always)]
	pub fn watch_descriptor(&self) -> i32
	{
		self.wd
	}

	/// Underlying watch descriptor is the same as `inotify_watch_descriptor`.
	#[inline(always)]
	pub fn watch_descriptor_is(&self, inotify_watch_descriptor: &InotifyWatchDescriptor) -> bool
	{
		inotify_watch_descriptor.is(self.wd)
	}

	/// Event flags.
	#[inline(always)]
	pub fn flags(&self) -> InotifyEventFlags
	{
		unsafe { transmute(self.mask) }
	}

	/// This is only used for rename events.
	///
	/// It allows the resulting pair of `MovedFrom` and `MovedTo` events to be connected.
	#[inline(always)]
	pub fn cookie(&self) -> u32
	{
		self.cookie
	}

	/// This is only present when an event is returned for a file inside a watched directory.
	#[inline(always)]
	pub fn pathname_relative_to_watch_directory(&self) -> Option<&CStr>
	{
		if self.len == 0
		{
			None
		}
		else
		{
			Some(unsafe { CStr::from_ptr(self.name.as_ptr()) })
		}
	}
}
