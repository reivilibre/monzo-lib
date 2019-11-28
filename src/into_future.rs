use std::future::Future;

/// Placeholder trait- waiting on the IntoFuture to stabilise.
/// This replaces the `send()` method currently used to turn requests into
/// futures. in future, `await` will desugar into `into_future().await` which
/// allow us to `await` the requests directly.
pub trait IntoFuture {
    /// The output that the future will produce on completion.
    type Output;
    /// Which kind of future are we turning this into?
    type Future: Future<Output = Self::Output>;

    /// Creates a future from a value.
    fn into_future(self) -> Self::Future;
}
