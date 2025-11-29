# FeLISP

a toy lisp interpreter, following the blueprint of kanaka/mal

## Build & Run

```bash
cargo build
cargo run
```

## Examples

```lisp
(+ 1 2)
(let* (x 1 y 2) (+ x y))
(fn* (x) (* x x))
(def! square (fn* (x) (* x x)))
((fn* (x y) (+ (square x) (square y))) 3 4)
(def! abs (fn* (n) (if (<= 0 n) n (- 0 n))))
(abs -1)
(def! fact (fn* (n) (if (<= n 0) 1 (* n (fact (- n 1))))))
(fact 6)
```

## TODO

- strings
- list operations
- files
- macros
- exceptions

