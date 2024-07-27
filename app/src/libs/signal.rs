use tokio::{
    signal::{
        ctrl_c,
        unix::{signal, SignalKind},
    },
    task::AbortHandle,
};

pub async fn shutdown_signal(abort_handler: AbortHandle) {
    let ctrl_c = async {
        ctrl_c().await.expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal(SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => { abort_handler.abort() },
        _ = terminate => { abort_handler.abort() },
    }
}
