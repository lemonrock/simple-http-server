// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


use super::*;
use super::character_device::CharacterDeviceFileDescriptor;
use self::c::*;
use ::libc::O_NOCTTY;
use ::std::collections::BTreeMap;
use ::std::io;
use ::std::io::Initializer;
use ::std::io::Read;
use ::std::io::Write;
use ::std::ops::Deref;
use ::std::ops::DerefMut;
use ::strum::IntoEnumIterator;


mod c;


include!("BackspaceDelay.rs");
include!("BaudRate.rs");
include!("BitsPerByte.rs");
include!("CanonicalEchoKillCharacter.rs");
include!("CanonicalSettings.rs");
include!("CarriageReturnDelay.rs");
include!("Character.rs");
include!("CharacterSettings.rs");
include!("ControlModeFlagSettings.rs");
include!("CurrentTerminalSettings.rs");
include!("Echo.rs");
include!("FlagSetting.rs");
include!("FormFeedDelay.rs");
include!("HorizontalTabDelay.rs");
include!("InputModeFlag.rs");
include!("InputModeFlagSettings.rs");
include!("LocalModeFlagSettings.rs");
include!("MiscellaneousControlModeFlag.rs");
include!("MiscellaneousControlModeFlagSettings.rs");
include!("MiscellaneousLocalModeFlag.rs");
include!("MiscellaneousLocalModeFlagSettings.rs");
include!("MiscellaneousOutputModeFlag.rs");
include!("MiscellaneousOutputModeFlagSettings.rs");
include!("MultipleBits.rs");
include!("NewLineDelay.rs");
include!("OutputModeFlagSettings.rs");
include!("Parity.rs");
include!("SignalRaising.rs");
include!("StopBits.rs");
include!("TerminalFileDescriptor.rs");
include!("TerminalMode.rs");
include!("TerminalSettings.rs");
include!("TerminalSettingsError.rs");
include!("VerticalTabDelay.rs");
include!("WhenToChangeTerminalAttributes.rs");
