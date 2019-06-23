(mod prelude
  (load stdlib/arithmetic.rs)
  (load stdlib/logic.rs)

  (let repeat
    (fn (from to f)
      (if (< from (- to 1))
        (do
          (f)
          (repeat (+ from 1) to f))
        (f))))
)
