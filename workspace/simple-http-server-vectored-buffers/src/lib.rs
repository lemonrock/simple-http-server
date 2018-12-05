// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![deny(missing_docs)]
#![deny(unreachable_patterns)]
#![feature(core_intrinsics)]


//! #simple-http-server-vectored-buffers
//! 
//! Vectored Buffers, a bit like iovec, are used to manage per-HTTP(S) connection management fairly and securely.


#[macro_use] extern crate likely;


use ::std::alloc::GlobalAlloc;
use ::std::alloc::Layout;
use ::std::alloc::System;
use ::std::borrow::Borrow;
use ::std::borrow::BorrowMut;
use ::std::cell::*;
use ::std::convert::AsMut;
use ::std::convert::AsRef;
use ::std::io;
use ::std::io::Read;
use ::std::io::Write;
use ::std::mem::align_of;
use ::std::mem::ManuallyDrop;
use ::std::mem::size_of;
use ::std::mem::uninitialized;
use ::std::ops::Deref;
use ::std::ops::DerefMut;
use ::std::ops::Index;
use ::std::ops::IndexMut;
use ::std::ptr::drop_in_place;
use ::std::ptr::NonNull;
use ::std::ptr::read;
use ::std::ptr::write;
use ::std::rc::Rc;
use ::std::slice::from_raw_parts;
use ::std::slice::from_raw_parts_mut;
use ::std::sync::Arc;
use ::std::sync::atomic::AtomicUsize;
use ::std::sync::atomic::Ordering::Relaxed;
use ::std::sync::atomic::Ordering::SeqCst;


include!("Allocation.rs");
include!("AllocationObserver.rs");
include!("AllocationObserverIdentifier.rs");
include!("Buffer.rs");
include!("BufferArena.rs");
include!("BufferHeader.rs");
include!("BufferReference.rs");
include!("BufferIndex.rs");
include!("Bytes.rs");
include!("ExclusiveToOffset.rs");
include!("ImmutableVectoredBufferSlice.rs");
include!("ImmutableVectoredBufferSliceIterator.rs");
include!("InclusiveFromOffset.rs");
include!("MutableVectoredBufferSlice.rs");
include!("MutableVectoredBufferSliceIterator.rs");
include!("RingBuffer.rs");
include!("Status.rs");
include!("VectoredBuffer.rs");
include!("VectoredBufferOffset.rs");
