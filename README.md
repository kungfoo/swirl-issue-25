# Swirl Issue #25 Repro Case

## How to run it

- Spin up a database for example using using `docker-compose up -d`
- Run `diesel migration run`
- Run `cargo run --bin job-runner`
- Run `cargo run --bin submit-job`
- Observe output of `job-runner`

## Expected behaviour

Both `cargo run` and `cargo run --release` should produce the same output.

## Observed behaviour

When running the `--release` binary of `job-runner`, one can observe the following output:

```sh
jobs::images] scale_image: from: file:///var/swirl-sample/images/c73f6708-5942-4eab-844c-dd00eaa4ff51.jpeg, to: file:///var/swirl-sample/images/scaled/c73f6708-5942-4eab-844c-dd00eaa4ff51.jpeg, dimension: Dimension { width: 400, height: 1200 }
```