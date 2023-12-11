use super::frame::Frame;

#[derive(Default)]
pub struct CanNodeConfig {}

pub struct CanNode;

impl CanNode {
    pub fn init(self, _config: CanNodeConfig) -> Result<CanNode, ()> {
        // TODO
        Ok(self)
    }

    pub fn transmit(&self, _frame: &Frame) -> Result<(), ()> {
        // TODO
        Ok(())
    }
}
