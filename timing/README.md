# timing
Small util for timing code. 

There are other libs like this one, that in my opinion does too much - like defining global statics, using mutexes/interior mutability etc.
Such constructs can be added on top of this in your code, if you feel the need. (like not passing the benchmark around to functions which can be a bit intrusive), 

        let mut benchmark = BenchMark::new();

        benchmark.start("foo");
        std::thread::sleep(Duration::from_millis(200));
        benchmark.stop("foo");

        for _i in 0..10 {
            benchmark.start("bar");
            std::thread::sleep(Duration::from_millis(20));
            benchmark.stop("bar");

            // ..other stuff here

            benchmark.start("bar");
            std::thread::sleep(Duration::from_millis(20));
            benchmark.stop("bar");
        }

        {
            let _scope = benchmark.time_scope("scope");
            std::thread::sleep(Duration::from_millis(200));
        }

        for _i in 0..10 {
            let _scope_loop = benchmark.time_scope("scope_loop");
            std::thread::sleep(Duration::from_millis(20));
        }

        println!("{}", benchmark);

 will produce:
 
bar total: 475.893ms, mean: 23.794ms, samples: 20

scope_loop total: 239.745ms, mean: 23.974ms, samples: 10

scope total: 203.97ms, mean: 203.97ms, samples: 1

foo total: 203.178ms, mean: 203.178ms, samples: 1

