// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


// TODO Share a file descriptor across threads
// SO_REUSEPORT with SO_INCOMING_CPU
// EPOLLEXCLUSIVE
// setsockopt(http->fd, SOL_SOCKET, SO_REUSEPORT, &val, sizeof(val));

use ::libc::SIG_BLOCK;
use ::libc::pthread_sigmask;
use ::libc::sigfillset;
use ::libc::SIGHUP;
use ::libc::SIGINT;
use ::libc::SIGQUIT;
use ::libc::SIGILL;
use ::libc::SIGTRAP;
use ::libc::SIGABRT;
use ::libc::SIGBUS;
use ::libc::SIGFPE;
use ::libc::SIGUSR1;
use ::libc::SIGSEGV;
use ::libc::SIGUSR2;
use ::libc::SIGPIPE;
use ::libc::SIGALRM;
use ::libc::SIGTERM;
use ::libc::SIGSTKFLT;
use ::libc::SIGCHLD;
use ::libc::SIGCONT;
use ::libc::SIGTSTP;
use ::libc::SIGTTIN;
use ::libc::SIGTTOU;
use ::libc::SIGXCPU;
use ::libc::SIGXFSZ;
use ::libc::SIGVTALRM;
use ::libc::SIGPROF;
use ::libc::SIGWINCH;
use ::libc::SIGIO;
use ::libc::SIGPWR;
use ::libc::SIGSYS;
use ::std::process::abort;
use ::std::process::exit;

/// The signals `SIGKILL` and `SIGSTOP` can not be handled.
pub trait SignalHandler
{
	/// Generic functionality for exiting; exists to allow easy customization of handling of a number of non-fatal signals.
	#[inline(always)]
	fn terminate(&mut self)
	{
		exit(1);
	}

	/// Handle the `SIGHUP` signal.
	///
	/// By default calls `self.terminate()`.
	///
	/// The `SIGHUP` signal is sent to a process when its controlling terminal is closed
	/// It was originally designed to notify the process of a serial line drop (a hangup).
	/// In modern systems, this signal usually means that the controlling pseudo or virtual terminal has been closed.
	/// Many daemons will reload their configuration files and reopen their logfiles instead of exiting when receiving this signal.
	#[inline(always)]
	fn hup(&mut self)
	{
		self.terminate()
	}

	/// Handle the `SIGINT` signal.
	///
	/// By default calls `self.terminate()`.
	/// 
	/// The `SIGINT` signal is sent to a process by its controlling terminal when a user wishes to interrupt the process.
	/// This is typically initiated by pressing `Ctrl+C`.
	#[inline(always)]
	fn int(&mut self)
	{
		self.terminate()
	}

	/// Handle the `SIGQUIT` signal.
	///
	/// By default calls `self.terminate()`.
	///
	/// The `SIGQUIT` signal is sent to a process by its controlling terminal when the user requests that the process quit and perform a core dump.
	#[inline(always)]
	fn quit(&mut self)
	{
		self.terminate()
	}

	/// Handle the `SIGILL` signal.(illegal instruction).
	///
	/// By default aborts.
	///
	/// The `SIGILL` signal is sent to a process when it attempts to execute an illegal, malformed, unknown, or privileged instruction.
	#[inline(always)]
	fn ill(&mut self)
	{
		abort();
	}

	/// Handle the `SIGTRAP` signal.
	///
	/// By default aborts.
	///
	/// The `SIGTRAP` signal is sent to a process when an exception (or trap) occurs: a condition that a debugger has requested to be informed of – for example, when a particular function is executed, or when a particular variable changes value.
	#[inline(always)]
	fn trap(&mut self)
	{
		abort();
	}

	/// Handle the `SIGABRT` signal.(also known as `SIGIOT`).
	///
	/// By default aborts.
	///
	/// The `SIGABRT` signal is sent to a process to tell it to abort, ie to terminate.
	/// The signal is usually initiated by the process itself when it calls `abort()` function of the C Standard Library, but it can be sent to the process from outside like any other signal.
	#[inline(always)]
	fn abrt(&mut self)
	{
		abort();
	}

	/// Handle the `SIGBUS` signal.
	///
	/// By default aborts.
	///
	/// The `SIGBUS` signal is sent to a process when it causes a bus error.
	/// The conditions that lead to the signal being sent are, for example, incorrect memory access alignment or non-existent physical address.
	#[inline(always)]
	fn bus(&mut self)
	{
		abort();
	}

	/// Handle the `SIGFPE` signal.
	///
	/// By default aborts.
	///
	/// The `SIGFPE` signal is sent to a process when it executes an erroneous arithmetic operation, such as division by zero (the name "FPE", standing for floating-point exception, is a misnomer as the signal covers integer-arithmetic errors as well).
	#[inline(always)]
	fn fpe(&mut self)
	{
		abort();
	}

	/// Handle the `SIGUSR1` signal.
	///
	/// By default does nothing.
	#[inline(always)]
	fn usr1(&mut self)
	{
	}

	/// Handle the `SIGSEGV` signal.
	///
	/// By default aborts.
	///
	/// The `SIGSEGV` signal is sent to a process when it makes an invalid virtual memory reference, or segmentation fault, i.e. when it performs a segmentation violation.
	/// This can happen a `mmap`'d file shared with another process is truncated.
	#[inline(always)]
	fn segv(&mut self)
	{
		abort();
	}

	/// Handle the `SIGSUSR2` signal.
	///
	/// By default does nothing.
	#[inline(always)]
	fn usr2(&mut self)
	{
	}

	/// Handle the `SIGPIPE` signal.
	///
	/// By default calls `self.terminate()`.
	///
	/// The `SIGPIPE` signal is sent to a process when it attempts to write to a pipe without a process connected to the other end, or when a socket's remote peer has shutdown.
	#[inline(always)]
	fn pipe(&mut self)
	{
		self.terminate()
	}

	/// Handle the `SIGALRM` signal.
	///
	/// By default panics, as `SIGALRM` is an ancient approach using POSIX timers.
	#[inline(always)]
	fn alrm(&mut self)
	{
		panic!("Signal `SIGALRM` received; no modern code should be using POSIX timers")
	}

	/// Handle the `SIGTERM` signal.
	///
	/// By default calls `self.terminate()`.
	///
	/// The `SIGTERM` signal is sent to a process to request its termination.
	/// Unlike the `SIGKIL`L signal, it can be caught and interpreted or ignored by the process.
	/// This allows the process to perform nice termination releasing resources and saving state if appropriate.
	/// It is typically used by daemon management tools to ask for a graceful process termination.
	#[inline(always)]
	fn term(&mut self)
	{
		self.terminate()
	}

	/// Handle the `SIGSTKFLT` signal.
	///
	/// By default panics, as it should not occur.
	///
	/// The `SIGSTKFLT` signal is sent to a process when the coprocessor experiences a stack fault (ie popping when the stack is empty or pushing when it is full).
	/// It is defined by, but not used on Linux, where a x87 coprocessor stack fault will generate `SIGFPE` instead.
	#[inline(always)]
	fn stkflt(&mut self)
	{
		panic!("Signal `SIGSTKFLT` received; this should not occur on Linux")
	}

	/// Handle the `SIGCHLD` signal.
	///
	/// By default does nothing.
	///
	/// The `SIGCHLD` signal is sent to a process when a child process terminates, is interrupted, or resumes after being interrupted.
	/// One common usage of the signal is to instruct the operating system to clean up the resources used by a child process after its termination without an explicit call to the wait system call.
	#[inline(always)]
	fn chld(&mut self)
	{
	}

	/// Handle the `SIGCONT` signal.
	///
	/// By default does nothing.
	///
	/// The `SIGCONT` signal instructs the operating system to continue (restart) a process previously paused by the `SIGSTOP` or `SIGTSTP` signal.
	/// One important use of this signal is in job control in the Unix shell.
	#[inline(always)]
	fn cont(&mut self)
	{
	}

	/// Handle the `SIGTSTP` signal.
	///
	/// By default does nothing.
	///
	/// The `SIGTSTP` signal is sent to a process by its controlling terminal to request it to stop (terminal stop).
	/// It is commonly initiated by the user pressing `Ctrl+Z`.
	/// Unlike `SIGSTOP`, the process can register a signal handler for or ignore the signal.
	#[inline(always)]
	fn tstp(&mut self)
	{
	}

	/// Handle the `SIGTTIN` signal.
	///
	/// By default does nothing.
	///
	/// The `SIGTTIN` signal is  sent to a process when it attempts to read in from the tty while in the background
	/// Typically, these signals are received only by processes under job control; daemons do not have controlling terminals and, therefore, should never receive these signals.
	#[inline(always)]
	fn ttin(&mut self)
	{
	}

	/// Handle the `SIGTTOU` signal.
	///
	/// By default does nothing.
	///
	/// The `SIGTTOU` signal is  sent to a process when it attempts to write out from the tty while in the background
	/// Typically, these signals are received only by processes under job control; daemons do not have controlling terminals and, therefore, should never receive these signals.
	#[inline(always)]
	fn ttou(&mut self)
	{
	}

	/// Handle the `SIG§` signal.
	///
	/// By default panics, as `SIGURG` is an ancient approach to accessing TCP's deprecated out-of-band data.
	///
	/// The `SIGURG` signal is sent to a process when a socket has urgent or out-of-band data available to read.
	#[inline(always)]
	fn urg(&mut self)
	{
		panic!("Signal `SIGALRM` received; no modern code should be accessing TCP's deprecated out-of-band data using signals")
	}

	/// Handle the `SIGXCPU` signal.
	///
	/// By default calls `self.terminate()`.
	///
	/// The `SIGXCPU` signal is sent to a process when it has used up the CPU for a duration that exceeds a certain predetermined user-settable value.
	/// The arrival of a `SIGXCPU` signal provides the receiving process a chance to quickly save any intermediate results and to exit gracefully, before it is terminated by the operating system using the `SIGKILL` signal.
	#[inline(always)]
	fn xcpu(&mut self)
	{
		self.terminate()
	}

	/// Handle the `SIGXFSZ` signal.
	///
	/// By default calls `self.terminate()`.
	///
	/// The `SIGXFSZ` signal is sent to a process when it grows a file that exceeds than the maximum allowed size.
	#[inline(always)]
	fn xfsz(&mut self)
	{
		self.terminate()
	}

	/// Handle the `SIGVTALRM` signal.
	///
	/// By default calls `self.terminate()`.
	///
	/// The `SIGVTALRM` signal is sent when CPU time used by the process elapses
	#[inline(always)]
	fn vtalrm(&mut self)
	{
		self.terminate()
	}

	/// Handle the `SIGPROF` signal.
	///
	/// By default calls `self.terminate()`.
	///
	/// The `SIGPROF` signal is sent when CPU time used by the process and by the system on behalf of the process elapses.
	/// Unsurprisingly, this is often during profiling.
	#[inline(always)]
	fn prof(&mut self)
	{
		self.terminate()
	}

	/// Handle the `SIGWINCH` signal.
	///
	/// By default ignored.
	///
	/// The `SIGWINCH` signal is sent to a process when its controlling terminal changes its size (a window change)
	#[inline(always)]
	fn winch(&mut self)
	{
	}

	/// Handle the `SIGIO` signal.(also known as `SIGPOLL`).
	///
	/// By default panics, as `SIGIO` is an ancient approach to waiting for I/O to become ready.
	///
	/// The `SIGIO` signal is sent when an event occurred on an explicitly watched file descriptor.
	#[inline(always)]
	fn io(&mut self)
	{
		panic!("Signal `SIGIO` received; no modern code should be using this")
	}

	/// Handle the `SIGPWR` signal.
	///
	/// By default calls `self.terminate()`.
	///
	/// The `SIGPWR` signal is sent to a process when the system experiences a power failure.
	#[inline(always)]
	fn pwr(&mut self)
	{
		self.terminate()
	}

	/// Handle the `SIGSYS` signal (also known as `SIGUNUSED`).
	///
	/// By default aborts.
	///
	/// The `SIGSYS` signal is sent to a process when it passes a bad argument to a system call.
	/// In practice, this kind of signal is rarely encountered since applications rely on the libc library to make the call for them.
	/// `SIGSYS` can be received by applications violating the Linux Seccomp security rules configured to restrict them.
	#[inline(always)]
	fn sys(&mut self)
	{
		abort()
	}
}

/// This object forces all signals to be handled using epoll.
pub struct AllSignalReactor(Option<SignalFileDescriptor>);

impl AllSignalReactor
{
	/// Creates a new instance.
	#[inline(always)]
	pub const fn new() -> Self
	{
		AllSignalReactor(None)
	}

	/// Register with epoll.
	///
	/// Starts blocking signals at this point.
	#[inline(always)]
	pub fn register_with_epoll(&mut self, epoll_file_descriptor: &EPollFileDescriptor) -> Result<(), SignalEPollRegistrationError>
	{
		let mut signal_mask = unsafe { uninitialized() };
		let result = unsafe {  sigfillset(&mut signal_mask) };
		if likely!(result == 0)
		{
			()
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				EINVAL => panic!("Invalid arguments"),
				_ => unreachable!(),
			}
		}
		else
		{
			unreachable!();
		}

		let result = unsafe { pthread_sigmask(SIG_BLOCK, &mut signal_mask, null_mut()) };
		if unlikely!(result != 0)
		{
			match result
			{
				EFAULT => panic!("The `set` or `oldset` argument points outside the process's allocated address space"),
				EINVAL => panic!("Either the value specified in `how` was invalid or the kernel does not support the size passed in `sigsetsize`"),
				_ => unreachable!(),
			}
		}

		// We don't have to have one instance; we can have many, one for each signal we want to handle. This avoid the need for a select in react!
		let signal_file_descriptor = SignalFileDescriptor::new(&signal_mask)?;

		epoll_file_descriptor.add(signal_file_descriptor.as_raw_fd(), epoll_event::EPOLLIN | epoll_event::EPOLLET, signal_token)?;

		self.0 = Some(signal_file_descriptor);

		Ok(())
	}


	// React needs to tell the loop:-
	// stop processing;
	// remove this instance (or can we do this for ourselves?)
	// restart the application loop (re-configured)
	pub fn react(&mut self, epoll_file_descriptor: &EPollFileDescriptor, token: u64, _flags: u32)
	{
		// TODO: Test flags for HUP, Error, etc?

		use self::SignalReadError::*;

		let mut signals = [signalfd_siginfo; 32];
		match signal_file_descriptor.read(&mut signals)
		{
			Err(WouldBlock) => x, // no more signals for now.
			Err(Cancelled) => (), // ?now what?
			Err(Interrupted) => (), // ?should this be possible if the mask is set up correctly?
			Ok(ready_signals) => for ready_signal in ready_signals
			{
				match ready_signal.ssi_signo
				{
					SIGUSR1 => x,
					_ => unreachable!(),
				}
			}
		}
	}
}

pub fn signal_handling_event_loop(terminate: Terminate, time_out_milliseconds: u16) -> Result<(), EPollCreationError_or_SignalEPollRegistrationError>
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
