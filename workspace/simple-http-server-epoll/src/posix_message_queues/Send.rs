// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Receive a message to a queue.
pub trait Send: PosixMessageQueue
{
	/// Sends a message.
	///
	/// A message may be empty.
	fn send(&self, message_buffer: &[u8], message_priority: PosixMessagePriority) -> Result<(), StructWriteError>;
}
