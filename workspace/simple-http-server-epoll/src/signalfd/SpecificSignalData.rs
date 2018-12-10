// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Potentially specific signal data.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SpecificSignalData<C: Code>
{
	/// Generic signal data.
	Generic(GenericSignalData),

	/// For some signals, such as `SIGBUS`, the data will have a code and some valid fields.
	///
	/// These signals originate within the kernel and the data is safe to rely upon.
	///
	/// The first field is data, the second is a signal code (not the same thing as a signal number).
	Specific(C::Data, C),
}
