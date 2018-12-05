// This file is part of cpu-affinity. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cpu-affinity/master/COPYRIGHT. No part of cpu-affinity, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of cpu-affinity. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cpu-affinity/master/COPYRIGHT.


/// Logical core utilization.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogicalCoreUtilization
{
	/// Tries to assign cores to threads.
	///
	/// Does not work if the process has logical cores it can not access.
	Simple
	{
		/// Number of logical CPUs to utilize (ie count simultaneous multi-threads (SMT), also known as hyper-threads).
		///
		/// If not specified (None) will default to machine maximum.
		///
		/// If specified as 0 will default to 1.
		number_of_logical_cores_to_utilize: Option<usize>,
	},

	/// Allows detailed specification of thread bindings.
	Complex
	{
		/// Typically `0`.
		main_loop: LogicalCores,

		/// If no worker loops are specified, then the settings for the `main_loop` are used for one worker.
		worker_loops: Vec<LogicalCores>,
	}
}

impl LogicalCoreUtilization
{
	/// Get detail.
	pub fn detail<'a>(&'a self) -> LogicalCoreUtilizationDetail<'a>
	{
		use self::LogicalCoreUtilization::*;

		match self
		{
			Simple { number_of_logical_cores_to_utilize } => Self::simple(number_of_logical_cores_to_utilize),

			Complex { main_loop, ref worker_loops } => LogicalCoreUtilizationDetail
			{
				main_loop,
				worker_loops: Cow::Borrowed(&worker_loops[..]),
			},
		}
	}

	#[inline(always)]
	fn simple<'a>(number_of_logical_cores_to_utilize: Option<usize>) -> LogicalCoreUtilizationDetail<'a>
	{
		#[inline(always)]
		fn number_of_cpus_cap() -> usize
		{
			match num_cpus::get()
			{
				0 | 1 => 1,
				other @ _ => other - 1,
			}
		}

		let number_of_cpus_cap = number_of_cpus_cap();

		let number_of_worker_threads = match number_of_logical_cores_to_utilize
		{
			Some(0) => 1,
			Some(maximum) => min(number_of_cpus_cap, maximum),
			None => number_of_cpus_cap,
		};

		LogicalCoreUtilizationDetail
		{
			main_loop: LogicalCores::from(0),
			worker_loops: Cow::Owned
			(
				if number_of_cpus_cap == 1
				{
					vec![LogicalCores::from(0)]
				}
				else
				{
					let mut worker_loops = Vec::with_capacity(number_of_worker_threads);
					for index in 1 .. number_of_worker_threads
					{
						worker_loops.push(LogicalCores::from(index))
					}
					worker_loops
				}
			)
		}
	}
}
