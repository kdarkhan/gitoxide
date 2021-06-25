//!
pub(crate) const SPACE: &[u8; 1] = b" ";

use crate::Time;
use bstr::BString;

/// A mutable signature is created by an actor at a certain time.
///
/// Note that this is not a cryptographical signature.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Signature {
    /// The actors name.
    pub name: BString,
    /// The actor's email.
    pub email: BString,
    /// The time stamp at which the signature is performed.
    pub time: Time,
}

mod convert {
    use crate::{immutable, mutable};

    impl From<immutable::Signature<'_>> for mutable::Signature {
        fn from(other: immutable::Signature<'_>) -> mutable::Signature {
            let immutable::Signature { name, email, time } = other;
            mutable::Signature {
                name: name.to_owned(),
                email: email.to_owned(),
                time,
            }
        }
    }
}

mod signature {
    use crate::mutable::{Signature, SPACE};
    use bstr::{BStr, ByteSlice};
    use quick_error::quick_error;
    use std::io;

    quick_error! {
        /// The Error produced in by [`Signature::write_to()`].
        #[derive(Debug)]
        #[allow(missing_docs)]
        enum Error {
            IllegalCharacter {
                display("Signature name or email must not contain '<', '>' or \\n")
            }
        }
    }

    impl From<Error> for io::Error {
        fn from(err: Error) -> Self {
            io::Error::new(io::ErrorKind::Other, err)
        }
    }

    /// Output
    impl Signature {
        /// Serialize this instance to `out` in the git serialization format for actors.
        pub fn write_to(&self, mut out: impl io::Write) -> io::Result<()> {
            out.write_all(validated_token(self.name.as_bstr())?)?;
            out.write_all(SPACE)?;
            out.write_all(&b"<"[..])?;
            out.write_all(validated_token(self.email.as_bstr())?)?;
            out.write_all(&b"> "[..])?;
            self.time.write_to(out)?;
            Ok(())
        }
    }

    fn validated_token(name: &BStr) -> Result<&BStr, Error> {
        if name.find_byteset(b"<>\n").is_some() {
            return Err(Error::IllegalCharacter);
        }
        Ok(name)
    }
}
