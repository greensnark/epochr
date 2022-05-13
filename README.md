# epoch

epoch prints the current time in epoch millis, or accepts an epoch time on the
command-line and prints the corresponding RFC3339 time.

For example:

```sh
$ epoch
1652400221740

$ epoch 1652400221740
2022-05-13T00:03:41.740+00:00

$ epoch 2022-05-13T00:03:41.740+00:00
1652400221740
```

`src/main.rs` is also directly runnable using [rust-script](https://crates.io/crates/rust-script)
