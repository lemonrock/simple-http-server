// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// An explanation of the `FilePathInvalid` error that can occur during binding of a socket instance.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FilePathInvalidReason
{
	/// A nonexistent interface was requested or the requested address was not local (eg it was on a NFS mount).
	AddressUnavailable,

	/// Too many symbolic links were encountered in resolving the file path.
	TooManySymbolicLinksInFilePath,

	/// The file path does not exist.
	DoesNotExist,

	/// A component of the file path prefix is not a directory.
	FilePathPrefixComponentIsNotADirectory,

	/// The socket inode would reside on a read-only file system.
	FilePathIsReadOnly,
}
