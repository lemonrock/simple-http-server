// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


extern "C"
{
	/// `write()` writes up to `count` bytes from the buffer pointed `buf` to the file referred to by the file descriptor `fd`.
	///
	/// The number of bytes written may be less than count if, for example, there is insufficient space on the underlying physical medium, or the `RLIMIT_FSIZE` resource limit is encountered (see `man 2 setrlimit`), or the call was interrupted by a signal handler after having written less than `count` bytes.
	///
	/// If `count` is zero and `fd` refers to a file other than a regular file, the results are not specified.
	///
	/// On success, the number of bytes written is returned (zero indicates nothing was written).
	/// On error, `-1` is returned, and `errno` is set appropriately.
	///
	/// The known errors that can be set in `errno` are:-
	///
	/// * `EAGAIN`: The file descriptor `fd` refers to a file other than a socket and has been marked nonblocking (`O_NONBLOCK`), and the write would block.
	/// * `EAGAIN`: The file descriptor `fd` refers to a socket and has been marked nonblocking (`O_NONBLOCK`), and the write would block.
	/// * `EBADF`: `fd` is not a valid file descriptor or is not open for writing.
	/// * `EDESTADDRREQ`: `fd` refers to a datagram socket for which a peer address has not been set using `connect()`.
	/// * `EDQUOT`: The user's quota of disk blocks on the file system containing the file referred to by `fd` has been exhausted.
	/// * `EFAULT`: `buf` is outside your accessible address space.
	/// * `EFBIG`: An attempt was made to write a file that exceeds the implementation-defined maximum file size or the process's file size limit, or to write at a position past the maximum allowed offset.
	/// * `EINTR`: The call was interrupted by a signal before any data was written.
	/// * `EINVAL`: `fd` is attached to an object which is unsuitable for writing; or the file was opened with the `O_DIRECT` flag, and either the address specified in `buf`, the value specified in `count`, or the current file offset is not suitably aligned.
	/// * `EIO`: A low-level I/O error occurred while modifying the inode.
	/// * `ENOSPC`: The device containing the file referred to by `fd` has no room for the data.
	/// * `EPIPE`: `fd` is connected to a pipe or socket whose reading end is closed. When this happens the writing process will also receive a `SIGPIPE` signal. (Thus, the write return value is seen only if the program catches, blocks or ignores this signal).
	pub(crate) fn write(fd: RawFd, buf: *const c_void, count: size_t) -> ssize_t;
}

