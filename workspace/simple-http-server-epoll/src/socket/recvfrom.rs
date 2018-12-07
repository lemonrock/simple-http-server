// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// The flags argument to a recv() call is a bitwise or of the constants starting `MSG_*`.
extern "C"
{
	/// Receives data from a socket.
	///
	/// Pass `NULL` for `addr` and `addrlen` for connected sockets (eg TCP connections).
	///
	/// On success, returns the number of bytes sent.
	/// On error, `-1` is returned, and `errno` is set appropriately.
	///
	/// The known errors that can be set in `errno` are:-
	///
	/// * `EAGAIN`: The socket is marked nonblocking and the receive operation would block, or a receive timeout had been set and the timeout expired before data was received.
	/// * `EBADF`: The argument `sockfd` is an invalid descriptor.
	/// * `ECONNREFUSED`: A remote host refused to allow the network connection (typically because it is not running the requested service).
	/// * `EFAULT`: The receive buffer pointer(s) point outside the process's address space.
	/// * `EINTR`: The receive was interrupted by delivery of a signal before any data were available.
	/// * `EINVAL`: Invalid argument passed.
	/// * `ENOMEM`: Could not allocate memory.
	/// * `ENOTCONN`: The socket is associated with a connection-oriented protocol and has not been connected.
	/// * `ENOTSOCK`: The argument `sockfd` does not refer to a socket.
	///
	/// Whilst not documented, it seems possible that `EOPNOTSUPP` could occur (eg because some flags in the `flags` argument are inappropriate for the socket type).
	///
	/// Additionally, [this stack overflow question](https://stackoverflow.com/questions/10387082/unix-ipc-socket-closing-one-end-without-reading-from-it) seems to imply that `ECONNRESET` can occur for Unix domain sockets.
	pub(crate) fn recvfrom(sockfd: RawFd, buf: *const c_void, len: size_t, flags: c_int, addr: *const sockaddr_storage, addrlen: *mut socklen_t) -> ssize_t;
}

/// Error and Send and Receive flag.
///
///
/// **For Error**
///
/// Is returned to indicate that expedited or out-of-band data were received.
///
///
/// **For Send and Receive**
///
/// This flag requests receipt of out-of-band data that would not be received in the normal data stream.
///
/// Some protocols place expedited data at the head of the normal data queue, and thus this flag cannot be used with such protocols.
///
/// For TCP, the use of out-of-band data is deprecated.
pub const MSG_OOB: c_int = 0x0001;

/// Receive flag.
///
/// This flag causes the receive operation to return data from the beginning of the receive queue without removing that data from the queue.
///
/// A subsequent receive call will return the same data.
pub const MSG_PEEK: c_int = 0x0002;

/// Send flag.
///
/// Don't use a gateway to send out the packet, send to hosts only on directly connected networks.
///
/// This is usually used only by diagnostic or routing programs.
///
/// This is defined only for protocol families that route; packet sockets don't.
pub const MSG_DONTROUTE: c_int = 0x0004;

/// Send flag.
///
/// Synonym of `MSG_DONTROUTE` used in DECnet.
pub const MSG_TRYHARD: c_int = MSG_DONTROUTE;

/// Error flag.
///
/// Indicates that some control data were discarded due to lack of space in the buffer for ancillary data.
pub const MSG_CTRUNC: c_int = 0x0008;

/// Send flag.
///
/// Also known as `MSG_PROBE`.
///
/// Do not send.
///
/// Only probe path, for example, for MTU.
pub const MSG_PROXY: c_int = 0x0010;

/// Error and Receive flag.
///
///
/// **For Error**
///
/// Indicates that the trailing portion of a datagram was discarded because the datagram was larger than the buffer supplied.
///
///
/// **For Receive**
///
/// Returns the real length of the packet of datagram, even when it was longer than the passed buffer.
///
/// Supported for:-
///
/// * Raw (`AF_PACKET`) sockets;
/// * UDP sockets since Linux 2.6.8;
/// * Netlink sockets since Linux 2.6.22;
/// * Unix domain sockets with datagrams since Linux 3.4;
/// * For TCP, this flag causes the received bytes of data to be discarded, rather than passed back in a caller-supplied buffer. Also works in conjunction with `MSG_OOB` and `MSG_PEEK`.
///
/// Unsupported for:-
///
/// * Unix domain sockets with streams
pub const MSG_TRUNC: c_int = 0x0020;

/// Send and Receive flag.
///
/// Enables nonblocking operation; if the operation would block, the call fails with the error `EAGAIN`.
///
/// Not normally used.
pub const MSG_DONTWAIT: c_int = 0x0040;

/// Error and Send flag.
///
/// **For Error**
///
/// The `msg_flags` field in the `msghdr` is set on return of `recvmsg()`.
/// It indicates end-of-record; the data returned completed a record (generally used with sockets of type `SOCK_SEQPACKET` (eg SCTP)).
///
///
/// ** For Send **
///
/// Terminates a record (when this notion is supported, as for sockets of type `SOCK_SEQPACKET` (eg SCTP)).
pub const MSG_EOR: c_int = 0x0080;

/// Receive flag.
///
/// This flag requests that the operation block until the full request is satisfied.
///
/// However, the call may still return less data than requested if a signal is caught, an error or disconnect occurs, or the next data to be received is of a different type than that returned.
pub const MSG_WAITALL: c_int = 0x0100;

/// Send flag.
///
///
pub const MSG_FIN: c_int = 0x0200;

/// Send flag.
///
///
pub const MSG_SYN: c_int = 0x0400;

/// Send flag.
///
/// ARP functionality ("Confirm path validity").
///
/// Tell the link layer that forward progress happened: you got a successful reply from the other side.
/// If the link layer doesn't get this it will regularly reprobe the neighbor (eg via an unicast ARP).
/// Valid only on `SOCK_DGRA`M and `SOCK_RAW` and currently implemented only for IPv4 and IPv6.
pub const MSG_CONFIRM: c_int = 0x0800;

/// Send flag.
///
///
pub const MSG_RST: c_int = 0x1000;

/// Error and Receive flag.
///
///
/// **For Error**
///
/// Indicates that no data was received but an extended error from the socket error queue.
///
///
/// ** For Receive**
///
/// This flag specifies that queued errors should be received from the socket error queue.
///
/// The error is passed in an ancillary message with a type dependent on the protocol (for IPv4 `IP_RECVERR`).
/// The user should supply a buffer of sufficient size. See cmsg(3) and ip(7) for more information.
/// The payload of the original packet that caused the error is passed as normal data via `msg_iovec`.
/// The original destination address of the datagram that caused the error is supplied via `msg_name`.
/// For local errors, no address is passed (this can be checked with the `cmsg_len` member of the `cmsghdr` struct).
/// For error receives, the `MSG_ERRQUEUE` is set in the `msghdr`.
/// After an error has been passed, the pending socket error is regenerated based on the next queued error and will be passed on the next socket operation.
///
/// The error is supplied in a `sock_extended_err` structure:-
///
/// ```
/// #define SO_EE_ORIGIN_NONE 0
/// #define SO_EE_ORIGIN_LOCAL 1
/// #define SO_EE_ORIGIN_ICMP 2
/// #define SO_EE_ORIGIN_ICMP6 3
///
/// struct sock_extended_err
/// {
/// 	uint32_t ee_errno; /* error number */
/// 	uint8_t ee_origin; /* where the error originated */
/// 	uint8_t ee_type; /* type */
/// 	uint8_t ee_code; /* code */
/// 	uint8_t ee_pad; /* padding */
/// 	uint32_t ee_info; /* additional information */
/// 	uint32_t ee_data; /* other data */
/// 	/* More data may follow */
/// };
///
/// struct sockaddr *SO_EE_OFFENDER(struct sock_extended_err *);
/// ```
///
/// `ee_errno` contains the `errno` number of the queued error.
/// `ee_origin` is the origin code of where the error originated.
/// The other fields are protocol-specific.
/// The macro `SOCK_EE_OFFENDER` returns a pointer to the address of the network object where the error originated from given a pointer to the ancillary message.
/// If this address is not known, the `sa_family` member of the `sockaddr` contains `AF_UNSPEC` and the other fields of the `sockaddr` are undefined.
/// The payload of the packet that caused the error is passed as normal data.
///
/// For local errors, no address is passed (this can be checked with the `cmsg_len` member of the `cmsghdr`).
/// For error receives, the `MSG_ERRQUEUE` is set in the `msghdr`.
/// After an error has been passed, the pending socket error is regenerated based on the next queued error and will be passed on the next socket operation.
pub const MSG_ERRQUEUE: c_int = 0x2000;

/// Send flag.
///
/// Don't generate a `SIGPIPE` signal if the peer on a stream-oriented socket has closed the connection.
///
/// The `EPIPE` error is still returned.
///
/// This provides similar behavior to using `sigaction()` to ignore `SIGPIPE`, but, whereas `MSG_NOSIGNAL` is a per-call feature, ignoring `SIGPIPE` sets a process attribute that affects all threads in the process.
pub const MSG_NOSIGNAL: c_int = 0x4000;

/// Send flag.
///
/// The caller has more data to send.
///
/// This flag is used with TCP sockets to obtain the same effect as the `TCP_CORK` socket option, with the difference that this flag can be set on a per-call basis.
///
/// This flag is also supported for UDP sockets and informs the kernel to package all of the data sent in calls with this flag set into a single datagram which is transmitted only when a call is performed that does not specify this flag.
/// (See also the `UDP_CORK` socket option).
pub const MSG_MORE: c_int = 0x8000;

/// Send flag.
///
/// Used only by `recvmmsg()`
///
/// Block until 1+ packets available.
pub const MSG_WAITFORONE: c_int = 0x10000;

/// Send flag.
///
/// Used only by `sendmmsg()`
///
/// More messages coming.
pub const MSG_BATCH: c_int = 0x40000;

/// Send flag.
///
/// The `MSG_ZEROCOPY` flag enables copy avoidance for socket send calls.
///
/// The feature is currently implemented for TCP sockets.
///
/// For more information see <https://www.kernel.org/doc/html/latest/networking/msg_zerocopy.html>.
pub const MSG_ZEROCOPY: c_int = 0x4000000;

/// Send flag.
///
/// Send data in TCP SYN packet.
pub const MSG_FASTOPEN: c_int = 0x20000000;

/// Receive flag.
///
/// Set the close-on-exec flag for the file descriptor received via a UNIX domain file descriptor using the `SCM_RIGHTS` operation (described in `man 7 unix`)
///
/// Since Linux 2.6.23.
pub const MSG_CMSG_CLOEXEC: c_int = 0x40000000;
