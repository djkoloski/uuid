// Copyright 2013-2014 The Rust Project Developers.
// Copyright 2018 The Uuid Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::prelude::*;
use bytecheck::CheckBytes;

impl<C: ?Sized> CheckBytes<C> for Uuid {
    type Error = <Bytes as CheckBytes<C>>::Error;

    unsafe fn check_bytes<'a>(value: *const Self, context: &mut C) -> Result<&'a Self, Self::Error> {
        // Safety: cast is OK because Uuid is repr(transparent)
        Bytes::check_bytes(value.cast(), context)?;
        Ok(&*value)
    }
}

#[cfg(test)]
mod bytecheck_tests {
    use crate::prelude::*;
    use bytecheck::CheckBytes;

    #[test]
    fn test_check_bytes() {
        let uuid_str = "f9168c5e-ceb2-4faa-b6bf-329bf39fa1e4";
        let u = Uuid::parse_str(uuid_str).unwrap();

        // Safety: the pointer is aligned and points to enough bytes to represent a Uuid
        unsafe {
            Uuid::check_bytes(&u as *const Uuid, &mut ())
                .expect("failed to check uuid");
        }
    }
}
