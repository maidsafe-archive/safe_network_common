// Copyright 2015 MaidSafe.net limited.
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

quick_error! {
    /// Errors in Get (non-mutating) operations involving Core and Vaults
    #[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, RustcEncodable, RustcDecodable)]
    pub enum GetError {
        /// SAFE Account does not exist for client
        NoSuchAccount {
            description("No such account")
            display("SAFE Account does not exist for client")
        }
        /// Requested data not found
        NoSuchData {
            description("No such data")
            display("Requested data not found")
        }
        /// Network error occurring at Vault level which has no bearing on clients, e.g.
        /// serialisation failure or database failure
        NetworkOther(error: String) {
            description(error)
            display("Error on Vault network: {}", error)
        }
    }
}

quick_error! {
    /// Errors in Put/Post/Delete (mutating) operations involving Core and Vaults
    #[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, RustcEncodable, RustcDecodable)]
    pub enum MutationError {
        /// SAFE Account does not exist for client
        NoSuchAccount {
            description("No such account")
            display("Account does not exist for client")
        }
        /// Attempt to take an account network name that already exists
        AccountExists {
            description("Account exists")
            display("Account already exists for client")
        }
        /// Requested data not found
        NoSuchData {
            description("No such data")
            display("Requested data not found")
        }
        /// Attempt to create a mutable data when data with such a name already exists
        DataExists {
            description("Data exists")
            display("Data given already exists")
        }
        /// Insufficient balance for performing a given mutating operation
        LowBalance {
            description("Low account balance")
            display("Insufficient account balance for this operation")
        }
        /// Invalid successor for performing a given mutating operation, e.g. signature mismatch or
        /// invalid data versioning
        InvalidSuccessor {
            description("Invalid data successor")
            display("Data given is not a valid successor of stored data")
        }
        /// Invalid Operation such as a POST on ImmutableData
        InvalidOperation {
            description("Invalid operation")
            display("Requested operation is not allowed")
        }
        /// The loss of sacrificial copies indicates the network as a whole is no longer having
        /// enough space to accept further put request so have to wait for more nodes to join
        NetworkFull {
            description("Network full")
            display("Network cannot store any further data")
        }
        /// Network error occurring at Vault level which has no bearing on clients, e.g.
        /// serialisation failure or database failure
        NetworkOther(error: String) {
            description(error)
            display("Error on Vault network: {}", error)
        }
    }
}
