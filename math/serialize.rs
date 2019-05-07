/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::io::{self, Write};

use byteorder::{WriteBytesExt, LE};

use crate::{MathRequest, MathResult, Operation};

pub trait Serializable {
    fn serialize_to<T: Write>(&self, buf: &mut T) -> io::Result<()>;
}

pub trait Serializer: Write + Sized {
    fn serialize<T: Serializable>(&mut self, obj: &T) -> io::Result<()> {
        obj.serialize_to(self)
    }
}

impl<T> Serializer for T where T: Write + Sized {}

impl Serializable for u32 {
    fn serialize_to<T: Write>(&self, buf: &mut T) -> io::Result<()> {
        buf.write_u32::<LE>(*self)?;
        Ok(())
    }
}

impl Serializable for f64 {
    fn serialize_to<T: Write>(&self, buf: &mut T) -> io::Result<()> {
        buf.write_f64::<LE>(*self)?;
        Ok(())
    }
}

impl Serializable for Operation {
    fn serialize_to<T: Write>(&self, buf: &mut T) -> io::Result<()> {
        let val: u32 = match self {
            Operation::Addition => 0,
            Operation::Subtraction => 1,
            Operation::Multiplication => 2,
            Operation::Division => 3,
        };

        val.serialize_to(buf)
    }
}

impl Serializable for MathRequest {
    fn serialize_to<T: Write>(&self, buf: &mut T) -> io::Result<()> {
        self.id.serialize_to(buf)?;
        self.operation.serialize_to(buf)?;
        self.a.serialize_to(buf)?;
        self.b.serialize_to(buf)?;

        Ok(())
    }
}

impl Serializable for MathResult {
    fn serialize_to<T: Write>(&self, buf: &mut T) -> io::Result<()> {
        self.id.serialize_to(buf)?;
        self.res.serialize_to(buf)?;

        Ok(())
    }
}
