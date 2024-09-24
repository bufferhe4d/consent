use crate::utils::PublicParams;

pub struct IdentityManager {
    public_params: PublicParams,
}

impl IdentityManager {
    pub fn new(public_params: PublicParams) -> Self {
        IdentityManager { public_params }
    }

    pub fn process_request(&self, request: &str) -> String {
        format!("IdM Response to: {}", request)
    }
}