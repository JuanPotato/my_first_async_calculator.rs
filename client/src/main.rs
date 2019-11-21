/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::io;

use crate::calculator::Calculator;

mod calculator;


#[tokio::main]
async fn main() -> io::Result<()> {
    let mut calc = Calculator::new();

    let res = calc.add(40.0, 200.0).await;
    println!("{:?}", res);

    let res = calc.subtract(40.0, 2.0).await;
    println!("{:?}", res);

    let res = calc.multiply(991.0, 997.0).await;
    println!("{:?}", res);

    let res = calc.divide(988027.0, 991.0).await;
    println!("{:?}", res);

    Ok(())
}
