// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


pub(crate) const SCM_RIGHTS: c_int = 0x01;

pub(crate) const SCM_CREDENTIALS: c_int = 0x02;

pub(crate) const SCM_TIMESTAMP: c_int = SO_TIMESTAMP;

pub(crate) const SCM_TIMESTAMPNS: c_int = SO_TIMESTAMPNS;

pub(crate) const SCM_TIMESTAMPING: c_int = SO_TIMESTAMPING;

pub(crate) const SCM_WIFI_STATUS: c_int = SO_WIFI_STATUS;

pub(crate) const SCM_TIMESTAMPING_OPT_STATS: c_int = 54;

pub(crate) const SCM_TIMESTAMPING_PKTINFO: c_int = 58;
