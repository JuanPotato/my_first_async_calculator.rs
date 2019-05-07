/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

#![feature(async_await, await_macro, generators)]

use std::io;

use futures::executor;

use calculator::Calculator;

mod calculator;


fn main() -> io::Result<()> {
    executor::block_on(async {
        let mut calc = Calculator::new();
        let res = await!(calc.add(40.0, 200.0));
        println!("{:?}", res);

        let res = await!(calc.subtract(40.0, 2.0));
        println!("{:?}", res);

        let res = await!(calc.multiply(991.0, 997.0));
        println!("{:?}", res);

        let res = await!(calc.divide(988027.0, 991.0));
        println!("{:?}", res);

        Ok(())
    })
}
