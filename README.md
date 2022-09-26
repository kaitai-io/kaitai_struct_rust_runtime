# Kaitai Rust support (in development)

### Before each commit, make sure to run:

```sh
kaitai_rust# ./tests.sh && cargo test && ./clean.sh
```
## Add new test
Lets add new test called `expr_0.ksy`
- Taking file (usually) from `kaitai_struct_tests/formats`, lets add it to `kaitai_rust/formats` dir.
- if necessary, add `.bin`-file to `kaitai_rust/formats/bin` dir.
- create `kaitai_rust/tests/expr_0_tests.rs` test file, here is template:
```rust
#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use kaitai::*;

mod helpers_tests;
use helpers_tests::*;

mod expr_0;
use expr_0::*;

#[test]
fn basic_parse() {
    let bytes = get_file_as_byte_vec("formats/bin/str_encodings.bin");
    let reader = BytesReader::new(&bytes);

    let mut test = Expr0::default();
    {
        let res = test.read(&reader, None, KStructUnit::parent_stack());
        println!("{:?}", res);
        assert!(res.is_ok());
    }

    assert_eq!(10, test.len_of_1);
    // etc
}
```
- `./tests.sh && cargo test`
