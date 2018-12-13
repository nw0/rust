// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use spec::{LinkerFlavor, Target, TargetResult};

pub fn target() -> TargetResult {
    let mut base = super::freebsd_base::opts();
    base.cpu = "cheri".to_string();
    base.max_atomic_width = Some(64);
    // base.pre_link_args.get_mut(&LinkerFlavor::Gcc).unwrap().push("-m64".to_string());
    // base.stack_probes = true;

    Ok(Target {
        llvm_target: "cheri-unknown-freebsd".to_string(),
        target_endian: "big".to_string(),
        target_pointer_width: "128".to_string(),
        target_c_int_width: "32".to_string(),
        data_layout: "E-m:m-pf200:128:128-i8:8:32-i16:16:32-i64:64-n32:64-S128".to_string(),
        arch: "cheri".to_string(),
        target_os: "freebsd".to_string(),
        target_env: String::new(),
        target_vendor: "unknown".to_string(),
        linker_flavor: LinkerFlavor::Gcc,
        options: base,
    })
}
