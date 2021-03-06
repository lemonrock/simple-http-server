// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Choices for character settings.
///
/// The value `CharacterSettings::DisabledCharacterValue` is interpreted as disabling a character.
///
/// This value is ASCII `NUL`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CharacterSettings(BTreeMap<Character, u8>);

impl Default for CharacterSettings
{
	#[inline(always)]
	fn default() -> Self
	{
		use self::Character::*;

		let mut this = Self(BTreeMap::new());
		this.insert(ReadMinimumNumberOfCharacters, 1);
		this.insert(ReadTimeOut, 1);

		this
	}
}

impl From<BTreeMap<Character, u8>> for CharacterSettings
{
	#[inline(always)]
	fn from(map: BTreeMap<Character, u8>) -> Self
	{
		Self(map)
	}
}

impl Into<BTreeMap<Character, u8>> for CharacterSettings
{
	#[inline(always)]
	fn into(self) -> BTreeMap<Character, u8>
	{
		self.0
	}
}

impl Deref for CharacterSettings
{
	type Target = BTreeMap<Character, u8>;

	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl DerefMut for CharacterSettings
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		&mut self.0
	}
}

impl CharacterSettings
{
	/// Value to use to disable processing of a character.
	pub const DisabledCharacterValue: u8 = _POSIX_VDISABLE;

	#[inline(always)]
	pub(crate) fn change_character_settings(&self, terminal_options: &mut termios)
	{
		let settings = &mut terminal_options.c_cc;
		for (character, setting) in self.0.iter()
		{
			*(unsafe { settings.get_unchecked_mut((*character) as usize) }) = *setting;
		}
	}

	#[inline(always)]
	pub(crate) fn from_terminal_options(terminal_options: &termios) -> Self
	{
		let settings = &terminal_options.c_cc;

		let mut this = Self(BTreeMap::new());

		for character in Character::iter()
		{
			let raw_value = *(unsafe {settings.get_unchecked(character as usize) });
			this.insert(character, raw_value);
		}

		this
	}
}
