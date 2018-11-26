// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


/// Represents a list of tags for a `X-Robots-Tag` header.
///
/// This, along with the `Set-Cookie` header, is one of the very few that can occur more than once but is NOT comma separated.
///
/// In this particular case, only headers with the same `robot_user_agent` can be comma-coalseced.
// pub(crate) const X_Robots_Tag_Data: &'static str = "noindex,nofollow,noarchive,nosnippet,notranslate,noimageindex";
pub struct XRobotsTagHeaderResponseBuffer(ArrayVec<[Cow<'static, [u8]>; 12]>);

impl HeaderResponseBuffer for XRobotsTagHeaderResponseBuffer
{
	#[inline(always)]
	fn report_buffer(&self, buffer_index: usize, from_offset: usize) -> &[u8]
	{
		debug_assert!(buffer_index < self.0.len(), "buffer_index `{}` out of range", buffer_index);

		let buffer = (unsafe { self.0.get_unchecked(buffer_index) }).borrow();

		debug_assert!(from_offset < buffer.len(), "from_offset `{}` out of range for buffer_index `{}`", from_offset, buffer_index);

		&buffer[from_offset .. ]
	}
}
impl XRobotsTagHeaderResponseBuffer
{
	/// Create a new instance.
	pub fn new(robot_user_agent: Option<Cow<'static, [u8]>>, mut robot_tags: BTreeSet<RobotTag>) -> Self
	{
		use self::RobotTag::*;

		debug_assert_ne!(robot_tags.len(), 0, "robot_tags should always contain at least one tag");
		debug_assert!(!(robot_tags.len() > 1 && robot_tags.contains(all)), "all should never be present with other tags");

		if robot_tags.contains(noindex) && robot_tags.contains(nofollow)
		{
			robot_tags.remove(noindex);
			robot_tags.remove(nofollow);
			robot_tags.insert(none);
		}

		#[cfg(debug_assertions)]
		{
			let mut encountered_unavailable_after = false;
			for robot_tag in robot_tags.iter()
			{
				match robot_tag
				{
					unavailable_after(_) => if encountered_unavailable_after
					{
						panic!("unavailable_after included more than once")
					}
					else
					{
						encountered_unavailable_after = true;
					},
					_ => false
				};
			}
		}

		let mut buffers = ArrayVec::new();

		use self::Cow::Borrowed;

		buffers.push(Borrowed(b"X-Robots-Tag:"));

		if let Some(robot_user_agent) = robot_user_agent
		{
			buffers.push(robot_user_agent);
			buffers.push(Borrowed(b":"));
		}

		let mut robot_tags = robot_tags.iter();
		let first_robot_tag = robot_tags.next.unwrap();
		buffers.push(first_robot_tag.buffer());

		for robot_tag in robot_tags
		{
			buffers.push(robot_tag.with_leading_comma_buffer());
		}
		buffers.push(Borrowed(b"\r\n"));

		XRobotsTagHeaderResponseBuffer(buffers)
	}
}
