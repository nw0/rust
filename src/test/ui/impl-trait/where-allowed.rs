// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A simple test for testing many permutations of allowedness of
//! impl Trait
use std::fmt::Debug;

// Allowed
fn in_parameters(_: impl Debug) { panic!() }

// Allowed
fn in_return() -> impl Debug { panic!() }

// Allowed
fn in_adt_in_parameters(_: Vec<impl Debug>) { panic!() }

// Allowed
fn in_adt_in_return() -> Vec<impl Debug> { panic!() }

// Disallowed
fn in_fn_parameter_in_parameters(_: fn(impl Debug)) { panic!() }
//~^ ERROR `impl Trait` not allowed outside of function and inherent method return types

// Disallowed
fn in_fn_return_in_parameters(_: fn() -> impl Debug) { panic!() }
//~^ ERROR `impl Trait` not allowed outside of function and inherent method return types

// Disallowed
fn in_fn_parameter_in_return() -> fn(impl Debug) { panic!() }
//~^ ERROR `impl Trait` not allowed outside of function and inherent method return types

// Disallowed
fn in_fn_return_in_return() -> fn() -> impl Debug { panic!() }
//~^ ERROR `impl Trait` not allowed outside of function and inherent method return types

// Disallowed
fn in_dyn_Fn_parameter_in_parameters(_: &dyn Fn(impl Debug)) { panic!() }
//~^ ERROR `impl Trait` not allowed outside of function and inherent method return types

// Disallowed
fn in_dyn_Fn_return_in_parameters(_: &dyn Fn() -> impl Debug) { panic!() }
//~^ ERROR `impl Trait` not allowed outside of function and inherent method return types

// Disallowed
fn in_dyn_Fn_parameter_in_return() -> &'static dyn Fn(impl Debug) { panic!() }
//~^ ERROR `impl Trait` not allowed outside of function and inherent method return types

// Disallowed
fn in_dyn_Fn_return_in_return() -> &'static dyn Fn() -> impl Debug { panic!() }
//~^ ERROR `impl Trait` not allowed outside of function and inherent method return types

// Disallowed
fn in_impl_Fn_parameter_in_parameters(_: &impl Fn(impl Debug)) { panic!() }
//~^ ERROR `impl Trait` not allowed outside of function and inherent method return types
//~^^ ERROR nested `impl Trait` is not allowed

// Disallowed
fn in_impl_Fn_return_in_parameters(_: &impl Fn() -> impl Debug) { panic!() }
//~^ ERROR `impl Trait` not allowed outside of function and inherent method return types

// Disallowed
fn in_impl_Fn_parameter_in_return() -> &'static impl Fn(impl Debug) { panic!() }
//~^ ERROR `impl Trait` not allowed outside of function and inherent method return types
//~^^ ERROR nested `impl Trait` is not allowed

// Disallowed
fn in_impl_Fn_return_in_return() -> &'static impl Fn() -> impl Debug { panic!() }
//~^ ERROR `impl Trait` not allowed outside of function and inherent method return types

// Disallowed
fn in_Fn_parameter_in_generics<F: Fn(impl Debug)> (_: F) { panic!() }
//~^ ERROR `impl Trait` not allowed outside of function and inherent method return types

// Disallowed
fn in_Fn_return_in_generics<F: Fn() -> impl Debug> (_: F) { panic!() }
//~^ ERROR `impl Trait` not allowed outside of function and inherent method return types


// Allowed
fn in_impl_Trait_in_parameters(_: impl Iterator<Item = impl Iterator>) { panic!() }

// Allowed
fn in_impl_Trait_in_return() -> impl IntoIterator<Item = impl IntoIterator> {
    vec![vec![0; 10], vec![12; 7], vec![8; 3]]
}

// Disallowed
struct InBraceStructField { x: impl Debug }
//~^ ERROR `impl Trait` not allowed outside of function and inherent method return types

// Disallowed
struct InAdtInBraceStructField { x: Vec<impl Debug> }
//~^ ERROR `impl Trait` not allowed outside of function and inherent method return types

// Disallowed
struct InTupleStructField(impl Debug);
//~^ ERROR `impl Trait` not allowed outside of function and inherent method return types

// Disallowed
enum InEnum {
    InBraceVariant { x: impl Debug },
    //~^ ERROR `impl Trait` not allowed outside of function and inherent method return types
    InTupleVariant(impl Debug),
    //~^ ERROR `impl Trait` not allowed outside of function and inherent method return types
}

// Allowed
trait InTraitDefnParameters {
    fn in_parameters(_: impl Debug);
}

// Disallowed
trait InTraitDefnReturn {
    fn in_return() -> impl Debug;
    //~^ ERROR `impl Trait` not allowed outside of function and inherent method return types
}

// Allowed and disallowed in trait impls
trait DummyTrait {
    type Out;
    fn in_trait_impl_parameter(_: impl Debug);
    fn in_trait_impl_return() -> Self::Out;
}
impl DummyTrait for () {
    type Out = impl Debug;
    //~^ ERROR `impl Trait` not allowed outside of function and inherent method return types

    fn in_trait_impl_parameter(_: impl Debug) { }
    // Allowed

    fn in_trait_impl_return() -> impl Debug { () }
    //~^ ERROR `impl Trait` not allowed outside of function and inherent method return types
}

// Allowed
struct DummyType;
impl DummyType {
    fn in_inherent_impl_parameters(_: impl Debug) { }
    fn in_inherent_impl_return() -> impl Debug { () }
}

// Disallowed
extern "C" {
    fn in_foreign_parameters(_: impl Debug);
    //~^ ERROR `impl Trait` not allowed outside of function and inherent method return types

    fn in_foreign_return() -> impl Debug;
    //~^ ERROR `impl Trait` not allowed outside of function and inherent method return types
}

// Allowed
extern "C" fn in_extern_fn_parameters(_: impl Debug) {
}

// Allowed
extern "C" fn in_extern_fn_return() -> impl Debug {
    22
}

type InTypeAlias<R> = impl Debug;
//~^ ERROR `impl Trait` not allowed outside of function and inherent method return types

type InReturnInTypeAlias<R> = fn() -> impl Debug;
//~^ ERROR `impl Trait` not allowed outside of function and inherent method return types

// Disallowed in impl headers
impl PartialEq<impl Debug> for () {
    //~^ ERROR `impl Trait` not allowed outside of function and inherent method return types
}

// Disallowed in impl headers
impl PartialEq<()> for impl Debug {
    //~^ ERROR `impl Trait` not allowed outside of function and inherent method return types
}

// Disallowed in inherent impls
impl impl Debug {
    //~^ ERROR `impl Trait` not allowed outside of function and inherent method return types
}

// Disallowed in inherent impls
struct InInherentImplAdt<T> { t: T }
impl InInherentImplAdt<impl Debug> {
    //~^ ERROR `impl Trait` not allowed outside of function and inherent method return types
}

// Disallowed in where clauses
fn in_fn_where_clause()
    where impl Debug: Debug
//~^ ERROR `impl Trait` not allowed outside of function and inherent method return types
{
}

// Disallowed in where clauses
fn in_adt_in_fn_where_clause()
    where Vec<impl Debug>: Debug
//~^ ERROR `impl Trait` not allowed outside of function and inherent method return types
{
}

// Disallowed
fn in_trait_parameter_in_fn_where_clause<T>()
    where T: PartialEq<impl Debug>
//~^ ERROR `impl Trait` not allowed outside of function and inherent method return types
{
}

// Disallowed
fn in_Fn_parameter_in_fn_where_clause<T>()
    where T: Fn(impl Debug)
//~^ ERROR `impl Trait` not allowed outside of function and inherent method return types
{
}

// Disallowed
fn in_Fn_return_in_fn_where_clause<T>()
    where T: Fn() -> impl Debug
//~^ ERROR `impl Trait` not allowed outside of function and inherent method return types
{
}

fn main() {
    let _in_local_variable: impl Fn() = || {};
    //~^ ERROR `impl Trait` not allowed outside of function and inherent method return types
    let _in_return_in_local_variable = || -> impl Fn() { || {} };
    //~^ ERROR `impl Trait` not allowed outside of function and inherent method return types
}
