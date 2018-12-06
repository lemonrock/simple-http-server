// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


// This should be an epoll main loop

// Needs to handle signals properly
// eg sigprocmask


// TODO Share a file descriptor across threads
// SO_REUSEPORT with SO_INCOMING_CPU
// EPOLLEXCLUSIVE


// Bind an acceptor() per thread using SO_REUSEPORT



pub struct TcpListenerSocketFileDescriptor(RawFd);

// SO_REUSEPORT
	// glibc, musl, bionic, emscripten, fuschia
	// probably with SO_REUSEADDR

// setsockopt(http->fd, SOL_SOCKET, SO_REUSEPORT, &val, sizeof(val));



pub struct EventLoop
{

}
