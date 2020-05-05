//! 一些可能用到，而又不好找库的数据结构

mod segment_tree_allocator;
mod stacked_allocator;
mod unsafe_wrapper;

pub use segment_tree_allocator::SegmentTreeAllocator;
pub use stacked_allocator::StackedAllocator;
pub use unsafe_wrapper::{UnsafeWrapper, StaticUnsafeWrapper};

/// 分配器：固定容量，每次分配 / 回收一个元素
pub trait Allocator {
    /// 给定容量，创建分配器
    fn new(capacity: usize) -> Self;
    /// 分配一个元素，无法分配则返回 `None`
    fn alloc(&mut self) -> Option<usize>;
    /// 回收一个元素
    fn dealloc(&mut self, index: usize);
}
