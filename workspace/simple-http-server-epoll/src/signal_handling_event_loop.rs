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
