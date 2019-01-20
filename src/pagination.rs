pub struct PaginationFactory {
    pub page: u32,
    pub limit: u32
}

impl Default for PaginationFactory {
    fn default() -> Self {
        Self {
            page: 1,
            limit: 10
        }
    }
}

impl PaginationFactory {
    pub fn page(mut self, page: u32) -> Self {
        self.page = page;
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = limit;
        self
    }
}