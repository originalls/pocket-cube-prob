use crate::enums::{CubeRealSide, CubeSide};
use crate::geom::{self, ColoredCube, ColoredCubeSide}; // , PocketCube
use crate::rubiks_cube::PocketCube;
use rand::Rng;
use three_d::*;

pub struct MeshArr<const N: usize> {
    vertices: [Vec3; N],
    indices: Vec<u32>,
    colors: [Srgba; N],
}

pub struct MeshVec {
    pub vertices: Vec<Vec3>,
    pub indices: Vec<u32>,
    pub colors: Vec<Srgba>,
}

const COLOR_RED: Srgba = Srgba::new_opaque(0xFF, 0x00, 0x00);
const COLOR_ORANGE: Srgba = Srgba::new_opaque(0xFF, 0x80, 0x00);
const COLOR_YELLOW: Srgba = Srgba::new_opaque(0xFF, 0xFF, 0x00);
const COLOR_GREEN: Srgba = Srgba::new_opaque(0x00, 0xDD, 0x00);
const COLOR_BLUE: Srgba = Srgba::new_opaque(0x00, 0x00, 0xFF);
const COLOR_WHITE: Srgba = Srgba::new_opaque(0xFF, 0xFF, 0xFF);

pub fn mainloop() {
    // Create a window (a canvas on web)
    let window = Window::new(WindowSettings {
        title: "Pocket Cube".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();

    // Get the graphics context from the window
    let context = window.gl();

    // Create a camera
    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(0.0, 0.0, -2.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        10.0,
    );

    let cube = geom::Cube {
        center: Vec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        },
        size: 0.5,
    };
    let cube_mesh = geom::CubeMesh::new(cube);

    let mut rng = rand::thread_rng();
    let colors: Vec<Srgba> = (0..8)
        .map(|_| {
            let r: u8 = rng.gen_range(0..=255);
            let g: u8 = rng.gen_range(0..=255);
            let b: u8 = rng.gen_range(0..=255);
            Srgba::new(r, g, b, 255)
        })
        .collect();

    let colored_sides = vec![
        ColoredCubeSide {
            color: Srgba::RED,
            side: CubeSide::PosX,
        },
        ColoredCubeSide {
            color: Srgba::new(0xFF, 0x80, 0x00, 0xFF),
            side: CubeSide::NegX,
        },
        ColoredCubeSide {
            color: Srgba::new(0xFF, 0xFF, 0x00, 0xFF),
            side: CubeSide::PosY,
        },
        ColoredCubeSide {
            color: Srgba::WHITE,
            side: CubeSide::NegY,
        },
        ColoredCubeSide {
            color: Srgba::BLUE,
            side: CubeSide::PosZ,
        },
        ColoredCubeSide {
            color: Srgba::new(0x00, 0xDD, 0x00, 0xFF),
            side: CubeSide::NegZ,
        },
    ];
    let colored_cube = ColoredCube::new(&colored_sides, 0.5);
    let pocket_cube = PocketCube::new_default();

    let mesh = pocket_cube.get_mesh_vec();
    let cpu_mesh = mesh.to_cpu_mesh();

    // Construct a model, with a default color material, thereby transferring the mesh data to the GPU
    let mut model = Gm::new(Mesh::new(&context, &cpu_mesh), ColorMaterial::default());

    // Add an animation to the triangle.
    model.set_animation(|time| {
        let angle = radians(time * 0.0012);
        Mat4::from_angle_x(angle * 1.8)
            * Mat4::from_angle_y(angle * 1.4)
            * Mat4::from_angle_z(angle)
    });
    model.set_transformation(Mat4::from_scale(0.5));

    let mut angle_x: Rad<f32> = radians(0.);
    let mut angle_y: Rad<f32> = radians(0.);
    let mut angle_z: Rad<f32> = radians(0.);

    // Control setup
    let mut control = OrbitControl::new(*camera.target(), 1.0, 100.0);

    // Start the main render loop
    window.render_loop(
        move |mut frame_input| // Begin a new frame with an updated frame input
    {
    control.handle_events(&mut camera, &mut frame_input.events);

        // for event in frame_input.events.iter() {
        // 	if let Event::MouseMotion { button, delta, position, modifiers, handled } = *event {
        //  angle_x += radians(delta.0);
        //  angle_y += radians(delta.1);
        //  println!("delta {} {}", angle_x.0, angle_y.0);
        //  }
        // }

        // Ensure the viewport matches the current window viewport which changes if the window is resized
        camera.set_viewport(frame_input.viewport);

        // Update the animation of the triangle
        // model.animate(frame_input.accumulated_time as f32);

        // Get the screen render target to be able to render something on the screen
        frame_input.screen()
            // Clear the color and depth of the screen render target
            .clear(ClearState::color_and_depth(0.1, 0.1, 0.15, 1.0, 1.0))
            // Render the triangle with the color material which uses the per vertex colors defined at construction
            .render(
                &camera, &model, &[]
            );

        // Returns default frame output to end the frame
        FrameOutput::default()
    },
    );
}

impl<const N: usize> MeshArr<N> {
    pub fn new(vertices: [Vec3; N], indices: Vec<u32>, colors: [Srgba; N]) -> Self {
        MeshArr {
            vertices,
            indices,
            colors,
        }
    }
    pub fn to_mesh_vec(&self) -> MeshVec {
        MeshVec {
            vertices: self.vertices.to_vec(),
            indices: self.indices.clone(),
            colors: self.colors.to_vec(),
        }
    }
}

impl MeshVec {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
            colors: Vec::new(),
        }
    }

    pub fn to_cpu_mesh(&self) -> CpuMesh {
        CpuMesh {
            positions: Positions::F32(self.vertices.clone()),
            colors: Some(self.colors.clone()),
            indices: Indices::U32(self.indices.clone()),
            ..Default::default()
        }
    }

    pub fn vertices_count(&self) -> u32 {
        self.vertices.len().try_into().unwrap()
    }

    pub fn indexed(&self, index: u32) -> Self {
        Self {
            vertices: self.vertices.clone(),
            colors: self.colors.clone(),
            indices: self.indices.iter().map(|x| x + index).collect(),
        }
    }

    pub fn concat(&self, mesh: &MeshVec) -> Self {
        let mut vertices: Vec<Vec3> = self.vertices.clone();
        let mut indices: Vec<u32> = self.indices.clone();
        let mut colors: Vec<Srgba> = self.colors.clone();

        vertices.extend(mesh.vertices.clone());
        indices.extend(mesh.indices.clone());
        colors.extend(mesh.colors.clone());

        Self {
            vertices,
            indices,
            colors,
        }
    }
}

impl CubeRealSide {
    pub fn to_srgba(&self) -> Srgba {
        match self {
            CubeRealSide::R => COLOR_RED,
            CubeRealSide::L => COLOR_ORANGE,
            CubeRealSide::U => COLOR_YELLOW,
            CubeRealSide::D => COLOR_WHITE,
            CubeRealSide::F => COLOR_BLUE,
            CubeRealSide::B => COLOR_GREEN,
        }
    }
}
