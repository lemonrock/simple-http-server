// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) struct SendToWorkers
{
	send_to_workers: Vec<SendToWorker>,
	index_of_next_worker_to_send_to: usize,
}

impl SendToWorkers
{
	#[inline(always)]
	pub(crate) fn new(number_of_worker_threads: usize) -> Self
	{
		Self
		{
			send_to_workers: Vec::with_capacity(number_of_worker_threads),
			index_of_next_worker_to_send_to: 0,
		}
	}

	#[inline(always)]
	pub(crate) fn add(&mut self, worker_thread_join_handle: JoinHandle<()>, send_to_worker: Sender<SendToWorkerEvent>)
	{
		self.send_to_workers.push
		(
			SendToWorker
			{
				worker_thread_join_handle,
				send_to_worker,
			}
		);
	}

	/// Returns `true` if there are no workers to send the event to.
	#[inline(always)]
	pub(crate) fn send_event(&mut self, mut send_to_worker_event: SendToWorkerEvent) -> bool
	{
		let mut initial_next = self.index_of_next_worker_to_send_to;

		let mut index = self.index_of_next_worker_to_send_to;
		while self.current_length() != 0
		{
			match self.send_event_to_channel(index, send_to_worker_event)
			{
				Ok(()) =>
				{
					self.index_of_next_worker_to_send_to = if index == self.current_length() - 1
					{
						0
					}
					else
					{
						index + 1
					};

					return false
				}

				Err(send_error) =>
				{
					use self::SendError::*;
					match send_error
					{
						Io(io_error) => panic!("An IO error should not be possible for a channel: `{}`", io_error),

						Disconnected(returned_send_to_worker_event) =>
						{
							send_to_worker_event = returned_send_to_worker_event;

							self.send_to_workers.swap_remove(index);
							if index == self.current_length()
							{
								index = 0;
							}
						}
					}
				}
			}
		}

		true
	}

	#[inline(always)]
	fn send_event_to_channel(&self, index: usize, send_to_worker_event: SendToWorkerEvent) -> Result<(), SendError<SendToWorkerEvent>>
	{
		(unsafe { self.send_to_workers.get_unchecked(index) }).send_event()
	}

	#[inline(always)]
	pub(crate) fn current_length(&self) -> usize
	{
		self.send_to_workers.len()
	}
}
