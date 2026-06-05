use raylib::prelude::*;


fn main() {
    let ip = raytris::Tetrimino::I;
    let j = raytris::Tetrimino::J;
    let l = raytris::Tetrimino::L;
    let o = raytris::Tetrimino::O;
    let s = raytris::Tetrimino::S;
    let t = raytris::Tetrimino::T;
    let z = raytris::Tetrimino::Z;
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Tetris but worse")
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        for i in 0..4 {
            ip.draw(
                &mut d, 
                50*i, 
                0, 
                10, 
                10, 
                (i as usize).try_into().unwrap()
            );

            j.draw(
                &mut d, 
                50*i, 
                50, 
                10, 
                10, 
                (i as usize).try_into().unwrap()
            );

            l.draw(
                &mut d, 
                50*i, 
                100, 
                10, 
                10, 
                (i as usize).try_into().unwrap()
            );

            o.draw(
                &mut d, 
                50*i, 
                150, 
                10, 
                10, 
                (i as usize).try_into().unwrap()
            );

            s.draw(
                &mut d, 
                50*i, 
                200, 
                10, 
                10, 
                (i as usize).try_into().unwrap()
            );

            t.draw(
                &mut d, 
                50*i, 
                250, 
                10, 
                10, 
                (i as usize).try_into().unwrap()
            );

            z.draw(
                &mut d, 
                50*i, 
                300, 
                10, 
                10, 
                (i as usize).try_into().unwrap()
            );
        }
    }
}
