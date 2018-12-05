// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


#[inline(always)]
pub(crate) fn worker_thread_loop(terminate: Terminate, poll: &Poll, mut events: Events, poll_time_out: Option<Duration>, receive_from_main_loop: Receiver<SendToWorkerEvent>, send_to_main_loop: Sender<ReceiveFromWorkerEvent>)
{
	while terminate.should_continue()
	{
		if let Err(error) = poll.poll(&mut events, poll_time_out)
		{
			use self::ErrorKind::*;

			match error.kind()
			{
				// Spurious wake up.
				Interrupted => continue,

				// Should have been handled by mio?
				TimedOut => continue,

				_ => panic!(error),
			}
		}

		for event in events.iter()
		{
			debug_assert!(event.token() == OurToken, "token of event is not ours");
			debug_assert!(event.ready() == Ready::readable(), "event is not readable");

			use self::TryRecvError::*;

			match receive_from_main_loop.try_recv()
			{
				Err(Empty) => continue,

				Err(Disconnected) => return (),

				Ok(value) => XXXXXXXX,
			}
		}
	}
}
