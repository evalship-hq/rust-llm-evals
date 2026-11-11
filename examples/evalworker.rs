// tinyeval worker: evals/support.sh starts it with `worker cargo run -q --release --example evalworker`.
use std::sync::LazyLock;
use support::{Error, Support};
use tinyeval::CouldntRun;

// Built on the first input, once per worker, not once per test case.
static SUPPORT: LazyLock<Support> = LazyLock::new(Support::new);

fn main() -> std::io::Result<()> {
    tinyeval::worker(|input| match SUPPORT.answer(&input) {
        Ok(reply) => Ok(reply),
        // Not the app's fault, so tinyeval counts it as couldn't run instead of failed.
        Err(e @ (Error::NoKey | Error::Network(_) | Error::Api { status: 401..=403 | 408 | 429 | 500.., .. })) => {
            Err(CouldntRun(e.to_string()).into())
        }
        // A 400 or 404 comes from the request the app built: grade it as the reply.
        Err(e) => Err(e.into()),
    })
}
