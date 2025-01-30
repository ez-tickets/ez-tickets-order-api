use axum::Router;
use error_stack::{Report, ResultExt};
use tokio::net::TcpListener as TokioTcpListener;
use server::errors::UnrecoverableError;

#[tokio::main]
async fn main() -> Result<(), Report<UnrecoverableError>> {
    let _guard = server::logging::init();

    let router = Router::new();

    let tcpl = TokioTcpListener::bind("0.0.0.0:3651")
        .await
        .change_context_lazy(|| UnrecoverableError)?;

    axum::serve(tcpl, router.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .change_context_lazy(|| UnrecoverableError)?;

    Ok(())
}

async fn shutdown_signal() {
    let user_interrupt = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install keyboard interrupt.")
    };

    tokio::select! {
        _ = user_interrupt => {}
    }
}