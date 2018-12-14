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
	pub(crate) fn new_transmission_control_protocol_over_internet_protocol_version_4_server_listener(socket_address: SocketAddrV4, send_buffer_size_in_bytes: usize, receive_buffer_size_in_bytes: usize, idles_before_keep_alive_seconds: u16, keep_alive_interval_seconds: u16, maximum_keep_alive_probes: u16, linger_seconds: u16, linger_in_FIN_WAIT2_seconds: u16, maximum_SYN_transmits: u16, back_log: u32) -> Result<ServerListenerSocketFileDescriptor<sockaddr_in>, NewSocketServerListenerError>
	{
		let this = SocketFileDescriptor::<sockaddr_in>::new_transmission_control_protocol_over_internet_protocol_version_4(send_buffer_size_in_bytes, receive_buffer_size_in_bytes, idles_before_keep_alive_seconds, keep_alive_interval_seconds, maximum_keep_alive_probes, linger_seconds, linger_in_FIN_WAIT2_seconds, maximum_SYN_transmits)?;
		this.set_internet_protocol_server_listener_socket_options();
		this.set_tcp_server_listener_socket_options();
		this.bind_internet_protocol_version_4_socket(socket_address)?;
		Ok(this.listen(back_log)?)
	}

	/// Creates a new instance of a Transmission Control Protocol (TCP) socket over Internet Protocol (IP) version 4 client.
	#[inline(always)]
	pub(crate) fn new_transmission_control_protocol_over_internet_protocol_version_4_client(socket_address: SocketAddrV4, send_buffer_size_in_bytes: usize, receive_buffer_size_in_bytes: usize, idles_before_keep_alive_seconds: u16, keep_alive_interval_seconds: u16, maximum_keep_alive_probes: u16, linger_seconds: u16, linger_in_FIN_WAIT2_seconds: u16, maximum_SYN_transmits: u16) -> Result<(), NewSocketClientError>
	{
		let this = SocketFileDescriptor::<sockaddr_in>::new_transmission_control_protocol_over_internet_protocol_version_4(send_buffer_size_in_bytes, receive_buffer_size_in_bytes, idles_before_keep_alive_seconds, keep_alive_interval_seconds, maximum_keep_alive_probes, linger_seconds, linger_in_FIN_WAIT2_seconds, maximum_SYN_transmits)?;
		this.connect_internet_protocol_version_4_socket(socket_address)?;
		Ok(())
	}

	/// Creates a new instance of a User Datagram Protocol (UDP) socket over Internet Protocol (IP) version 4 server listener.
	#[inline(always)]
	pub(crate) fn new_user_datagram_protocol_over_internet_protocol_version_4_server_listener(socket_address: SocketAddrV4, send_buffer_size_in_bytes: usize, receive_buffer_size_in_bytes: usize) -> Result<(), NewSocketServerListenerError>
	{
		let this = SocketFileDescriptor::<sockaddr_in>::new_user_datagram_protocol_over_internet_protocol_version_4(send_buffer_size_in_bytes, receive_buffer_size_in_bytes)?;
		this.set_internet_protocol_server_listener_socket_options();
		this.bind_internet_protocol_version_4_socket(socket_address)?;
		Ok(())
	}

	/// Creates a new instance of a User Datagram Protocol (UDP) socket over Internet Protocol (IP) version 4 client.
	#[inline(always)]
	pub(crate) fn new_user_datagram_protocol_over_internet_protocol_version_4_client(socket_address: SocketAddrV4, send_buffer_size_in_bytes: usize, receive_buffer_size_in_bytes: usize) -> Result<(), NewSocketClientError>
	{
		let this = SocketFileDescriptor::<sockaddr_in>::new_user_datagram_protocol_over_internet_protocol_version_4(send_buffer_size_in_bytes, receive_buffer_size_in_bytes)?;
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
	pub(crate) fn new_transmission_control_protocol_over_internet_protocol_version_6_server_listener(socket_address: SocketAddrV6, send_buffer_size_in_bytes: usize, receive_buffer_size_in_bytes: usize, idles_before_keep_alive_seconds: u16, keep_alive_interval_seconds: u16, maximum_keep_alive_probes: u16, linger_seconds: u16, linger_in_FIN_WAIT2_seconds: u16, maximum_SYN_transmits: u16, back_log: u32) -> Result<ServerListenerSocketFileDescriptor<sockaddr_in6>, NewSocketServerListenerError>
	{
		let this = SocketFileDescriptor::<sockaddr_in6>::new_transmission_control_protocol_over_internet_protocol_version_6(send_buffer_size_in_bytes, receive_buffer_size_in_bytes, idles_before_keep_alive_seconds, keep_alive_interval_seconds, maximum_keep_alive_probes, linger_seconds, linger_in_FIN_WAIT2_seconds, maximum_SYN_transmits)?;
		this.set_internet_protocol_server_listener_socket_options();
		this.set_tcp_server_listener_socket_options();
		this.bind_internet_protocol_version_6_socket(socket_address)?;
		Ok(this.listen(back_log)?)
	}

	/// Creates a new instance of a Transmission Control Protocol (TCP) socket over Internet Protocol (IP) version 6 client.
	#[inline(always)]
	pub(crate) fn new_transmission_control_protocol_over_internet_protocol_version_6_client(socket_address: SocketAddrV6, send_buffer_size_in_bytes: usize, receive_buffer_size_in_bytes: usize, idles_before_keep_alive_seconds: u16, keep_alive_interval_seconds: u16, maximum_keep_alive_probes: u16, linger_seconds: u16, linger_in_FIN_WAIT2_seconds: u16, maximum_SYN_transmits: u16) -> Result<(), NewSocketClientError>
	{
		let this = SocketFileDescriptor::<sockaddr_in6>::new_transmission_control_protocol_over_internet_protocol_version_6(send_buffer_size_in_bytes, receive_buffer_size_in_bytes, idles_before_keep_alive_seconds, keep_alive_interval_seconds, maximum_keep_alive_probes, linger_seconds, linger_in_FIN_WAIT2_seconds, maximum_SYN_transmits)?;
		this.connect_internet_protocol_version_6_socket(socket_address)?;
		Ok(())
	}

	/// Creates a new instance of a User Datagram Protocol (UDP) socket over Internet Protocol (IP) version 6 server listener.
	#[inline(always)]
	pub(crate) fn new_user_datagram_protocol_over_internet_protocol_version_6_server_listener(socket_address: SocketAddrV6, send_buffer_size_in_bytes: usize, receive_buffer_size_in_bytes: usize) -> Result<(), NewSocketServerListenerError>
	{
		let this = SocketFileDescriptor::<sockaddr_in6>::new_user_datagram_protocol_over_internet_protocol_version_6(send_buffer_size_in_bytes, receive_buffer_size_in_bytes)?;
		this.set_internet_protocol_server_listener_socket_options();
		this.bind_internet_protocol_version_6_socket(socket_address)?;
		Ok(())
	}

	/// Creates a new instance of a User Datagram Protocol (UDP) socket over Internet Protocol (IP) version 6 client.
	#[inline(always)]
	pub(crate) fn new_user_datagram_protocol_over_internet_protocol_version_6_client(socket_address: SocketAddrV6, send_buffer_size_in_bytes: usize, receive_buffer_size_in_bytes: usize) -> Result<(), NewSocketClientError>
	{
		let this = SocketFileDescriptor::<sockaddr_in6>::new_user_datagram_protocol_over_internet_protocol_version_6(send_buffer_size_in_bytes, receive_buffer_size_in_bytes)?;
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
	/// Tries to send file descriptors to a remote peer over an Unix Domain Socket.
	///
	/// `file_descriptors`: File Descriptors to send.
	#[inline(always)]
	pub fn send_file_descriptors(&self, file_descriptors: &[RawFd])
	{
		self.send_ancillary_data(SOL_SOCKET, SCM_RIGHTS, file_descriptors)
	}

	/// Tries to send credentials to a remote peer over an Unix Domain Socket.
	///
	/// `process_identifier`: Process identifier (also known as `pid`). Unless the process has capability `CAP_SYS_ADMIN`, this must be its own `process_identifier`.
	/// `user_identifier`: User identifier (also known as `uid`). Unless the process has capability `CAP_SETUID`, this must be its own `user_identifier`, effective `user_identifier` or saved-set `user_identifier`.
	/// `group_identifier`: Group identifier (also known as `gid`). Unless the process has capability `CAP_SETGID`, this must be its own `group_identifier`, effective `group_identifier` or saved-set `group_identifier`.
	#[inline(always)]
	pub fn send_credentials(&self, process_identifier: pid_t, user_identifier: uid_t, group_identifier: gid_t)
	{
		#[repr(C)]
		struct ucred
		{
			pid: pid_t,
			uid: uid_t,
			gid: gid_t,
		}

		let credentials: [ucred; 1] =
		[
			ucred
			{
				pid: process_identifier,
				uid: user_identifier,
				gid: group_identifier,
			}
		];

		self.send_ancillary_data(SOL_SOCKET, SCM_CREDENTIALS, &credentials)





		// TODO: ssize_t sendmsg(int sockfd, const struct msghdr *msg, int flags);




	}

	/// Send ancillary data over this socket.
	///
	/// `level`: A `SOL_*` constant such as `SOL_SOCKET`.
	/// `type`: A `SCM_*` constant such as `SCM_RIGHTS`.
	/// `array`: array of ancillary data to send.
	pub(crate) fn send_ancillary_data<T: Sized>(&self, level: c_int, type_: c_int, array: &[T]) -> io::Result<()>
	{
		let mut ancillary_data_buffer: Vec<u8> = Vec::with_capacity(cmsghdr::CMSG_SPACE(size_of::<T>() * array.len()));

		let mut msg = msghdr::new(null_mut(), 0, null_mut(), 0, ancillary_data_buffer.as_mut_ptr() as *mut _, ancillary_data_buffer.len(), 0);
		let cmsg = msg.initialize_first_header(level, type_, array);

		// Sum of the length of all control messages in the buffer.
		msg.msg_controllen = cmsg.cmsg_len;

		let result = unsafe { sendmsg(self.0, &msg, SendFlags::NoSigPipeSignal.bits) };

		if likely!(result > 0)
		{
			Ok(())
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

	/// Creates a new streaming Unix Domain server listener socket.
	///
	/// This is local socket akin to a Transmission Control Protocol (TCP) socket.
	#[inline(always)]
	pub(crate) fn new_streaming_unix_domain_socket_server_listener(path: impl AsRef<Path>, send_buffer_size_in_bytes: usize) -> Result<ServerListenerSocketFileDescriptor<sockaddr_un>, NewSocketServerListenerError>
	{
		let this = SocketFileDescriptor::<sockaddr_un>::new_streaming_unix_domain_socket(send_buffer_size_in_bytes)?;
		this.bind_unix_domain_socket(path)?;
		Ok(this.listen(0)?)
	}

	/// Creates a new streaming Unix Domain client socket.
	///
	/// This is local socket akin to a Transmission Control Protocol (TCP) socket.
	#[inline(always)]
	pub(crate) fn new_streaming_unix_domain_socket_client(path: impl AsRef<Path>, send_buffer_size_in_bytes: usize) -> Result<(), NewSocketClientError>
	{
		let this = SocketFileDescriptor::<sockaddr_un>::new_streaming_unix_domain_socket(send_buffer_size_in_bytes)?;
		this.connect_unix_domain_socket(path)?;
		Ok(())
	}

	/// Creates a new datagram Unix Domain server listener socket.
	///
	/// This is local socket akin to an User Datagram Protocol (UDP) socket.
	#[inline(always)]
	pub(crate) fn new_datagram_unix_domain_socket_server_listener(path: impl AsRef<Path>, send_buffer_size_in_bytes: usize) -> Result<(), NewSocketServerListenerError>
	{
		let this = SocketFileDescriptor::<sockaddr_un>::new_datagram_unix_domain_socket(send_buffer_size_in_bytes)?;
		this.bind_unix_domain_socket(path)?;
		Ok(())
	}

	/// Creates a new datagram Unix Domain client socket.
	///
	/// This is local socket akin to an User Datagram Protocol (UDP) socket.
	#[inline(always)]
	pub(crate) fn new_datagram_unix_domain_socket_client(path: impl AsRef<Path>, send_buffer_size_in_bytes: usize) -> Result<(), NewSocketClientError>
	{
		let this = SocketFileDescriptor::<sockaddr_un>::new_datagram_unix_domain_socket(send_buffer_size_in_bytes)?;
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
	fn set_socket_option_true(&self, level: c_int, optname: c_int)
	{
		static is_true: c_int = 1;
		self.set_socket_option(level, optname, &is_true);
	}

	#[inline(always)]
	fn set_socket_option<T>(&self, level: c_int, optname: c_int, value: &T)
	{
		let result = unsafe { setsockopt(self.0, level, value as *const _ as *const _, size_of::<T>() as socklen_t) };

		if likely!(result == 0)
		{
			return
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				EBADF => panic!("The argument `sockfd` is not a valid descriptor"),
				EFAULT => panic!("The address pointed to by `optval` is not in a valid part of the process address space"),
				EINVAL => panic!("`optlen` is invalid, or there is an invalid value in `optval`"),
				ENOPROTOOPT => panic!("The option is unknown at the level indicated"),
				ENOTSOCK => panic!("The argument `sockfd` is a file, not a socket"),

				_ => unreachable!(),
			}
		}
		else
		{
			unreachable!();
		}
	}

	#[inline(always)]
	fn set_send_buffer_size(&self, send_buffer_size_in_bytes: usize)
	{
		debug_assert!(send_buffer_size_in_bytes >= 2048, "receive_buffer_size_in_bytes must be at least 2048 bytes; maximum is in `/proc/sys/net/core/wmem_max`");

		let send_buffer_halved: c_int = (send_buffer_size_in_bytes / 2) as c_int;
		self.set_socket_option(SOL_SOCKET, SO_SNDBUF, &send_buffer_halved);
	}

	#[inline(always)]
	fn set_receive_buffer_size(&self, receive_buffer_size_in_bytes: usize)
	{
		debug_assert!(receive_buffer_size_in_bytes >= 256, "receive_buffer_size_in_bytes must be at least 256 bytess; maximum is in `/proc/sys/net/core/rmem_max`");

		let send_buffer_halved: c_int = (send_buffer_size_in_bytes / 2) as c_int;
		self.set_socket_option(SOL_SOCKET, SO_SNDBUF, &send_buffer_halved);
	}

	#[inline(always)]
	fn set_broadcast(&self)
	{
		self.set_socket_option_true(SOL_SOCKET, SO_BROADCAST)
	}

	#[inline(always)]
	fn set_out_of_band_inline(&self)
	{
		self.set_socket_option_true(SOL_SOCKET, SO_OOBINLINE)
	}

	#[inline(always)]
	fn disable_nagle_algorithm(&self)
	{
		self.set_socket_option_true(SOL_TCP, TCP_NODELAY)
	}

	#[inline(always)]
	fn set_tcp_max_SYN_transmits(&self, maximum_SYN_transmits: u16)
	{
		let maximum_SYN_transmits: i32 = maximum_SYN_transmits as i32;
		this.set_socket_option(SOL_TCP, TCP_SYNCNT, &maximum_SYN_transmits);
	}

	#[inline(always)]
	fn set_tcp_linger(&self, linger_seconds: u16)
	{
		#[repr(C)]
		struct linger
		{
			l_onoff: c_int,
			l_linger: c_int,
		}

		let linger = linger
		{
			l_onoff: 1,
			l_linger: linger_seconds as i32,
		};
		this.set_socket_option(SOL_SOCKET, SO_LINGER, &linger);
	}

	#[inline(always)]
	fn set_internet_protocol_socket_options(&self, send_buffer_size_in_bytes: usize, receive_buffer_size_in_bytes: usize)
	{
		this.set_send_buffer_size(send_buffer_size_in_bytes);
		this.set_receive_buffer_size(receive_buffer_size_in_bytes);
	}

	#[inline(always)]
	fn set_tcp_socket_options(&self, idles_before_keep_alive_seconds: u16, keep_alive_interval_seconds: u16, maximum_keep_alive_probes: u16, linger_seconds: u16, linger_in_FIN_WAIT2_seconds: u16, maximum_SYN_transmits: u16)
	{
		debug_assert!(maximum_SYN_transmits > 0, "maximum_SYN_transmits is zero");
		//TODO: SOL_SOCKET,SO_BINDTODEVICE,CStr => force use of device such as `eth0`.

		this.set_socket_option_true(SOL_SOCKET, SO_KEEPALIVE);

		this.set_out_of_band_inline();

		this.disable_nagle_algorithm();

		let idles_before_keep_alive_seconds: i32 = idles_before_keep_alive_seconds as i32;
		this.set_socket_option(SOL_TCP, TCP_KEEPALIVE, &idles_before_keep_alive_seconds);

		let keep_alive_interval_seconds: i32 = keep_alive_interval_seconds as i32;
		this.set_socket_option(SOL_TCP, TCP_KEEPINTVL, &keep_alive_interval_seconds);

		let maximum_keep_alive_probes: i32 = maximum_keep_alive_probes as i32;
		this.set_socket_option(SOL_TCP, TCP_KEEPCNT, &maximum_keep_alive_probes);

		this.set_tcp_linger(linger_seconds);

		let linger_in_FIN_WAIT2_seconds: i32 = linger_in_FIN_WAIT2_seconds as i32;
		this.set_socket_option(SOL_TCP, TCP_LINGER2, &linger_in_FIN_WAIT2_seconds);

		this.set_tcp_max_SYN_transmits(maximum_SYN_transmits);
	}

	#[inline(always)]
	fn set_udp_socket_options(&self)
	{
		this.set_broadcast();
	}

	#[inline(always)]
	fn set_internet_protocol_server_listener_socket_options(&self)
	{
		this.set_socket_option_true(SOL_SOCKET, SO_REUSEADDR);
		this.set_socket_option_true(SOL_SOCKET, SO_REUSEPORT);
	}

	#[inline(always)]
	fn set_tcp_server_listener_socket_options(&self)
	{
		this.set_socket_option_true(SOL_TCP, TCP_DEFER_ACCEPT);
		this.set_socket_option_true(SOL_TCP, TCP_FASTOPEN);
	}

	#[inline(always)]
	fn new_transmission_control_protocol_over_internet_protocol_version_4(send_buffer_size_in_bytes: usize, receive_buffer_size_in_bytes: usize, idles_before_keep_alive_seconds: u16, keep_alive_interval_seconds: u16, maximum_keep_alive_probes: u16, linger_seconds: u16, linger_in_FIN_WAIT2_seconds: u16, maximum_SYN_transmits: u16) -> Result<Self, CreationError>
	{
		Self::new(AF_INET, SOCK_STREAM, IPPROTO_TCP).map(|this|
		{
			this.set_internet_protocol_socket_options(send_buffer_size_in_bytes, receive_buffer_size_in_bytes);
			this.set_tcp_socket_options(idles_before_keep_alive_seconds, keep_alive_interval_seconds, maximum_keep_alive_probes, linger_seconds, linger_in_FIN_WAIT2_seconds, maximum_SYN_transmits);
			this
		})
	}

	#[inline(always)]
	fn new_transmission_control_protocol_over_internet_protocol_version_6(send_buffer_size_in_bytes: usize, receive_buffer_size_in_bytes: usize, idles_before_keep_alive_seconds: u16, keep_alive_interval_seconds: u16, maximum_keep_alive_probes: u16, linger_seconds: u16, linger_in_FIN_WAIT2_seconds: u16, maximum_SYN_transmits: u16) -> Result<Self, CreationError>
	{
		Self::new(AF_INET6, SOCK_STREAM, IPPROTO_TCP).map(|this|
		{
			this.set_internet_protocol_socket_options(send_buffer_size_in_bytes, receive_buffer_size_in_bytes);
			this.set_tcp_socket_options(idles_before_keep_alive_seconds, keep_alive_interval_seconds, maximum_keep_alive_probes, linger_seconds, linger_in_FIN_WAIT2_seconds, maximum_SYN_transmits);
			this
		})
	}

	#[inline(always)]
	fn new_user_datagram_protocol_over_internet_protocol_version_4(send_buffer_size_in_bytes: usize, receive_buffer_size_in_bytes: usize) -> Result<Self, CreationError>
	{
		Self::new(AF_INET, SOCK_DGRAM, IPPROTO_UDP).map(|this|
		{
			this.set_internet_protocol_socket_options(send_buffer_size_in_bytes, receive_buffer_size_in_bytes);
			this.set_udp_socket_options();
			this
		})
	}

	#[inline(always)]
	fn new_user_datagram_protocol_over_internet_protocol_version_6(send_buffer_size_in_bytes: usize, receive_buffer_size_in_bytes: usize) -> Result<Self, CreationError>
	{
		Self::new(AF_INET6, SOCK_DGRAM, IPPROTO_UDP).map(|this|
		{
			this.set_internet_protocol_socket_options(send_buffer_size_in_bytes, receive_buffer_size_in_bytes);
			this.set_udp_socket_options();
			this
		})
	}

	#[inline(always)]
	fn new_streaming_unix_domain_socket(send_buffer_size_in_bytes: usize) -> Result<Self, CreationError>
	{
		Self::new(AF_UNIX, SOCK_STREAM, 0).map(|this|
		{
			this.set_send_buffer_size(send_buffer_size_in_bytes);
			this
		})
	}

	#[inline(always)]
	fn new_datagram_unix_domain_socket(send_buffer_size_in_bytes: usize) -> Result<Self, CreationError>
	{
		Self::new(AF_UNIX, SOCK_DGRAM, 0).map(|this|
		{
			this.set_send_buffer_size(send_buffer_size_in_bytes);
			this
		})
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
