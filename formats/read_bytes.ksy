meta:
  id: read_bytes
seq:
  - id: len
    type: u1
  - id: val
    size: len
  - id: padding
    size: 2
