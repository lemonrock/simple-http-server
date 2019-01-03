// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


extern "C"
{
	/// `read()` attempts to read up to `count` bytes from file descriptor `fd` into the buffer starting at `buf`.
	///
	/// In the absence of any errors, or if `read()` does not check for errors, a `read()` with a `count` of `0` returns zero and has no other effects.
	///
	/// If `count` is greater than `SSIZE_MAX`, the result is unspecified.
	///
	/// On success, the number of bytes read is returned (zero indicates end of file *unless* `count` was zero).
	///
	/// It is not an error if this number is smaller than the number of bytes requested; this may happen for example because fewer bytes are actually available right now (maybe because we were close to end-of-file, or because we are reading from a pipe, or from a terminal), or because `read()` was interrupted by a signal.
	///
	/// On error, `-1` is returned, and `errno` is set appropriately.
	///
	/// The known errors that can be set in `errno` are:-
	///
	/// * `EAGAIN`: The file descriptor `fd` refers to a file other than a socket and has been marked nonblocking (`O_NONBLOCK`), and the read would block.
	/// * `EAGAIN`: The file descriptor `fd` refers to a socket and has been marked nonblocking (`O_NONBLOCK`), and the read would block.
	/// * `EBADF`: `fd` is not a valid file descriptor or is not open for reading.
	/// * `EFAULT`: `buf` is outside your accessible address space.
	/// * `EINTR`: The call was interrupted by a signal before any data was read.
	/// * `EINVAL`: `fd` is attached to an object which is unsuitable for reading; or the file was opened with the `O_DIRECT` flag, and either the address specified in `buf`, the value specified in `count`, or the current file offset is not suitably aligned.
	/// * `EINVAL`: `fd` was created via a call to `timerfd_create()` and the wrong size buffer was given to `read()`.
	/// * `EIO`: I/O error. This will happen for example when the process is in a background process group, tries to read from its controlling terminal, and either it is ignoring or blocking `SIGTTIN` or its process group is orphaned. It may also occur when there is a low-level I/O error while reading from a disk or tape.
	/// * `EISDIR`: `fd` refers to a directory.
	pub(crate) fn read(fd: RawFd, buf: *mut c_void, count: size_t) -> ssize_t;
}
