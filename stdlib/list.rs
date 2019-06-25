(let list (fn (x) (quote (x))))

(let car (fn (x) (head x)))
(let cdr (fn (x) (rest x)))

(let len (fn (x) (if (= x ()) 0 (+ (len (cdr x)) 1))))

(let list (fn (x) (cons x ()))) ;; NOTE: does not work with multiple elements
