use tokio::task::JoinHandle;

/// a potentially unevaluated value
/// this is returned to prevent deadlocks in awaits
pub enum Unevaluated<T> {
    /// a concrete, evaluated value
    Concrete(T),
    /// an unevaluated value
    Unevaluated(JoinHandle<T>),
}

impl<T> Unevaluated<T> {
    /// returns the evaluated value
    /// waits until it is evaluated, if not
    pub async fn evaluate(self) -> T {
        match self {
            Self::Concrete(value) => value,
            Self::Unevaluated(handle) => handle.await.unwrap(),
        }
    }
}

impl<T> From<T> for Unevaluated<T> {
    fn from(value: T) -> Self {
        Self::Concrete(value)
    }
}
