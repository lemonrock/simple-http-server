// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Represents the receiving half of a pipe.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReceivePipeFileDescriptor(RawFd);

impl Drop for ReceivePipeFileDescriptor
{
	#[inline(always)]
	fn drop(&mut self)
	{
		if self.0 != Self::StandardInFileDescriptor
		{
			self.0.close()
		}
	}
}

impl AsRawFd for ReceivePipeFileDescriptor
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		self.0
	}
}

impl IntoRawFd for ReceivePipeFileDescriptor
{
	#[inline(always)]
	fn into_raw_fd(self) -> RawFd
	{
		self.0
	}
}

impl Read for ReceivePipeFileDescriptor
{
	/// This particular implementation can only return an `io::ErrorKind` of:-
	///
	/// * `UnexpectedEof`
	/// * `WouldBlock`
	/// * `Interrupted`
	/// * `Other` (which is for when the kernel reports `ENOMEM`, ie it is out of memory).
	#[inline(always)]
	fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>
	{
		let length = buf.len();

		debug_assert!(length < ::std::isize::MAX as usize, "length can not exceed SSIZE_MAX for read()");

		if unlikely!(length == 0)
		{
			return Ok(0)
		}

		let result = unsafe { read(self.as_raw_fd(), buf.as_mut_ptr() as *mut c_void, length) };

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
							EIO => Other,
							EBADF => panic!("The argument `sockfd` is an invalid descriptor"),
							EFAULT => panic!("The receive buffer pointer(s) point outside the process's address space"),
							EINVAL => panic!("Invalid argument passed"),
							EISDIR => panic!("`fd` refers to a directory"),
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

impl ReceivePipeFileDescriptor
{
	const StandardInFileDescriptor: RawFd = 0;

	/// Opens a pipe (FIFO) named in the file system suitable for receiving data from.
	///
	/// Sadly, there is no way to atomically detect if the provided path is **not** a FIFO.
	#[inline(always)]
	pub fn open_fifo_for_receive(fifo_file_path: impl AsRef<Path>) -> Result<Self, SpecialFileOpenError>
	{
		SendPipeFileDescriptor::open_fifo(fifo_file_path, O_RDONLY, Self).map(|optional| optional.expect("ENXIO should not occur when open() is used with the flag O_RDONLY"))
	}

	/// Creates a new pipe.
	///
	/// Identical functionality is provided by `SendPipeFileDescriptor::new_anonymous_pipe()`.
	#[inline(always)]
	pub fn new_anonymous_pipe() -> Result<(SendPipeFileDescriptor, Self), CreationError>
	{
		SendPipeFileDescriptor::new_anonymous_pipe()
	}

	/// Wraps the standard in pipe.
	#[inline(always)]
	pub fn standard_in() -> Self
	{
		Self(Self::StandardInFileDescriptor)
	}
}
