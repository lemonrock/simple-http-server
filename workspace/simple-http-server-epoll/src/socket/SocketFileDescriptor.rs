// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Represents a socket instance.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SocketFileDescriptor<SD: SocketData>(RawFd, PhantomData<SD>);

impl<SD: SocketData> Drop for SocketFileDescriptor<SD>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.0.close()
	}
}

impl<SD: SocketData> AsRawFd for SocketFileDescriptor<SD>
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		self.0
	}
}

impl<SD: SocketData> IntoRawFd for SocketFileDescriptor<SD>
{
	#[inline(always)]
	fn into_raw_fd(self) -> RawFd
	{
		self.0
	}
}


impl SocketFileDescriptor<sockaddr_in>
{
	/// Creates a new instance of a Transmission Control Protocol (TCP) socket over Internet Protocol (IP) version 4 server listener.
	///
	/// `back_log` can not exceed `::std::i32::MAX` and is capped by the Operating System to the value in `/proc/sys/net/core/somaxconn`.
	///
	/// The default value in `/proc/sys/net/core/somaxconn` is `128`.
	#[inline(always)]
	pub(crate) fn new_transmission_control_protocol_over_internet_protocol_version_4_server_listener(socket_address: SocketAddrV4, back_log: u32) -> Result<ServerListenerSocketFileDescriptor<sockaddr_in>, NewSocketServerListenerError>
	{
		let this = SocketFileDescriptor::<sockaddr_in>::new_transmission_control_protocol_over_internet_protocol_version_4()?;
		this.bind_internet_protocol_version_4_socket(socket_address)?;
		Ok(this.listen(back_log)?)
	}

	/// Creates a new instance of a Transmission Control Protocol (TCP) socket over Internet Protocol (IP) version 4 client.
	#[inline(always)]
	pub(crate) fn new_transmission_control_protocol_over_internet_protocol_version_4_client(socket_address: SocketAddrV4) -> Result<(), NewSocketClientError>
	{
		let this = SocketFileDescriptor::<sockaddr_in>::new_transmission_control_protocol_over_internet_protocol_version_4()?;
		this.connect_internet_protocol_version_4_socket(socket_address)?;
		Ok(())
	}

	/// Creates a new instance of a User Datagram Protocol (UDP) socket over Internet Protocol (IP) version 4 server listener.
	#[inline(always)]
	pub(crate) fn new_user_datagram_protocol_over_internet_protocol_version_4_server_listener(socket_address: SocketAddrV4) -> Result<(), NewSocketServerListenerError>
	{
		let this = SocketFileDescriptor::<sockaddr_in>::new_user_datagram_protocol_over_internet_protocol_version_4()?;
		this.bind_internet_protocol_version_4_socket(socket_address)?;
		Ok(())
	}

	/// Creates a new instance of a User Datagram Protocol (UDP) socket over Internet Protocol (IP) version 4 client.
	#[inline(always)]
	pub(crate) fn new_user_datagram_protocol_over_internet_protocol_version_4_client(socket_address: SocketAddrV4) -> Result<(), NewSocketClientError>
	{
		let this = SocketFileDescriptor::<sockaddr_in>::new_user_datagram_protocol_over_internet_protocol_version_4()?;
		this.connect_internet_protocol_version_4_socket(socket_address)?;
		Ok(())
	}

	#[inline(always)]
	fn connect_internet_protocol_version_4_socket(&self, socket_address: SocketAddrV4) -> Result<(), SocketConnectError>
	{
		self.connect(&Self::internet_protocol_version_4_socket_data(socket_address))
	}

	#[inline(always)]
	fn bind_internet_protocol_version_4_socket(&self, socket_address: SocketAddrV4) -> Result<(), SocketBindError>
	{
		self.bind(&Self::internet_protocol_version_4_socket_data(socket_address))
	}

	#[inline(always)]
	fn internet_protocol_version_4_socket_data(socket_address: SocketAddrV4) -> sockaddr_in
	{
		unsafe { transmute(socket_address) }
	}
}

impl SocketFileDescriptor<sockaddr_in6>
{
	/// Creates a new instance of a Transmission Control Protocol (TCP) socket over Internet Protocol (IP) version 6 server listener.
	///
	/// `back_log` can not exceed `::std::i32::MAX` and is capped by the Operating System to the value in `/proc/sys/net/core/somaxconn`.
	///
	/// The default value in `/proc/sys/net/core/somaxconn` is `128`.
	#[inline(always)]
	pub(crate) fn new_transmission_control_protocol_over_internet_protocol_version_6_server_listener(socket_address: SocketAddrV6, back_log: u32) -> Result<ServerListenerSocketFileDescriptor<sockaddr_in6>, NewSocketServerListenerError>
	{
		let this = SocketFileDescriptor::<sockaddr_in6>::new_transmission_control_protocol_over_internet_protocol_version_6()?;
		this.bind_internet_protocol_version_6_socket(socket_address)?;
		Ok(this.listen(back_log)?)
	}

	/// Creates a new instance of a Transmission Control Protocol (TCP) socket over Internet Protocol (IP) version 6 client.
	#[inline(always)]
	pub(crate) fn new_transmission_control_protocol_over_internet_protocol_version_6_client(socket_address: SocketAddrV6) -> Result<(), NewSocketClientError>
	{
		let this = SocketFileDescriptor::<sockaddr_in6>::new_transmission_control_protocol_over_internet_protocol_version_6()?;
		this.connect_internet_protocol_version_6_socket(socket_address)?;
		Ok(())
	}

	/// Creates a new instance of a User Datagram Protocol (UDP) socket over Internet Protocol (IP) version 6 server listener.
	#[inline(always)]
	pub(crate) fn new_user_datagram_protocol_over_internet_protocol_version_6_server_listener(socket_address: SocketAddrV6) -> Result<(), NewSocketServerListenerError>
	{
		let this = SocketFileDescriptor::<sockaddr_in6>::new_user_datagram_protocol_over_internet_protocol_version_6()?;
		this.bind_internet_protocol_version_6_socket(socket_address)?;
		Ok(())
	}

	/// Creates a new instance of a User Datagram Protocol (UDP) socket over Internet Protocol (IP) version 6 client.
	#[inline(always)]
	pub(crate) fn new_user_datagram_protocol_over_internet_protocol_version_6_client(socket_address: SocketAddrV6) -> Result<(), NewSocketClientError>
	{
		let this = SocketFileDescriptor::<sockaddr_in6>::new_user_datagram_protocol_over_internet_protocol_version_6()?;
		this.connect_internet_protocol_version_6_socket(socket_address)?;
		Ok(())
	}

	#[inline(always)]
	fn connect_internet_protocol_version_6_socket(&self, socket_address: SocketAddrV6) -> Result<(), SocketConnectError>
	{
		self.connect(&Self::internet_protocol_version_6_socket_data(socket_address))
	}

	#[inline(always)]
	fn bind_internet_protocol_version_6_socket(&self, socket_address: SocketAddrV6) -> Result<(), SocketBindError>
	{
		self.bind(&Self::internet_protocol_version_6_socket_data(socket_address))
	}

	#[inline(always)]
	fn internet_protocol_version_6_socket_data(socket_address: SocketAddrV6) -> sockaddr_in6
	{
		unsafe { transmute(socket_address) }
	}
}

impl SocketFileDescriptor<sockaddr_un>
{
	/// Creates a new streaming Unix Domain server listener socket.
	///
	/// This is local socket akin to a Transmission Control Protocol (TCP) socket.
	#[inline(always)]
	pub(crate) fn new_streaming_unix_domain_socket_server_listener(path: impl AsRef<Path>) -> Result<ServerListenerSocketFileDescriptor<sockaddr_un>, NewSocketServerListenerError>
	{
		let this = SocketFileDescriptor::<sockaddr_un>::new_streaming_unix_domain_socket()?;
		this.bind_unix_domain_socket(path)?;
		Ok(this.listen(0)?)
	}

	/// Creates a new streaming Unix Domain client socket.
	///
	/// This is local socket akin to a Transmission Control Protocol (TCP) socket.
	#[inline(always)]
	pub(crate) fn new_streaming_unix_domain_socket_client(path: impl AsRef<Path>) -> Result<(), NewSocketClientError>
	{
		let this = SocketFileDescriptor::<sockaddr_un>::new_streaming_unix_domain_socket()?;
		this.connect_unix_domain_socket(path)?;
		Ok(())
	}

	/// Creates a new datagram Unix Domain server listener socket.
	///
	/// This is local socket akin to an User Datagram Protocol (UDP) socket.
	#[inline(always)]
	pub(crate) fn new_datagram_unix_domain_socket_server_listener(path: impl AsRef<Path>) -> Result<(), NewSocketServerListenerError>
	{
		let this = SocketFileDescriptor::<sockaddr_un>::new_datagram_unix_domain_socket()?;
		this.bind_unix_domain_socket(path)?;
		Ok(())
	}

	/// Creates a new datagram Unix Domain client socket.
	///
	/// This is local socket akin to an User Datagram Protocol (UDP) socket.
	#[inline(always)]
	pub(crate) fn new_datagram_unix_domain_socket_client(path: impl AsRef<Path>) -> Result<(), NewSocketClientError>
	{
		let this = SocketFileDescriptor::<sockaddr_un>::new_datagram_unix_domain_socket()?;
		this.connect_unix_domain_socket(path)?;
		Ok(())
	}

	#[inline(always)]
	fn connect_unix_domain_socket(&self, path: impl AsRef<Path>) -> Result<(), SocketConnectError>
	{
		self.connect(&Self::unix_domain_socket_data(path))
	}

	#[inline(always)]
	fn bind_unix_domain_socket(&self, path: impl AsRef<Path>) -> Result<(), SocketBindError>
	{
		self.bind(&Self::unix_domain_socket_data(path))
	}

	#[inline(always)]
	fn unix_domain_socket_data(path: impl AsRef<Path>) -> sockaddr_un
	{
		let mut socket_data = sockaddr_un
		{
			sun_family: AF_UNIX as sa_family_t,
			sun_path: unsafe { zeroed() },
		};

		let path_bytes = path_bytes_without_trailing_nul(&path);
		let path_bytes_length = path_bytes.len();
		debug_assert!(path_bytes_length <= socket_data.sun_path.len(), "Path converted to bytes is more than 108-bytes long");
		unsafe { socket_data.sun_path.as_mut_ptr().copy_from_nonoverlapping(path_bytes.as_ptr() as *const _, path_bytes_length) };

		socket_data
	}
}

impl<SD: SocketData> SocketFileDescriptor<SD>
{
	#[inline(always)]
	fn listen(self, back_log: u32) -> Result<ServerListenerSocketFileDescriptor<SD>, SocketListenError>
	{
		debug_assert!(back_log <= ::std::i32::MAX as u32, "back_log can not be greater than :std::i32::MAX");

		let result = unsafe { listen(self.0, back_log as i32) };
		if likely!(result == 0)
		{
			Ok(ServerListenerSocketFileDescriptor(self))
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				EADDRINUSE => Err(SocketListenError::AddressInUse),
				EBADF => panic!("`sockfd` is not a valid descriptor"),
				ENOTSOCK => panic!("`sockfd` is not a socket file descriptor"),
				EOPNOTSUPP => panic!("The socket is not of a type that supports the `listen()` operation"),

				_ => unreachable!(),
			}
		}
		else
		{
			unreachable!()
		}
	}

	#[inline(always)]
	fn bind(&self, socket_data: &SD) -> Result<(), SocketBindError>
	{
		use self::SocketBindError::*;
		use self::FilePathInvalidReason::*;

		let result = unsafe { bind(self.0, &socket_data as *const _ as *const sockaddr_storage, size_of::<SD>() as socklen_t) };
		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			Err
			(
				match errno().0
				{
					EACCES => PermissionDenied,
					EADDRINUSE => AddressInUse,
					ENOMEM => KernelWouldBeOutOfMemory,
					EBADF => panic!("`sockfd` is not a valid descriptor"),
					EINVAL => panic!("already bound, or the `addrlen` is wrong, or the socket was not in the `AF_UNIX` family"),
					ENOTSOCK => panic!("`sockfd` is not a socket file descriptor"),

					EADDRNOTAVAIL => FilePathInvalid(AddressUnavailable),
					EFAULT => panic!("`addr` points outside the user's accessible address space"),
					ELOOP => FilePathInvalid(TooManySymbolicLinksInFilePath),
					ENOENT => FilePathInvalid(DoesNotExist),
					ENOTDIR => FilePathInvalid(FilePathPrefixComponentIsNotADirectory),
					EROFS => FilePathInvalid(FilePathIsReadOnly),

					EAFNOSUPPORT => panic!("Invalid `sa_family_t` value"),

					_ => unreachable!(),
				}
			)
		}
		else
		{
			unreachable!()
		}
	}

	#[inline(always)]
	fn connect(&self, socket_data: &SD) -> Result<(), SocketConnectError>
	{
		use self::SocketConnectError::*;

		let result = unsafe { connect(self.0, &socket_data as *const _ as *const sockaddr_storage, size_of::<SD>() as socklen_t) };
		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			Err
			(
				match errno().0
				{
					EACCES | EPERM => PermissionDenied,
					EADDRINUSE => AddressInUse,
					EAGAIN => NoMoreFreeLocalPorts,
					ECONNREFUSED => ConnectionRefused,
					EINPROGRESS => InProgress,
					EINTR => Interrupted,
					ETIMEDOUT => TimedOut,
					ENETUNREACH => NetworkUnreachable,
					EISCONN => panic!("The socket is already connected."),
					EALREADY => panic!("The socket is nonblocking and a previous connection attempt has not yet been completed."),
					EBADF => panic!("`sockfd` is not a valid descriptor"),
					EINVAL => panic!("already bound, or the `addrlen` is wrong, or the socket was not in the `AF_UNIX` family"),
					ENOTSOCK => panic!("`sockfd` is not a socket file descriptor"),
					EFAULT => panic!("`addr` points outside the user's accessible address space"),
					EAFNOSUPPORT => panic!("Invalid `sa_family_t` value"),

					_ => unreachable!(),
				}
			)
		}
		else
		{
			unreachable!()
		}
	}

	#[inline(always)]
	fn new_transmission_control_protocol_over_internet_protocol_version_4() -> Result<Self, CreationError>
	{
		Self::new(AF_INET, SOCK_STREAM, IPPROTO_TCP)
	}

	#[inline(always)]
	fn new_transmission_control_protocol_over_internet_protocol_version_6() -> Result<Self, CreationError>
	{
		Self::new(AF_INET6, SOCK_STREAM, IPPROTO_TCP)
	}

	#[inline(always)]
	fn new_user_datagram_protocol_over_internet_protocol_version_4() -> Result<Self, CreationError>
	{
		Self::new(AF_INET, SOCK_DGRAM, IPPROTO_UDP)
	}

	#[inline(always)]
	fn new_user_datagram_protocol_over_internet_protocol_version_6() -> Result<Self, CreationError>
	{
		Self::new(AF_INET6, SOCK_DGRAM, IPPROTO_UDP)
	}

	#[inline(always)]
	fn new_streaming_unix_domain_socket() -> Result<Self, CreationError>
	{
		Self::new(AF_UNIX, SOCK_STREAM, 0)
	}

	#[inline(always)]
	fn new_datagram_unix_domain_socket() -> Result<Self, CreationError>
	{
		Self::new(AF_UNIX, SOCK_DGRAM, 0)
	}

	/// Creates a new instance.
	#[inline(always)]
	fn new(domain: c_int, type_: c_int, ethernet_protocol: c_int) -> Result<Self, CreationError>
	{
		const Flags: c_int = SOCK_NONBLOCK | SOCK_CLOEXEC;

		let result = unsafe { socket(domain, type_ | Flags, ethernet_protocol) };
		if likely!(result != -1)
		{
			Ok(SocketFileDescriptor(result, PhantomData))
		}
		else
		{
			use self::CreationError::*;

			Err
			(
				match errno().0
				{
					EMFILE => PerProcessLimitOnNumberOfFileDescriptorsWouldBeExceeded,
					ENFILE => SystemWideLimitOnTotalNumberOfFileDescriptorsWouldBeExceeded,
					ENOBUFS | ENOMEM => KernelWouldBeOutOfMemory,
					EINVAL => panic!("Invalid arguments"),
					EACCES => panic!("Permission denined"),
					EAFNOSUPPORT => panic!("The implementation does not support the specified address family"),
					EPROTONOSUPPORT => panic!("The protocol type or the specified protocol is not supported within this domain"),
					_ => unreachable!(),
				}
			)
		}
	}
}
