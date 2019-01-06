// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(not(any(target_os = "ios", target_os = "macos")), repr(u32))]
#[cfg_attr(all(any(target_os = "ios", target_os = "macos"), target_pointer_width = "32"), repr(u32))]
#[cfg_attr(all(any(target_os = "ios", target_os = "macos"), target_pointer_width = "64"), repr(u64))]
pub enum InputModeFlag
{
	/// Signal interrupt on `BREAK`.
	///
	/// If `IgnoreBreakCondition` is set, a `BREAK` is ignored.
	///
	/// If it is not set but `SignalInterruptOnBreak` is set, then a `BREAK` character causes the input and output queues to be flushed, and if the terminal is the controlling terminal of a foreground process group, it will cause a `SIGINT` to be sent to this foreground process group.
	///
	/// When neither `IgnoreBreakCondition` nor `SignalInterruptOnBreak` are set, a `BREAK` character reads as a null byte, `\0'`, except when `MarkParityErrors` is set, in which case it reads as the sequence `\377 \0 \0`.
	SignalInterruptOnBreak = BRKINT,

	/// Map carriage return, `CR` (`\r`), to new line, `NL`, (also known as line feed, `\n`).
	///
	/// Conflicts with `MapNewLineToCarriageReturn`.
	///
	/// Disabled by `IgnoreCarriageReturn`.
	MapCarriageReturnToNewLine = ICRNL,

	/// Ignore `BREAK` condition.
	IgnoreBreakCondition = IGNBRK,

	/// Ignore carriage return, `CR`.
	///
	/// Disables `MapCarriageReturnToNewLine`.
	IgnoreCarriageReturn = IGNCR,

	/// Ignore characters with parity errors.
	///
	/// (Linux: Ignore framing errors and parity errors).
	IgnoreCharactersWithParityErrors = IGNPAR,

	/// Map new line, `NL`, (also known as line feed, `\n`) to carriage return, `CR` (`\r`).
	///
	/// Conflicts with `MapCarriageReturnToNewLine`.
	MapNewLineToCarriageReturn = INLCR,

	/// Enable input parity checking.
	EnableParityChecking = INPCK,

	/// Strips off the eighth bit of every character.
	StripOffEigthBitOfEveryCharacter = ISTRIP,

	/// Enable any character to restart output.
	///
	/// The default is to allow just the `START` character to restart output.
	AnyCharacterToRestartOutput = IXANY,

	/// Enable `XON/XOFF` flow control (also known as 'start/stop input control') on input.
	EnableXOnXOffFlowControlOnInput = IXOFF,

	/// Enable `XON/XOFF` flow control (also known as 'start/stop output control') on output.
	EnableXOnXOffFlowControlOnOutput = IXON,

	/// Mark parity errors.
	///
	/// If `IgnoreCharactersWithParityErrors` is not set, prefix a character with a parity error or framing error with `\377 \0`.
	///
	/// If neither `IgnoreCharactersWithParityErrors` nor `MarkParityErrors` is set, read a character with a parity error or framing error as `\0`.
	///
	/// See also documentation for `SignalInterruptOnBreak`.
	MarkParityErrors = PARMRK,

	/// Map uppercase to lowercase.
	///
	/// This is considered a legacy feature by later POSIX standards.
	#[cfg(target_os = "android", target_os = "fuschia", target_os = "linux")] MapUppercaseToLowercase = IUCLC,

	/// Ring bell when input queue is full.
	///
	/// Linux does not implement this bit, and acts as if it is always set.
	///
	/// This input flag is not in any POSIX standard.
	RingBellWhenInputQueueIsFull = IMAXBEL,

	/// Input is UTF8.
	///
	/// This allows character-erase to be correctly performed in cooked mode.
	///
	/// This input flag is not in any POSIX standard.
	///
	/// Valid on Linux since Linux 2.6.4 and on iOS / macos.
	#[cfg(target_os = "android", target_os = "fuschia", target_os = "ios", target_os = "linux", target_os = "macos")] Utf8 = IUTF8,
}
