// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Represents a character device for reading and writing to.
///
/// A character device can be a (USB) serial port.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TerminalFileDescriptor(CharacterDeviceFileDescriptor);

impl AsRawFd for TerminalFileDescriptor
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		self.0.as_raw_fd()
	}
}

impl IntoRawFd for TerminalFileDescriptor
{
	#[inline(always)]
	fn into_raw_fd(self) -> RawFd
	{
		self.0.into_raw_fd()
	}
}

impl Read for TerminalFileDescriptor
{
	/// This particular implementation can only return an `io::ErrorKind` of:-
	///
	/// * `UnexpectedEof`
	/// * `WouldBlock`
	/// * `Interrupted`
	/// * `Other` (which is for when the kernel reports `ENOMEM`, ie it is out of memory).
	#[inline(always)]
	fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>
	{
		self.0.read(buf)
	}

	#[inline(always)]
	unsafe fn initializer(&self) -> Initializer
	{
		CharacterDeviceFileDescriptor::initializer()
	}
}

impl Write for TerminalFileDescriptor
{
	/// This particular implementation can only return an `io::ErrorKind` of:-
	///
	/// * `WriteZero` (implying end-of-file).
	/// * `WouldBlock`
	/// * `Interrupted`
	/// * `BrokenPipe`
	#[inline(always)]
	fn write(&mut self, buf: &[u8]) -> io::Result<usize>
	{
		self.0.write(buf)
	}

	#[inline(always)]
	fn flush(&mut self) -> io::Result<()>
	{
		let result = unsafe { tcdrain(self.as_raw_fd()) };

		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			Err(io::Error::last_os_error())
		}
		else
		{
			unreachable!()
		}
	}
}

impl TerminalFileDescriptor
{
	/*
	from http://man7.org/linux/man-pages/man2/ioctl_tty.2.html

	   TODO: Exclusive mode
       TIOCEXCL  void
              Put the terminal into exclusive mode.  No further open(2)
              operations on the terminal are permitted.  (They fail with
              EBUSY, except for a process with the CAP_SYS_ADMIN capabil‐
              ity.)

       TIOCGEXCL int *argp
              (since Linux 3.8) If the terminal is currently in exclusive
              mode, place a nonzero value in the location pointed to by
              argp; otherwise, place zero in *argp.

       TIOCNXCL  void
              Disable exclusive mode.

   Line discipline
       TIOCGETD  int *argp
              Get the line discipline of the terminal.

       TIOCSETD  const int *argp
              Set the line discipline of the terminal.

	TODO:
       TCSBRKP   int arg
              So-called "POSIX version" of TCSBRK (tcsendbreak() with working interval).  It treats nonzero arg as
              a timeinterval measured in deciseconds, and does nothing when
              the driver does not support breaks.

       TIOCSBRK  void
              Turn break on, that is, start sending zero bits.

       TIOCCBRK  void
              Turn break off, that is, stop sending zero bits.


These are ERRORS from ioctl-backed functions.
TODO: ERRORS         top

       EINVAL Invalid command parameter.

       ENOIOCTLCMD
              Unknown command.

       ENOTTY Inappropriate fd.

       EPERM  Insufficient permission.  - preferable.

TODO: Make some propert bitflags for various terminal control flags; implement cfmakeraw() as a default for terminal settings - see http://man7.org/linux/man-pages/man3/termios.3.html

See also https://github.com/dcuddeback/serial-rs/blob/master/serial-unix/src/tty.rs

	*/


	/// Opens a terminal character device named in the file system suitable for sending data to.
	///
	/// Sadly, there is no way to atomically detect if the provided path is **not** a terminal character device.
	#[inline(always)]
	pub fn open_terminal_character_device(terminal_character_device_file_path: impl AsRef<Path>, baud_rate: BaudRate, input_mode_settings: &InputModeFlagSettings, output_mode_settings: &OutputModeFlagSettings) -> Result<Self, SpecialFileOpenError>
	{
		use self::SpecialFileOpenError::*;

		let this = Self(CharacterDeviceFileDescriptor::open_character_device_internal(terminal_character_device_file_path, O_NOCTTY)?);

		let mut terminal_options: termios = unsafe { uninitialized() };
		Self::handle_terminal_error(unsafe { tcgetattr(this.as_raw_fd(), &mut terminal_options, NotATerminal) })?;

		baud_rate.set_terminal_input_and_output_speed(&mut terminal_options);


		// Input mode - unset canonical mode (ie raw mode), echo, echo erase and signal raising.
		terminal_options.c_lflag &= !(ICANON | ECHO | ECHOE | ISIG);
		input_mode_settings.change_input_mode_flags(&mut terminal_options);
		output_mode_settings.change_output_mode_flags(&mut terminal_options);

		Self::handle_terminal_error(unsafe { tcsetattr(file_descriptor, TCSANOW, &terminal_options, CouldNotSetTerminalAttributes) })?;

		this
	}

	/// Discard input.
	#[inline(always)]
	pub fn discard_input_received_but_not_read(&self) -> io::Result<()>
	{
		Self::handle_generic_io_error(unsafe { tcflush(self.as_raw_fd(), TCIFLUSH) })
	}

	/// Discard output.
	#[inline(always)]
	pub fn discard_output_written_but_not_transmitted(&self) -> io::Result<()>
	{
		Self::handle_generic_io_error(unsafe { tcflush(self.as_raw_fd(), TCOFLUSH) })
	}

	/// Discard input and output.
	#[inline(always)]
	pub fn discard_input_received_but_not_read_and_output_written_but_not_transmitted(&self) -> io::Result<()>
	{
		Self::handle_generic_io_error(unsafe { tcflush(self.as_raw_fd(), TCIOFLUSH) })
	}

	/// Sends a break (a stream of zero bits) for between 0.25 and 0.5 seconds.
	#[inline(always)]
	pub fn send_break(&self) -> io::Result<()>
	{
		// Note that Linux does not support an implementation-defined (non-zero) value; see the inner logic of musl at <https://github.com/bminor/musl/blob/05ac345f895098657cf44d419b5d572161ebaf43/src/termios/tcsendbreak.c>.
		Self::handle_generic_io_error(unsafe { tcsendbreak(self.as_raw_fd(), 0) })
	}

	/// Suspends output.
	///
	/// Does not use a guard and move self as in practice a file descriptor will be held in an arena, Vec, etc and so can not be moved.
	#[inline(always)]
	pub fn suspend_output(&self) -> io::Result<>
	{
		Self::handle_generic_io_error(unsafe { tcflow(self.as_raw_fd(), TCOOFF) })
	}

	/// Resumes output.
	///
	/// Should not be used unless `suspend_output()` has been called exactly once before.
	#[inline(always)]
	pub fn resume_output(&self) -> io::Result<()>
	{
		Self::handle_generic_io_error(unsafe { tcflow(self.as_raw_fd(), TCOON) })
	}

	/// Transmits a STOP character, which stops the terminal device from transmitting data to the system, ie suspends input.
	#[inline(always)]
	pub fn stop(&self) -> io::Result<()>
	{
		Self::handle_generic_io_error(unsafe { tcflow(self.as_raw_fd(), TCIOFF) })
	}

	/// Transmits a START character, which starts the terminal device transmitting data to the system, ie resumes input.
	///
	/// Should not be used unless `start()` has been called exactly once before.
	#[inline(always)]
	pub fn start(&self) -> io::Result<()>
	{
		Self::handle_generic_io_error(unsafe { tcflow(self.as_raw_fd(), TCIOFF) })
	}

	#[inline(always)]
	fn handle_terminal_error(result: c_int, constructor: FnOnce(Errno) -> SpecialFileOpenError) -> Result<(), SpecialFileOpenError>
	{
		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			Err(constructor(errno()))
		}
		else
		{
			unreachable!()
		}
	}

	#[inline(always)]
	fn handle_generic_io_error(result: c_int) -> io::Result<()>
	{
		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			Err(io::Error::last_os_error())
		}
		else
		{
			unreachable!()
		}
	}
}
