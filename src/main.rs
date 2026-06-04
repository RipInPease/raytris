use raylib::prelude::*;


fn main() {
    let tetriminio = raytris::Tetrimino::O;
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Tetris but worse")
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        tetriminio.draw(
            &mut d, 
            0, 
            5, 
            10, 
            10, 
            raytris::Rotation::None
        );

        tetriminio.draw(
            &mut d, 
            50, 
            5, 
            10, 
            10, 
            raytris::Rotation::Ninety
        );

        tetriminio.draw(
            &mut d, 
            100, 
            5, 
            10, 
            10, 
            raytris::Rotation::OneEighty
        );

        tetriminio.draw(
            &mut d, 
            150, 
            5, 
            10, 
            10, 
            raytris::Rotation::TwoSeventy
        );
    }
}
