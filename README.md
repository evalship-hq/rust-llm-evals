# LLM evals for Rust

A Rust support bot whose AI call is tested on every pull request with [tinyeval](https://tinyeval.com).

## Run it

Clone the repo, or use it as a template. Then:

```sh
export ANTHROPIC_API_KEY=sk-ant-...
cargo install tinyeval                       # the tinyeval command
cargo build --release --example evalworker   # the first build takes a while; later runs start fast
tinyeval
```

```
support
  ✓ refund after the window 5.2s
  ✓ missing package 4.3s
  ✓ refund inside the window 4.4s

All 3 evals passed (eval-results.json)
```

## What's here

- `src/lib.rs`: the support bot. `Support::answer` sends a message to Claude and returns the reply. Rust has no official Anthropic SDK, so it calls the Messages API with `ureq` and `serde_json`.
- `prompts/support.md`: the bot's system prompt.
- `examples/evalworker.rs`: the tinyeval worker. It builds `Support` once, then answers each test case with the `tinyeval` crate's worker helper.
- `evals/support.sh`: three test cases and the checks on their replies.
- `.github/workflows/evals.yml`: runs the evals on pull requests and on pushes to main.

## Use it for your app

- Replace `answer` with your AI call, and keep the worker so your app starts once, not once per test case.
- The worker returns `CouldntRun` for a missing key, 401, 429, 5xx and network errors, so an outage doesn't show as a failed eval. Keep that split when you swap in your own client.
- Write your test cases in `evals/`. The [check lines](https://github.com/evalship-hq/tinyeval#writing-evals) are `contains`, `matches`, `json`, `judge` and a few more.
- Set up your clients in the worker's `LazyLock`, so nothing they print mixes into a reply. An axum or actix service can call its handler logic directly, without a running server.

## In CI

Add `ANTHROPIC_API_KEY` under **Settings** > **Secrets and variables** > **Actions**. The judge uses the same key.

The workflow builds the worker and installs the `tinyeval` command at the version in `Cargo.lock`. Then it runs each test case 3 times. A test case passes when at least 2 of the 3 runs pass. A failing eval fails the job, so you can require it before merging. The results go up as the `eval-results` artifact, also when evals fail.

Without the secret, the workflow skips the evals and stays green. Pull requests from forks get no secrets, so their evals skip too. Dependabot's pull requests get none either, so their evals skip with the same notice.

## Updating

Dependabot opens a pull request each week when tinyeval, a crate or an action has a new release. To update tinyeval yourself:

```sh
cargo add --dev tinyeval   # the worker helper, in Cargo.toml and Cargo.lock
cargo install tinyeval     # the command
```

`cargo update` alone won't move tinyeval to a new 0.x minor version.

## Compare pull requests with main

A fresh CI run has nothing to compare with. [evalship](https://evalship.com) reads `eval-results.json`, compares each pull request with main, tells flaky evals from real drops and comments on the pull request.

## License

MIT, see [LICENSE](LICENSE).
