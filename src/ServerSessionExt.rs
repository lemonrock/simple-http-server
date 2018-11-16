// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


trait ServerSessionExt
{
	fn readiness(&self) -> Ready;

	fn poll_opt(&self) -> PollOpt;

	#[inline(always)]
	fn register(&self, poll: &Poll, socket: &TcpStream, client_token: Token) -> io::Result<()>
	{
		poll.register(socket, client_token, self.readiness(), self.poll_opt())
	}

	#[inline(always)]
	fn reregister(&self, poll: &Poll, socket: &TcpStream, client_token: Token) -> io::Result<()>
	{
		poll.reregister(socket, client_token, self.readiness(), self.poll_opt())
	}
}

impl ServerSessionExt for ServerSession
{
	#[inline(always)]
	fn readiness(&self) -> Ready
	{
		let wants_read = server_session.wants_read();
		let wants_write = server_session.wants_write();

		let mut readiness = Ready::empty();

		if server_session.wants_read()
		{
			readiness |= Ready::readable();
		}

		if server_session.wants_write()
		{
			readiness |= Ready::writable();
		}

		debug_assert_ne!(readiness, Ready::empty(), "Session wants neither read nor write");

		readiness | UnixReady::error() | UnixReady::hup()
	}

	#[inline(always)]
	fn poll_opt(&self) -> PollOpt
	{
		PollOpt::level()
	}
}
