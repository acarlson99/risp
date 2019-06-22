(mod arithmetic

  (let factorial
    (fn (n) (if (< n 2) n (* n (factorial (- n 1)))))))
