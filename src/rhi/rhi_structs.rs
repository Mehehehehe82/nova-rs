use super::{rhi_enums::*, rhi_traits::*};
use crate::shaderpack;
use std::sync::Arc;

use super::super::shaderpack::*;

/// Describes what kind of command allocator you want to create
#[derive(Debug, Clone)]
pub struct CommandAllocatorCreateInfo {
    /// The type of command lists which will be allocated by this command allocator
    command_list_type: QueueType,

    // A bitmask of the GPU that the new command allocator will allocate commands for. Only one GPU mey be used
    node_mask: u32,
}

/// Information about a physical device!
///
/// This information can come from multiple API calls, but I've merged all the information together here
///
/// This structure has things like the capabilities of the device, its hardware limits, its manufacturer and model
/// number, etc
#[derive(Debug, Clone)]
pub struct PhysicalDeviceProperties {
    manufacturer: PhysicalDeviceManufacturer,

    device_id: u32,

    device_name: String,

    device_type: PhysicalDeviceType,

    max_color_attachments: u32,
}

#[derive(Debug, Clone)]
pub enum ResourceSpecificData {
    Image { aspect: ImageAspectFlags },
    Buffer { offset: u64, size: u64 },
}

#[derive(Clone)]
pub struct ResourceBarrier {
    resource: Arc<dyn Resource>,

    initial_state: ResourceState,

    final_state: ResourceState,

    access_before_barrier: ResourceAccessFlags,

    access_after_barrier: ResourceAccessFlags,

    source_queue: QueueType,

    destination_queue: QueueType,

    resource_info: ResourceSpecificData,
}

#[derive(Clone)]
pub enum DescriptorUpdateInfo {
    Image {
        image: Arc<dyn Image>,
        format: shaderpack::TextureFormat,
        sampler: Arc<dyn Sampler>,
    },
}

#[derive(Clone)]
pub struct DescriptorSetWrite {
    set: Arc<dyn DescriptorSet>,

    binding: u32,

    update_info: DescriptorUpdateInfo,
}

#[derive(Debug, Clone)]
pub struct ResourceBindingDescription {
    /// Descriptor set that his binding belongs to
    set: u32,

    /// Binding of this resource binding
    binding: u32,

    /// Number of bindings. Useful if you have an array of descriptors
    count: u32,

    /// The type of object that will be bound
    descriptor_type: DescriptorType,

    /// The shader stages that need access to this binding
    stages: ShaderStageFlags,
}

#[derive(Debug, Clone)]
pub struct BufferCreateInfo {
    size: usize,

    buffer_usage: BufferUsage,

    allocation: DeviceMemoryAllocation,
}

#[derive(Debug, Clone)]
pub struct DeviceMemoryAllocation;
