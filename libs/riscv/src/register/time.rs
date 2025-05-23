// Copyright 2025 Jonas Kruckenberg
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Time Register

use super::{read_composite_csr, read_csr_as_usize};

read_csr_as_usize!(0xC01);
read_composite_csr!(super::timeh::read(), read());
