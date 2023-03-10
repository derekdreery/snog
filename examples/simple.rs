use rand::Rng;
use snog::{
    kurbo::{Affine, Circle, Line, Rect, Size},
    peniko::{Brush, Color, Fill, Stroke},
    App, RenderCtx,
};

fn main() {
    let data = Data::new();
    App::new_with_data(data).with_render(render).run()
}

// TODO both the lifetime of the RenderCtx and the ref to user data could be the same - nothing is
// gained by having one longer than the other.
fn render(data: &mut Data, mut ctx: RenderCtx<'_>) {
    let Size { width, height } = ctx.screen().size();

    let stroke = Stroke::new(2.);
    let scale = Affine::scale_non_uniform(width, height);
    for (line, color) in data.lines.iter().copied() {
        let brush = Brush::Solid(color);
        ctx.stroke(&stroke, Affine::IDENTITY, &brush, None, &(scale * line));
    }
}

struct Data {
    lines: Vec<(Line, Color)>,
}

impl Data {
    fn new() -> Self {
        const NUM: usize = 1_000;
        let mut lines = Vec::with_capacity(NUM);
        let mut rng = rand::thread_rng();
        for _ in 0..NUM {
            let line = Line::new((rng.gen(), rng.gen()), (rng.gen(), rng.gen()));
            let color = Color::hlca(
                rng.gen::<f64>() * 360.,
                rng.gen::<f64>() * 100.,
                rng.gen::<f64>() * 127.,
                1.,
            );
            lines.push((line, color));
        }
        Data { lines }
    }
}
