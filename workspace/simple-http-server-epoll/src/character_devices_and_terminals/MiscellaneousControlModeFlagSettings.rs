// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Choices for output mode flag control.
///
/// Default does nothing.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MiscellaneousControlModeFlagSettings(BTreeMap<MiscellaneousControlModeFlag, FlagSetting>);

impl Default for MiscellaneousControlModeFlagSettings
{
	#[inline(always)]
	fn default() -> Self
	{
		use self::MiscellaneousControlModeFlag::*;
		use self::FlagSetting::*;

		Self(BTreeMap::new())
	}
}

impl From<BTreeMap<MiscellaneousControlModeFlag, FlagSetting>> for MiscellaneousControlModeFlagSettings
{
	#[inline(always)]
	fn from(map: BTreeMap<MiscellaneousControlModeFlag, FlagSetting>) -> Self
	{
		Self(map)
	}
}

impl Into<BTreeMap<MiscellaneousControlModeFlag, FlagSetting>> for MiscellaneousControlModeFlagSettings
{
	#[inline(always)]
	fn into(self) -> BTreeMap<MiscellaneousControlModeFlag, FlagSetting>
	{
		self.0
	}
}

impl Deref for MiscellaneousControlModeFlagSettings
{
	type Target = BTreeMap<MiscellaneousControlModeFlag, FlagSetting>;

	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl DerefMut for MiscellaneousControlModeFlagSettings
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		&mut self.0
	}
}

impl MiscellaneousControlModeFlagSettings
{
	#[inline(always)]
	pub(crate) fn change_mode_flags(&self, existing_flags: tcflag_t) -> tcflag_t
	{
		use self::FlagSetting::*;

		let mut flags_on = 0;
		let mut flags_off = 0;

		for (flag, setting) in self.0.iter()
		{
			let flag_value = flag.into();

			match setting
			{
				On => flags_on |= flag_value,
				Off => flags_off |= flag_value,
				Inherit => (),
			}
		}

		(existing_flags | flags_on) & !flags_on
	}

	#[inline(always)]
	pub(crate) fn from_mode_flags(control_mode_flags: tcflag_t) -> Self
	{
		let mut this = Self(BTreeMap::new());

		use self::MiscellaneousControlModeFlag::*;
		use self::FlagSetting::*;

		this.insert_flag_setting(EnableReceiver, control_mode_flags);
		this.insert_flag_setting(HangUpOnLastClose, control_mode_flags);
		this.insert_flag_setting(IgnoreModemStatusLines, control_mode_flags);
		#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] this.insert_flag_setting(Ignore, control_mode_flags);
		this.insert_flag_setting(RequestToSendClearToSendFlowControlOfInputAndOutput, control_mode_flags);
		#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] this.insert_flag_setting(ClearToSendFlowControlOfOutput, control_mode_flags);
		#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] this.insert_flag_setting(RequestToSendFlowControlOfInput, control_mode_flags);
		#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos"))] this.insert_flag_setting(DataTerminalReadyFlowControlOfInput, control_mode_flags);
		#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos"))] this.insert_flag_setting(DataSetReadyFlowControlOfOutput, control_mode_flags);
		#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] this.insert_flag_setting(DataCarrierDetectFlowControlOfOutput, control_mode_flags);

		this
	}

	#[inline(always)]
	fn insert_flag_setting(&mut self, miscellaneous_control_mode_flag: MiscellaneousControlModeFlag, control_mode_flags: tcflag_t)
	{
		let flag_setting = FlagSetting::from(control_mode_flags & miscellaneous_control_mode_flag.into() != 0);
		this.insert(miscellaneous_control_mode_flag, flag_setting);
	}
}
