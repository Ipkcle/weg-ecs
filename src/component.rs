pub type ComponentMask = u32;
pub trait Component {
    fn get_mask(&self) -> ComponentMask;
}
