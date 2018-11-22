// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


use super::*;
use super::super::header_domain::*;
use self::time::*;


pub(crate) mod time;


include!("AccessControlAllowHeadersHeaderResponseBuffer.rs");
include!("AccessControlAllowMethodsHeaderResponseBuffer.rs");
include!("AccessControlMaxAgeResponseBuffer.rs");
include!("AllowHeaderResponseBuffer.rs");
include!("CacheControlHeaderResponseBuffer.rs");
include!("ContentLengthHeaderResponseBuffer.rs");
include!("DateHeaderResponseBuffer.rs");
include!("DenyXFrameOptionsHeaderResponseBuffer.rs");
include!("EndOfHeadersHeaderResponseBuffer.rs");
include!("ETagHeaderResponseBuffer.rs");
include!("HeaderResponseBuffer.rs");
include!("LastModifiedHeaderResponseBuffer.rs");
include!("ModeBlockXXSSProtectionHeaderResponseBuffer.rs");
include!("NosniffXContentTypeOptionsHeaderResponseBuffer.rs");
include!("StatusLineHeaderResponseBuffer.rs");
include!("VaryHeaderResponseBuffer.rs");
include!("XRobotsTagHeaderResponseBuffer.rs");
