// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// An Unix socket address.
pub enum UnixSocketAddress<FilePath: AsRef<Path>>
{
	/// A file in a file system.
	File
	{
		/// An Unix Domain Socket file path of up to 107 bytes.
		socket_file_path: FilePath,

		/// `u32` is the permissions of the parent folder of the file path; it is only used by listeners.
		parent_folder_mode: u32,
	},

	/// A Linux-specific abstractly named socket.
	Abstract
	{
		/// An abstract name of zero or more bytes.
		abstract_name: ArrayVec<[u8; sockaddr_un::PathLength - 1]>,
	}
}
