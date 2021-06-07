(module
  (type (;0;) (func (param i32 i32) (result i32)))
  (type (;0;) (func (param i32) (result i32)))
  (func $min (type 0) (param i32 i32) (result i32)
   (if (result i32)
    (i32.lt_s
      (local.get 0)
      (local.get 1)
    )
    (then
      (local.get 0)
    )
    (else
      (local.get 1)
    )
   )
  )

  (func $fibr (type 1) (param i32) (result i32)
    (if (result i32)
      (i32.eqz (local.get 0))
      (then
        (i32.const 0)
      )
      (else
        (if (result i32)
          (i32.eq
            (local.get 0)
            (i32.const 1)
          )
          (then
            (i32.const 1)
          )
          (else
            (i32.add
              (call $fibr
                (i32.sub
                  (local.get 0)
                  (i32.const 1)
                )
              )
              (call $fibr
                (i32.sub
                  (local.get 0)
                  (i32.const 2)
                )
              )
            )
          )
        )
      )
    )
  )

  (func $fibi (type 1) (param i32) (result i32) (local i32 i32 i32)
    (local.set 1
      (i32.const 0)
    )
    (local.set 2
      (i32.const 1)
    )
    (block (result i32)
      (loop
        (if
          (i32.eqz
            (local.get 0)
          )
          (then
            (local.get 1)
            (br 2)
          )
          (else
            (local.set 3
              (local.get 1)
            )
            (local.set 2
              (i32.add
                (local.get 1)
                (local.get 2)
              )
            )
            (local.set 1
              (local.get 3)
            )
            (local.set 0
              (i32.sub
                (local.get 0)
                (i32.const 1)
              )
            )
            (br 0)
          )
        )
      )
      (unreachable)
    )
  )
  (export "min" (func $min))
  (export "fibr" (func $fibr))
  (export "fibi" (func $fibi))
)
