// Copyright 2016 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under (1) the MaidSafe.net Commercial License,
// version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
// licence you accepted on initial access to the Software (the "Licences").
//
// By contributing code to the SAFE Network Software, or to this project generally, you agree to be
// bound by the terms of the MaidSafe Contributor Agreement, version 1.0.  This, along with the
// Licenses can be found in the root directory of this project at LICENSE, COPYING and CONTRIBUTOR.
//
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.
//
// Please review the Licences for the specific language governing permissions and limitations
// relating to use of the SAFE Network Software.

/// Length of the GUID (unique ID) of a message (16 bytes).
pub const GUID_SIZE: usize = 16;
/// Maximum allowed inbox size for an account (128 MiB).
pub const MAX_INBOX_SIZE: usize = 1 << 27;
/// Maximum allowed outbox size for an account (128 MiB).
pub const MAX_OUTBOX_SIZE: usize = 1 << 27;

mod error;
mod mpid_header;
mod mpid_message;
mod mpid_message_wrapper;

pub use self::error::Error;
pub use self::mpid_message_wrapper::MpidMessageWrapper;
pub use self::mpid_message::{MpidMessage, MAX_BODY_SIZE};
pub use self::mpid_header::{MpidHeader, MAX_HEADER_METADATA_SIZE};

use std::fmt::Write;

// Format a vector of bytes as a hexadecimal number, ellipsising all but the first and last three.
//
// For three bytes with values 1, 2, 3, the output will be "010203".  For more than six bytes, e.g.
// for fifteen bytes with values 1, 2, ..., 15, the output will be "010203..0d0e0f".
fn format_binary_array<V: AsRef<[u8]>>(input: V) -> String {
    let input_ref = input.as_ref();
    if input_ref.len() <= 6 {
        let mut ret = String::new();
        for byte in input_ref.iter() {
            unwrap!(write!(ret, "{:02x}", byte));
        }
        return ret;
    }
    format!("{:02x}{:02x}{:02x}..{:02x}{:02x}{:02x}",
            input_ref[0],
            input_ref[1],
            input_ref[2],
            input_ref[input_ref.len() - 3],
            input_ref[input_ref.len() - 2],
            input_ref[input_ref.len() - 1])
}

#[cfg(test)]
fn generate_random_bytes(size: usize) -> Vec<u8> {
    use rand;
    use rand::Rng;
    rand::thread_rng().gen_iter().take(size).collect()
}
