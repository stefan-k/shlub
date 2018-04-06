// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! # Todo

// Create the Error, ErrorKind, ResultExt, and Result types
error_chain!{
    foreign_links {
        // doesnt seem to work, check again
        IO(::std::io::Error);
    }
}
