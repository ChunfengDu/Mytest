// Copyright (C) 2018-2020 Twitter, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![allow(unknown_lints)]
#![allow(clippy)]
#![allow(clippy_pedantic)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

// TODO(sean): fix the build script here so that it only generates the necessary bindings
// Note: This is a hack to get CI to pass, otherwise the warnings
// are so long that travis fails the build due to excessive output.
#![allow(improper_ctypes)]

include!(concat!(env!("OUT_DIR"), "/ffigen.rs"));
