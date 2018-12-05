// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) struct ConnectionObserver<SCCUF: ServedClientConnectionUserFactory>
{
	maximum_connections: usize,

	current_connections: AtomicUsize,

	internet_protocol_version_4_access_control_list: IpLookupTable<Ipv4Addr, ()>,

	internet_protocol_version_6_access_control_list: IpLookupTable<Ipv6Addr, ()>,

	served_client_connection_user_factory: SCCUF,
}

impl<SCCUF: ServedClientConnectionUserFactory> ConnectionObserver<SCCUF>
{
	#[inline(always)]
	pub(crate) fn new(maximum_connections: usize, permitted_internet_protocol_version_4_addresses: &HashSet<PermittedInternetProtocolVersionAddresses<Ipv4Addr>>, permitted_internet_protocol_version_6_addresses: &HashSet<PermittedInternetProtocolVersionAddresses<Ipv6Addr>>, served_client_connection_user_factory: SCCUF) -> Self
	{
		// NOTE: At this point in time `A` in `IpLookupTable` is a private type so we can't de-deduplicate this logic.
		let mut internet_protocol_version_4_access_control_list = IpLookupTable::with_capacity();
		for &(address, mask) in permitted_internet_protocol_version_4_addresses.iter()
		{
			internet_protocol_version_4_access_control_list.insert(address, mask, ());
		}
		let mut internet_protocol_version_6_access_control_list = IpLookupTable::with_capacity();
		for &(address, mask) in permitted_internet_protocol_version_6_addresses.iter()
		{
			internet_protocol_version_6_access_control_list.insert(address, mask, ());
		}

		Self
		{
			maximum_connections,
			current_connections: AtomicUsize::new(0),
			internet_protocol_version_4_access_control_list,
			internet_protocol_version_6_access_control_list,
			served_client_connection_user_factory,
		}
	}

	#[inline(always)]
	pub(crate) fn connect(&self, remote_address: SocketAddr) -> Result<SCCUF::User, ConnectionObserverConnectError<SCCUF::Error>>
	{
		use self::ConnectionObserverConnectError::*;
		use self::SocketAddr::*;

		let is_blocked = match remote_address
		{
			V4(internet_protocol_version_4_address) => self.internet_protocol_version_4_access_control_list.longest_match(internet_protocol_version_4_address).is_none(),
			V6(internet_protocol_version_6_address) => self.internet_protocol_version_6_access_control_list.longest_match(internet_protocol_version_6_address).is_none(),
		};

		if unlikely!(is_blocked)
		{
			return Err(RemoteAddressBlocked)
		}

		let mut current_connections = self.current_connections.load(Relaxed);
		loop
		{
			if unlikely!(current_connections == self.maximum_connections)
			{
				return Err(MaximumConnections)
			}
			match self.current_connections.compare_exchange(current_connections, current_connections + 1, SeqCst, Relaxed)
			{
				Err(updated_current_connections) => current_connections = updated_current_connections,
				Ok(_) => break,
			}
		}

		self.served_client_connection_user_factory.connect(remote_address).map_err(|error|
		{
			disconnect(remote_address);

			ConnectionObserverConnectError(error)
		})
	}

	#[inline(always)]
	pub(crate) fn disconnect(&self, remote_address: SocketAddr)
	{
		self.served_client_connection_user_factory.disconnect(remote_address);

		let mut current_connections = self.current_connections.load(Relaxed);
		loop
		{
			debug_assert_ne!(current_connections, 0, "Mismatched connect and disconnect calls");

			match self.current_connections.compare_exchange(current_connections, current_connections - 1, SeqCst, Relaxed)
			{
				Err(updated_current_connections) => current_connections = updated_current_connections,
				Ok(_) => break,
			}
		}
	}
}
