// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) struct SendToWorker
{
	worker_thread_join_handle: JoinHandle<()>,
	send_to_worker: Sender<SendToWorkerEvent>,
}

impl SendToWorker
{
	#[inline(always)]
	pub(crate) fn send_event(&self, send_to_worker_event: SendToWorkerEvent) -> Result<(), SendError<SendToWorkerEvent>>
	{
		self.send_to_worker.send(send_to_worker_event)
	}
}
