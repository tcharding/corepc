// SPDX-License-Identifier: CC0-1.0

//! Tests for methods that are `== Hidden ==` and not in the API docs of Bitcoin Core.

#![allow(non_snake_case)] // Test names intentionally use double underscore.

use integration_test::{Node, NodeExt as _};
use node::vtype::*; // All the version specific types.
use node::mtype;
