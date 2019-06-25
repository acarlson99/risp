(load stdlib/logic.rs)
(load stdlib/list.rs)
(load stdlib/math.rs)

(let null?
  (fn (x)
    (= () x)))