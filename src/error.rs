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

quick_error! {
    /// Error types.
    #[derive(Debug)]
    pub enum Error {
        /// Wrapper for a `::std::env::VarError`
        Env(err: ::std::env::VarError) {
            description("Environment error")
            display("Environment error: {}", err)
            cause(err)
            from()
        }
        /// Wrapper for a `::std::io::Error`
        Io(err: ::std::io::Error) {
            description("IO error")
            display("IO error: {}", err)
            cause(err)
            from()
        }
        /// Wrapper for a `::rustc_serialize::json::DecoderError`
        JsonDecoder(err: ::rustc_serialize::json::DecoderError) {
            description("Json decoder error")
            display("Json decoder error: {}", err)
            cause(err)
            from()
        }
        /// Wrapper for a `::rustc_serialize::json::EncoderError`
        JsonEncoder(err: ::rustc_serialize::json::EncoderError) {
            description("Json encoder error")
            display("Json encoder error: {}", err)
            cause(err)
            from()
        }
        /// Wrapper for a `rustc_serialize::json::ParserError`
        JsonParser(err: ::rustc_serialize::json::ParserError) {
            description("Json parse error")
            display("Json parse error: {}", err)
            cause(err)
            from()
        }
    }
}
