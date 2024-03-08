# ABI STABLE RUST STRUCTS FOR REPRESENTING A SNIPPET CONFIG FILE

An ABI (Application Binary Interface) is used to define thing like the memory layout, size and order of fields within a composite type (language types composed of multiple other types like a struct, object, etc).
If a language has a stable ABI it makes it possible to import all the structs, enums, functions, etc of a library written and compiled in a given language at load time or runtime (AKA dynamically) of a program written and compiled in the same language.
if a binary interface is available between 2 programming languages it may allow the same in the other interfacable language.
This is ideal for a type of plugin system.
Rust does not have a standard stable ABI but it does have a number of stable ABIs as libraries.
This library defines ABI stable structs for representing a snippet config file like those for [VSCode](https://code.visualstudio.com/docs/editor/userdefinedsnippets#_create-your-own-snippets) and [Pulsar](https://pulsar-edit.dev/docs/launch-manual/sections/using-pulsar/sections/snippets.html#creating-your-own-snippets#snippet-format) (formerly Atom).
This library uses the library [stabby](https://docs.rs/crate/stabby/3.0.3) to establish a stable ABI for its exported structs.
