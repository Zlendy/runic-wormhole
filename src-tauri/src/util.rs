use futures::{pin_mut, select, FutureExt};
use std::future::Future;

use crate::error::RunicError;

pub async fn cancellable<T>(
    future: impl Future<Output = T>,
    cancel: impl Future<Output = ()>,
) -> Result<T, RunicError> {
    let future = future.fuse();
    let cancel = cancel.fuse();

    pin_mut!(future, cancel);

    select! {
        res = future => {
            Ok(res)
        },
        () = cancel => {
            Err(RunicError::Cancelled)
        }
    }
}
