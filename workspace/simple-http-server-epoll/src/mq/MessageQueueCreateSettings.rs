// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Settings for creating a queue.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct MessageQueueCreateSettings
{
	/// File-like permissions to use.
	pub permissions: mode_t,

	/// Optional create settings.
	///
	/// If `None`, then Linux applies a default (see documentation of fields on `OptionalMessageQueueCreateSettings`).
	pub optional_create_settings: Option<OptionalMessageQueueCreateSettings>,
}

impl Default for MessageQueueCreateSettings
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			permissions: S_IRUSR | S_IWUSR,
			optional_create_settings: None,
		}
	}
}

impl MessageQueueCreateSettings
{
	#[inline(always)]
	pub(crate) fn invoke_mq_open(&self, name_pointer: *const c_char, oflag: i32) -> c_int
	{
		let mode = self.permissions;

		match self.optional_create_settings
		{
			None => unsafe { mq_open(name_pointer, oflag, mode, null_mut::<mq_attr>()) },

			Some(ref optional_create_settings) =>
			{
				let mut attributes = mq_attr::for_create(optional_create_settings);
				unsafe { mq_open(name_pointer, oflag, mode, &mut attributes) }
			}
		}
	}
}
