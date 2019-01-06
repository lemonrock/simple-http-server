// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


use super::*;
use super::pipes_and_fifos::syscall::open;
use self::c;
use ::libc::ENAMETOOLONG;
use ::libc::ENXIO;
use ::libc::EOVERFLOW;
use ::libc::EROFS;
use ::libc::ETXTBSY;
use ::std::collections::BTreeMap;
use ::std::ffi::CString;
use ::std::io;
use ::std::io::ErrorKind;
use ::std::io::Initializer;
use ::std::io::Read;
use ::std::io::Write;
use ::std::ops::BitOr;
use ::std::ops::Deref;
use ::std::ops::DerefMut;


mod c;


include!("BackspaceTabDelay.rs");
include!("BaudRate.rs");
include!("BitsPerByte.rs");
include!("CarriageReturnDelay.rs");
include!("CharacterDeviceFileDescriptor.rs");
include!("FlagSetting.rs");
include!("FormFeedTabDelay.rs");
include!("HorizontalTabDelay.rs");
include!("InputModeFlag.rs");
include!("InputModeFlagSettings.rs");
include!("MiscellaneousControlModeFlag.rs");
include!("MiscellaneousControlModeFlagSettings.rs");
include!("MiscellaneousOutputModeFlag.rs");
include!("MiscellaneousOutputModeFlagSettings.rs");
include!("MultipleBits.rs");
include!("NewLineDelay.rs");
include!("OutputModeFlagSettings.rs");
include!("Parity.rs");
include!("StopBits.rs");
include!("TerminalFileDescriptor.rs");
include!("VerticalTabDelay.rs");





/*

On freebsd: tcsetsid  and  cfmakesane
not on linux.



pub use {CSIZE,CS5,CS6,CS7,CS8,CSTOPB,CREAD,PARENB,PARODD,HUPCL,CLOCAL}; // control modes
pub use {ECHO,ECHOE,ECHOK,ECHONL,ICANON,IEXTEN,ISIG,NOFLSH,TOSTOP}; // local modes

pub use {TCSANOW,TCSADRAIN,TCSAFLUSH}; // attribute selection
pub use {TCIFLUSH,TCIOFLUSH,TCOFLUSH,TCIOFF,TCION,TCOOFF,TCOON}; // line control

*/
