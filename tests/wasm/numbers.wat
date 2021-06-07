(module
  (type (;0;) (func (param i32 i32) (result i32)))
  (type (;0;) (func (param f32 f32) (result f32)))
  (func $add (type 0) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    i32.add
  )

  (func $sub (type 0) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    i32.sub
  )

  (func $mul (type 0) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    i32.mul
  )

  (func $fadd (type 1) (param f32 f32) (result f32)
    local.get 0
    local.get 1
    f32.add
  )

  (func $fsub (type 1) (param f32 f32) (result f32)
    local.get 0
    local.get 1
    f32.sub
  )

  (func $fmul (type 1) (param f32 f32) (result f32)
    local.get 0
    local.get 1
    f32.mul
  )

  (export "add" (func $add))
  (export "sub" (func $sub))
  (export "mul" (func $mul))
  (export "fadd" (func $fadd))
  (export "fsub" (func $fsub))
  (export "fmul" (func $fmul))
)
