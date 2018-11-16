// This file is part of simple-http-server. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT. No part of simple-http-server, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of simple-http-server. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/simple-http-server/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum ServerConfigurationError
{
	CouldNotOpenCertificateAuthoritiesPemFile(io::Error),

	CouldNotReadCertificateAuthoritiesPemFile,

	NoValidCertificateAuthoritiesInCertificateAuthoritiesPemFile,

	CouldNotOpenServerCertificateFile(io::Error),

	CouldNotReadServerCertificateFile,

	CouldNotOpenServerPrivateKeyFile(io::Error),

	CouldNotReadServerPkcs8PrivateKey,

	CouldNotReadServerRsaPrivateKey,

	ThereIsNeitherAPkcs8OrRsaServerPrivateKey,

	CouldNotOpenOnlineCertificateStatusProtocolFile(io::Error),

	CouldNotReadOnlineCertificateStatusProtocolFile(io::Error),

	CouldNotOpenSignedCertificateTimestampStatusFile(io::Error),

	CouldNotReadSignedCertificateTimestampStatusFile(io::Error),

	CouldNotSetCertificateChainAndPrivateKey(TLSError),
}

impl Display for ServerConfigurationError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for ServerConfigurationError
{
	#[inline(always)]
	fn source(&self) -> Option<&(error::Error + 'static)>
	{
		use self::ServerConfigurationError::*;

		match self
		{
			&CouldNotOpenCertificateAuthoritiesPemFile(ref error) => Some(error),

			&CouldNotReadCertificateAuthoritiesPemFile => None,

			&NoValidCertificateAuthoritiesInCertificateAuthoritiesPemFile => None,

			&CouldNotOpenServerCertificateFile(ref error) => Some(error),

			&CouldNotReadServerCertificateFile => None,

			&CouldNotOpenServerPrivateKeyFile(ref error) => Some(error),

			&CouldNotReadServerPkcs8PrivateKey => None,

			&CouldNotReadServerRsaPrivateKey => None,

			&ThereIsNeitherAPkcs8OrRsaServerPrivateKey => None,

			&CouldNotOpenOnlineCertificateStatusProtocolFile(ref error) => Some(error),

			&CouldNotReadOnlineCertificateStatusProtocolFile(ref error) => Some(error),

			&CouldNotOpenSignedCertificateTimestampStatusFile(ref error) => Some(error),

			&CouldNotReadSignedCertificateTimestampStatusFile(ref error) => Some(error),

			&CouldNotSetCertificateChainAndPrivateKey(ref error) => Some(error),
		}
	}
}
