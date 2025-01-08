# cargo test versus cargo test-sbf discrepancy

```
$ cargo test-sbf
running 1 test
[2025-01-08T16:54:33.090643000Z INFO  solana_program_test] "hello_world" SBF program from /Users/alpharush/hello_world/target/deploy/hello_world.so, modified 2 weeks, 23 hours, 55 minutes, 546 ms, 595 µs and 29 ns ago
[2025-01-08T16:54:33.169148000Z DEBUG solana_runtime::message_processor::stable_log] Program 11111111111111111111111111111111 invoke [1]
[2025-01-08T16:54:33.169249000Z DEBUG solana_runtime::message_processor::stable_log] Program 11111111111111111111111111111111 success
[2025-01-08T16:54:33.173860000Z DEBUG solana_runtime::message_processor::stable_log] Program 1111111QLbz7JHiBTspS962RLKV8GndWFwiEaqKM invoke [1]
[2025-01-08T16:54:33.177595000Z DEBUG solana_runtime::message_processor::stable_log] Program 1111111QLbz7JHiBTspS962RLKV8GndWFwiEaqKM consumed 613 of 200000 compute units
[2025-01-08T16:54:33.177653000Z DEBUG solana_runtime::message_processor::stable_log] Program 1111111QLbz7JHiBTspS962RLKV8GndWFwiEaqKM success
test test::test_hello_world ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.14s

   Doc-tests hello_world

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

```
$ cargo test
running 1 test
[2025-01-08T16:55:44.748601000Z INFO  solana_program_test] "hello_world" builtin program
[2025-01-08T16:55:44.823555000Z DEBUG solana_runtime::message_processor::stable_log] Program 11111111111111111111111111111111 invoke [1]
[2025-01-08T16:55:44.823834000Z DEBUG solana_runtime::message_processor::stable_log] Program 11111111111111111111111111111111 success
[2025-01-08T16:55:44.825603000Z DEBUG solana_runtime::message_processor::stable_log] Program 1111111QLbz7JHiBTspS962RLKV8GndWFwiEaqKM invoke [1]
[2025-01-08T16:55:44.825628000Z DEBUG solana_runtime::message_processor::stable_log] Program 1111111QLbz7JHiBTspS962RLKV8GndWFwiEaqKM invoke [1]
[2025-01-08T16:55:44.826093000Z DEBUG solana_runtime::message_processor::stable_log] Program 1111111QLbz7JHiBTspS962RLKV8GndWFwiEaqKM success
[2025-01-08T16:55:44.826113000Z DEBUG solana_runtime::message_processor::stable_log] Program 1111111QLbz7JHiBTspS962RLKV8GndWFwiEaqKM success
test test::test_hello_world ... FAILED

failures:

---- test::test_hello_world stdout ----
1
thread 'test::test_hello_world' panicked at src/lib.rs:82:9:
assertion `left == right` failed
  left: 0
 right: 165
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    test::test_hello_world

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.09s