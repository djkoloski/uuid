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
use rkyv::{Archive, ArchiveCopy, Deserialize, Fallible, Serialize};

impl Archive for Uuid {
    type Archived = Uuid;
    type Resolver = ();

    fn resolve(&self, _: usize, _: Self::Resolver) -> Self::Archived {
        *self
    }
}

impl<S: Fallible + ?Sized> Serialize<S> for Uuid {
    fn serialize(&self, _: &mut S) -> Result<Self::Resolver, S::Error> {
        Ok(())
    }
}

// Safety: UUID is Copy and doesn't need to transform its data during
// serialization
unsafe impl ArchiveCopy for Uuid {}

impl<D: Fallible + ?Sized> Deserialize<Uuid, D> for Uuid {
    fn deserialize(&self, _: &mut D) -> Result<Uuid, D::Error> {
        Ok(*self)
    }
}

#[cfg(test)]
mod rkyv_tests {
    use crate::prelude::*;
    use rkyv::{archived_value, Aligned, Deserialize, Infallible, ser::{serializers::BufferSerializer, Serializer}};

    #[test]
    fn test_serialize_deserialize() {
        let uuid_str = "f9168c5e-ceb2-4faa-b6bf-329bf39fa1e4";
        let u = Uuid::parse_str(uuid_str).unwrap();

        let mut serializer = BufferSerializer::new(Aligned([0u8; 256]));
        let pos = serializer.serialize_value(&u)
            .expect("failed to archive uuid");
        let buf = serializer.into_inner();
        let archived = unsafe { archived_value::<Uuid>(buf.as_ref(), pos) };

        assert_eq!(&u, archived);

        let deserialized = archived.deserialize(&mut Infallible)
            .expect("failed to deserialize uuid");

        assert_eq!(u, deserialized);
    }
}
