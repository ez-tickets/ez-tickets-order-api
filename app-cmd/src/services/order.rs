use async_trait::async_trait;
use error_stack::{Report, ResultExt};
use kernel::entities::order::{Order, OrderId};
use kernel::io::command::order::OrderCommand;
use kernel::services::product::{DependOnProductInquiryService, ProductInquiryService};
use crate::adapter::{self, DependOnEventProjector, DependOnProcessManager};
use crate::errors::ApplicationError;

#[async_trait]
pub trait OrderCommandService: 'static + Sync + Send 
where
    Self: DependOnProcessManager
        + DependOnEventProjector
        + DependOnProductInquiryService
{
    async fn execute<I>(&self, id: I, cmd: OrderCommand) -> Result<(), Report<ApplicationError>> 
    where
        I: Into<Option<OrderId>> + Sync + Send + 'static
    {
        let manager = self.process_manager();
        
        let refs = if let OrderCommand::Create { .. } = &cmd {
            let id = OrderId::default();

            let order = Order::try_from((id, cmd.clone()))
                .change_context_lazy(|| ApplicationError::Formation)?;

            manager.spawn(id, order, 0).await
                .change_context_lazy(|| ApplicationError::Process)?
        } else {
            let id = id.into()
                .ok_or(ApplicationError::RequiredId)?;
            adapter::utils::find_or_replay(id, manager, self.event_projector()).await?
        };
        
        if let OrderCommand::AddProducts { products } = &cmd {
            for product in products.keys() {
                let _ = self.product_inquiry_service()
                    .get_product(product)
                    .await
                    .change_context_lazy(|| ApplicationError::Io)?
                    .ok_or(ApplicationError::NotFound)?;
            } 
        }

        refs.employ(cmd).await
            .change_context_lazy(|| ApplicationError::Process)?
            .change_context_lazy(|| ApplicationError::Kernel)?;
        
        Ok(())
    }
}