use std::fmt::Display;
use std::future::Future;
use std::sync::atomic::{AtomicBool, Ordering};

use tracing::{event, field, span, Instrument, Level, Span};

static TELEMETRY_ENABLED: AtomicBool = AtomicBool::new(false);

pub fn set_enabled(enabled: bool) {
    TELEMETRY_ENABLED.store(enabled, Ordering::Relaxed);
}

pub fn is_enabled() -> bool {
    TELEMETRY_ENABLED.load(Ordering::Relaxed)
}

pub fn new_span(name: &str, attrs: &[(&str, String)]) -> Span {
    if !is_enabled() {
        return Span::none();
    }

    let span = span!(
        target: "rpsn.telemetry",
        Level::INFO,
        "rpsn.span",
        otel.name = field::Empty,
        otel.status_code = field::Empty,
        otel.status_message = field::Empty,
        "cli.command" = field::Empty,
        "cli.args" = field::Empty,
        "command.group" = field::Empty,
        "op.phase" = field::Empty,
        cwd = field::Empty,
        input_path = field::Empty,
        "http.method" = field::Empty,
        "http.endpoint" = field::Empty,
        "http.status_code" = field::Empty,
        "payload.kind" = field::Empty
    );

    set_span_attr(&span, "otel.name", name);
    for (key, value) in attrs {
        set_span_attr(&span, key, value);
    }

    span
}

pub fn set_span_attr(span: &Span, key: &str, value: impl Display) {
    if !is_enabled() {
        return;
    }

    match key {
        "otel.name" => {
            span.record("otel.name", field::display(value));
        }
        "otel.status_code" => {
            span.record("otel.status_code", field::display(value));
        }
        "otel.status_message" => {
            span.record("otel.status_message", field::display(value));
        }
        "cli.command" => {
            span.record("cli.command", field::display(value));
        }
        "cli.args" => {
            span.record("cli.args", field::display(value));
        }
        "command.group" => {
            span.record("command.group", field::display(value));
        }
        "op.phase" => {
            span.record("op.phase", field::display(value));
        }
        "cwd" => {
            span.record("cwd", field::display(value));
        }
        "input_path" => {
            span.record("input_path", field::display(value));
        }
        "http.method" => {
            span.record("http.method", field::display(value));
        }
        "http.endpoint" => {
            span.record("http.endpoint", field::display(value));
        }
        "http.status_code" => {
            span.record("http.status_code", field::display(value));
        }
        "payload.kind" => {
            span.record("payload.kind", field::display(value));
        }
        _ => {}
    }
}

pub fn mark_span_error(span: &Span, message: impl Display) {
    if !is_enabled() {
        return;
    }

    let msg = message.to_string();
    set_span_attr(span, "otel.status_code", "ERROR");
    set_span_attr(span, "otel.status_message", &msg);
    event!(target: "rpsn.telemetry", parent: span, Level::ERROR, error.message = %msg, "span error");
}

pub fn with_span<T>(name: &str, attrs: &[(&str, String)], f: impl FnOnce() -> T) -> T {
    if !is_enabled() {
        return f();
    }

    let span = new_span(name, attrs);
    let _entered = span.enter();
    f()
}

pub fn with_span_result<T, E>(
    name: &str,
    attrs: &[(&str, String)],
    f: impl FnOnce() -> Result<T, E>,
) -> Result<T, E>
where
    E: Display,
{
    if !is_enabled() {
        return f();
    }

    let span = new_span(name, attrs);
    let _entered = span.enter();
    let result = f();

    if let Err(err) = &result {
        mark_span_error(&span, err);
    }

    result
}

pub async fn with_span_async_result<T, E, Fut>(
    name: &str,
    attrs: &[(&str, String)],
    f: impl FnOnce() -> Fut,
) -> Result<T, E>
where
    E: Display,
    Fut: Future<Output = Result<T, E>>,
{
    if !is_enabled() {
        return f().await;
    }

    let span = new_span(name, attrs);
    let result = f().instrument(span.clone()).await;

    if let Err(err) = &result {
        mark_span_error(&span, err);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use once_cell::sync::Lazy;
    use std::sync::Mutex;

    static TEST_LOCK: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

    #[test]
    fn with_span_result_passthrough_when_disabled() {
        let _guard = TEST_LOCK.lock().expect("lock telemetry tests");
        set_enabled(false);

        let result = with_span_result("test.span", &[], || Ok::<_, anyhow::Error>(42));

        assert_eq!(result.unwrap(), 42);
    }

    #[tokio::test]
    async fn with_span_async_result_passthrough_error_when_disabled() {
        let _guard = TEST_LOCK.lock().expect("lock telemetry tests");
        set_enabled(false);

        let result = with_span_async_result("test.span", &[], || async {
            Err::<(), anyhow::Error>(anyhow::anyhow!("boom"))
        })
        .await;

        assert!(result.is_err());
        assert_eq!(result.err().unwrap().to_string(), "boom");
    }
}
