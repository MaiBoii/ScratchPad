import rust

rust_struct = rust.RustStruct(data="some data", vector=[255, 255, 255])
rust_struct.extend_vector([1, 1, 1, 1])
rust_struct.printer()
#type(rust_struct)