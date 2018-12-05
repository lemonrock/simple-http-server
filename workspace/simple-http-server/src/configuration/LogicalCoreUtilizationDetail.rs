// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Logical core utilization detail.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogicalCoreUtilizationDetail<'a>
{
	/// Typically `0`.
	pub main_loop: LogicalCores,

	/// If no worker loops are specified, then the settings for the `main_loop` are used for one worker.
	pub worker_loops: Cow<'a, [LogicalCores]>,
}
