compile wat -> wasm
`wat2wasm add1.wat -o add1.wasm`

compile wasm -> wat
`wasm2wat add1.wasm -o add1_roundtrip.wat`

compile wasm -> c
`wasm2c add1.wasm -o add1_roundtrip.c`
