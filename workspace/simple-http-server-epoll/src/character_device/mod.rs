// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


use super::*;
use super::pipes_and_fifos::SpliceRecipient;
use super::pipes_and_fifos::SpliceSender;
use super::pipes_and_fifos::syscall::open;
use ::libc::ENAMETOOLONG;
use ::libc::ENXIO;
use ::libc::EOVERFLOW;
use ::libc::EROFS;
use ::libc::ETXTBSY;
use ::std::ffi::CString;
use ::std::io;
use ::std::io::ErrorKind;
use ::std::io::Initializer;
use ::std::io::Read;
use ::std::io::Write;


include!("CharacterDeviceFileDescriptor.rs");
