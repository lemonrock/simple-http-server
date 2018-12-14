// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


// TODO Share a file descriptor across threads
// SO_REUSEPORT with SO_INCOMING_CPU
// EPOLLEXCLUSIVE
// setsockopt(http->fd, SOL_SOCKET, SO_REUSEPORT, &val, sizeof(val));


// NEXT: eventfd reactors
// NEXT: timerfd reactor
// NEXT: socket reactors
// TODO: posix message queues and fanotify


// We need an enum to handle all the different types.

// More space efficient to have multiple arenas.

/*

TODO: TcpFastOpen, MSG_ZEROCOPY

TODO: Linux abstract unix domain sockets.

TODO: ?socketpair() (anonymous Unix domain sockets)
	- has a read-write datagram / stream socket
	- has a read-write datagram / stream channel

TODO: ?pipe2() - unidirectional anonymous stream-like, can be epoll monitored?
	- has a read-only stream pipe.
	- has a write-only stream pipe.
	- splice(), tee(), vmsplice().

socket options

connecting client streaming sockets and turning into indistinbuishable streaming ones

TODO: ARENA(s) and an enum pattern.
	- enum pattern could include an 'arena number'?


abstract: an abstract socket address is distinguished (from a
          pathname socket) by the fact that sun_path[0] is a null byte
          ('\0').  The socket's address in this namespace is given by the
          additional bytes in sun_path that are covered by the specified
          length of the address structure.  (Null bytes in the name have no
          special significance.)  The name has no connection with filesystem
          pathnames.  When the address of an abstract socket is returned,
          the returned addrlen is greater than sizeof(sa_family_t) (i.e.,
          greater than 2), and the name of the socket is contained in the
          first (addrlen - sizeof(sa_family_t)) bytes of sun_path.

Linux also supports an 'autobind'.

The SO_SNDBUF socket option does have an effect for UNIX domain sock‐
       ets, but the SO_RCVBUF option does not.  For datagram sockets, the
       SO_SNDBUF value imposes an upper limit on the size of outgoing data‐
       grams.  This limit is calculated as the doubled (see socket(7))
       option value less 32 bytes used for overhead.

*/





// TODO: terminate.


pub fn event_loop(terminate: Terminate, time_out_milliseconds: u16) -> Result<(), EPollCreationError_or_SignalEPollRegistrationError>
{
	let epoll_file_descriptor = EPollFileDescriptor::new()?;

	let signal_reactor = AllSignalReactor::new();
	signal_reactor.register_with_epoll(&epoll_file_descriptor)?

	let ready_event_handler = |epoll_file_descriptor, token, flags|
	{
		// TODO: Define signal_token; maybe have a scheme where there is a tag in token for each of the various fd kinds.
		if token == signal_token
		{
			signal_reactor.react(epoll_file_descriptor, token, flags)
		}
	};

	let mut events: [epoll_event; 1024] = unsafe { uninitialized() };
	let epoll_time_out = EPollTimeOut::in_n_milliseconds(time_out_milliseconds);
	while terminate.should_continue()
	{
		if let Err(error) = epoll_file_descriptor.wait_until_ready(&mut events, epoll_time_out, ready_event_handler)
		{
			debug_assert_eq!(error, EPollWaitError::Interrupted, "error other than interuppted")
		}
	}

	Ok(())
}
