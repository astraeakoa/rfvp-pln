use std::io::Read;
use std::thread::sleep;
use std::time::Duration;
use std::{io::BufReader, path::Path, time::Instant};

use rodio::OutputStream;
use rodio::Sink;
use futures::executor::block_on;
use rfvp::subsystem::resources::videoplayer::MpegVideoDecoder;
use wgpu::{CompositeAlphaMode, SamplerBindingType, SurfaceConfiguration, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages};
use winit::{event::{Event, WindowEvent}, event_loop::ControlFlow};

struct VideoPlayerTest {
    decoder: MpegVideoDecoder,
}

impl VideoPlayerTest {
    fn new() -> Self {
        let path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/testcase/01.mp4"));

        let decoder = MpegVideoDecoder::new(path).unwrap();

        Self { 
            decoder,
        }
    }

    fn run(&mut self) {
        let event_loop = winit::event_loop::EventLoop::new().expect("Event loop could not be created");
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

        let window_builder: winit::window::WindowBuilder = winit::window::WindowBuilder::new()
            .with_title("app".to_string())
            .with_inner_size(winit::dpi::LogicalSize::new(1024, 640));

        let window = window_builder
            .build(&event_loop)
            .expect("An error occured while building the main game window");

        // init wgpu
        let backend = wgpu::util::backend_bits_from_env().unwrap_or_else(wgpu::Backends::all);
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: backend,
            dx12_shader_compiler: wgpu::Dx12Compiler::Fxc,
            flags: wgpu::InstanceFlags::default(),
            gles_minor_version: wgpu::Gles3MinorVersion::Automatic
        });

        let (size, surface) = unsafe {
            let size = window.inner_size();
            let surface = instance.create_surface(&window).expect("Surface unsupported by adapter");
            (size, surface)
        };

        // let adapter =
        //     wgpu::util::initialize_adapter_from_env_or_default(&instance, Some(&surface))
        //         .await
        //         .expect("No suitable GPU adapters found on the system!");

        let adapter = block_on(async {
            let adapter =
                wgpu::util::initialize_adapter_from_env_or_default(&instance, Some(&surface))
                    .await
                    .expect("No suitable GPU adapters found on the system!");
            adapter
        });

        let needed_limits =
            wgpu::Limits::downlevel_webgl2_defaults().using_resolution(adapter.limits());
        let trace_dir = std::env::var("WGPU_TRACE");

        let (device, queue) = block_on(async {
            let (device, queue) = adapter
                .request_device(
                    &wgpu::DeviceDescriptor {
                        label: None,
                        required_features: wgpu::Features::empty(),
                        required_limits: needed_limits,
                    },
                    trace_dir.ok().as_ref().map(std::path::Path::new),
                )
                .await
                .expect("Unable to find a suitable GPU adapter!");
            (device, queue)
        });

        let config = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_capabilities(&adapter).formats[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: CompositeAlphaMode::Auto,
            view_formats: vec![TextureFormat::Bgra8UnormSrgb],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        let texture = device.create_texture(&TextureDescriptor {
            label: Some("Post-Processing Frame Texture"),
            size: wgpu::Extent3d {
                width: size.width,
                height: size.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: config.format,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST // Added as part of wgpu 0.12 migration
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });

        // self.window = Some(window);

        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("uniform_bind_group_layout"),
            });

            let texture_bind_group_layout =
                device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Texture {
                                multisampled: false,
                                view_dimension: wgpu::TextureViewDimension::D2,
                                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Sampler(SamplerBindingType::Filtering),
                            count: None,
                        },
                    ],
                    label: Some("texture_bind_group_layout"),
                });

        let output_stream = OutputStream::try_default();
        let (_stream, handle) = output_stream.expect("Error creating output stream");
        let sink = Sink::try_new(&handle).expect("Error creating sink");
        // sink.sleep_until_end();

        let now = Instant::now();
        let window = &window;

        //render video
        event_loop
            .run(move |event, target| {
                // Have the closure take ownership of the resources.
                // `event_loop.run` never returns, therefore we must do this to ensure
                // the resources are properly cleaned up.

                target.set_control_flow(ControlFlow::Poll);

                if let Event::WindowEvent { window_id, event } = event {
                    match event {
                        WindowEvent::Resized(new_size) => {
                        }
                        WindowEvent::RedrawRequested => {
                            let elapsed = now.elapsed().as_millis() as u64;
                            let image = self.decoder.take_frame(elapsed);
                            if let Ok(Some(image)) = image {
                                let frame = surface.get_current_texture().unwrap();
                                let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
                        
                                let mut encoder =
                                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

                                let buffer = image.to_vec();
                                println!("width: {}, height: {}", image.width(), image.height());

                                queue.write_texture(
                                    // Tells wgpu where to copy the pixel data
                                    wgpu::ImageCopyTexture {
                                        texture: &texture,
                                        mip_level: 0,
                                        origin: wgpu::Origin3d::ZERO,
                                        aspect: wgpu::TextureAspect::All,
                                    },
                                    // The actual pixel data
                                    &buffer,
                                    // The layout of the texture
                                    wgpu::ImageDataLayout {
                                        offset: 0,
                                        bytes_per_row: Some(image.width() * 4),
                                        rows_per_image: Some(image.height()),
                                    },
                                    wgpu::Extent3d {
                                        width: image.width(),
                                        height: image.height(),
                                        depth_or_array_layers: 1,
                                    },
                                );
                                frame.present();
                            }
                        }
                        WindowEvent::CloseRequested => {
                            target.exit();
                        }
                        _ => {}
                    }
                }
                else if event == Event::AboutToWait {
                    sleep(Duration::from_millis(10));
                    window.request_redraw();
                }
            })
            .unwrap();
    }
}


fn main() {
    env_logger::init();
    let mut player = VideoPlayerTest::new();
    player.run();
}
