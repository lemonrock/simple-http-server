// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Represents a server listener socket instance.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ServerListenerSocketFileDescriptor<SD: SocketData>(SocketFileDescriptor<SD>);

impl<SD: SocketData> AsRawFd for ServerListenerSocketFileDescriptor<SD>
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		self.0.as_raw_fd()
	}
}

impl<SD: SocketData> IntoRawFd for ServerListenerSocketFileDescriptor<SD>
{
	#[inline(always)]
	fn into_raw_fd(self) -> RawFd
	{
		self.0.into_raw_fd()
	}
}

impl<SD: SocketData> ServerListenerSocketFileDescriptor<SD>
{
	/// Accepts any pending connections.
	#[inline(always)]
	pub fn accept(&self) -> Result<AcceptedConnection<SD>, SocketAcceptError>
	{
		use self::SocketAcceptError::*;
		use self::ConnectionFailedReason::*;

		// Rust bug (as of 1.30) prevents this being a constant.
		let SocketDataLength: socklen_t = size_of::<SD>() as socklen_t;

		let mut peer_address: SD = unsafe { uninitialized() };
		let mut peer_address_length = SocketDataLength;

		let result = unsafe { accept4(self.as_raw_fd(), &mut peer_address as *mut _ as *mut _, &mut peer_address_length, SOCK_NONBLOCK | SOCK_CLOEXEC) };

		if likely!(result == 0)
		{
			debug_assert_eq!(peer_address_length, SocketDataLength, "peer_address was truncated");

			Ok
			(
				AcceptedConnection
				{
					streaming_socket_file_descriptor: StreamingSocketFileDescriptor(SocketFileDescriptor(result, PhantomData)),
					peer_address
				}
			)
		}
		else if likely!(result == -1)
		{
			Err
			(
				match errno().0
				{
					EAGAIN => Again,

					EINTR => Interrupted,

					ECONNABORTED => ConnectionFailed(Aborted),
					EPERM => ConnectionFailed(FirewallPermissionDenied),
					ETIMEDOUT => ConnectionFailed(TimedOut),
					EPROTO => ConnectionFailed(Protocol),

					EMFILE => PerProcessLimitOnNumberOfFileDescriptorsWouldBeExceeded,
					ENFILE => SystemWideLimitOnTotalNumberOfFileDescriptorsWouldBeExceeded,
					ENOBUFS | ENOMEM | ENOSR => KernelWouldBeOutOfMemory,

					EINVAL => panic!("Socket is not listening for connections, or `addrlen` is invalid, or the `flags` are invalid"),
					EFAULT => panic!("`addr` points outside the user's accessible address space"),
					EBADF => panic!("`sockfd` is not a valid descriptor"),
					ENOTSOCK => panic!("`sockfd` is not a socket file descriptor"),
					EOPNOTSUPP => panic!("The socket is not of a type that supports the `accept()` operation"),
					ESOCKTNOSUPPORT => panic!("ESOCKTNOSUPPORT"),
					EPROTONOSUPPORT => panic!("EPROTONOSUPPORT"),

					_ => unreachable!(),
				}
			)
		}
		else
		{
			unreachable!()
		}
	}
}
