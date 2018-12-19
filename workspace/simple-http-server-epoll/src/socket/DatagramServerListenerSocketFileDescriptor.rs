// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Represents a datagram socket instance server listener.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DatagramServerListenerSocketFileDescriptor<SD: SocketData>(SocketFileDescriptor<SD>);

impl<SD: SocketData> Drop for DatagramServerListenerSocketFileDescriptor<SD>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		let result = unsafe { shutdown(self.as_raw_fd(), SHUT_RDWR) };
		if likely!(result == 0)
		{
			return
		}
		else if likely!(result != -1)
		{
			match errno().0
			{
				EBADF => panic!("The argument `sockfd` is an invalid descriptor"),
				EINVAL => panic!("An invalid value was specified in `how`"),
				ENOTCONN => panic!("The socket is associated with a connection-oriented protocol and has not been connected"),
				ENOTSOCK => panic!("The argument `sockfd` does not refer to a socket"),
				_ => unreachable!(),
			}
		}
		else
		{
			unreachable!()
		}
	}
}

impl<SD: SocketData> AsRawFd for DatagramServerListenerSocketFileDescriptor<SD>
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		self.0.as_raw_fd()
	}
}
