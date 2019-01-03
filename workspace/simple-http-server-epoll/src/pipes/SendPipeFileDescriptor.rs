// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Represents the sending half of a pipe.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SendPipeFileDescriptor(RawFd);

impl Drop for SendPipeFileDescriptor
{
	#[inline(always)]
	fn drop(&mut self)
	{
		match self.0
		{
			Self::StandardOutFileDescriptor | Self::StandardErrorFileDescriptor => (),
			_ => self.0.close(),
		}
	}
}

impl AsRawFd for SendPipeFileDescriptor
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		self.0
	}
}

impl IntoRawFd for SendPipeFileDescriptor
{
	#[inline(always)]
	fn into_raw_fd(self) -> RawFd
	{
		self.0
	}
}

impl Write for SendPipeFileDescriptor
{
	/// This particular implementation can only return an `io::ErrorKind` of:-
	///
	/// * `WriteZero` (implying end-of-file).
	/// * `WouldBlock`
	/// * `Interrupted`
	/// * `BrokenPipe`
	#[inline(always)]
	fn write(&mut self, buf: &[u8]) -> io::Result<usize>
	{
		let length = buf.len();

		if unlikely!(length == 0)
		{
			return Ok(0)
		}

		let result = unsafe { write(self.as_raw_fd(), buf.as_ptr() as *const c_void, buf.len()) };

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
							EPIPE => BrokenPipe,
							ENOSPC => panic!("The device containing the file referred to by `fd` has no room for the data"),
							EBADF => panic!("The argument `fd` is an invalid descriptor"),
							EFAULT => panic!("The write buffer pointer(s) point outside the process's address space"),
							EINVAL => panic!("Invalid argument passed"),
							EDESTADDRREQ => panic!("`fd` refers to a datagram socket for which a peer address has not been set using `connect()`"),
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

impl SendPipeFileDescriptor
{
	const StandardOutFileDescriptor: RawFd = 1;

	const StandardErrorFileDescriptor: RawFd = 2;

	/// Opens a pipe (FIFO) named in the file system suitable for sending data to.
	///
	/// Sadly, there is no way to atomically detect if the provided path is **not** a FIFO.
	///
	/// Returns `Ok(Some(Self))` if successful.
	/// Returns `Ok(None)` if there wasn't a process already receiving from this FIFO.
	#[inline(always)]
	pub fn open_fifo_for_send(fifo_path: impl AsRef<Path>) -> Result<Option<Self>, FifiOpenError>
	{
		Self::open_fifo(fifo_path, O_WRONLY, Self)
	}

	/// Opens a pipe (FIFO) named in the file system suitable for sending data to.
	///
	/// Sadly, there is no way to atomically detect if the provided path is **not** a FIFO.
	///
	/// Opens regardless of whether another process is already receiving from this FIFO.
	#[inline(always)]
	pub fn open_fifo_for_send_irrespective_of_another_process_already_having_opened_the_fifo_for_receive(fifo_path: impl AsRef<Path>) -> Result<Self, FifiOpenError>
	{
		Self::open_fifo(fifo_path, O_RDWR, Self).map(|optional| optional.expect("ENXIO should not occur with O_RDWR set in open()"))
	}

	/// Creates a new pipe.
	///
	/// Identical functionality is provided by `ReceivePipeFileDescriptor::new_anonymous_pipe()`.
	#[inline(always)]
	pub fn new_anonymous_pipe() -> Result<(Self, ReceivePipeFileDescriptor), CreationError>
	{
		let mut pipe_file_descriptors = unsafe { uninitialized() };
		let result = unsafe { pipe2(&mut pipe_file_descriptors, O_NONBLOCK | O_CLOEXEC) };
		if likely!(result == 0)
		{
			Ok((SendPipeFileDescriptor(pipe_file_descriptors[1]), ReceivePipeFileDescriptor(pipe_file_descriptors[0])))
		}
		else if likely!(result == -1)
		{
			use self::CreationError::*;

			Err
			(
				match errno().0
				{
					EMFILE => PerProcessLimitOnNumberOfFileDescriptorsWouldBeExceeded,
					ENFILE => SystemWideLimitOnTotalNumberOfFileDescriptorsWouldBeExceeded,
					EFAULT => panic!("`pipefd` is not valid"),
					EINVAL => panic!("Invalid value in `flags`"),

					_ => unreachable!(),
				}
			)
		}
		else
		{
			unreachable!()
		}
	}

	/// Wraps the standard out pipe.
	///
	/// Normally of very limited value as standard out is nearly always writable.
	#[inline(always)]
	pub fn standard_out() -> Self
	{
		Self(Self::StandardOutFileDescriptor)
	}

	/// Wraps the standard error pipe.
	///
	/// Normally of very limited value as standard error is nearly always writable.
	#[inline(always)]
	pub fn standard_error() -> Self
	{
		Self(Self::StandardErrorFileDescriptor)
	}

	#[inline(always)]
	pub(crate) fn open_fifo<PFD>(fifo_path: impl AsRef<Path>, access_flag: c_int, constructor: impl FnOnce(RawFd) -> PFD) -> Result<Option<PFD>, FifiOpenError>
	{
		let fifo_path = CString::new(path_bytes_without_trailing_nul(&fifo_path)).unwrap();

		const CommonFlags: c_int = O_CLOEXEC | O_NONBLOCK;

		let result = unsafe { open(fifo_path.as_ptr(), access_flag | CommonFlags) };
		if likely!(result != -1)
		{
			Ok(Some(constructor(result)))
		}
		else
		{
			use self::CreationError::*;
			use self::FifiOpenError::*;
			use self::InvalidFifoPathReason::*;

			Err
			(
				match errno().0
				{
					EACCES => Common(PermissionDenied),
					EMFILE => Common(PerProcessLimitOnNumberOfFileDescriptorsWouldBeExceeded),
					ENFILE => Common(SystemWideLimitOnTotalNumberOfFileDescriptorsWouldBeExceeded),
					ENOMEM => Common(KernelWouldBeOutOfMemory),
					EAGAIN => WouldBlock,
					EINTR => Interrupted,
					ELOOP => InvalidFifoPath(TooManySymbolicLinks),
					ENAMETOOLONG => InvalidFifoPath(TooLong),
					EISDIR => InvalidFifoPath(IsADirectory),
					ENOENT => InvalidFifoPath(DoesNotExist),
					ENOTDIR => InvalidFifoPath(ParentComponentIsNotADirectory),
					ENODEV | EROFS | ETXTBSY => InvalidFifoPath(ExistsButCanNotBeUsed),

					ENXIO => if access_flag == O_WRONLY
					{
						return Ok(None)
					}
					else
					{
						InvalidFifoPath(ExistsButCanNotBeUsed)
					},

					EDQUOT => panic!("Where `O_CREAT `is specified, the file does not exist, and the user's quota of disk blocks or inodes on the file system has been exhausted"),
					EEXIST => panic!("`pathname` already exists and `O_CREAT` and `O_EXCL` were used"),
					EFAULT => panic!("`pathname` points outside your accessible address space"),
					EFBIG | EOVERFLOW => panic!("`pathname` refers to a regular file that is too large to be opened. The usual scenario here is that an application compiled on a 32-bit platform without `-D_FILE_OFFSET_BITS=64` tried to open a file whose size exceeds `(2<<31)-1` bits; see also `O_LARGEFILE` above. This is the error specified by POSIX.1-2001; in kernels before 2.6.24, Linux gave the error `EFBIG` for this case"),
					ENOSPC => panic!("`pathname` was to be created but the device containing `pathname` has no room for the new file"),
					EPERM => panic!("The `O_NOATIME` flag was specified, but the effective user ID of the caller did not match the owner of the file and the caller was not privileged (`CAP_FOWNER`)"),

					_ => unreachable!(),
				}
			)
		}
	}

}
