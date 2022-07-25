meta:
  id: test_parent
seq:
  - id: root_byte
    type: u1
  - id: child
    type: child_struct
types:
  child_struct:
    seq:
      - id: child_byte
        type: u1
      - id: child2
        type: child2_struct
        repeat: expr
        repeat-expr: _root.root_byte
    types:
      child2_struct:
        seq:
          - id: child2_byte
            type: u1
            repeat: expr
            repeat-expr: _parent.child_byte

