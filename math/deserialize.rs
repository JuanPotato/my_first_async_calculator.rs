/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::io::{self, Read};

use byteorder::{ReadBytesExt, LE};

use crate::{MathRequest, MathResult, Operation};

pub trait Deserializable: Sized {
    fn deserialize_from<T: Read>(buf: &mut T) -> io::Result<Self>;
}

pub trait Deserializer: Read + Sized {
    fn deserialize<T: Deserializable>(&mut self) -> io::Result<T> {
        T::deserialize_from(self)
    }
}

impl<T> Deserializer for T where T: Read + Sized {}

impl Deserializable for u32 {
    fn deserialize_from<T: Read>(buf: &mut T) -> io::Result<u32> {
        buf.read_u32::<LE>()
    }
}

impl Deserializable for f64 {
    fn deserialize_from<T: Read>(buf: &mut T) -> io::Result<f64> {
        buf.read_f64::<LE>()
    }
}

impl Deserializable for Operation {
    fn deserialize_from<T: Read>(buf: &mut T) -> io::Result<Operation> {
        Ok(match buf.deserialize::<u32>()? {
            0 => Operation::Addition,
            1 => Operation::Subtraction,
            2 => Operation::Multiplication,
            3 => Operation::Division,

            _ => unreachable!(),
        })
    }
}

impl Deserializable for MathRequest {
    fn deserialize_from<T: Read>(buf: &mut T) -> io::Result<MathRequest> {
        Ok(MathRequest {
            id: buf.deserialize()?,
            operation: buf.deserialize()?,
            a: buf.deserialize()?,
            b: buf.deserialize()?,
        })
    }
}

impl Deserializable for MathResult {
    fn deserialize_from<T: Read>(buf: &mut T) -> io::Result<MathResult> {
        Ok(MathResult {
            id: buf.deserialize()?,
            res: buf.deserialize()?,
        })
    }
}
