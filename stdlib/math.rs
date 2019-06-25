(let pow (fn (n e) (if (<= e 0) 1 (* (pow n (- e 1)) n))))
(let 1+ (fn (n) (+ n 1)))
(let -1+ (fn (n) (- n 1)))
