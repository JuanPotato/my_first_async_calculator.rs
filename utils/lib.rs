/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::fmt;

pub use crate::deserialize::{Deserializable, Deserializer};
pub use crate::serialize::{Serializable, Serializer};

pub use crate::packet_streamer::PacketStreamer;
pub use crate::packet_sink::PacketSink;

pub use crate::sereal_streamer::SerealStreamer;
pub use crate::sereal_sink::SerealSink;

mod deserialize;
mod serialize;
mod fancy_packet_streamer;
mod packet_streamer;
mod packet_sink;
mod sereal_streamer;
mod sereal_sink;

#[derive(Debug)]
pub enum Operation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

#[derive(Debug)]
pub struct MathRequest {
    pub id: u32,
    pub operation: Operation,
    pub a: f64,
    pub b: f64,
}

#[derive(Debug)]
pub struct MathResult {
    pub id: u32,
    pub res: f64,
}

impl MathRequest {
    pub fn add(a: f64, b: f64) -> MathRequest {
        MathRequest {
            id: rand::random(),
            operation: Operation::Addition,
            a,
            b,
        }
    }

    pub fn subtract(a: f64, b: f64) -> MathRequest {
        MathRequest {
            id: rand::random(),
            operation: Operation::Subtraction,
            a,
            b,
        }
    }

    pub fn multiply(a: f64, b: f64) -> MathRequest {
        MathRequest {
            id: rand::random(),
            operation: Operation::Multiplication,
            a,
            b,
        }
    }

    pub fn divide(a: f64, b: f64) -> MathRequest {
        MathRequest {
            id: rand::random(),
            operation: Operation::Division,
            a,
            b,
        }
    }
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match self {
            Operation::Addition => '+',
            Operation::Subtraction => '-',
            Operation::Multiplication => '*',
            Operation::Division => '/',
        };

        write!(f, "{}", c)
    }
}

impl fmt::Display for MathRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.a, self.operation, self.b)
    }
}
