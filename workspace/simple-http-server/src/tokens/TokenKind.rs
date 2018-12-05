// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// On a 32-bit platform, limited to 4 variants; ona 64-bit platform, limited to 8 variants.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub(crate) enum TokenKind
{
	ServerListener = 0,

	ServedClientConnection = 1,

	ReceiveFromWorkerThread = 2,
}

impl From<u8> for TokenKind
{
	#[inline(always)]
	fn from(tag: u8) -> Self
	{
		unsafe { transmute(tag) }
	}
}

impl TokenKind
{
	#[inline(always)]
	pub(crate) fn handle_event<SCCUF: ServedClientConnectionUserFactory>(token: Token, readiness: Ready, poll: &Poll, drop_token_when_all_events_handled: &mut HashSet<Token>)
	{
		use self::TokenKind::*;

		// This can happen because of the limitations of the Operating System implementation details `Poll` relies on:-
		// * an event per kind of readiness can be generated;
		// * spurious events can occur
		if unlikely!(drop_token_when_all_events_handled.contains(token))
		{
			return
		}

		let drop = match token.tag_enum::<Self>()
		{
			ServerListener => token.as_mut::<ServerListenerToken<SCCUF>>().handle_event(poll),

			ServedClientConnection => token.as_mut::<ArenaItem<ServedClientConnectionToken<SCCUF>>>().handle_event(poll, readiness, token),

			ReceiveFromWorkerThread => token.as_mut::<ReceiveFromWorkerThreadToken<SCCUF>>().handle_event(),
		};

		if unlikely!(drop)
		{
			drop_token_when_all_events_handled.insert(token)
		}
	}

	#[inline(always)]
	pub(crate) fn drop_tokens<SCCUF: ServedClientConnectionUserFactory>(served_client_connection_arena: &Arena<ServedClientConnectionToken<SCCUF>>, drop_token_when_all_events_handled: &mut HashSet<Token>)
	{
		use self::TokenKind::*;

		for token in drop_token_when_all_events_handled.drain()
		{
			match token.tag_enum::<Self>()
			{
				ServerListener => drop(token.as_box::<ServerListenerToken<SCCUF>>()),

				ServedClientConnection => served_client_connection_arena.deallocate(token.as_non_null_pointer::<ArenaItem<ServedClientConnectionToken<SCCUF>>>()),

				ReceiveFromWorkerThread =>  drop(token.as_box::<ReceiveFromWorkerThreadToken<SCCUF>>()),
			}
		}
	}

	#[inline(always)]
	pub(crate) fn into_token_from_box<T: Sized>(self, boxed: Box<T>) -> Token
	{
		Self::into_token(Box::into_raw(boxed))
	}

	#[inline(always)]
	pub(crate) fn into_token<T: Sized>(self, raw_pointer: *mut T) -> Token
	{
		let tag = (self as u8);
		let token_inner = (raw_pointer as usize) | (tag as usize);
		Token(token_inner)
	}
}
