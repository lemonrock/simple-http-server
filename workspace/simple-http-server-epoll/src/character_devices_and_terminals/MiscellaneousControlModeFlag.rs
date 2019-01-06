// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Miscellaneous control mode flags.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(not(any(target_os = "ios", target_os = "macos")), repr(u32))]
#[cfg_attr(all(any(target_os = "ios", target_os = "macos"), target_pointer_width = "32"), repr(u32))]
#[cfg_attr(all(any(target_os = "ios", target_os = "macos"), target_pointer_width = "64"), repr(u64))]
pub enum MiscellaneousControlModeFlag
{
	/// Enable receiver.
	EnableReceiver = CREAD,

	/// Hang up on last close.
	HangUpOnLastClose = HUPCL,

	/// Ignore modem status lines (actually, ignores only the `CD` signal).
	IgnoreModemStatusLines = CLOCAL,

	/// Ignore control flags.
	#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] Ignore = CIGNORE,

	/// Enable hardware flow control of the input and output, using the Clear-To-Send (`CTS`) and Request-To-Send (`RTS`) RS-232 signals.
	///
	/// Can be separated into `RequestToSendFlowControlOfInput` and `ClearToSendFlowControlOfOutput` on some platforms.
	RequestToSendClearToSendFlowControlOfInputAndOutput = CRTSCTS,

	/// Enable hardware flow control of the output using the Clear-To-Send (`CTS`) RS-232 signal.
	#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos"))] ClearToSendFlowControlOfOutput = CCTS_OFLOW,

	/// Enable hardware flow control of the input using the Request-To-Send (`RTS`) RS-232 signal.
	#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos"))] RequestToSendFlowControlOfInput = CRTS_IFLOW,

	/// Enable hardware flow control of the input according to the Data-Terminal-Ready (`DTR`) RS-232 signal.
	#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos"))] DataTerminalReadyFlowControlOfInput = CDTR_IFLOW,

	/// Enable hardware flow control of the output according to the Data-Set-Ready (`DSR`) RS-232 signal.
	#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos"))] DataSetReadyFlowControlOfOutput = CDSR_OFLOW,

	/// Enable hardware flow control of the output using the Data-Carrier-Detect (`DCD`, also known as `CD`) RS-232 modem carrier signal.
	#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos"))] DataCarrierDetectFlowControlOfOutput = CCAR_OFLOW,
	#[cfg(target_os = "openbsd")] DataCarrierDetectFlowControlOfOutput = MDMBUF,
}

impl MiscellaneousControlModeFlag
{
	/// Enable hardware flow control of the output using the Clear-To-Send (`CTS`) RS-232 signal.
	#[cfg(target_os = "openbsd")] pub const ClearToSendFlowControlOfOutput: Self = MiscellaneousControlModeFlag::CRTSCTS;

	/// Enable hardware flow control of the input using the Request-To-Send (`RTS`) RS-232 signal.
	#[cfg(target_os = "openbsd")] pub const RequestToSendFlowControlOfInput: Self = MiscellaneousControlModeFlag::CRTSCTS;
}
