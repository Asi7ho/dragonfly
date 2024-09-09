#[cfg(test)]
mod tests {

    fn create_test_device() -> wgpu::Device {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());

        let adapter =
            pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions::default()))
                .unwrap();
        let (device, _) =
            pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor::default(), None))
                .unwrap();
        device
    }

    #[test]
    fn test_simple_triangle_shader_module() {
        let device = create_test_device();

        let _shader =
            device.create_shader_module(wgpu::include_wgsl!("../shader/simple_triangle.wgsl"));
    }

    #[test]
    fn test_simple_figures_shader_module() {
        let device = create_test_device();

        let _shader = device.create_shader_module(wgpu::include_wgsl!("../shader/shader.wgsl"));
    }
}
