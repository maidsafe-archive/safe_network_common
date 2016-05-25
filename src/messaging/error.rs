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

use maidsafe_utilities::serialisation::SerialisationError;

quick_error! {
    /// Error types relating to MPID messaging.
    #[derive(Debug)]
    pub enum Error {
        /// Used where the length of a [header's `metadata`](struct.MpidHeader.html#method.new)
        /// exceeds [`MAX_HEADER_METADATA_SIZE`](constant.MAX_HEADER_METADATA_SIZE.html).
        MetadataTooLarge {
            description("Header too large")
            display("Message header too large")
        }
        /// Used where the length of a [message's `body`](struct.MpidMessage.html#method.new)
        /// exceeds [`MAX_BODY_SIZE`](constant.MAX_BODY_SIZE.html).
        BodyTooLarge {
            description("Body too large")
            display("Message body too large")
        }
        /// Serialisation error.
        Serialisation(error: SerialisationError) {
            cause(error)
            description(error.description())
            display("Serialisation error: {}", error)
            from()
        }
    }
}
