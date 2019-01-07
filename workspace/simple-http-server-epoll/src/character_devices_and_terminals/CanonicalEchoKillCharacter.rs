// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Abstracts the canonical setting of the echoing of the `KILL` character.
///
/// Defaults to `CanonicalEchoKillCharacter::Off`, which is the most sensible choice.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(not(any(target_os = "ios", target_os = "macos")), repr(u32))]
#[cfg_attr(all(any(target_os = "ios", target_os = "macos"), target_pointer_width = "32"), repr(u32))]
#[cfg_attr(all(any(target_os = "ios", target_os = "macos"), target_pointer_width = "64"), repr(u64))]
pub enum CanonicalEchoKillCharacter
{
	/// No echoing.
	///
	/// This is the default.
	Off = 0,

	/// If set then the `KILL` character either erases the current line from the display or outputs the new line, `NL`, character (to emphasize that the entire line was erased).
	///
	/// This flag is disabled by setting the `ECHOKE` flag.
	///
	/// This occurs irrespective of whether the terminal has been set to echo its input.
	///
	/// Equivalent to the `ECHOK` flag.
	K = ECHOK,

	/// If set then the `KILL` character is echoed by erasing each character on the line.
	/// The way in which each character is erased is selected by the `ECHOE` and `ECHOPRT` flags.
	///
	/// Setting this flag disables the `ECHOK` flag.
	///
	/// This occurs irrespective of whether the terminal has been set to echo its input.
	///
	/// Equivalent to the `ECHOKE` flag.
	Erase = ECHOKE,
}

impl Default for CanonicalEchoKillCharacter
{
	#[inline(always)]
	fn default() -> Self
	{
		CanonicalEchoKillCharacter::Off
	}
}
