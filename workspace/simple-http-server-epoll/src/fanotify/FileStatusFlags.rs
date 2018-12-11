// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


bitflags!
{
	/// Flags control the status of a newly opened file descriptor by fanotify.
	///
	/// Before Linux 3.18 the features these flags imply was very buggily implemented.
	pub struct FileStatusFlags: u32
	{
		/// This value allows only read access.
		const ReadOnly = O_RDONLY as u32;

		/// This value allows only write access.
		const WriteOnly = O_WRONLY as u32;

		/// This value allows read and write access.
		const ReadAndWrite = O_RDWR as u32;

		/// Enable support for files exceeding 2 GB.
		///
		/// Failing to set this flag will result in an `EOVERFLOW` error when trying to open a large file which is monitored by an fanotify group on a 32-bit system.
		///
		/// Note that this value differs widely across architectures, and even is the same as `ReadOnly` on x86-64.
		const LargeFile = O_LARGEFILE as u32;

		/// Enable the close-on-exec flag for the file descriptor.
		const CloseOnExce = O_CLOEXEC as u32;

		/// Enable the non-blocking flag.
		const NonBlocking = O_NONBLOCK as u32;

		/// Enable append.
		const Append = O_APPEND as u32;

		/// Every `write()` to the file returns only when the contents of the file have been written to disk.
		///
		/// This provides the guarantee that when the system call returns the file data is on disk.
		const DataSynchronize = O_DSYNC as u32;

		/// Do not adjust access time.
		const NoAccessTime = O_NOATIME as u32;

		/// Every `write()` to the file returns only when the contents of the file have been written to disk and also the file metadata has been written to disk.
		///
		/// This provides the guarantee that when the system call returns the file data is on disk.
		///
		/// It is a stronger guarantee than `DataSynchronize`.
		const Synchronize = O_SYNC as u32;
	}
}
