(let min (fn (x y) (if (< x y) x y)))
(let max (fn (x y) (if (> x y) x y)))

(let null? (fn (x) (= x (quote ()))))
