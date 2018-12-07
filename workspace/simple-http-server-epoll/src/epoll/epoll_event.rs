// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Represents an event that occurs after waiting on an epoll file descriptor.
#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(target_arch = "x86_64", repr(C, packed))]
#[cfg_attr(not(target_arch = "x86_64"), repr(C))]
pub struct epoll_event
{
	/// The events member is a bit mask composed by ORing together zero or more of the following available event types (`EPOLL*` constants).
	pub(crate) events: uint32_t,

	/// An union containing the data associated when epoll_ctl was called.
	pub(crate) data: epoll_data_t,
}

impl epoll_event
{
	/// The associated file is available for read operations.
	///
	/// Valid on Linux.
	/// Valid on Solaris.
	pub const EPOLLIN: uint32_t = 0x001;

	/// There is an exceptional condition on the file descriptor.
	///
	/// Identical behaviour to the use of `POLLPRI` with `poll()`.
	///
	/// For TCP file descriptors, this usually means that out-of-band data (which is a deprecated concept) is available.
	///
	/// Valid on Linux.
	/// Valid on Solaris.
	pub const EPOLLPRI: uint32_t = 0x002;

	/// The associated file is available for write operations.
	///
	/// Valid on Linux.
	/// Valid on Solaris.
	pub const EPOLLOUT: uint32_t = 0x004;

	/// Error condition happened on the associated file descriptor.
	///
	/// `epoll_wait()` will always report this event; it is not necessary to set it in `events`.
	///
	/// This event is also reported for the write end of a pipe when the read end has been closed.
	///
	/// Valid on Linux.
	/// Valid on Solaris.
	pub const EPOLLERR: uint32_t = 0x008;

	/// Hang up happened on the associated file descriptor.
	///
	/// `epoll_wait()` will always report this event; it is not necessary to set it in `events`.
	///
	/// Note that when reading from a channel such as a pipe or a stream socket, this event merely indicates that the peer closed its end of the channel.
	///
	/// Subsequent reads from the channel will return 0 (end of file) only after all outstanding data in the channel has been consumed.
	///
	/// ***In effect, receiving this event is simply a courtesy that can be acted on to close a connection early.***
	///
	/// Valid on Linux.
	/// Valid on Solaris.
	pub const EPOLLHUP: uint32_t = 0x010;

	/// Valid since Linux 4.16
	pub const EPOLLNVAL: uint32_t = 0x020;

	/// Valid on Linux.
	/// Valid on Solaris.
	pub const EPOLLRDNORM: uint32_t = 0x040;

	/// Valid on Linux.
	/// Valid on Solaris.
	pub const EPOLLRDBAND: uint32_t = 0x080;

	/// Valid on Linux.
	/// Valid on Solaris.
	pub const EPOLLWRNORM: uint32_t = 0x100;

	/// Valid on Linux.
	/// Valid on Solaris.
	pub const EPOLLWRBAND: uint32_t = 0x200;

	/// Valid on Linux.
	/// Valid on Solaris.
	pub const EPOLLMSG: uint32_t = 0x400;

	/// Stream socket peer closed connection, or shut down writing half of connection.
	///
	/// This flag is especially useful for writing simple code to detect peer shutdown when using Edge Triggered monitoring.
	///
	/// Valid since Linux 2.6.17.
	/// Valid on Solaris.
	pub const EPOLLRDHUP: uint32_t = 0x2000;

	/// Flag for exclusive wakeup mode when an event source fd is attached to multiple epoll fds but they should not all receive the events.
	///
 	/// Sets an exclusive wakeup mode for the epoll file descriptor that is being attached to the target file descriptor, `fd`.
	/// When a wakeup event occurs and multiple epoll file descriptors are attached to the same target file using `EPOLLEXCLUSIVE`, one or more of the epoll file descriptors will receive an event with `epoll_wait()`.
	/// The default in this scenario (when `EPOLLEXCLUSIVE` is not set) is for all epoll file descriptors to receive an event.
	/// `EPOLLEXCLUSIVE` is thus useful for avoiding thundering herd problems in certain scenarios.
	///
	/// If the same file descriptor is in multiple epoll instances, some with the `EPOLLEXCLUSIVE` flag, and others without, then events will be provided to all epoll instances that did not specify `EPOLLEXCLUSIVE`, and at least one of the epoll instances that did specify `EPOLLEXCLUSIVE`.
	///
	/// The following values may be specified in conjunction with `EPOLLEXCLUSIVE`: `EPOLLIN`, `EPOLLOUT`, `EPOLLWAKEUP`, and `EPOLLET`.
	/// `EPOLLHUP` and `EPOLLERR` can also be specified, but this is not required; these events are always reported if they occur, regardless of whether they are specified in events.
	/// Attempts to specify other values in `events` yield an error.
	/// `EPOLLEXCLUSIVE` may be used only in an `EPOLL_CTL_ADD` operation; attempts to employ it with `EPOLL_CTL_MOD` yield an error.
	/// If `EPOLLEXCLUSIVE` has been set using `epoll_ctl()`, then a subsequent `EPOLL_CTL_MOD` on the same `epfd`, `fd` pair yields an error.
	/// A call to `epoll_ctl()` that specifies `EPOLLEXCLUSIVE` in `events` and specifies the target file descriptor `fd` as an epoll instance will likewise fail.
	/// The error in all of these cases is `EINVAL`.
	///
	/// Valid since Linux 4.5.
	/// Valid on Solaris.
	pub const EPOLLEXCLUSIVE: uint32_t = 1 << 28;

	/// Flag to prevent suspend while epoll events are ready.
	///
	/// If `EPOLLONESHOT` and `EPOLLET` are clear and the process has the `CAP_BLOCK_SUSPEND` capability, ensure that the system does not enter "suspend" or "hibernate" while this event is pending or being processed.
	///
	/// The event is considered as being "processed" from the time when it is returned by a call to `epoll_wait()` until the next call to `epoll_wait()` on the same epoll file descriptor, the closure of that file descriptor, the removal of the event file descriptor with `EPOLL_CTL_DEL`, or the clearing of `EPOLLWAKEUP` for the event file descriptor with `EPOLL_CTL_MOD`.
	///
	/// Might be buggy.
	///
	/// Valid since Linux 3.5.
	pub const EPOLLWAKEUP: uint32_t = 1 << 29;

	/// Sets the one-shot behavior for the associated file descriptor.
	///
	/// This means that after an event is pulled out with `epoll_wait()` the associated file descriptor is internally disabled and no other events will be reported by the epoll interface.
	///
	/// The user must call `epoll_ctl()` with `EPOLL_CTL_MOD` to rearm the file descriptor with a new event mask.
	///
	/// Valid since Linux 2.6.2.
	/// Valid on Solaris.
	///
	/// ***In effect, use of this is expensive as it substantially increases the number of syscalls that need to be made.***
	pub const EPOLLONESHOT: uint32_t = 1 << 30;

	/// Sets the Edge Triggered behavior for the associated file descriptor.
	/// 
	/// The default behavior for epoll is Level Triggered.
	///
	/// Valid on Linux.
	/// Valid on Solaris.
	pub const EPOLLET: uint32_t = 1 << 31;

	/// Returns readiness flags.
	#[inline(always)]
	pub fn flags(&self) -> u32
	{
		self.events
	}

	/// Token.
	#[inline(always)]
	pub fn token(&self) -> u64
	{
		unsafe { self.data.u64 }
	}
}
