// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Represents a socket instance.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SocketFileDescriptor(RawFd);

impl Drop for SocketFileDescriptor
{
	#[inline(always)]
	fn drop(&mut self)
	{
		// Please see <http://austingroupbugs.net/view.php?id=529> and <http://austingroupbugs.net/view.php?id=529> for why ignoring the `EINTR` error on close is actually sane.
		//
		// Frankly, the defects here are those of POSIX: (a) signals, and (b) using a file descriptor so small that it isn't thread safe.
		//
		// To be far, both signals and file descriptors predate threads by a long way.
		unsafe { close(self.0) };
	}
}

impl AsRawFd for SocketFileDescriptor
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		self.0
	}
}

impl IntoRawFd for SocketFileDescriptor
{
	#[inline(always)]
	fn into_raw_fd(self) -> RawFd
	{
		self.0
	}
}

impl SocketFileDescriptor
{
	/// Creates a new instance of a Transmission Control Protocol (TCP) socket over Internet Protocol (IP) version 4.
	#[inline(always)]
	fn new_transmission_control_protocol_over_internet_protocol_version_4() -> Result<Self, SocketCreationError>
	{
		Self::new(AF_INET, SOCK_STREAM, IPPROTO_TCP)
	}

	/// Creates a new instance of a Transmission Control Protocol (TCP) socket over Internet Protocol (IP) version 6.
	#[inline(always)]
	fn new_transmission_control_protocol_over_internet_protocol_version_6() -> Result<Self, SocketCreationError>
	{
		Self::new(AF_INET6, SOCK_STREAM, IPPROTO_TCP)
	}

	/// Creates a new instance of a User Datagram Protocol (UDP) socket over Internet Protocol (IP) version 4.
	#[inline(always)]
	fn new_user_datagram_protocol_over_internet_protocol_version_4() -> Result<Self, SocketCreationError>
	{
		Self::new(AF_INET, SOCK_DGRAM, IPPROTO_UDP)
	}

	/// Creates a new instance of a User Datagram Protocol (UDP) socket over Internet Protocol (IP) version 6.
	#[inline(always)]
	fn new_user_datagram_protocol_over_internet_protocol_version_6() -> Result<Self, SocketCreationError>
	{
		Self::new(AF_INET6, SOCK_DGRAM, IPPROTO_UDP)
	}

	/// Creates a new streaming Unix Domain server listener socket.
	///
	/// This is local socket akin to a Transmission Control Protocol (TCP) socket.
	#[inline(always)]
	pub fn new_streaming_unix_domain_socket_server_listener(path: impl AsRef<Path>) -> Result<(), NewSocketServerListenerError>
	{
		let this = Self::new_streaming_unix_domain_socket()?;
		this.bind_unix_domain_socket(path)?;
		Ok(())
	}

	/// Creates a new streaming Unix Domain client socket.
	///
	/// This is local socket akin to a Transmission Control Protocol (TCP) socket.
	#[inline(always)]
	pub fn new_streaming_unix_domain_socket_client(path: impl AsRef<Path>) -> Result<(), NewSocketClientError>
	{
		let this = Self::new_streaming_unix_domain_socket()?;
		this.connect_unix_domain_socket(path)?;
		Ok(())
	}

	/// Creates a new datagram Unix Domain server listener socket.
	///
	/// This is local socket akin to an User Datagram Protocol (UDP) socket.
	#[inline(always)]
	pub fn new_datagram_unix_domain_socket_server_listener(path: impl AsRef<Path>) -> Result<(), NewSocketServerListenerError>
	{
		let this = Self::new_datagram_unix_domain_socket()?;
		this.bind_unix_domain_socket(path)?;
		Ok(())
	}

	/// Creates a new datagram Unix Domain client socket.
	///
	/// This is local socket akin to an User Datagram Protocol (UDP) socket.
	#[inline(always)]
	pub fn new_datagram_unix_domain_socket_client(path: impl AsRef<Path>) -> Result<(), NewSocketClientError>
	{
		let this = Self::new_datagram_unix_domain_socket()?;
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

		let path_bytes = path.as_ref().as_os_str().as_bytes();
		let path_bytes_length = path_bytes.len();
		debug_assert!(path_bytes_length <= socket_data.sun_path.len(), "Path converted to bytes is more than 108-bytes long");
		unsafe { socket_data.sun_path.as_mut_ptr().copy_from_nonoverlapping(path_bytes.as_ptr() as *const _, path_bytes_length) };

		socket_data
	}

	#[inline(always)]
	fn bind<SD: SocketData>(&self, socket_data: &SD) -> Result<(), SocketBindError>
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
	fn connect<SD: SocketData>(&self, socket_data: &SD) -> Result<(), SocketConnectError>
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

//	#[inline(always)]
//	pub fn bind(self) -> ServerSocketFileDescriptor
//	{
//		ServerSocketFileDescriptor(self)
//	}
//
//	#[inline(always)]
//	pub fn connect(self) -> ClientSocketFileDescriptor
//	{
//		ClientSocketFileDescriptor(self)
//	}
	/*

	Solaris

pub type socklen_t = ::c_uint;
pub type sa_family_t = u16;

    pub struct sockaddr {
        pub sa_family: sa_family_t,
        pub sa_data: [::c_char; 14],
}
    pub struct sockaddr_in {
        pub sin_family: sa_family_t,
        pub sin_port: ::in_port_t,
        pub sin_addr: ::in_addr,
        pub sin_zero: [::c_char; 8]
    }

    pub struct sockaddr_in6 {
        pub sin6_family: sa_family_t,
        pub sin6_port: ::in_port_t,
        pub sin6_flowinfo: u32,
        pub sin6_addr: ::in6_addr,
        pub sin6_scope_id: u32,
        pub __sin6_src_id: u32
    }

    pub struct sockaddr_un {
        pub sun_family: sa_family_t,
        pub sun_path: [c_char; 108]



   Fuschia


pub type socklen_t = u32;
pub type sa_family_t = u16;

    pub struct sockaddr {
        pub sa_family: sa_family_t,
        pub sa_data: [::c_char; 14],
    }

    pub struct sockaddr_in {
        pub sin_family: sa_family_t,
        pub sin_port: ::in_port_t,
        pub sin_addr: ::in_addr,
        pub sin_zero: [u8; 8],
    }

    pub struct sockaddr_in6 {
        pub sin6_family: sa_family_t,
        pub sin6_port: ::in_port_t,
        pub sin6_flowinfo: u32,
        pub sin6_addr: ::in6_addr,
        pub sin6_scope_id: u32,
    }

    pub struct sockaddr_un {
        pub sun_family: sa_family_t,
        pub sun_path: [::c_char; 108]
    }

    pub struct sockaddr_storage {
        pub ss_family: sa_family_t,
        __ss_align: ::size_t,
        __ss_pad2: [u8; 128 - 2 * 8],
}


	Linux

pub type sa_family_t = u16;


    pub struct sockaddr {
        pub sa_family: sa_family_t,
        pub sa_data: [::c_char; 14],
    }

    pub struct sockaddr_in {
        pub sin_family: sa_family_t,
        pub sin_port: ::in_port_t,
        pub sin_addr: ::in_addr,
        pub sin_zero: [u8; 8],
    }

    pub struct sockaddr_in6 {
        pub sin6_family: sa_family_t,
        pub sin6_port: ::in_port_t,
        pub sin6_flowinfo: u32,
        pub sin6_addr: ::in6_addr,
        pub sin6_scope_id: u32,
    }

    pub struct sockaddr_un {
        pub sun_family: sa_family_t,
        pub sun_path: [::c_char; 108]
    }

    pub struct sockaddr_storage {
        pub ss_family: sa_family_t,
        __ss_align: ::size_t,
        #[cfg(target_pointer_width = "32")]
        __ss_pad2: [u8; 128 - 2 * 4],
        #[cfg(target_pointer_width = "64")]
        __ss_pad2: [u8; 128 - 2 * 8],
}

}


		For a client, call connect()

		For a server, call bind()
	*/



	#[inline(always)]
	fn new_streaming_unix_domain_socket() -> Result<Self, SocketCreationError>
	{
		Self::new(AF_UNIX, SOCK_STREAM, 0)
	}

	#[inline(always)]
	fn new_datagram_unix_domain_socket() -> Result<Self, SocketCreationError>
	{
		Self::new(AF_UNIX, SOCK_DGRAM, 0)
	}

	/// Creates a new instance.
	#[inline(always)]
	fn new(domain: c_int, type_: c_int, ethernet_protocol: c_int) -> Result<Self, SocketCreationError>
	{
		const Flags: c_int = SOCK_NONBLOCK | SOCK_CLOEXEC;

		let result = unsafe { socket(domain, type_ | Flags, ethernet_protocol) };
		if likely!(result != -1)
		{
			Ok(SocketFileDescriptor(result))
		}
		else
		{
			use self::SocketCreationError::*;

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
