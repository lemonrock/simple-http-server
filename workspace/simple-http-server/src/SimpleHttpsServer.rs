// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// A simple HTTPS server.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SimpleHttpsServer<'a, SCCUF: ServedClientConnectionsUserFactory>
{
	/// Poll constraints.
	pub poll_constraints: &'a PollConstraints,

	/// Number of CPU logical cores to utilize.
	pub logical_core_utilization_detail: LogicalCoreUtilizationDetail<'a>,

	// TODO: Gets 'used up' creating ConnectionObserver.
	/// Server listeners.
	pub server_listeners: Vec<(ServerListenerConfiguration<'a, SCCUF>, SCCUF)>,

	/// Configuration of worker threads.
	pub worker_thread_configuration: WorkerThreadConfiguration,
}

impl<'a, SCCUF: ServedClientConnectionsUserFactory> SimpleHttpsServer<'a, SCCUF>
{
	/// Main loop that repeats infinitely unless:-
	///
	/// * it can't be created due to an error;
	/// * `terminate` is signalled (this can be done internally by a thread if it panics)
	pub fn execute(&mut self, terminate: Terminate) -> Result<(), MainLoopError>
	{
		// TODO: All errors MUST trigger terminate once workers are running.

		let poll = Self::poll()?;

		let served_client_connection_arena_capacity = self.start_server_listeners()?;

		let served_client_connection_arena = Arena::with_capacity(served_client_connection_arena_capacity);

		let XXXXX = self.start_worker_threads(&terminate, &self.logical_core_utilization_detail.worker_loops)?;

		let result_of_looping = self.loop_around(terminate, poll, served_client_connection_arena);

		// TODO: Register Box'd tokens with something so we can drop them safely.
			// Reconsider the idea of an arena!

		result_of_looping
	}

	fn loop_around(&self, terminate: Terminate, poll: Poll, served_client_connection_arena: Arena<ServedClientConnectionToken<SCCUF>>) -> Result<(), MainLoopError>
	{
		// TODO: Decide which signals to allow through / allow client to control if this is not a main thread.
		block_all_signals();

		self.set_main_loop_thread_affinity();

		let poll_time_out = self.poll_time_out();
		let mut events = self.events();
		let mut drop_token_when_all_events_handled = HashSet::with_capacity(16);

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

					_ => return Err(MainLoopError::PollLoop(error)),
				}
			}

			for event in events.iter()
			{
				TokenKind::handle_event(event.token(), event.ready(), &mut drop_token_when_all_events_handled)
			}

			TokenKind::drop_tokens(&served_client_connection_arena, &mut drop_token_when_all_events_handled)
		}
	}

	/*
		TODO: https://jmarshall.com/easy/http/#conclusion


		// TODO: We can use edge notification if we then hange onto connections with 'remaining' bytes inside a worker thread.

		// TODO: Cap at 10% or 1% the maximum number of connections from any one IP address.

		TODO: 3 Timeout slow and no-progress connections
			- record how much 'real' (ie plain text) data read and written (vs outstanding)
			-
			- consider using the timer wheel design from the networking stack WITH mio's timer, eg
				- wake up every second
				- check for slow connections (schedule a connection for a check)
				- kill connections
	*/


// TODO: TLS
// let mut server_config = self.server_configuration()?;
//	/// Essential TLS configuration.
//	pub tls_configuration: &'a TlsConfiguration,
//	#[inline(always)]
//	fn server_configuration(&self) -> Result<ServerConfig, MainLoopError>
//	{
//		self.tls_configuration.server_configuration().map_err(|error| ServerConfiguration(error))
//	}

	#[inline(always)]
	fn start_server_listeners(&mut self) -> Result<usize, MainLoopError>
	{
		let mut served_client_connection_arena_capacity = 0;

		for (server_listener_configuration, served_client_connection_user_factory) in self.server_listeners.drain()
		{
			served_client_connection_arena_capacity += server_listener_configuration.maximum_connections();
			ServerListenerToken::new(poll, server_listener_configuration, served_client_connection_user_factory)?;
		}

		Ok(served_client_connection_arena_capacity)
	}

	#[inline(always)]
	fn start_worker_threads(&self) -> Result<XXXX, MainLoopError>
	{
		self.worker_thread_configuration.start_worker_threads(&terminate, self.logical_core_utilization_detail).map_err(|error| MainLoopError::WorkerCreation(error))
	}

	#[inline(always)]
	fn set_main_loop_thread_affinity(&self)
	{
		self.logical_core_utilization_detail.main_loop.set_current_thread_affinity()
	}

	#[inline(always)]
	fn poll() -> Result<Poll, MainLoopError>
	{
		Poll::new().map_err(|error| PollCreation(error))
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
}
