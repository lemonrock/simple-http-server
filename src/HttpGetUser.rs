// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Called for a HTTP GET.
pub trait HttpGetUser
{
	/// Type of errors returned.
	type Error: error::Error;

	/// Called for each valid HTTP GET.
	fn use_http_get(&self, headers: Vec<HeaderField>, target_uri: Url, client_end_entity_certificate: EndEntityCert) -> Result<(), Self::Error>;
}
