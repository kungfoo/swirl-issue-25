# Swirl Issue #25 Repro Case

## How to reproduce

- Spin up a database for example using using `docker-compose up -d`
- Run `cd db && diesel migration run`
- Run `cargo run --bin job-runner`
- Run `cargo run --bin submit-job`
- Observe output of `job-runner`, it works as expected.

Then:

- Run `cargo run --release --bin job-runner`
- Run `cargo run --release --bin submit-job`
- Observe output of `job-runner`, it logs an error 

## Expected behaviour

Both `cargo run` and `cargo run --release` should produce the same output.

## Observed behaviour

When running the `--release` binary of `job-runner`, one can observe the following output:

```sh
Job ${n} failed to run: Unknown job type scale_image
```