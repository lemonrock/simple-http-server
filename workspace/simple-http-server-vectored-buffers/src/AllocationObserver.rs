// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// An observer of allocations (and deallocations) which can also veto any allocation.
pub trait AllocationObserver
{
	/// Observes an allocation just prior to it occurring.
	///
	/// Returns `true` if allocation should be vetoed.
	fn observe_and_veto_forthcoming_allocation(&self, allocation_oberserver_identifier: AllocationObserverIdentifier) -> bool;

	/// Observes a deallocation just before it occurs.
	fn observe_forthcoming_deallocation(&self, allocation_oberserver_identifier: AllocationObserverIdentifier);
}
