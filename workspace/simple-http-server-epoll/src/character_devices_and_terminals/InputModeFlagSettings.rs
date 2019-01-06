// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Choices for input mode flag control.
///
/// Default is sane for a raw terminal (no XON/XOFF flow control on output, no mapping or ignoring of characters, 8-bit clean, no signal interrupts, no ignoring of break conditions.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct InputModeFlagSettings(BTreeMap<MiscellaneousOutputModeFlag, FlagSetting>);

impl Default for InputModeFlagSettings
{
	#[inline(always)]
	fn default() -> Self
	{
		use self::InputModeFlag::*;
		use self::FlagSetting::*;

		let mut this = Self(BTreeMap::new());

		this.insert(IgnoreBreakCondition, Off);
		this.insert(SignalInterruptOnBreak, Off);
		this.insert(MarkParityErrors, Off);
		this.insert(StripOffEigthBitOfEveryCharacter, Off);
		this.insert(MapNewLineToCarriageReturn, Off);
		this.insert(IgnoreCarriageReturn, Off);
		this.insert(MapCarriageReturnToNewLine, Off);
		this.insert(EnableXOnXOffFlowControlOnOutput, Off);
		this.insert(AnyCharacterToRestartOutput, Off);
		#[cfg(target_os = "android", target_os = "fuschia", target_os = "linux")] this.insert(MapUppercaseToLowercase, Off);
		this.insert(RingBellWhenInputQueueIsFull, Off);
		#[cfg(target_os = "android", target_os = "fuschia", target_os = "ios", target_os = "linux", target_os = "macos")] this.insert(Utf8, Off);

		this
	}
}

impl From<BTreeMap<MiscellaneousOutputModeFlag, FlagSetting>> for InputModeFlagSettings
{
	#[inline(always)]
	fn from(map: BTreeMap<MiscellaneousOutputModeFlag, FlagSetting>) -> Self
	{
		Self(map)
	}
}

impl Into<BTreeMap<MiscellaneousOutputModeFlag, FlagSetting>> for InputModeFlagSettings
{
	#[inline(always)]
	fn into(self) -> BTreeMap<MiscellaneousOutputModeFlag, FlagSetting>
	{
		self.0
	}
}

impl Deref for InputModeFlagSettings
{
	type Target = BTreeMap<MiscellaneousOutputModeFlag, FlagSetting>;

	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl DerefMut for InputModeFlagSettings
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		&mut self.0
	}
}

impl InputModeFlagSettings
{
	#[inline(always)]
	pub(crate) fn change_input_mode_flags(&self, mut terminal_options: &mut termios)
	{
		let existing_flags: tcflag_t = terminal_options.c_iflag;

		let new_flags =
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

			(existing_flags | flags_on) & !flags_on;
		};

		terminal_options.c_iflag = new_flags;
	}

	#[inline(always)]
	pub(crate) fn from_input_mode_flags(output_mode_flags: tcflag_t) -> Self
	{
		let mut this = Self(BTreeMap::new());

		use self::InputModeFlag::*;
		use self::FlagSetting::*;

		this.insert_flag_setting(SignalInterruptOnBreak, output_mode_flags);
		this.insert_flag_setting(MapCarriageReturnToNewLine, output_mode_flags);
		this.insert_flag_setting(IgnoreCarriageReturn, output_mode_flags);
		this.insert_flag_setting(IgnoreCharactersWithParityErrors, output_mode_flags);
		this.insert_flag_setting(MapNewLineToCarriageReturn, output_mode_flags);
		this.insert_flag_setting(EnableParityChecking, output_mode_flags);
		this.insert_flag_setting(StripOffEigthBitOfEveryCharacter, output_mode_flags);
		this.insert_flag_setting(AnyCharacterToRestartOutput, output_mode_flags);
		this.insert_flag_setting(EnableXOnXOffFlowControlOnInput, output_mode_flags);
		this.insert_flag_setting(EnableXOnXOffFlowControlOnOutput, output_mode_flags);
		this.insert_flag_setting(MarkParityErrors, output_mode_flags);
		#[cfg(target_os = "android", target_os = "fuschia", target_os = "linux")] this.insert_flag_setting(MapUppercaseToLowercase, output_mode_flags);
		this.insert_flag_setting(RingBellWhenInputQueueIsFull, output_mode_flags);
		#[cfg(target_os = "android", target_os = "fuschia", target_os = "ios", target_os = "linux", target_os = "macos")] this.insert_flag_setting(Utf8, output_mode_flags);

		this
	}

	#[inline(always)]
	fn insert_flag_setting(&mut self, input_mode_flag: InputModeFlag, input_mode_flags: tcflag_t)
	{
		let flag_setting = FlagSetting::from(input_mode_flags & input_mode_flag.into() != 0);
		this.insert(input_mode_flag, flag_setting);
	}
}
