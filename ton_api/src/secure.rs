use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};

use ::secstr::*;
use serde::{Deserialize, Serialize};

use crate::{BareDeserialize, BareSerialize};

/// SecureBytes built-in type.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct SecureBytes(SecVec<u8>);

impl SecureBytes {
    pub fn new(cont: Vec<u8>) -> Self {
        Self(SecVec::<u8>::new(cont))
    }

    /// Borrow the contents of the string.
    pub fn unsecure(&self) -> &[u8] {
        self.0.unsecure()
    }

    /// Mutably borrow the contents of the string.
    pub fn unsecure_mut(&mut self) -> &mut [u8] {
        self.0.unsecure_mut()
    }

    /// Overwrite the string with zeros. This is automatically called in the destructor.
    pub fn zero_out(&mut self) {
        self.0.zero_out()
    }
}

impl Default for SecureBytes {
    fn default() -> Self {
        Self::new(Vec::default())
    }
}

impl Debug for SecureBytes {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl Display for SecureBytes {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

#[allow(clippy::derive_hash_xor_eq)]
impl Hash for SecureBytes {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.unsecure().hash(state);
    }
}

impl BareSerialize for SecureBytes {
    fn serialize_bare(&self, ser: &mut crate::Serializer) -> crate::Result<()> {
        self.0.unsecure().serialize_bare(ser)
    }
}

impl BareDeserialize for SecureBytes {
    fn deserialize_bare(de: &mut crate::Deserializer) -> crate::Result<Self> {
        Vec::<u8>::deserialize_bare(de).map(|vec| SecureBytes(SecVec::<u8>::new(vec)))
    }
}

/// SecureString built-in type.
pub type SecureString = SecureBytes;
