// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// A struct to allow clean termination.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Terminate
{
	should_finish: Arc<AtomicBool>,
}

impl Terminate
{
	#[inline(always)]
	pub fn begin_termination(&self)
	{
		self.should_finish.store(true, Relaxed)
	}

	#[inline(always)]
	pub(crate) fn begin_termination_due_to_configuration_thread_failure(&self)
	{
		self.begin_termination()
	}

	#[inline(always)]
	pub(crate) fn begin_termination_due_to_panic(&self, panic_info: &PanicInfo)
	{
		eprintln!("Thread {:?} panicked: `{}`", current().name().unwrap_or("(unnamed)"), panic_info);

		self.begin_termination()
	}

	#[inline(always)]
	pub(crate) fn should_finish(&self) -> bool
	{
		self.should_finish.load(Relaxed)
	}

	#[inline(always)]
	pub(crate) fn should_continue(&self) -> bool
	{
		!self.should_finish()
	}
}
