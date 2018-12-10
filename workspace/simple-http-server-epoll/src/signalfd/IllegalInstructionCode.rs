// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Represents a code that can be associated with a kernel-raised `SIGILL` signal.
///
/// Definitions valid as of Linux v4.20-rc5.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
pub enum IllegalInstructionCode
{
	/// Illegal opcode.
	///
	/// Known as `ILL_ILLOPC` in Linux sources.
	IllegalOpcode = 1,

	/// Illegal operand.
	///
	/// Known as `ILL_ILLOPN` in Linux sources.
	IllegalOperand = 2,

	/// Illegal addressing mode.
	///
	/// Known as `ILL_ILLADR` in Linux sources.
	IllegalAddressingMode = 3,

	/// Illegal trap.
	///
	/// Known as `ILL_ILLTRP` in Linux sources.
	IllegalTrap = 4,

	/// Priveleged opcode (eg using a ring 0 opcode in ring 3 on x86).
	///
	/// Known as `ILL_PRVOPC` in Linux sources.
	PrivelegedOpcode = 5,

	/// Priveleged register (eg using a ring 0 only register in ring 3 on x86).
	///
	/// Known as `ILL_PRVOPC` in Linux sources.
	PrivelegedRegister = 6,

	/// Coprocessor error.
	///
	/// Known as `ILL_COPROC` in Linux sources.
	CoprocessorError = 7,

	/// Internal stack error.
	///
	/// Known as `ILL_BADSTK` in Linux sources.
	InternalStackError = 8,

	/// Unimplemented instruction address.
	///
	/// Known as `ILL_BADIADDR` in Linux sources.
	UnimplementedInstructionAddress = 9,

	/// Illegal break.
	///
	/// Known as `__ILL_BREAK` in Linux sources.
	IllegalBreak = 10,

	/// Bundle-update (modification) in progress.
	///
	/// Known as `__ILL_BNDMOD` in Linux sources.
	BundleUpdateInProgress = 11,
}

impl Into<i32> for IllegalInstructionCode
{
	#[inline(always)]
	fn into(self) -> i32
	{
		self as i32
	}
}

impl Code for IllegalInstructionCode
{
	type Data = FaultData;

	const InclusiveMaximum: Self = IllegalInstructionCode::BundleUpdateInProgress;

	#[inline(always)]
	fn convert(code: i32) -> Self
	{
		unsafe { transmute(code) }
	}
}
