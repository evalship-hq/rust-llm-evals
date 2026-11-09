// tinyeval worker: evals/support.sh starts it with `worker cargo run -q --release --bin evalworker`.
// It reads each input up to a NUL byte on stdin, and prints the reply and a NUL byte.
use std::io::{self, BufRead, Write};
use support::{Error, Support};

fn main() -> io::Result<()> {
    let support = Support::new(); // once per worker, not once per test case
    let mut stdout = io::stdout().lock();
    for input in io::stdin().lock().split(0) {
        let reply = match support.answer(&String::from_utf8_lossy(&input?)) {
            Ok(reply) => reply,
            // Not the app's fault, so tinyeval counts it as couldn't run instead of failed.
            Err(e @ (Error::NoKey | Error::Network(_) | Error::Api { status: 401..=403 | 408 | 429 | 500.., .. })) => {
                format!("#tinyeval couldnt_run {}\n", e.to_string().replace('\n', " "))
            }
            // A 400 or 404 comes from the request the app built: grade it as the reply.
            Err(e) => format!("error: {e}"),
        };
        stdout.write_all(reply.as_bytes())?;
        stdout.write_all(b"\0")?;
        stdout.flush()?;
    }
    Ok(())
}
