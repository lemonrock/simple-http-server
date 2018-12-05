// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.



#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WorkerThreadConfiguration
{
	/// Poll constraints.
	pub poll_constraints: PollConstraints,

	/// Stack size of worker thread.
	///
	/// Will be rounded up to a minimum stack size for the Operating System.
	pub stack_size: Option<usize>,
}

impl WorkerThreadConfiguration
{
	pub(crate) fn start_worker_threads(&self, terminate: &Terminate, worker_loops: &[LogicalCores]) -> Result<SendToWorkers, WorkerCreationError>
	{
		let mut send_to_workers = SendToWorkers::new(worker_loops.len());

		for logical_cores in worker_loops.iter()
		{
			let (worker_thread_join_handle, send_to_worker, receive_from_worker) = match self.start_worker_thread(terminate.clone(), send_to_workers.current_length(), logical_cores)
			{
				Err(error) =>
				{
					terminate.begin_termination_due_to_configuration_thread_failure();
					return Err(error)
				}

				Ok((worker_thread_join_handle, send_to_worker, receive_from_worker)) => (worker_thread_join_handle, send_to_worker, receive_from_worker),
			};

			send_to_workers.add(worker_thread_join_handle, send_to_worker);

			// TODO: Register this with poll.
			// TODO: Find a clean-cut way to kill this.
			// Possibly a BoxLike structure with a thread or stack local allocator.
			// Yep - a stack-local allocator would work very well. But we do need to call self.drop().

			/*

				Re-work token to be a tag + relative index
					- calculation would then need to scale the relative index (rel_index * size + base_offset)
				Or change arena to include the enum

				Once tokens can go to threads, duplicate / spurious events become problematic

				Avoid the drop hashset by having an 'unoccupied' field?
					- problem is then when can we recycle a token's memory for a new allocation
			*/


			let token = ReceiveFromWorkerThreadToken::new(receive_from_worker);
		}

		// TODO: DO something with receive_from_worker

		Ok(send_to_workers)
	}

	fn start_worker_thread(&self, terminate: Terminate, index: usize, logical_cores: &LogicalCores) -> Result<(JoinHandle<()>, Sender<SendToWorkerEvent>, Receiver<ReceiveFromWorkerEvent>), WorkerCreationError>
	{
		use self::WorkerCreationError::*;

		const OurToken: Token = Token(0);

		let poll = Self::poll()?;

		let (send_to_worker, receive_from_main_loop) = channel::<SendToWorkerEvent>();
		poll.register(&receive_from_main_loop, OurToken, Ready::readable(), PollOpt::edge()).map_err(|error| CouldNotRegisterReceiveChannelWithPoll(error))?;

		let (send_to_main_loop, receive_from_worker) = channel::<ReceiveFromWorkerEvent>();
		let poll_time_out = self.poll_time_out();
		let mut events = self.events();

		let worker_thread_join_handle = self.builder().spawn(move ||
		{
			set_hook(Box::new(|panic_info| terminate.begin_termination_due_to_panic(panic_info)));

			block_all_signals();

			logical_cores.set_current_thread_affinity();

			worker_thread_loop(terminate, poll, events, poll_time_out, receive_from_main_loop, send_to_main_loop)

			()
		}).map_err(|error| CouldNotSpawnWorkerThread(error))?;

		Ok((worker_thread_join_handle, send_to_worker, receive_from_worker))
	}

	#[inline(always)]
	fn poll() -> Result<Poll, WorkerCreationError>
	{
		Poll::new().map_err(|error| WorkerCreationError(error))
	}

	#[inline(always)]
	fn poll_time_out(&self) -> Option<Duration>
	{
		self.poll_constraints.poll_time_out()
	}

	#[inline(always)]
	fn events(&self) -> Events
	{
		self.poll_constraints.events()
	}

	#[inline(always)]
	fn spawn<F: FnOnce() -> T, T: Send + 'static>(self, f: F) -> Result<JoinHandle<T>> where F: Send + 'static
	{
		self.builder().spawn(f)
	}

	#[inline(always)]
	fn builder(&self) -> Builder
	{
		let builder = Builder::new().name(format!("Worker{}", index));
		match self.stack_size
		{
			Some(stack_size) => builder.stack_size(stack_size),
			None => builder,
		}
	}
}
