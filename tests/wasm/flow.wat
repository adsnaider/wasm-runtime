(module
  (func (result i32)
   (if (result i32)
    (i32.lt_s
     (;(get_local $x);)
     (i32.const 11)
     (i32.const 10)
    )
    (then
     (i32.const 11)
    )
    (else
     (i32.const 10)
    )
   )
  )
  (start 0)
)
