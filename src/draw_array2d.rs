use crate::*;
use idek::Vertex;

pub fn draw_grid<T>(
    builder: &mut GraphicsBuilder,
    state: &Array2D<T>,
    mut color: impl FnMut(&T) -> [f32; 3],
    z: f32,
) {
    let cell_width = 2. / state.width() as f32;
    let cell_height = 2. / state.height() as f32;

    for i in 0..state.width() {
        let i_frac = (i as f32 / state.width() as f32) * 2. - 1.;
        for j in 0..state.height() {
            let j_frac = (j as f32 / state.height() as f32) * 2. - 1.;

            let color = color(&state[(i, j)]);

            let mut push = |dx: f32, dy: f32| {
                let pos = [i_frac + dx, j_frac + dy, z];
                builder.push_vertex(Vertex::new(pos, color))
            };

            let tl = push(0., 0.);
            let tr = push(cell_width, 0.);

            let bl = push(0., cell_height);
            let br = push(cell_width, cell_height);

            builder.push_indices(&[bl, tr, tl, bl, br, tr]);
        }
    }
}
