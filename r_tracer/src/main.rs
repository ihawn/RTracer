use std::io::{stdin, stdout, Read, Write};
use r_tracer::datatypes::vector3::Vector3;
use r_tracer::datatypes::material::Material;
use r_tracer::utilities::frame_handler::FrameHandler;
use r_tracer::datatypes::color::Color;
use r_tracer::datatypes::vector2d::Vector2D;
use r_tracer::spacial::scene::Scene;
use r_tracer::spacial::camera::Camera;
use r_tracer::spacial::mesh_object::MeshObject;
use r_tracer::utilities::file_utilities::{load_model, save_vector2d_as_png, import_texture};
use std::time::Instant;




fn main() {

    let red = Material::new(Color::new(1.0, 0.3, 0.3), Color::black(), 
        Color::white(), Color::black(), 0.0, 0.9, 0.15, 0.0, 
        0.0, 0.0, true, None, None, None, 
        None, None, None, None
    );
    let green = Material::new(Color::new(0.3, 1.0, 0.3), Color::black(), 
        Color::white(), Color::black(), 0.0, 0.9, 0.15, 0.0, 
        0.0,0.0, true, None, None, None, 
        None, None, None, None
    );
    let blue = Material::new(Color::new(0.3, 0.3, 1.0), Color::black(), 
        Color::white(), Color::black(), 0.0, 0.9, 0.15, 0.0, 
        0.0, 0.0, true, None, None, None, 
        None, None, None, None
    );
    let yellow = Material::new(Color::new(1.0, 1.0, 0.3), Color::black(), 
        Color::white(), Color::black(), 0.0, 0.9, 0.0, 0.0, 
        0.0, 0.0, true, None, None, None, 
        None, None, None, None
    );
    let white = Material::new(Color::white(), Color::black(), 
        Color::white(),  Color::black(),0.0, 1.0, 0.0, 0.0, 
        0.0, 0.0, true, None, None, None, 
        None, None, None, None
    );

    let mirror = Material::new(Color::white(), Color::black(), 
        Color::white(),  Color::black(),0.0, 1.0, 1.0, 0.0, 
        0.0, 0.0, true, None, None, None, 
        None, None, None, None
    );
    let mirror_rough = Material::new(Color::white(), Color::black(), 
        Color::white() * 0.9,  Color::black(),0.0, 0.8, 1.0, 
        0.0, 0.0, 0.0, true, None, None, None, 
        None, None, None, None
    );
    /*let mirror_rough2 = Material::new(Color::white(), Color::black(), 
    Color::white() * 0.9, 0.0, 0.8, 1.0, 0.0, 0.0, true
);

    let glossy_white1 = Material::new(Color::white(), Color::black(),
        Color::white(), 0.0, 1.0, 1.0, 0.0, 0.0, true
    );
    let glossy_white2 = Material::new(Color::white(), Color::black(),
        Color::white(), 0.0, 1.0, 0.5, 0.0, 0.0, true
    );
    let glossy_white3 = Material::new(Color::white(), Color::black(),
        Color::white(), 0.0, 1.0, 0.1, 0.0, 0.0, true
    );
    let glossy_white4 = Material::new(Color::white(), Color::black(),
        Color::white(), 0.0, 1.0, 0.05, 0.0, 0.0, true
    );


    let red1 = Material::new(Color::new(0.6, 0.3, 0.3), Color::black(), 
        Color::white(), 0.0, 0.9, 0.07, 0.0, 0.0, true
    );
    let yellow1 = Material::new(Color::new(1.0, 1.0, 0.3), Color::black(), 
        Color::white(), 0.0, 0.9, 0.07, 0.0, 0.0, true
    );
    let green1 = Material::new(Color::new(0.3, 0.6, 0.3), Color::black(), 
        Color::white(), 0.0, 0.9, 0.07, 0.0, 0.0, true
    );
    let blue1 = Material::new(Color::new(0.3, 0.3, 0.6), Color::black(), 
        Color::white(), 0.0, 0.9, 0.07, 0.0, 0.0, true
    );


    let red2 = Material::new(Color::new(1.0, 0.3, 0.3), Color::black(), 
        Color::new(1.0, 0.3, 0.3), 0.0, 1.0, 1.0, 0.0, 0.0, true
    );
    let yellow2 = Material::new(Color::new(1.0, 1.0, 0.3), Color::black(), 
        Color::new(1.0, 1.0, 0.3), 0.0, 1.0, 1.0, 0.0, 0.0, true
    );
    let green2 = Material::new(Color::new(0.3, 1.0, 0.3), Color::black(), 
        Color::new(0.3, 1.0, 0.3), 0.0, 1.0, 1.0, 0.0, 0.0, true
    );
    let blue2 = Material::new(Color::new(0.3, 0.3, 1.0), Color::black(), 
        Color::new(0.3, 0.3, 1.0), 0.0, 1.0, 1.0, 0.0, 0.0, true
    );


    let mirror1 = Material::new(Color::white(), Color::black(), 
        Color::white(), 0.0, 1.0, 1.0, 0.0, 0.0, true
    );
    let mirror2 = Material::new(Color::white(), Color::black(), 
        Color::white(), 0.0, 0.75, 1.0, 0.0, 0.0, true
    );
    let mirror3 = Material::new(Color::white(), Color::black(), 
        Color::white(), 0.0, 0.5, 1.0, 0.0, 0.0, true
    );
    let mirror4 = Material::new(Color::white(), Color::black(), 
        Color::white(), 0.0, 0.3, 1.0, 0.0, 0.0, true
    );*/

    let glossy_white = Material::new(Color::new(0.95, 0.05, 1.0), Color::black(),
        Color::new(0.1, 1.0, 0.1),  Color::black(),0.0, 1.0, 0.5, 0.0, 
        0.0, 0.0, true, None, None, None, 
        None, None, None, None
    );



    let emiss_mat_1 = Material::new(Color::black(), Color::white(),
        Color::white(), Color::black(), 1.0, 0.0, 0.0, 0.0, 
        0.0, 0.0, true, None, None, None, 
        None, None, None, None
    );
    let plastic = Material::new(Color::white()*0.9, Color::black(),
        Color::white(), Color::white()*0.9, 0.0, 0.95, 0.35, 0.65, 
        1.5, 0.0, true, None, None, None, 
        None, None, None, None
    );
    let glass = Material::new(Color::white()*0.9, Color::black(),
        Color::white(), Color::white()*0.9, 0.0, 0.9, 0.0, 1.0, 
        1.5, 0.0, true, None, None, None, 
        None, None, None, None
    );

    let brick = Material::new(Color::black(), Color::black(),
        Color::black(),  Color::black(),
        0.0, 0.0, 0.0, 0.0, 
        0.0, 1.0, true, Some(1), None, None, 
        None, Some(2), Some(3), Some(12)
    );
    let tile = Material::new(Color::white(), Color::black(),
        Color::white(),  Color::black(),
        0.0, 1.0, 0.1, 0.0, 
        0.0, 1.0, true, Some(4), None, None, 
        None, Some(5), Some(6), Some(7)
    );
    let hardwood = Material::new(Color::white(), Color::black(),
        Color::white(),  Color::black(),
        0.0, 1.0, 0.1, 0.0, 
        0.0, 1.0, true, Some(8), None, None, 
        None, Some(9), Some(10), Some(11)
    );
    let metal_plate = Material::new(Color::white(), Color::black(),
        Color::white(),  Color::black(),
        0.0, 1.0, 0.1, 0.0, 
        0.0, 1.0, true, Some(13), None, None, 
        None, Some(14), Some(15), Some(16)
    );
    /*let frosted_glass = Material::new_dieletric(Color::white(), 0.9, 1.5);
    let water = Material::new_dieletric(Color::white(), 1.0, 1.333);
    let blue_glass = Material::new_dieletric(Color::new(0.8, 0.8, 1.0), 1.0, 1.5);*/

    //let fluid = load_model("../Models/fluid.stl", glass);
    //let test_tris = load_model("../Models/test_tris.stl", mirror);
    //let suzanne_noeyes = load_model("../Models/suzanne_noeyes.stl", yellow1);
    //let suzanne_eyes = load_model("../Models/suzanne_eyes.stl", emiss_mat_1);
    let suzanne = load_model("../Models/suzanne.obj", plastic);
    let suzanne2 = load_model("../Models/suzanne2.stl", glossy_white);
    let suzanne3 = load_model("../Models/suzanne3.stl", plastic);
    let suzanne4 = load_model("../Models/suzanne4.stl", mirror);
    let suzanne5 = load_model("../Models/suzanne5.stl", mirror_rough);
    //let test_plane = load_model("../Models/test_plane.stl", emiss_mat_1);
    //let fluid_splash = load_model("../Models/fluid_splash.stl", glass);
    //let ceiling = load_model("../Models/ceil.stl", white);
    let floor = load_model("../Models/floor.obj", tile);
    let side1 = load_model("../Models/side1.stl", red);
    let side2 = load_model("../Models/side2.stl", green);
    let side3 = load_model("../Models/side3.stl", white);
    let side4 = load_model("../Models/side4.stl", blue);

    let wall1 = load_model("../Models/wall1.obj", brick);
    let wall2 = load_model("../Models/wall2.obj", brick);
    let wall3 = load_model("../Models/wall3.obj", brick);
    let wall4 = load_model("../Models/wall4.obj", mirror);

    let top_light = load_model("../Models/top_light.stl", emiss_mat_1);
    let top_light_big = load_model("../Models/top_light_big.stl", emiss_mat_1);
   // let test_sphere = load_model("../Models/test_sphere.obj", hardwood);
    /*let bot_light = load_model("../Models/bot_light.stl", emiss_mat_1);
    let ico_sphere = load_model("../Models/ico.stl", mirror);*/


    let mut meshes: Vec<MeshObject> = vec![];
    //meshes.push(MeshObject::new(ceiling, false));
    meshes.push(MeshObject::new(floor, false));
    /*meshes.push(MeshObject::new(side1, false));
    meshes.push(MeshObject::new(side2, false));
    meshes.push(MeshObject::new(side3, false));
    meshes.push(MeshObject::new(side4, false));*/
    //meshes.push(MeshObject::new(top_light, false));
    meshes.push(MeshObject::new(top_light_big, false));
    //meshes.push(MeshObject::new(bot_light, false));
    /*meshes.push(MeshObject::new(top_light1, false));
    meshes.push(MeshObject::new(top_light2, false));
    meshes.push(MeshObject::new(top_light3, false));*/
    meshes.push(MeshObject::new(suzanne, true));
    //meshes.push(MeshObject::new(test_sphere, true));
    /*meshes.push(MeshObject::new(suzanne2, true));
    meshes.push(MeshObject::new(suzanne3, true));
    meshes.push(MeshObject::new(suzanne4, true));
    meshes.push(MeshObject::new(suzanne5, true));*/
    //meshes.push(MeshObject::new(fluid_splash, true));
    //meshes.push(MeshObject::new(test_plane, true));
    //meshes.push(MeshObject::new(fluid, true));
    //meshes.push(MeshObject::new(light_ball, true));
    //meshes.push(MeshObject::new(suzanne_eyes));
    //meshes.push(MeshObject::new(suzanne_noeyes));

    meshes.push(MeshObject::new(wall1, false));
    meshes.push(MeshObject::new(wall2, false));
    meshes.push(MeshObject::new(wall3, false));
    meshes.push(MeshObject::new(wall4, false));

    //meshes.push(MeshObject::new(dave8));


    let size_x: usize = 1200;
    let size_y: usize = 800;

    let mut maps: Vec<Vector2D<Color>> = vec![];

    let test_uv: Vector2D<Color> = import_texture("../Textures/uv_test.jpg");
    let stone_brick_col: Vector2D<Color> = import_texture("C:/Users/Isaac/Desktop/TilesCeramicSubwayOffsetCrackle002/TilesCeramicSubwayOffsetCrackle002_COL_3K.jpg");
    let stone_brick_normal: Vector2D<Color> = import_texture("C:/Users/Isaac/Desktop/TilesCeramicSubwayOffsetCrackle002/TilesCeramicSubwayOffsetCrackle002_NRM_3K.jpg");
    let stone_brick_smoothness: Vector2D<Color> = import_texture("C:/Users/Isaac/Desktop/TilesCeramicSubwayOffsetCrackle002/TilesCeramicSubwayOffsetCrackle002_GLOSS_3K.jpg");
    let stone_brick_specular: Vector2D<Color> = import_texture("C:/Users/Isaac/Desktop/TilesCeramicSubwayOffsetCrackle002/TilesCeramicSubwayOffsetCrackle002_REFL_3K.jpg");

    let tile_col: Vector2D<Color> = import_texture("C:/Users/Isaac/Desktop/TilesTerracottaRedHexagon001/TilesTerracottaRedHexagon001_COL_3K.jpg");
    let tile_normal: Vector2D<Color> = import_texture("C:/Users/Isaac/Desktop/TilesTerracottaRedHexagon001/TilesTerracottaRedHexagon001_NRM_3K.jpg");
    let tile_smoothness: Vector2D<Color> = import_texture("C:/Users/Isaac/Desktop/TilesTerracottaRedHexagon001/TilesTerracottaRedHexagon001_AO_3K.jpg");
    let tile_specular: Vector2D<Color> = import_texture("C:/Users/Isaac/Desktop/TilesTerracottaRedHexagon001/TilesTerracottaRedHexagon001_GLOSS_3K.jpg");

    let hardwood_col: Vector2D<Color> = import_texture("C:/Users/Isaac/Desktop/WoodFlooring044/WoodFlooring044_COL_3K.jpg");
    let hardwood_normal: Vector2D<Color> = import_texture("C:/Users/Isaac/Desktop/WoodFlooring044/WoodFlooring044_NRM_3K.jpg");
    let hardwood_smoothness: Vector2D<Color> = import_texture("C:/Users/Isaac/Desktop/WoodFlooring044/WoodFlooring044_GLOSS_3K.jpg");
    let hardwood_specular: Vector2D<Color> = import_texture("C:/Users/Isaac/Desktop/WoodFlooring044/WoodFlooring044_REFL_3K.jpg");

    let metal_col: Vector2D<Color> = import_texture("C:/Users/Isaac/Desktop/MetalCorrodedHeavy001/MetalCorrodedHeavy001_COL_3K_METALNESS.jpg");
    let metal_normal: Vector2D<Color> = import_texture("C:/Users/Isaac/Desktop/MetalCorrodedHeavy001/MetalCorrodedHeavy001_NRM_3K_METALNESS.jpg");
    let metal_smoothness: Vector2D<Color> = import_texture("C:/Users/Isaac/Desktop/MetalCorrodedHeavy001/MetalCorrodedHeavy001_SMOOTHNESS_3K_METALNESS.jpg");
    let metal_specular: Vector2D<Color> = import_texture("C:/Users/Isaac/Desktop/MetalCorrodedHeavy001/MetalCorrodedHeavy001_METALNESS_3K_METALNESS.jpg");
    maps.push(test_uv);
    maps.push(stone_brick_col);
    maps.push(stone_brick_normal);
    maps.push(stone_brick_smoothness);
    maps.push(tile_col);
    maps.push(tile_normal);
    maps.push(tile_smoothness);
    maps.push(tile_specular);
    maps.push(hardwood_col);
    maps.push(hardwood_normal);
    maps.push(hardwood_smoothness);
    maps.push(hardwood_specular);
    maps.push(stone_brick_specular);
    maps.push(metal_col);
    maps.push(metal_normal);
    maps.push(metal_smoothness);
    maps.push(metal_specular);

    let scene: Scene = Scene::new(meshes, maps, Color::white() * 0.0);
    let camera: Camera = Camera::new(
        Vector3::new(-200.0, 0.0, 10.0),
        Vector3::new(0.0, 22.0, 0.0),
        scene, 2.8, 
        size_x, size_y,
        30, 3, 0.3, 
        0.0, 192.0, 1.3,
        0
    );

    let mut frame_handler: FrameHandler = FrameHandler::new(size_x, size_y, "RTracer");

    let start_time = Instant::now();
    let frame: Vector2D<Color> = camera.render_scene(frame_handler, 3000);
    let elapsed_time = start_time.elapsed();

    let hours = elapsed_time.as_secs() / 3600;
    let minutes = (elapsed_time.as_secs() % 3600) / 60;
    let seconds = elapsed_time.as_secs() % 60;
    let milliseconds = elapsed_time.subsec_millis();
    println!(
        "Elapsed time: {:02}:{:02}:{:02}:{:03}",
        hours, minutes, seconds, milliseconds
    );
    
    save_vector2d_as_png(&frame, "../Renders/render.png");
}
