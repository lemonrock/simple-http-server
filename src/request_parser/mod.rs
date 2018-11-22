// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


use super::*;
use self::Status::*;
use self::InvalidReason::*;


include!("Bytes.rs");
include!("HeaderReentryPoint.rs");
include!("HeaderUser.rs");
include!("InvalidReason.rs");
include!("NonNullExt.rs");
include!("parse_http_version.rs");
include!("parse_request_method.rs");
include!("Status.rs");
include!("TargetUriReentryPoint.rs");
include!("TargetUriUser.rs");








#[cfg(target_arch = "x86")] use ::std::arch::x86::*;
#[cfg(target_arch = "x86_64")] use ::std::arch::x86_64::*;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[derive(Debug, Copy, Clone)]
pub(crate) struct ThirtyTwoBytes(__m256i);

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
impl ThirtyTwoBytes
{
	#[target_feature(enable = "avx2")]
	#[inline(always)]
	pub(crate) fn zeroed() -> Self
	{
		ThirtyTwoBytes(unsafe { _mm256_setzero_si256() })
	}

	#[target_feature(enable = "avx2")]
	#[inline(always)]
	pub(crate) fn load_32_bytes(pointer: NonNull<u8>) -> Self
	{
		ThirtyTwoBytes(unsafe { _mm256_lddqu_si256(pointer.as_ptr() as *const _) })
	}

	#[target_feature(enable = "avx2")]
	#[inline(always)]
	pub(crate) fn repeat_32_times(comparator: u8) -> Self
	{
		ThirtyTwoBytes(unsafe { _mm256_set1_epi8(comparator) })
	}

	#[target_feature(enable = "avx2")]
	#[inline(always)]
	pub(crate) fn greater_than(self, right: Self) -> ThirtyTwoBytesComparison
	{
		ThirtyTwoBytesComparison(unsafe { _mm256_cmpgt_epi8(self.0, right.0) })
	}

	#[target_feature(enable = "avx2")]
	#[inline(always)]
	pub(crate) fn equal_to(self, right: Self) -> ThirtyTwoBytesComparison
	{
		ThirtyTwoBytesComparison(unsafe { _mm256_cmpeq_epi8(self.0, right.0) })
	}
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[derive(Debug, Copy, Clone)]
pub(crate) struct ThirtyTwoBytesComparison(__mm256i);

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
impl ThirtyTwoBytesComparison
{
	#[target_feature(enable = "avx2")]
	#[inline(always)]
	pub(crate) fn zeroed() -> Self
	{
		ThirtyTwoBytesComparison(unsafe { _mm256_setzero_si256() })
	}

	#[target_feature(enable = "avx2")]
	#[inline(always)]
	pub(crate) fn bitwise_or(self, right: Self) -> Self
	{
		ThirtyTwoBytesComparison(unsafe { _mm256_or_si256(self.0, right.0) })
	}

	#[target_feature(enable = "avx2")]
	#[inline(always)]
	pub(crate) fn bitwise_nand(self, right: Self) -> Self
	{
		ThirtyTwoBytesComparison(unsafe { _mm256_andnot_si256(self.0, right.0) })
	}

	#[target_feature(enable = "avx2")]
	#[inline(always)]
	pub(crate) fn equal_to(self, right: Self) -> Self
	{
		ThirtyTwoBytesComparison(unsafe { _mm256_cmpeq_epi8(self.0, right.0) })
	}

	#[target_feature(enable = "avx2")]
	#[inline(always)]
	pub(crate) fn create_mask_for_each_byte_from_top_most_bit(self) -> Self
	{
		ThirtyTwoBytesComparison(unsafe { _mm256_movemask_epi8(self.0) })
	}
}

#[target_feature(enable = "avx2")]
#[inline(always)]
fn compare_32_bytes_at_once(pointer: NonNull<u8>) -> usize
{
	const HorizontalTab: u8 = 0x09;

	// %x20-%x7e %x80-%xff


	let thirty_two_bytes = ThirtyTwoBytes::load_32_bytes(pointer);
	let low = thirty_two_bytes.greater_than(ThirtyTwoBytes::repeat_32_times(0x1F));
	let tab = thirty_two_bytes.greater_than(ThirtyTwoBytes::repeat_32_times(HorizontalTab));
	let del = thirty_two_bytes.greater_than(ThirtyTwoBytes::repeat_32_times(0x7F));

	let bit = del.bitwise_nand(low.bitwise_or(tab));
	let rev = bit.equal_to(ThirtyTwoBytesComparison::zeroed());
	let res = 0xFFFFFFFF_00000000 | rev.create_mask_for_each_byte_from_top_most_bit() as u64;

	(unsafe { _tzcnt_u64(res) }) as usize
}
