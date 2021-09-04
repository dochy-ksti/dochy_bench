Please execute the command below

cargo test -- --nocapture --test-threads 1

you can comment out or restore #[test] of the functions and see the results.

Dochy creates caches on saving, so if you want to benchmark loading, you shouldn't do save at once.