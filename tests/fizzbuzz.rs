(let fizzbuzz
  (fn (from to)
    (for i from to
      (cond
        ((&& (= (% i 3) 0) (= (% i 5) 0)) (write "FizzBuzz\n"))
        ((= (% i 3) 0) (write "Fizz\n"))
        ((= (% i 5) 0) (write "Buzz\n"))
        (otherwise (write "\n"))))))
