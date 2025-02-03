use async_trait::async_trait;
use error_stack::Report;
use crate::entities::order::ProductId;
use crate::errors::InquiryError;

#[async_trait]
pub trait ProductInquiryService: 'static + Sync + Send {
    type Response;
    async fn get_product(&self, id: &ProductId) -> Result<Option<Self::Response>, Report<InquiryError>>;
}

pub trait DependOnProductInquiryService {
    type ProductInquiryService: ProductInquiryService;
    fn product_inquiry_service(&self) -> &Self::ProductInquiryService;
}