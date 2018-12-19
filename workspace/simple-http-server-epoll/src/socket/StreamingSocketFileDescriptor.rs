// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Represents a streaming socket instance between a local peer and a remote peer.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StreamingSocketFileDescriptor<SD: SocketData>(SocketFileDescriptor<SD>);

impl<SD: SocketData> Drop for StreamingSocketFileDescriptor<SD>
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

impl<SD: SocketData> AsRawFd for StreamingSocketFileDescriptor<SD>
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		self.0.as_raw_fd()
	}
}

impl<SD: SocketData> Read for StreamingSocketFileDescriptor<SD>
{
	/// This particular implementation can only return an `io::ErrorKind` of:-
	///
	/// * `UnexpectedEof`
	/// * `WouldBlock`
	/// * `Interrupted`
	/// * `Other` (which is for when the kernel reports `ENOMEM`, ie it is out of memory).
	/// * `ConnectionReset` (seems to be posible in some circumstances for Unix domain sockets).
	/// * `ConnectionRefused` (only can happen for TCP client sockets; can not happen for sockets `accept()`ed by a server listener).
	#[inline(always)]
	fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>
	{
		let length = buf.len();
		if unlikely!(length == 0)
		{
			return Ok(0)
		}

		let result = unsafe { recvfrom(self.as_raw_fd(), buf.as_mut_ptr() as *mut c_void, length, ReceiveFlags::empty().bits, null(), null_mut()) };

		if likely!(result > 0)
		{
			Ok(result as usize)
		}
		else
		{
			use self::ErrorKind::*;

			Err
			(
				io::Error::from
				(
					if likely!(result == 0)
					{
						UnexpectedEof
					}
					else if likely!(result == -1)
					{
						match errno().0
						{
							EAGAIN => WouldBlock,
							EINTR => Interrupted,
							ENOMEM => Other,
							ECONNRESET => ConnectionReset,
							ECONNREFUSED => ConnectionRefused,
							EBADF => panic!("The argument `sockfd` is an invalid descriptor"),
							EFAULT => panic!("The receive buffer pointer(s) point outside the process's address space"),
							EINVAL => panic!("Invalid argument passed"),
							ENOTCONN => panic!("The socket is associated with a connection-oriented protocol and has not been connected"),
							ENOTSOCK => panic!("The argument `sockfd` does not refer to a socket"),
							EOPNOTSUPP => panic!("Some flags in the `flags` argument are inappropriate for the socket type"),
							_ => unreachable!(),
						}
					}
					else
					{
						unreachable!()
					}
				)
			)
		}
	}

	#[inline(always)]
	unsafe fn initializer(&self) -> Initializer
	{
		Initializer::nop()
	}
}

impl<SD: SocketData> Write for StreamingSocketFileDescriptor<SD>
{
	/// This particular implementation can only return an `io::ErrorKind` of:-
	///
	/// * `WriteZero`
	/// * `WouldBlock`
	/// * `Interrupted`
	/// * `Other` (which is for when the kernel reports `ENOMEM` or `ENOBUFS`, ie it is out of memory).
	/// * `BrokenPipe`
	/// * `PermissionDenied` (only for Unix domain sockets).
	/// * `ConnectionReset`
	#[inline(always)]
	fn write(&mut self, buf: &[u8]) -> io::Result<usize>
	{
		let length = buf.len();

		if unlikely!(length == 0)
		{
			return Ok(0)
		}

		let result = unsafe { send(self.as_raw_fd(), buf.as_ptr() as *const c_void, buf.len(), SendFlags::NoSigPipeSignal.bits) };

		if likely!(result > 0)
		{
			Ok(result as usize)
		}
		else
		{
			use self::ErrorKind::*;

			Err
			(
				io::Error::from
				(
					if likely!(result == 0)
					{
						WriteZero
					}
					else if likely!(result == -1)
					{
						match errno().0
						{
							EAGAIN => WouldBlock,
							EINTR => Interrupted,
							ENOMEM | ENOBUFS => Other,
							EPIPE => BrokenPipe,
							EACCES => PermissionDenied,
							ECONNRESET => ConnectionReset,
							EBADF => panic!("The argument `sockfd` is an invalid descriptor"),
							EFAULT => panic!("The receive buffer pointer(s) point outside the process's address space"),
							EINVAL => panic!("Invalid argument passed"),
							ENOTCONN => panic!("The socket is associated with a connection-oriented protocol and has not been connected"),
							ENOTSOCK => panic!("The argument `sockfd` does not refer to a socket"),
							EOPNOTSUPP => panic!("Some flags in the `flags` argument are inappropriate for the socket type"),
							EMSGSIZE => panic!("The socket type requires that message be sent atomically, and the size of the message to be sent made this impossible"),
							EISCONN => panic!("The connection-mode socket was connected already but a recipient was specified"),
							EDESTADDRREQ => panic!("The socket is not connection-mode, and no peer address is set"),
							_ => unreachable!(),
						}
					}
					else
					{
						unreachable!()
					}
				)
			)
		}
	}

	#[inline(always)]
	fn flush(&mut self) -> io::Result<()>
	{
		Ok(())
	}
}
