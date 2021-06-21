use alloc::vec::Vec;
use serde_encrypt_core::encrypt::plain_message_shared_key::PlainMessageSharedKeyDeterministicCore;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct PlainMessageSharedKeyDeterministic(Vec<u8>);

impl PlainMessageSharedKeyDeterministicCore for PlainMessageSharedKeyDeterministic {
    fn new(plain_message: Vec<u8>) -> Self
    where
        Self: Sized,
    {
        Self(plain_message)
    }

    fn into_vec(self) -> Vec<u8> {
        self.0
    }

    fn as_slice(&self) -> &[u8] {
        &self.0
    }
}
