# runtime-struct-field-names-as-array

[![crate-name at crates.io](https://img.shields.io/crates/v/runtime_struct_field_names_as_array.svg)](https://crates.io/crates/runtime_struct_field_names_as_array)
[![crate-name at docs.rs](https://docs.rs/runtime_struct_field_names_as_array/badge.svg)](https://docs.rs/runtime_struct_field_names_as_array)
[![Rust](https://github.com/0cv/runtime_struct_field_names_as_array/actions/workflows/rust.yml/badge.svg)](https://github.com/0cv/runtime_struct_field_names_as_array/actions/workflows/rust.yml)


Provides the `FieldNamesAsArray` procedural macro.
The macro adds the fn `field_names_as_array()` to the struct the macro is derived on. It contains the field names of the given 
struct, including the parents

**Note:** The macro can only be derived from named structs. 

**IMPORTANT** This crate has a runtime overhead while it has limited options. If you do NOT intend to use it on a nested struct, you shall use this [crate](https://github.com/jofas/struct_field_names_as_array) instead. See [discussion](https://github.com/jofas/struct_field_names_as_array/issues/4)

## Table of Contents

<!--ts-->
   * [Usage](#usage)
   * [Attributes](#attributes)
      * [Field Attributes](#field-attributes)
         * [Flatten](#flatten)
<!--te-->

## Usage

You can derive the `FieldNamesAsArray` macro like this:

```rust
use runtime_struct_field_names_as_array::FieldNamesAsArray;

#[derive(FieldNamesAsArray)]
struct Foo {
  bar: String,
  baz: String,
  bat: String,
}

assert_eq!(Foo::field_names_as_array(), ["bar", "baz", "bat"]);
```

## Attributes

The `FieldNamesAsArray` macro supports the
`field_names_as_array` attribute.
`field_names_as_array` can be applied to a field with only the `flatten` attribute

### Container Attributes

Container attributes are global attributes that change the behavior
of the whole field names array, rather than that of a single field.

### Field Attributes

Field attributes can be added to the fields of a named struct and 
change the behavior of a single field.

#### Flatten

The `flatten` attribute will add the parent fields. Option struct are also supported. If the attribute is not added on a struct type, it will be considered as a regular field.

```rust
use runtime_struct_field_names_as_array::FieldNamesAsArray;

#[derive(FieldNamesAsArray)]
struct Parent {
  foo: String,
}

#[derive(FieldNamesAsArray)]
struct Foo {
  bar: String,
  baz: String,
  #[field_names_as_array(flatten)]
  parent: Parent,
  #[field_names_as_array(flatten)]
  parent_option: Option<Parent>,
  another_parent: Parent,
}

assert_eq!(Foo::field_names_as_array(), ["bar", "baz", "parent.foo", "parent_option.foo", "another_parent"]);
```
