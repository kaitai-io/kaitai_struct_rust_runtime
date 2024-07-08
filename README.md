# Kaitai Rust support (in development)

### Currently not supported (TODO):
- `parent rewrite`
- `KStructUnit`
- `AnyType`

### Only 4 tests from [kaitai_struct_tests](https://github.com/Agile86/kaitai_struct_tests/tree/master/formats) not supported:
- `nav_parent_switch_cast.ksy` - common type have different parents, KStructUnit is parent.
- `params_pass_array_struct.ksy` - Instance, that return array of KStructUnit
- `params_pass_struct.ksy` - Params, that receive any struct (KStructUnit)
- `process_coerce_switch.ksy` - Instance "buf", that return AnyType
