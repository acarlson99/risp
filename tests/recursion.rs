(mod recursion
  
  (let factorial
    (fn (n) (if (<= n 1) n (* n (factorial (- n 1))))))

  (let fibonacci
    (fn (n) (if (<= n 2) (- n 1) (+ (fibonacci (- n 1)) (fibonacci (- n 2))))))

)
