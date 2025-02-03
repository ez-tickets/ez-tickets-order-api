use axum::Router;
use error_stack::{Report, ResultExt};
use server::errors::UnrecoverableError;
use tokio::net::TcpListener as TokioTcpListener;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> Result<(), Report<UnrecoverableError>> {
    let _guard = server::logging::init();

    // Todo: Add login logic.
    let backyard = Router::new()
        .nest_service("/backyard", ServeDir::new("../.ui.backyard/dist"));
    
    let client = Router::new()
        .nest_service("/app", ServeDir::new("../.ui.orderapp/dist"));
    
    let router = Router::new()
        .merge(client)
        .merge(backyard);

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
