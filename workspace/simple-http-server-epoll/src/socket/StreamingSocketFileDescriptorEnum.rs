// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Represents a streaming socket instance between two peers.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StreamingSocketFileDescriptorEnum
{
	/// An Internet Protocol (IP) version 4 streaming socket.
	InternetProtocolVersion4(StreamingSocketFileDescriptor<sockaddr_in>),

	/// An Internet Protocol (IP) version 6 streaming socket.
	InternetProtocolVersion6(StreamingSocketFileDescriptor<sockaddr_in6>),

	/// An Unix Domain streaming socket.
	UnixDomain(StreamingSocketFileDescriptor<sockaddr_un>),
}

impl AsRawFd for StreamingSocketFileDescriptorEnum
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		use self::StreamingSocketFileDescriptorEnum::*;

		match self
		{
			&InternetProtocolVersion4(ref streaming_listener_socket_file_descriptor) => streaming_listener_socket_file_descriptor.as_raw_fd(),
			&InternetProtocolVersion6(ref streaming_listener_socket_file_descriptor) => streaming_listener_socket_file_descriptor.as_raw_fd(),
			&UnixDomain(ref streaming_listener_socket_file_descriptor) => streaming_listener_socket_file_descriptor.as_raw_fd(),
		}
	}
}

impl Read for StreamingSocketFileDescriptorEnum
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
		use self::StreamingSocketFileDescriptorEnum::*;

		match self
		{
			&mut InternetProtocolVersion4(ref mut streaming_listener_socket_file_descriptor) => streaming_listener_socket_file_descriptor.read(buf),
			&mut InternetProtocolVersion6(ref mut streaming_listener_socket_file_descriptor) => streaming_listener_socket_file_descriptor.read(buf),
			&mut UnixDomain(ref mut streaming_listener_socket_file_descriptor) => streaming_listener_socket_file_descriptor.read(buf),
		}
	}

	#[inline(always)]
	unsafe fn initializer(&self) -> Initializer
	{
		Initializer::nop()
	}
}

impl Write for StreamingSocketFileDescriptorEnum
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
		use self::StreamingSocketFileDescriptorEnum::*;

		match self
		{
			&mut InternetProtocolVersion4(ref mut streaming_listener_socket_file_descriptor) => streaming_listener_socket_file_descriptor.write(buf),
			&mut InternetProtocolVersion6(ref mut streaming_listener_socket_file_descriptor) => streaming_listener_socket_file_descriptor.write(buf),
			&mut UnixDomain(ref mut streaming_listener_socket_file_descriptor) => streaming_listener_socket_file_descriptor.write(buf),
		}
	}

	#[inline(always)]
	fn flush(&mut self) -> io::Result<()>
	{
		Ok(())
	}
}
