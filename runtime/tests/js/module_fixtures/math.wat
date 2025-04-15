;; Convert this WAT file to `math.wasm` using https://webassembly.github.io/wabt/demo/wat2wasm/
;; See https://github.com/webassembly/wabt
(module
  (func (export "add") (param $a i32) (param $b i32) (result i32)
    local.get $a
    local.get $b
    i32.add
  )
)
