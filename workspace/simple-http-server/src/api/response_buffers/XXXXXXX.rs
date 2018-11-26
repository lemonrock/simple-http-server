// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.










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



/*
	ContentType
		- can have a charset parameter for a very small number of types (text/*);
		- can have a boundary parameter of 1 - 70 bytes for multipart responses.
		- case insensitive but lowercase recommended.
		- no space around equals
		- string can be quoted
     Content-Type: text/html; charset=ISO-8859-4

   A sender that generates a message containing a payload body SHOULD
   generate a Content-Type header field in that message unless the
   intended media type of the enclosed representation is unknown to the
   sender.  If a Content-Type header field is not present, the recipient
   MAY either assume a media type of "application/octet-stream"
*/


impl Response
{
	#[inline(always)]
	fn common_headers(statusCode: StatusCode, contentType: ContentType) -> Self
	{
		Response::new()

			TODO

				ContentType
				ContentLanguage     Content-Language: de-DE, en-CA
				?ContentEncoding?
				Location  (eg for redirects)
					- can use a relative URI
					- see https://docs.rs/uriparse/0.3.3/uriparse/uri/index.html; might also be more appropriate for HTTP, too.
					- hyper Uri struct is far more correct    https://docs.rs/hyper/0.12.14/hyper/struct.Uri.html
						- use this for requests, too.
				Retry-After - for service unavailable

			DONE
				.with_status(statusCode)
				Allow
				AccessControlMaxAge
				AccessControlAllowMethods
				ContentLength
				CacheControl
				LastModified
				.with_header(X_Content_Type_Options::Default)
				.with_header(X_XSS_Protection::Default)
				.with_header(Date(SystemTime::now().into()))
				.with_header(X_Frame_Options::Default)
				endofheaders



		URI









	}

	#[inline(always)]
	fn static_response<I: Into<Cow<'static, str>>>(isHead: bool, statusCode: StatusCode, contentType: ContentType, body: I) -> Self
	{
		let body = body.into();
		let response = Self::common_headers(statusCode, contentType).with_header(ContentLength(body.len() as u64));

		if isHead
			{
				response.with_body(Body::empty())
			}
			else
			{
				response.with_body(body)
			}
	}

	#[inline(always)]
	fn static_txt_response<I: Into<Cow<'static, str>>>(isHead: bool, statusCode: StatusCode, body: I) -> Self
	{
		Self::static_response(isHead, statusCode, content_type_text_plain_utf8(), body)
	}

	#[inline(always)]
	fn static_html_response<I: Into<Cow<'static, str>>>(isHead: bool, statusCode: StatusCode, body: I) -> Self
	{
		Self::static_response(isHead, statusCode, content_type_text_html_utf8(), body)
	}

	#[inline(always)]
	fn options(permittedMethods: Vec<Method>, responseToAccessControlRequest: Option<(AccessControlAllowOrigin, Option<Vec<Method>>, Option<Vec<Ascii<String>>>)>) -> Self
	{
		const CacheTimeInSeconds: u32 = 60;

		let response = Self::static_txt_response(false, StatusCode::Ok, "")
			.with_header(commonCacheControlHeader(CacheTimeInSeconds));

		let response = if let Some(responseToAccessControlRequest) = responseToAccessControlRequest
			{
				let mut response = response.with_header(AccessControlMaxAge(CacheTimeInSeconds)).with_header(responseToAccessControlRequest.0);

				if let Some(methods) = responseToAccessControlRequest.1
					{
						response = response.with_header(AccessControlAllowMethods(methods));
					}

				if let Some(headers) = responseToAccessControlRequest.2
					{
						response = response.with_header(AccessControlAllowHeaders(headers));
					}

				response
			}
			else
			{
				response
			};

		response.with_header(Allow(permittedMethods.clone()))
	}

	#[inline(always)]
	fn method_not_allowed(permittedMethods: Vec<Method>) -> Self
	{
		Self::static_txt_response(false, StatusCode::MethodNotAllowed, "")
			.with_header(commonCacheControlHeader(60))
			.with_header(Allow(permittedMethods))
	}

	#[inline(always)]
	fn misdirected_request(isHead: bool) -> Self
	{
		Self::static_txt_response(isHead, StatusCode::MisdirectedRequest, "")
			.with_header(commonCacheControlHeader(60))
	}

	#[inline(always)]
	fn old_permanent_redirect(isHead: bool, url: &Url) -> Self
	{
		Self::static_txt_response(isHead, StatusCode::MovedPermanently, "")
			.with_header(commonCacheControlHeader(31536000))
			.with_header(Location::new(url.as_ref().to_owned()))
	}

	#[inline(always)]
	fn old_temporary_redirect(isHead: bool, url: &Url) -> Self
	{
		Self::static_txt_response(isHead, StatusCode::Found, "")
			.with_header(commonCacheControlHeader(60))
			.with_header(Location::new(url.as_ref().to_owned()))
	}

	#[inline(always)]
	fn precondition_failed(isHead: bool, entityTag: &str, lastModified: HttpDate) -> Self
	{
		Self::static_txt_response(isHead, StatusCode::PreconditionFailed, "")
			.with_header(ETag(EntityTag::strong(entityTag.to_owned())))
			.with_header(LastModified(lastModified))
	}

	#[inline(always)]
	fn forbidden(isHead: bool) -> Self
	{
		Self::static_txt_response(isHead, StatusCode::Forbidden, "")
	}

	#[inline(always)]
	fn not_modified(entityTag: &str, lastModified: HttpDate, headers: &[(String, String)]) -> Self
	{
		let mut response = Response::new()
			.with_status(StatusCode::NotModified)
			.with_header(Date(SystemTime::now().into()))
			.with_header(ETag(EntityTag::strong(entityTag.to_owned())))
			.with_header(LastModified(lastModified));

		{
			let responseHeaders = response.headers_mut();

			for &(ref headerName, ref headerValue) in headers.iter()
				{
					match headerName.to_ascii_lowercase().as_str()
						{
							"cache-control" | "vary" => responseHeaders.append_raw(headerName.to_owned(), headerValue.to_owned()),
							"content-location" | "date" | "etag" | "expires" => responseHeaders.set_raw(headerName.to_owned(), headerValue.to_owned()),
							_ => (),
						}
				}
		}

		response
	}

	#[inline(always)]
	fn not_found(isHead: bool) -> Self
	{
		Self::static_html_response(isHead, StatusCode::NotFound, "<!doctype html><title>Not found</title><p>The document was not found here.".to_owned())
			.with_header(commonCacheControlHeader(60))
	}

	#[inline(always)]
	fn range_not_satisfiable(contentLength: u64) -> Self
	{
		Response::new()
			.with_status(StatusCode::RangeNotSatisfiable)
			.with_header(Date(SystemTime::now().into()))
			.with_header(ContentRange(ContentRangeSpec::Bytes
				{
					range: None,
					instance_length: Some(contentLength),
				}))
	}

	#[inline(always)]
	fn single_part_partial_content(isInResponseToIfRange: bool, contentType: &ContentType, entityTag: &str, lastModified: HttpDate, headers: &[(String, String)], fullBodyLength: usize, fromInclusive: usize, toExclusive: usize, contentFragment: &[u8]) -> Self
	{
		let mut response = Response::new()
			.with_status(StatusCode::PartialContent)
			.with_header(Date(SystemTime::now().into()))
			.with_header(ETag(EntityTag::strong(entityTag.to_owned())))
			.with_header(LastModified(lastModified))
			.with_header(ContentLength(contentFragment.len() as u64))
			.with_header(contentType.clone())
			.with_header(ContentRange(ContentRangeSpec::Bytes
				{
					range: Some((fromInclusive as u64, (toExclusive - 1) as u64)),
					instance_length: Some(fullBodyLength as u64),
				}));

		if isInResponseToIfRange
			{
				let responseHeaders = response.headers_mut();

				for &(ref headerName, ref headerValue) in headers.iter()
					{
						match headerName.to_ascii_lowercase().as_str()
							{
								"cache-control" | "vary" => responseHeaders.append_raw(headerName.to_owned(), headerValue.to_owned()),
								"content-location" | "date" | "etag" | "expires" => responseHeaders.set_raw(headerName.to_owned(), headerValue.to_owned()),
								_ => (),
							}
					}
			}
			else
			{
				let responseHeaders = response.headers_mut();

				responseHeaders.set(AcceptRanges(vec![RangeUnit::Bytes]));

				for &(ref headerName, ref headerValue) in headers.iter()
					{
						match headerName.to_ascii_lowercase().as_str()
							{
								"cache-control" | "vary" => responseHeaders.append_raw(headerName.to_owned(), headerValue.to_owned()),
								"content-location" | "date" | "etag" | "expires" => responseHeaders.set_raw(headerName.to_owned(), headerValue.to_owned()),
								_ => responseHeaders.append_raw(headerName.to_owned(), headerValue.to_owned()),
							}
					}
			}

		response.with_body(contentFragment.to_vec())
	}

	//noinspection SpellCheckingInspection
	#[inline(always)]
	fn multi_part_partial_content(isInResponseToIfRange: bool, entityTag: &str, lastModified: HttpDate, headers: &[(String, String)], body: Vec<u8>, boundary: Vec<u8>) -> Self
	{
		let mimeType = format!("multipart/byteranges; boundary={}", unsafe { String::from_utf8_unchecked(boundary) }).parse().unwrap();

		let mut response = Response::new()
			.with_status(StatusCode::PartialContent)
			.with_header(Date(SystemTime::now().into()))
			.with_header(ETag(EntityTag::strong(entityTag.to_owned())))
			.with_header(LastModified(lastModified))
			.with_header(ContentLength(body.len() as u64))
			.with_header(ContentType(mimeType));

		if isInResponseToIfRange
			{
				let responseHeaders = response.headers_mut();

				for &(ref headerName, ref headerValue) in headers.iter()
					{
						match headerName.to_ascii_lowercase().as_str()
							{
								"cache-control" | "vary" => responseHeaders.append_raw(headerName.to_owned(), headerValue.to_owned()),
								"content-location" | "date" | "etag" | "expires" => responseHeaders.set_raw(headerName.to_owned(), headerValue.to_owned()),
								_ => (),
							}
					}
			}
			else
			{
				let responseHeaders = response.headers_mut();

				responseHeaders.set(AcceptRanges(vec![RangeUnit::Bytes]));

				for &(ref headerName, ref headerValue) in headers.iter()
					{
						match headerName.to_ascii_lowercase().as_str()
							{
								"cache-control" | "vary" => responseHeaders.append_raw(headerName.to_owned(), headerValue.to_owned()),
								"content-location" | "date" | "etag" | "expires" => responseHeaders.set_raw(headerName.to_owned(), headerValue.to_owned()),
								_ => responseHeaders.append_raw(headerName.to_owned(), headerValue.to_owned()),
							}
					}
			}

		response.with_body(body)
	}

	// Bad Request gets displayed to the end user
	#[inline(always)]
	fn bad_request<I: Into<Cow<'static, str>>>(isHead: bool, body: I) -> Self
	{
		Self::static_txt_response(isHead, StatusCode::BadRequest, body)
	}

	#[inline(always)]
	fn invalid_request_uri(isHead: bool) -> Self
	{
		Self::bad_request(isHead, "")
	}

	#[inline(always)]
	fn http11_missing_host_header(isHead: bool) -> Self
	{
		Self::bad_request(isHead, "")
	}

	#[inline(always)]
	fn unsupported_http_version(isHead: bool) -> Self
	{
		Self::static_txt_response(isHead, StatusCode::HttpVersionNotSupported, "Only HTTP/1.1 and H2 over TLS are supported")
	}

	#[inline(always)]
	fn asterisk_form_request_uri_is_only_allowed_for_OPTIONS_method(isHead: bool) -> Self
	{
		Self::invalid_request_uri(isHead)
	}

	#[inline(always)]
	fn authority_form_request_uri_is_only_allowed_for_CONNECT_method(isHead: bool) -> Self
	{
		Self::invalid_request_uri(isHead)
	}

	#[inline(always)]
	fn path_is_not_validly_encoded(isHead: bool) -> Self
	{
		Self::invalid_request_uri(isHead)
	}

	#[inline(always)]
	fn query_is_not_validly_encoded(isHead: bool) -> Self
	{
		Self::invalid_request_uri(isHead)
	}

	#[inline(always)]
	fn unknown_or_unsupported_scheme_for_absolute_uri(isHead: bool) -> Self
	{
		Self::misdirected_request(isHead)
	}

	#[inline(always)]
	fn authority_port_is_not_ours(isHead: bool) -> Self
	{
		Self::misdirected_request(isHead)
	}

	#[inline(always)]
	fn authority_server_is_not_one_of_ours(isHead: bool) -> Self
	{
		Self::misdirected_request(isHead)
	}
}
