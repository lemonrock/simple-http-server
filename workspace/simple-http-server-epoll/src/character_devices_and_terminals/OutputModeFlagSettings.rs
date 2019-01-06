// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Represents settings for output mode flags.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OutputModeFlagSettings
{
	/// Backspace delay.
	pub backspace_delay: Option<BackspaceDelay>,

	/// Carriage Return delay.
	pub carriage_return_delay: Option<CarriageReturnDelay>,

	/// Form Feed delay.
	pub form_feed_delay: Option<FormFeedDelay>,

	/// Horizontal Tab delay.
	pub horizontal_tab_delay: Option<HorizontalTabDelay>,

	/// New Line delay.
	pub new_line_delay: Option<NewLineDelay>,

	/// Vertical Tab delay.
	pub vertical_tab_delay: Option<VerticalTabDelay>,

	/// Miscellaneous.
	pub miscellaneous: MiscellaneousOutputModeFlagSettings,
}

impl Default for OutputModeFlagSettings
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			backspace_delay: Some(BackspaceDelay::default()),
			carriage_return_delay: Some(CarriageReturnDelay::default()),
			form_feed_delay: Some(FormFeedDelay::default()),
			horizontal_tab_delay: Some(HorizontalTabDelay::default()),
			new_line_delay: Some(NewLineDelay::default()),
			vertical_tab_delay: Some(VerticalTabDelay::default()),
			miscellaneous: MiscellaneousOutputModeFlagSettings::default(),
		}
	}
}

impl OutputModeFlagSettings
{
	#[inline(always)]
	pub(crate) fn change_output_mode_flags(&self, mut terminal_options: &mut termios)
	{
		let existing_flags = terminal_options.c_oflag;

		let mut new_flags = existing_flags;;
		new_flags = MultipleBits::change_mode_flags(self.backspace_delay, new_flags);
		new_flags = MultipleBits::change_mode_flags(self.carriage_return_delay, new_flags);
		new_flags = MultipleBits::change_mode_flags(self.form_feed_delay, new_flags);
		new_flags = MultipleBits::change_mode_flags(self.horizontal_tab_delay, new_flags);
		new_flags = MultipleBits::change_mode_flags(self.new_line_delay, new_flags);
		new_flags = MultipleBits::change_mode_flags(self.vertical_tab_delay, new_flags);
		new_flags = self.miscellaneous.change_mode_flags(new_flags);

		terminal_options.c_oflag = new_flags;
	}
}
