// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


use super::*;
use self::Status::*;
use self::InvalidReason::*;


include!("Bytes.rs");
include!("HeaderReentryPoint.rs");
include!("InvalidReason.rs");
include!("NonNullExt.rs");
include!("RequestUser.rs");
include!("Status.rs");
include!("TargetUriReentryPoint.rs");








/*
	https://jmarshall.com/easy/http/#conclusion



	1 For efficient connection handling:-

		- smuggle a pointer in the mio Token
		- connection identifiers are just mio Tokens
		- pointer in the mio token needs to not affect counts of pointer usage
		- use the bottom 2 - 3 bits for a 'type' or the top 12-13 bits (x86-64 only)

	2 Multi-threading
		- use mio-more/extra to have a timer and a channel
		- dispatch new connections to a multicast channel
		- recipients pick up and manage the connection
			- perhaps with their own mio Polls
				- it is possible for things to become unbalanced
			- or return events to register back to the 'main' loop on another channel

	3 Timeout slow and no-progress connections
		- record how much 'real' (ie plain text) data read and written (vs outstanding)
		-
		- consider using the timer wheel design from the networking stack WITH mio's timer, eg
			- wake up every second
			- check for slow connections (schedule a connection for a check)
			- kill connections


	4 For persistent connections, we need to either:-
		- be able to reset a buffer, eg after parsing headers
			- potentially expensive as we need to move (memmove) data
		- extend a buffer
			- potentially never-ending
		-

	We need to be able to 'retire' pointers / slices for things like header values and target uris, as well as received data (especially if it comes in chunked).

		- retiring with one buffer requires memmove;

		- retiring with a ring buffer;
			- needs an equivalent to /dev/shm for FreeBSD and MacOSX
				- this is just tmpfs; may be /tmp?
				- need to mount tmpfs,  eg tmpfs                 3.9G  168K  3.9G   1% /dev/shm
				- mount -o remount,size=8G /dev/shm

		- retiring by requesting more buffers and abstracting across them
			- potentially expensive to deal with in terms of checks
			- useful to be able to allocate larger buffers for request or response bodies and smaller ones for headers
			- retirement still a concern
			- potentially more expensive as need to make calls to get and return a buffer


	APIs our buffer design needs to support

		TLS
			where `buf` is a slice we need to be able to provide

			fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>
			fn write(&mut self, buf: &[u8]) -> io::Result<usize>

		TcpStream
			fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>
			fn write(&mut self, buf: &[u8]) -> io::Result<usize>

			and maybe (only appropriate for unencrypted connections)

			fn read_bufs(&self, bufs: &mut [&mut IoVec]) -> io::Result<usize>
			fn write_bufs(&self, bufs: &[&IoVec]) -> io::Result<usize>



*/
