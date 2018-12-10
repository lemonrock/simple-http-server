// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Allows for customization of responses to signals.
///
/// Signals can be raised by the kernel or by user space; they can even be raised 'internally' by the libc.
///
/// When raised by user space, they are not necessarily populated with the same data.
/// Hence this signal handler differentiates between kernel and the various user space equivalents.
///
/// Signals have an associated `errno` field, but this is not passed to the handlers as Linux only ever populates it for device events in the USB URBS core, and these are raised as `SI_ASYNCIO` to mimic user space.
///
/// The signals `SIGKILL` and `SIGSTOP` can not be handled.
///
/// The signal `SIGEMT` can only occur on Alpha, MIPS and SPARC architectures (although Rust does not support the Alpha architecture).
///
// Sadly, the Linux kernel's implementation of user signal handling is still weak, despite CVE-2011-1182 (a kernel signal spoofing issue), and so one can not trust `si_code` nor the associated data.
pub trait SignalHandler
{
	/// Generic functionality for exiting; exists to allow easy customization of handling of a number of non-fatal signals.
	#[inline(always)]
	fn terminate(&self)
	{
		exit(1);
	}
	
	/// Generic functionality for aborting; exists to allow easy customization of handling of a number of non-fatal signals.
	#[inline(always)]
	fn abort(&self)
	{
		abort();
	}

	/// A POSIX real-time signal.
	///
	/// By default does nothing.
	///
	/// * `signal_number`: The value of the realtime signal; will be between `SIGRTMIN` and `SIGRTMAX` inclusive; `SIGRTMIN` is not zero and may be a value of 32 or greater.
	/// * `signal_data`: Whilst realtime signals are supposed to have associated data, in practice, it simply isn't trustworthy.
	#[inline(always)]
	#[allow(unused_variables)]
	fn handle_sigrt(&self, signal_number: u32, signal_data: GenericSignalData)
	{
	}

	/// An illegal signal number.
	///
	/// By default does nothing.
	///
	/// The Linux kernel's checks for validity of signals are quite scattered in the code base and it is not clear if all signal numbers are validated after being raised by user space in all possible scenarios.
	///
	/// In this event, the `signal_number` could be zero or a value greater than `SIGRTMAX`.
	#[inline(always)]
	#[allow(unused_variables)]
	fn handle_illegal_signal(&self, signal_number: u32, signal_data: GenericSignalData)
	{
	}

	/// A POSIX real-time signal which should not have been received.
	///
	/// These signal numbers are realtime signals reserved for the libc implementation; only code which makes syscalls bypassing libc can generate them.
	///
	/// By default does nothing.
	///
	/// * `signal_number`: The value of the realtime signal; will be between `32` inclusive and `SIGRTMAX` exclusive.
	/// * `signal_data`: Whilst realtime signals are supposed to have associated data, in practice, it simply isn't trustworthy.
	#[inline(always)]
	#[allow(unused_variables)]
	fn handle_illegal_sigrt(&self, signal_number: u32, signal_data: GenericSignalData)
	{
	}

	/// Handle the `SIGABRT` signal (also known as `SIGIOT`).
	///
	/// By default calls libc `raise(SIGKILL);` to replicate the original action.
	///
	/// The `SIGABRT` signal is sent to a process to tell it to abort, ie to terminate.
	/// The signal is usually initiated by the process itself when it calls `abort()` function of the C Standard Library, but it can be sent to the process from outside like any other signal.
	#[inline(always)]
	#[allow(unused_variables)]
	fn handle_sigabrt(&self, signal_data: GenericSignalData)
	{
		unsafe { raise(SIGKILL); };
	}

	/// Handle the `SIGALRM` signal.
	///
	/// By default panics, as `SIGALRM` is an ancient approach using timers.
	#[inline(always)]
	#[allow(unused_variables)]
	fn handle_sigalrm(&self, signal_data: GenericSignalData)
	{
		panic!("Signal `SIGALRM` received; no modern code should be using timers")
	}

	/// Handle the `SIGBUS` signal.
	///
	/// By default aborts.
	///
	/// The `SIGBUS` signal is sent to a process when it causes a bus error.
	/// The conditions that lead to the signal being sent are, for example, incorrect memory access alignment or non-existent physical address.
	#[inline(always)]
	#[allow(unused_variables)]
	fn handle_sigbus(&self, signal_data: SpecificSignalData<BusCode>)
	{
		self.abort()
	}

	/// Handle the `SIGCHLD` signal (also known as `SIGCLD`).
	///
	/// By default does nothing.
	///
	/// The `SIGCHLD` signal is sent to a process when a child process terminates, is interrupted, or resumes after being interrupted.
	/// One common usage of the signal is to instruct the operating system to clean up the resources used by a child process after its termination without an explicit call to the wait system call.
	#[inline(always)]
	#[allow(unused_variables)]
	fn handle_sigchld(&self, signal_data: SpecificSignalData<ChildCode>)
	{
	}

	/// Handle the `SIGCONT` signal.
	///
	/// By default does nothing.
	///
	/// The `SIGCONT` signal instructs the operating system to continue (restart) a process previously paused by the `SIGSTOP` or `SIGTSTP` signal.
	/// One important use of this signal is in job control in the Unix shell.
	#[inline(always)]
	#[allow(unused_variables)]
	fn handle_sigcont(&self, signal_data: GenericSignalData)
	{
	}

	/// Handle the `SIGEMT` signal.
	///
	/// By default aborts.
	///
	/// The` SIGEMT` signal is sent to a process when an emulator trap occurs.
	///
	/// This signal only occurs for the Alpha, MIPS and SPARC architectures (but Alpha isn't supported by Rust).
	#[inline(always)]
	#[allow(unused_variables)]
	fn handle_sigemt(&self, signal_data: SpecificSignalData<EmulatorTrapCode>)
	{
		#[cfg(any(target_arch = "mips", target_arch = "mips64", target_arch = "sparc64"))]
		{
			abort()
		}

		#[cfg(not(any(target_arch = "mips", target_arch = "mips64", target_arch = "sparc64")))]
		{
			panic!("Signal `SIGEMT` received from kernel; this should not occur on Linux except for the Alpha, MIPS and SPARC architectures")
		}
	}

	/// Handle the `SIGFPE` signal.
	///
	/// By default aborts.
	///
	/// The `SIGFPE` signal is sent to a process when it executes an erroneous arithmetic operation, such as division by zero (the name "FPE", standing for floating-point exception, is a misnomer as the signal covers integer-arithmetic errors as well).
	#[inline(always)]
	#[allow(unused_variables)]
	fn handle_sigfpe(&self, signal_data: SpecificSignalData<ArithmeticErrorCode>)
	{
		self.abort()
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
	#[allow(unused_variables)]
	fn handle_sighup(&self, signal_data: GenericSignalData)
	{
		self.terminate()
	}

	/// Handle the `SIGILL` signal.
	///
	/// By default aborts.
	///
	/// The `SIGILL` signal is sent to a process when it attempts to execute an illegal, malformed, unknown, or privileged instruction.
	#[inline(always)]
	#[allow(unused_variables)]
	fn handle_sigill(&self, signal_data: SpecificSignalData<IllegalInstructionCode>)
	{
		self.abort()
	}

	/// Handle the `SIGINT` signal.
	///
	/// By default calls `self.terminate()`.
	///
	/// The `SIGINT` signal is sent to a process by its controlling terminal when a user wishes to interrupt the process.
	/// This is typically initiated by pressing `Ctrl+C`.
	#[inline(always)]
	#[allow(unused_variables)]
	fn handle_sigint(&self, signal_data: GenericSignalData)
	{
		self.terminate()
	}

	/// Handle the `SIGIO` signal.(also known as `SIGPOLL`).
	///
	/// By default panics, as `SIGIO` is an ancient approach to waiting for I/O to become ready.
	///
	/// The `SIGIO` signal is sent when an I/O event occurred on an explicitly watched file descriptor.
	#[inline(always)]
	#[allow(unused_variables)]
	fn handle_sigio(&self, signal_data: SpecificSignalData<PollCode>)
	{
		panic!("Signal `SIGIO` received; no modern code should be using this")
	}

	/// Handle the `SIGPIPE` signal.
	///
	/// By default calls `self.terminate()`.
	///
	/// The `SIGPIPE` signal is sent to a process when it attempts to write to a pipe without a process connected to the other end, or when a socket's remote peer has shutdown.
	#[inline(always)]
	#[allow(unused_variables)]
	fn handle_sigpipe(&self, signal_data: GenericSignalData)
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
	#[allow(unused_variables)]
	fn handle_sigprof(&self, signal_data: GenericSignalData)
	{
		self.terminate()
	}

	/// Handle the `SIGPWR` signal.
	///
	/// By default calls `self.terminate()`.
	///
	/// The `SIGPWR` signal is sent to a process when the system experiences a power failure.
	#[inline(always)]
	#[allow(unused_variables)]
	fn handle_sigpwr(&self, signal_data: GenericSignalData)
	{
		self.terminate()
	}

	/// Handle the `SIGQUIT` signal.
	///
	/// By default calls `self.terminate()`.
	///
	/// The `SIGQUIT` signal is sent to a process by its controlling terminal when the user requests that the process quit and perform a core dump.
	/// It is commonly initiated by the user pressing `Ctrl+\`.
	#[inline(always)]
	#[allow(unused_variables)]
	fn handle_sigquit(&self, signal_data: GenericSignalData)
	{
		self.terminate()
	}

	/// Handle the `SIGSEGV` signal.
	///
	/// By default aborts.
	///
	/// The `SIGSEGV` signal is sent to a process when it makes an invalid virtual memory reference, or segmentation fault, i.e. when it performs a segmentation violation.
	/// This can happen a `mmap`'d file shared with another process is truncated.
	///
	/// The following parameters are valid if the signal was sent by the kernel:-
	///
	/// * `code`: A code describing a sub-case of this signal.
	/// * `address`: The address of the fault.
	/// * `trap_number`: The trap number of the fault (only valid on the Alpha, MIPS and SPARC architectures; Rust does not support the Alpha architecture).
	#[inline(always)]
	#[allow(unused_variables)]
	fn handle_sigsegv(&self, signal_data: SpecificSignalData<SegmentationFaultCode>)
	{
		self.abort()
	}

	/// Handle the `SIGSTKFLT` signal.
	///
	/// By default panics, as it should not occur.
	///
	/// The `SIGSTKFLT` signal is sent to a process when the coprocessor experiences a stack fault (ie popping when the stack is empty or pushing when it is full).
	/// It is defined by, but not used on Linux, where a x87 coprocessor stack fault will generate `SIGFPE` instead.
	#[inline(always)]
	#[allow(unused_variables)]
	fn handle_sigstkflt(&self, signal_data: GenericSignalData)
	{
		panic!("Signal `SIGSTKFLT` received from kernel; this should not occur on Linux")
	}

	/// Handle the `SIGSYS` signal (equivalent to `SIGUNUSED` on Linux, which is now considered a legacy definition).
	///
	/// By default aborts.
	///
	/// `SIGSYS` can be received by applications violating the Linux security filtering (`seccomp`) rules configured to restrict them.
	/// On Linux, `SIGSYS` is **not** received for a system call which uses an invalid number.
	///
	/// * `code`: A code describing a sub-case of this signal.
	/// * `address`: The address of the fault.
	/// * `architecture`: The system call architecture (see Linux's `AUDIT_ARCH_*`).
	#[inline(always)]
	#[allow(unused_variables)]
	fn handle_sigsys(&self, signal_data: SpecificSignalData<SystemCallCode>)
	{
		abort()
	}

	/// Handle the `SIGTERM` signal.
	///
	/// By default calls `self.terminate()`.
	///
	/// The `SIGTERM` signal is sent to a process to request its termination.
	/// Unlike the `SIGKILL` signal, it can be caught and interpreted or ignored by the process.
	/// This allows the process to perform nice termination releasing resources and saving state if appropriate.
	/// It is typically used by daemon management tools to ask for a graceful process termination.
	#[inline(always)]
	#[allow(unused_variables)]
	fn handle_sigterm(&self, signal_data: GenericSignalData)
	{
		self.terminate()
	}

	/// Handle the `SIGTRAP` signal.
	///
	/// By default aborts.
	///
	/// The `SIGTRAP` signal is sent to a process when an exception (or trap) occurs: a condition that a debugger has requested to be informed of – for example, when a particular function is executed, or when a particular variable changes value.
	///
	/// * `code`: A code describing a sub-case of this signal.
	/// * `address`: The address of the fault.
	/// * `trap_number`: The trap number of the fault (only valid on the Alpha, MIPS and SPARC architectures; Rust does not support the Alpha architecture).
	#[inline(always)]
	#[allow(unused_variables)]
	fn handle_sigtrap(&self, signal_data: SpecificSignalData<TrapCode>)
	{
		self.abort()
	}

	/// Handle the `SIGTSTP` signal.
	///
	/// By default does nothing.
	///
	/// The `SIGTSTP` signal is sent to a process by its controlling terminal to request it to stop (terminal stop).
	/// It is commonly initiated by the user pressing `Ctrl+Z`.
	/// Unlike `SIGSTOP`, the process can register a signal handler for or ignore the signal.
	#[inline(always)]
	#[allow(unused_variables)]
	fn handle_sigtstp(&self, signal_data: GenericSignalData)
	{
	}

	/// Handle the `SIGTTIN` signal.
	///
	/// By default does nothing.
	///
	/// The `SIGTTIN` signal is  sent to a process when it attempts to read in from the tty while in the background
	/// Typically, these signals are received only by processes under job control; daemons do not have controlling terminals and, therefore, should never receive these signals.
	#[inline(always)]
	#[allow(unused_variables)]
	fn handle_sigttin(&self, signal_data: GenericSignalData)
	{
	}

	/// Handle the `SIGTTOU` signal.
	///
	/// By default does nothing.
	///
	/// The `SIGTTOU` signal is  sent to a process when it attempts to write out from the tty while in the background
	/// Typically, these signals are received only by processes under job control; daemons do not have controlling terminals and, therefore, should never receive these signals.
	#[inline(always)]
	#[allow(unused_variables)]
	fn handle_sigttou(&self, signal_data: GenericSignalData)
	{
	}

	/// Handle the `SIGURG` signal.
	///
	/// By default panics, as `SIGURG` is an ancient approach to accessing TCP's deprecated out-of-band data.
	///
	/// The `SIGURG` signal is sent to a process when a socket has urgent or out-of-band data available to read.
	#[inline(always)]
	#[allow(unused_variables)]
	fn handle_sigurg(&self, signal_data: GenericSignalData)
	{
		panic!("Signal `SIGALRM` received; no modern code should be accessing TCP's deprecated out-of-band data using signals")
	}

	/// Handle the `SIGUSR1` signal.
	///
	/// By default panics, as this signal should not be sent by the kernel.
	#[inline(always)]
	#[allow(unused_variables)]
	fn handle_sigusr1(&self, signal_data: GenericSignalData)
	{
		panic!("Signal `SIGUSR1` received from kernel; this should not occur on Linux")
	}

	/// Handle the `SIGSUSR2` signal.
	///
	/// By default panics, as this signal should not be sent by the kernel.
	#[inline(always)]
	#[allow(unused_variables)]
	fn handle_sigusr2(&self, signal_data: GenericSignalData)
	{
		panic!("Signal `SIGUSR2` received from kernel; this should not occur on Linux")
	}

	/// Handle the `SIGVTALRM` signal.
	///
	/// By default calls `self.terminate()`.
	///
	/// The `SIGVTALRM` signal is sent when CPU time used by the process elapses.
	#[inline(always)]
	#[allow(unused_variables)]
	fn handle_sigvtalrm(&self, signal_data: GenericSignalData)
	{
		self.terminate()
	}

	/// Handle the `SIGWINCH` signal.
	///
	/// By default ignored.
	///
	/// The `SIGWINCH` signal is sent to a process when its controlling terminal changes its size (a `WIN`dow `CH`ange).
	#[inline(always)]
	#[allow(unused_variables)]
	fn handle_sigwinch(&self, signal_data: GenericSignalData)
	{
	}

	/// Handle the `SIGXCPU` signal.
	///
	/// By default calls `self.terminate()`.
	///
	/// The `SIGXCPU` signal is sent to a process when it has used up the CPU for a duration that exceeds a certain predetermined user-settable value.
	/// The arrival of a `SIGXCPU` signal provides the receiving process a chance to quickly save any intermediate results and to exit gracefully, before it is terminated by the operating system using the `SIGKILL` signal.
	#[inline(always)]
	#[allow(unused_variables)]
	fn handle_sigxcpu(&self, signal_data: GenericSignalData)
	{
		self.terminate()
	}

	/// Handle the `SIGXFSZ` signal.
	///
	/// By default calls `self.terminate()`.
	///
	/// The `SIGXFSZ` signal is sent to a process when it grows a file that exceeds than the maximum allowed size.
	#[inline(always)]
	#[allow(unused_variables)]
	fn handle_sigxfsz(&self, signal_data: GenericSignalData)
	{
		self.terminate()
	}
}
