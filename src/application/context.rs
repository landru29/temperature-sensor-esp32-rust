use super::storage::Nvs;

pub struct Context {
    pub nvs: Nvs,
}

impl Context {
    pub fn new(
        nvs: Nvs,
    ) -> anyhow::Result<Self> {
        let output = Self {
            nvs,
        };

        Ok(output)
    }
}


