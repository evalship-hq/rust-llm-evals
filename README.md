# LLM evals for Rust

A Rust support bot whose AI call is tested on every pull request with [tinyeval](https://tinyeval.com).

## Run it

Clone the repo, or use it as a template. Then:

```sh
export ANTHROPIC_API_KEY=sk-ant-...
cargo build --release   # the first build takes a while; later runs start fast
./tinyeval
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
- `src/bin/evalworker.rs`: the tinyeval worker. It builds `Support` once, then answers each test case.
- `evals/support.sh`: three test cases and the checks on their replies.
- `tinyeval`: the eval runner, one bash file.
- `.github/workflows/evals.yml`: runs the evals on pull requests and on pushes to main.

## Use it for your app

- Replace `answer` with your AI call, and keep the worker so your app starts once, not once per test case.
- The worker reports a missing key, 401, 429, 5xx and network errors as couldn't run, so an outage doesn't show as a failed eval. Keep that split when you swap in your own client.
- Write your test cases in `evals/`. The [check lines](https://github.com/evalship-hq/tinyeval#writing-evals) are `contains`, `matches`, `json`, `judge` and a few more.
- Build your clients once in the worker's `main`. An axum or actix service can call its handler logic directly, without a running server.

## In CI

Add `ANTHROPIC_API_KEY` under **Settings** > **Secrets and variables** > **Actions**. The judge uses the same key.

The workflow builds the worker, then runs each test case 3 times. A test case passes when at least 2 of the 3 runs pass. A failing eval fails the job, so you can require it before merging. The results go up as the `eval-results` artifact, also when evals fail.

Without the secret, the workflow skips the evals and stays green. Pull requests from forks get no secrets, so their evals skip too.

## Compare pull requests with main

A fresh CI run has nothing to compare with. [evalship](https://evalship.com) reads `eval-results.json`, compares each pull request with main, tells flaky evals from real drops and comments on the pull request.

## License

MIT, see [LICENSE](LICENSE).
