// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) struct ReceiveFromWorkerThreadToken<SCCUF: ServedClientConnectionUserFactory>
{
	receive_from_worker: Receiver<ReceiveFromWorkerEvent>,
}

impl<SCCUF: ServedClientConnectionUserFactory> ReceiveFromWorkerThreadToken<SCCUF>
{
	pub(crate) fn new(receive_from_worker: Receiver<ReceiveFromWorkerEvent>) -> Result<Token, MainLoopError>
	{
		let mut this = Box::new
		(
			Self
			{
				receive_from_worker,
			}
		);

		let receive_from_worker = &this.receive_from_worker as *const Receiver<Y>;

		let token = TokenKind::ReceiveFromWorkerThread.into_token_from_box(this);

		match poll.register(unsafe { & * receive_from_worker }, token, Ready::readable(), PollOpt::edge())
		{
			Ok(()) => Ok(token),

			Err(error) =>
			{
				drop(token.as_box::<Self>());
				Err(MainLoopError::CouldNotRegisterChannelWithPoll(error))
			}
		}

		Ok(token)
	}

	#[inline(always)]
	pub(crate) fn handle_event(&mut self, poll: &Poll) -> bool
	{
		use self::TryRecvError::*;

		match self.receive_from_worker.try_recv()
		{
			Err(Empty) => false,

			Err(Disconnected) => true,

			Ok(receive_from_worker_event) => receive_from_worker_event.handle_event(poll),
		}
	}
}
