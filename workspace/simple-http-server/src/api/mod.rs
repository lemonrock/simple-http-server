// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


use super::*;


/// HTTP header-related domain types.
pub mod header_domain;


/// Response buffers.
pub mod response_buffers;


/// A RFC 3986 URI adapted for use in HTTP targets ('GET /') and `Location` headers.
pub mod uri;


include!("RegistrationState.rs");
include!("ServedClientConnectionUser.rs");
include!("ServedClientConnectionUserFactory.rs");
include!("SimpleHttpServedClientConnectionUser.rs");
include!("SimpleHttpServedClientConnectionUserError.rs");
include!("SimpleHttpServedClientConnectionUserFactory.rs");
include!("SimplifiedServerSession.rs");
