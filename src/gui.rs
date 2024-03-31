use crate::enums::CubeRealSide;
use crate::geom; // , PocketCube
use crate::moves::{self, move_inv, MoveFunc};
use crate::rubiks_cube::PocketCube;
use three_d::*;
// use three_d_asset::io::load;
// use three_d_text_builder::{TextAlign, TextBuilder, TextBuilderSettings, TextPosition, TextRef};

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
        max_size: Some((720, 720)),
        ..Default::default()
    })
    .unwrap();

    // Get the graphics context from the window
    let context = window.gl();

    // Create a camera
    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(2.0, 1.5, 2.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(40.0),
        0.1,
        10.0,
        // window.viewport(),
        // vec3(5.0, 2.0, 2.5),
        // vec3(0.0, 0.0, -0.5),
        // vec3(0.0, 1.0, 0.0),
        // degrees(45.0),
        // 0.1,
        // 1000.0,
    );

    let cube = geom::Cube {
        center: Vec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        },
        size: 0.995,
    };
    let cube_cpu_mesh = geom::CubeMesh::new(cube).to_vec_mesh().to_cpu_mesh();
    let mut cube_model = Gm::new(
        Mesh::new(&context, &cube_cpu_mesh),
        ColorMaterial::default(),
    );
    cube_model.set_transformation(Mat4::from_scale(0.45));

    // Load the font
    // let assets = load(&["src/ubuntu.mono.ttf"]).unwrap();

    // Create a text builder
    // let mut text_builder = TextBuilder::new(
    //     assets.get("ubuntu.mono.ttf").unwrap(),
    //     TextBuilderSettings::default(),
    // )
    // .expect("Failed to create text builder from TTF font");

    // Create text
    // let text = TextRef {
    //     // The text to render
    //     text: "The quick brown fox jumps over the lazy dog",
    //     // Set the color
    //     color: Srgba::RED,
    //     // Align to the lower center edge of the viewport
    //     align: TextAlign::Viewport(0, -1),
    //     // Add some padding
    //     padding: vec2(0.0, 8.0),
    //     // Move up by 25% of the viewport's height
    //     position: TextPosition::Percentage(vec2(0.0, 0.25)),
    //     // Add a simple shadow effect
    //     shadow: Some((Srgba::BLACK, vec2(1.0, -1.0))),
    //     ..Default::default()
    // };

    let mut group_pc = crate::group::PocketCube::new();
    let mut pocket_cube = PocketCube::new(group_pc.to_facelets());

    let pc_to_model = move |pocket_cube: &PocketCube| -> Gm<Mesh, ColorMaterial> {
        let mesh = pocket_cube.get_mesh_vec();
        let cpu_mesh = mesh.to_cpu_mesh();
        let mut model = Gm::new(Mesh::new(&context, &cpu_mesh), ColorMaterial::default());
        model.set_animation(|time| {
            let angle = radians(time * 0.0012);
            Mat4::from_angle_x(angle * 1.8)
                * Mat4::from_angle_y(angle * 1.4)
                * Mat4::from_angle_z(angle)
        });
        model.set_transformation(Mat4::from_scale(0.45));
        model
    };

    let mut model = pc_to_model(&pocket_cube);

    // Control setup
    let mut control = OrbitControl::new(*camera.target(), 1.0, 100.0);

    // Start the main render loop
    window.render_loop(
        move |mut frame_input| // Begin a new frame with an updated frame input
    {



        for event in frame_input.events.iter_mut() {
        	if let Event::KeyPress { kind, modifiers, handled } = event {

         if *kind == Key::R {
       	group_pc.reset_cubics();
	           pocket_cube.set_facelets(group_pc.to_facelets());
	           model = pc_to_model(&pocket_cube);
       } else if *kind == Key::T {
       	group_pc.reset_cubics();
        group_pc.cubics[0].rotate(crate::enums::CornerTwist::Rot1);
	           pocket_cube.set_facelets(group_pc.to_facelets());
	           model = pc_to_model(&pocket_cube);
       } else {

         let is_inverse = modifiers.shift;
         let move_func = match *kind {
         Key::Q => Some(moves::move_front as MoveFunc),
         Key::W => Some(moves::move_up as MoveFunc),
         Key::E => Some(moves::move_right as MoveFunc),
         _ => None,
         };
         if let Some(move_func) = move_func {
         		if is_inverse {
	           group_pc.do_move(move_inv(move_func));
        } else {
	           group_pc.do_move(move_func);
        }
	           pocket_cube.set_facelets(group_pc.to_facelets());
	           model = pc_to_model(&pocket_cube);
				println!("{:?}", group_pc.get_branches());
				let id = group_pc.get_perm_id().get_id();
				println!("Cube ID: {}", id);

				let mut pc = crate::group::PocketCube::new();
				pc.apply_id(id.into());
				println!("AgainID: {}", pc.get_perm_id().get_id());

				println!("\n");



         }

       }
           // Update the animation of the triangle
                   // model.animate(frame_input.accumulated_time as f32);
          		*handled = true;
         }
        }
    control.handle_events(&mut camera, &mut frame_input.events);


        // Ensure the viewport matches the current window viewport which changes if the window is resized
        camera.set_viewport(frame_input.viewport);

        // let text_model = text_builder.build(&context, &[
        //             // Place the text above our cube in the 3D
        //             TextRef {
        //                 text: "Cube",
        //                 size: 24.0,
        //                 // Center onto the cube's screen position
        //                 ..Default::default()
        //             }
        //         ]);

        // Get the screen render target to be able to render something on the screen
        frame_input.screen()
            // Clear the color and depth of the screen render target
            .clear(ClearState::color_and_depth(0.1, 0.1, 0.15, 1.0, 1.0))
            // Render the triangle with the color material which uses the per vertex colors defined at construction
            .render(
                &camera, &model, &[]
            )
            .render(
                &camera, &cube_model, &[]
            )
            // .render(
            //     &camera, text_model, &[]
            // )
        ;

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
