(module
  (memory $mem 1) ;; allocate one 64KB page of memory for $mem

  (func $indexForPosition (param $x i32) (param $y i32) (result i32)
        (i32.add
          (i32.mul
            (i32.const 8)
            (local.get $y))
          (local.get $x)))

  ;; Offset = ( x + y * 8 ) * 4
  ;; For each position on the board, we use a 32-bit integer(4 bytes) slot to store its value
  ;; And bc each memory slot is 1 byte => we need to * 4
  (func $offsetForPosition (param $x i32) (param $y i32) (result i32)
        (i32.mul
          (call $indexForPosition (local.get $x) (local.get $y))
          (i32.const 4)))

  ;; Representing the pieces
  ;; unused 24 bits ...00000000 <-> 0 <-> Empty square
  ;; unused 24 bits ...00000001 <-> 1 <-> Black
  ;; unused 24 bits ...00000010 <-> 2 <-> White
  ;; unused 24 bits ...00000100 <-> 4 <-> King

  (global $BLACK i32 (i32.const 1))
  (global $WHITE i32 (i32.const 2))
  (global $CROWN i32 (i32.const 4))

  (func $isCrowned (param $piece i32) (result i32)
        (i32.eq
          (i32.and (local.get $piece) (global.get $CROWN))
          (global.get $CROWN)))

  ;; Determine if a piece is white
  (func $isWhite (param $piece i32) (result i32)
        (i32.eq
          (i32.and (local.get $piece) (global.get $WHITE))
          (global.get $WHITE)))

  ;; Determine if a piece is black
  (func $isBlack (param $piece i32) (result i32)
        (i32.eq
          (i32.and (local.get $piece) (global.get $BLACK))
          (global.get $BLACK)))

  ;; Adds a crown to a given piece (no mutation)
  (func $withCrown (param $piece i32) (result i32)
        (i32.or (local.get $piece) (global.get $CROWN)))

  ;; Removes a crown from a given piece (no mutation)
  (func $withoutCrown (param $piece i32) (result i32)
        (i32.and (local.get $piece) (i32.const 3)))

  (func $setPiece (param $x i32) (param $y i32) (param $piece i32)
        (i32.store
          (call $offsetForPosition
                (local.get $x)
                (local.get $y))
          (local.get $piece)))

  (func $getPiece (param $x i32) (param $y i32) (result i32)
        (if (result i32) ;; this specify the return type of this if block
          (i32.and ;; check if x, and y in range
            (call $inRange
                  (i32.const 0)
                  (i32.const 7)
                  (local.get $x))
            (call $inRange
                  (i32.const 0)
                  (i32.const 7)
                  (local.get $y)))
          (then
            (i32.load
              (call $offsetForPosition
                    (local.get $x)
                    (local.get $y))))
          (else
            (unreachable))))

  ;; Detect if values are within range (inclusive high and low)
  (func $inRange (param $low i32)  (param $high i32) (param $value i32) (result i32)
        (i32.and
          (i32.ge_s (local.get $value) (local.get $low))
          (i32.le_s (local.get $value) (local.get $high))))

  (export "offsetForPosition" (func $offsetForPosition))
  (export "isCrowned" (func $isCrowned))
  (export "isWhite" (func $isWhite))
  (export "isBlack" (func $isBlack))
  (export "withCrown" (func $withCrown))
  (export "withoutCrown" (func $withoutCrown))
  (export "setPiece" (func $setPiece))
  (export "getPiece" (func $getPiece))
  (export "inRange" (func $inRange)))
