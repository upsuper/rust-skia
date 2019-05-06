use crate::artifact::DrawingDriver;
use crate::resources;
use skia_safe::{
    scalar, AutoCanvasRestore, BlendMode, BlurStyle, Canvas, Color, ColorFilters, CornerPathEffect,
    DashPathEffect, DiscretePathEffect, Font, GradientShader, GradientShaderFlags,
    Line2DPathEffect, MaskFilter, Matrix, Paint, PaintStyle, Path, Path1DPathEffect,
    Path1DPathEffectStyle, Path2DPathEffect, PathEffect, PerlinNoiseShader, Point, Rect, Shaders,
    TableColorFilter, TextBlob, TileMode, Typeface,
};
use std::path::PathBuf;

pub fn draw<Driver: DrawingDriver>(path: &PathBuf) {
    let path = &path.join("SkPaint-Overview");

    Driver::draw_image_256(path, "01-three-paints", draw_three_paints);
    Driver::draw_image_256(path, "02-fill-and-stroke", draw_fill_and_stroke);
    Driver::draw_image_256(path, "03-gradient", draw_gradient);
    Driver::draw_image((576, 640), path, "04-transfer-modes", draw_transfer_modes);
    Driver::draw_image_256(path, "05-bitmap-shader", draw_bitmap_shader);
    Driver::draw_image_256(
        path,
        "06-radial-gradient-shader",
        draw_radial_gradient_shader,
    );
    Driver::draw_image_256(
        path,
        "07-two-point-conical-shader",
        draw_two_point_conical_shader,
    );
    Driver::draw_image_256(path, "08-sweep-gradient-shader", draw_sweep_gradient_shader);
    Driver::draw_image_256(
        path,
        "09-fractal-perlin-noise-shader",
        draw_fractal_perlin_noise_shader,
    );
    Driver::draw_image_256(
        path,
        "10-turbulence-perlin-noise-shader",
        draw_turbulence_perlin_noise_shader,
    );
    Driver::draw_image_256(path, "11-compose-shader", draw_compose_shader);
    Driver::draw_image_256(path, "12-mask-filter", draw_mask_filter);
    Driver::draw_image((256, 128), path, "13-color-filter", draw_color_filter);
    Driver::draw_image_256(path, "14-table-color-filter", draw_color_table_color_filter);
    Driver::draw_image_256(path, "15-path-2d-effect", draw_path_2d_effect);
    Driver::draw_image_256(path, "16-line-2d-effect", draw_line_2d_effect);
    Driver::draw_image_256(path, "17-path-1d-effect", draw_path_1d_effect);
    Driver::draw_image_256(path, "18-corner-path-effect", draw_corner_path_effect);
    Driver::draw_image_256(path, "19-dash-path-effect", draw_dash_path_effect);
    Driver::draw_image_256(path, "20-discrete-path-effect", draw_discrete_path_effect);
    Driver::draw_image_256(path, "21-compose-path-effect", draw_compose_path_effect);
    Driver::draw_image_256(path, "22-sum-path-effect", draw_sum_path_effect);
}

fn draw_three_paints(canvas: &mut Canvas) {
    let (paint1, paint2, paint3) = (
        &mut Paint::default(),
        &mut Paint::default(),
        &mut Paint::default(),
    );

    paint1
        .set_anti_alias(true)
        .set_color(Color::from_rgb(255, 0, 0))
        .set_style(PaintStyle::Fill);

    paint2
        .set_anti_alias(true)
        .set_color(Color::from_rgb(0, 136, 0))
        .set_style(PaintStyle::Stroke)
        .set_stroke_width(3.0);

    paint3
        .set_anti_alias(true)
        .set_color(Color::from_rgb(136, 136, 136));

    let blob1 = TextBlob::from_str(
        "Skia!",
        &Font::from_typeface_with_size_scale_and_skew(&Typeface::default(), 64.0, 1.0, 0.0),
    );
    let blob2 = TextBlob::from_str(
        "Skia!",
        &Font::from_typeface_with_size_scale_and_skew(&Typeface::default(), 64.0, 1.5, 0.0),
    );

    canvas.clear(Color::WHITE);
    canvas.draw_text_blob(&blob1, (20.0, 64.0), paint1);
    canvas.draw_text_blob(&blob1, (20.0, 144.0), paint2);
    canvas.draw_text_blob(&blob2, (20.0, 224.0), paint3);
}

fn draw_fill_and_stroke(canvas: &mut Canvas) {
    let fill_paint = &mut Paint::default();
    let stroke_paint = &mut Paint::default();
    stroke_paint
        .set_style(PaintStyle::Stroke)
        .set_stroke_width(3.0);

    canvas.draw_rect(Rect::from_point_and_size((10, 10), (60, 20)), fill_paint);
    canvas.draw_rect(Rect::from_point_and_size((80, 10), (60, 20)), stroke_paint);

    stroke_paint.set_stroke_width(5.0);
    canvas.draw_oval(Rect::from_point_and_size((150, 10), (60, 20)), stroke_paint);

    let blob = TextBlob::from_str(
        "SKIA",
        &Font::from_typeface_with_size(&Typeface::default(), 80.0),
    );

    fill_paint.set_color(Color::from_argb(0xFF, 0xFF, 0x00, 0x00));
    canvas.draw_text_blob(&blob, (20, 120), fill_paint);

    fill_paint.set_color(Color::from_argb(0xFF, 0x00, 0x00, 0xFF));
    canvas.draw_text_blob(&blob, (20, 220), fill_paint);
}

fn draw_gradient(canvas: &mut Canvas) {
    let points: (Point, Point) = ((0.0, 0.0).into(), (256.0, 256.0).into());
    let colors = [Color::BLUE, Color::YELLOW];
    let paint = &mut Paint::default();

    paint.set_shader(
        GradientShader::linear(points, colors.as_ref(), None, TileMode::Clamp, None, None).as_ref(),
    );
    canvas.draw_paint(paint);
}

fn draw_transfer_modes(canvas: &mut Canvas) {
    fn draw_str(c: &mut Canvas, text: &str, x: scalar, y: scalar, font: &Font, paint: &Paint) {
        c.draw_text_blob(&TextBlob::from_str(text, font), (x, y), paint);
    }

    let modes = [
        BlendMode::Clear,
        BlendMode::Src,
        BlendMode::Dst,
        BlendMode::SrcOver,
        BlendMode::DstOver,
        BlendMode::SrcIn,
        BlendMode::DstIn,
        BlendMode::SrcOut,
        BlendMode::DstOut,
        BlendMode::SrcATop,
        BlendMode::DstATop,
        BlendMode::Xor,
        BlendMode::Plus,
        BlendMode::Modulate,
        BlendMode::Screen,
        BlendMode::Overlay,
        BlendMode::Darken,
        BlendMode::Lighten,
        BlendMode::ColorDodge,
        BlendMode::ColorBurn,
        BlendMode::HardLight,
        BlendMode::SoftLight,
        BlendMode::Difference,
        BlendMode::Exclusion,
        BlendMode::Multiply,
        BlendMode::Hue,
        BlendMode::Saturation,
        BlendMode::Color,
        BlendMode::Luminosity,
    ];
    let rect = Rect::from_size((64.0, 64.0));
    let (stroke, src, dst) = (
        &mut Paint::default(),
        &mut Paint::default(),
        &mut Paint::default(),
    );
    stroke.set_style(PaintStyle::Stroke);
    let font = &Font::from_typeface_with_size(&Typeface::default(), 24.0);
    let src_points: (Point, Point) = ((0.0, 0.0).into(), (64.0, 0.0).into());
    let src_colors = [Color::MAGENTA & 0x00_FF_FF_FF, Color::MAGENTA];
    src.set_shader(
        GradientShader::linear(
            src_points,
            src_colors.as_ref(),
            None,
            TileMode::Clamp,
            None,
            None,
        )
        .as_ref(),
    );
    let src_colors = [Color::MAGENTA & 0x00FFFFFF, Color::MAGENTA];
    src.set_shader(
        GradientShader::linear(
            src_points,
            src_colors.as_ref(),
            None,
            TileMode::Clamp,
            None,
            None,
        )
        .as_ref(),
    );

    let dst_points: (Point, Point) = ((0.0, 0.0).into(), (0.0, 64.0).into());
    let dst_colors = [Color::CYAN & 0x00_FF_FF_FF, Color::CYAN];
    dst.set_shader(
        GradientShader::linear(
            dst_points,
            dst_colors.as_ref(),
            None,
            TileMode::Clamp,
            None,
            None,
        )
        .as_ref(),
    );
    canvas.clear(Color::WHITE);
    let n = modes.len();
    let k = (n - 1) / 3 + 1;
    assert_eq!(k * 64, 640); // tall enough
    for (i, mode) in modes.iter().enumerate() {
        let canvas = &mut AutoCanvasRestore::guard(canvas, true);
        canvas.translate((192.0 * (i / k) as scalar, 64.0 * (i % k) as scalar));
        let desc = mode.name();
        draw_str(canvas, desc, 68.0, 30.0, font, &Paint::default());
        canvas.clip_rect(Rect::from_size((64.0, 64.0)), Default::default());
        canvas.draw_color(Color::LIGHT_GRAY, BlendMode::default());
        canvas.save_layer(&Default::default());
        canvas.clear(Color::TRANSPARENT);
        canvas.draw_paint(dst);
        src.set_blend_mode(*mode);
        canvas.draw_paint(src);
        canvas.draw_rect(rect, stroke);
    }
}

fn draw_bitmap_shader(canvas: &mut Canvas) {
    let image = resources::color_wheel();

    canvas.clear(Color::WHITE);
    let mut matrix = Matrix::default();
    matrix.set_scale((0.75, 0.75), None);
    matrix.pre_rotate(30.0, None);
    let paint = &mut Paint::default();
    paint.set_shader(&image.as_shader((TileMode::Repeat, TileMode::Repeat), &matrix));
    paint.set_shader(Some(
        &image.as_shader((TileMode::Repeat, TileMode::Repeat), &matrix),
    ));
    canvas.draw_paint(paint);
}

fn draw_radial_gradient_shader(canvas: &mut Canvas) {
    let colors = [Color::BLUE, Color::YELLOW];
    let mut paint = Paint::default();
    paint.set_shader(
        GradientShader::radial(
            (128.0, 128.0),
            180.0,
            colors.as_ref(),
            None,
            TileMode::Clamp,
            GradientShaderFlags::default(),
            None,
        )
        .as_ref(),
    );
    canvas.draw_paint(&paint);
}

fn draw_two_point_conical_shader(canvas: &mut Canvas) {
    let colors = [Color::BLUE, Color::YELLOW];
    let paint = &mut Paint::default();
    paint.set_shader(
        GradientShader::two_point_conical(
            (128.0, 128.0),
            128.0,
            (128.0, 16.0),
            16.0,
            colors.as_ref(),
            None,
            TileMode::Clamp,
            None,
            None,
        )
        .as_ref(),
    );
    canvas.draw_paint(&paint);
}

fn draw_sweep_gradient_shader(canvas: &mut Canvas) {
    let colors = [Color::CYAN, Color::MAGENTA, Color::YELLOW, Color::CYAN];
    let paint = &mut Paint::default();
    paint.set_shader(
        GradientShader::sweep(
            (128.0, 128.0),
            colors.as_ref(),
            None,
            TileMode::default(),
            None,
            GradientShaderFlags::default(),
            None,
        )
        .as_ref(),
    );
    canvas.draw_paint(paint);
}

fn draw_fractal_perlin_noise_shader(canvas: &mut Canvas) {
    canvas.clear(Color::WHITE);
    let paint = &mut Paint::default();
    paint.set_shader(PerlinNoiseShader::fractal_noise((0.05, 0.05), 4, 0.0, None).as_ref());
    canvas.draw_paint(paint);
}

fn draw_turbulence_perlin_noise_shader(canvas: &mut Canvas) {
    canvas.clear(Color::WHITE);
    let paint = &mut Paint::default();
    paint.set_shader(PerlinNoiseShader::turbulence((0.05, 0.05), 4, 0.0, None).as_ref());
    canvas.draw_paint(paint);
}

fn draw_compose_shader(canvas: &mut Canvas) {
    let colors = [Color::BLUE, Color::YELLOW];
    let paint = &mut Paint::default();
    paint.set_shader(Some(
        Shaders::blend(
            BlendMode::Difference,
            &GradientShader::radial(
                (128.0, 128.0),
                180.0,
                colors.as_ref(),
                None,
                TileMode::Clamp,
                None,
                None,
            )
            .unwrap(),
            &PerlinNoiseShader::turbulence((0.025, 0.025), 2, 0.0, None).unwrap(),
        )
        .as_ref(),
    ));
    canvas.draw_paint(paint);
}

fn draw_mask_filter(canvas: &mut Canvas) {
    // TODO: make BlendMode optional in draw_color.
    canvas.draw_color(
        Color::from_argb(0xFF, 0xFF, 0xFF, 0xFF),
        BlendMode::default(),
    );
    let paint = &mut Paint::default();
    paint.set_mask_filter(&MaskFilter::blur(BlurStyle::Normal, 5.0, None));
    let blob = &TextBlob::from_str(
        "Skia",
        &Font::from_typeface_with_size(&Typeface::default(), 120.0),
    );
    canvas.draw_text_blob(blob, (0, 160), paint);
}

fn draw_color_filter(c: &mut Canvas) {
    fn f(c: &mut Canvas, (x, y): (scalar, scalar), color_matrix: &[scalar; 20]) {
        let paint = &mut Paint::default();
        paint.set_color_filter(&ColorFilters::matrix_row_major_255(color_matrix));

        let image = &resources::mandrill();

        c.draw_image(image, (x, y), Some(paint));
    }

    c.scale((0.25, 0.25));
    let color_matrix_1 = [
        0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        1.0, 0.0,
    ];
    f(c, (0.0, 0.0), &color_matrix_1);

    let grayscale = [
        0.21, 0.72, 0.07, 0.0, 0.0, 0.21, 0.72, 0.07, 0.0, 0.0, 0.21, 0.72, 0.07, 0.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
    ];
    f(c, (512.0, 0.0), &grayscale);
}

fn draw_color_table_color_filter(canvas: &mut Canvas) {
    let image = resources::mandrill();

    canvas.scale((0.5, 0.5));
    let ct = &mut [0u8; 256];
    for (i, v) in ct.iter_mut().enumerate() {
        let x = (i as i32 - 96) * 255 / 64;
        *v = x.max(0).min(255) as _;
    }
    let mut paint = Paint::default();
    paint.set_color_filter(&TableColorFilter::from_argb(
        None,
        Some(ct),
        Some(ct),
        Some(ct),
    ));
    canvas.draw_image(&image, (0, 0), Some(&paint));
}

fn draw_path_2d_effect(canvas: &mut Canvas) {
    let scale = 10.0;
    let path = &mut Path::default();
    let pts: [i8; 28] = [
        2, 2, 1, 3, 0, 3, 2, 1, 3, 1, 4, 0, 4, 1, 5, 1, 4, 2, 4, 3, 2, 5, 2, 4, 3, 3, 2, 3,
    ];
    path.move_to((2.0 * scale, 3.0 * scale));
    for i in (0..pts.len()).step_by(2) {
        path.line_to((
            scalar::from(pts[i]) * scale,
            scalar::from(pts[i + 1]) * scale,
        ));
    }
    path.close();
    let matrix = &Matrix::new_scale((4.0 * scale, 4.0 * scale));
    let paint = &mut Paint::default();
    paint
        .set_path_effect(&Path2DPathEffect::new(matrix, path))
        .set_anti_alias(true);
    canvas.clear(Color::WHITE);
    let bounds = Rect::new(-4.0 * scale, -4.0 * scale, 256.0, 256.0);
    canvas.draw_rect(bounds, paint);
}

fn draw_line_2d_effect(canvas: &mut Canvas) {
    let paint = &mut Paint::default();
    let lattice = &mut Matrix::default();
    lattice.set_scale((8.0, 8.0), None).pre_rotate(30.0, None);
    paint
        .set_path_effect(Line2DPathEffect::new(0.0, lattice).as_ref())
        .set_anti_alias(true);
    let bounds = Rect::from_size((256, 256)).with_outset((8.0, 8.0));
    canvas.clear(Color::WHITE);
    canvas.draw_rect(bounds, paint);
}

fn draw_path_1d_effect(canvas: &mut Canvas) {
    let paint = &mut Paint::default();
    let path = &mut Path::default();
    path.add_oval(Rect::from_size((16.0, 6.0)), None);
    paint
        .set_path_effect(
            Path1DPathEffect::new(path, 32.0, 0.0, Path1DPathEffectStyle::Rotate).as_ref(),
        )
        .set_anti_alias(true);
    canvas.clear(Color::WHITE);
    canvas.draw_circle((128.0, 128.0), 122.0, paint);
}

fn star() -> Path {
    const R: scalar = 115.2;
    const C: scalar = 128.0;
    let mut path = Path::default();
    path.move_to((C + R, C));
    for i in 1..8 {
        #[allow(clippy::excessive_precision)]
        let a = 2.692_793_7 * i as scalar;
        path.line_to((C + R * a.cos(), C + R * a.sin()));
    }
    path.close();
    path
}

fn draw_corner_path_effect(canvas: &mut Canvas) {
    let paint = &mut Paint::default();
    paint
        .set_path_effect(CornerPathEffect::new(32.0).as_ref())
        .set_style(PaintStyle::Stroke)
        .set_anti_alias(true);
    canvas.clear(Color::WHITE);
    canvas.draw_path(&star(), paint);
}

fn draw_dash_path_effect(canvas: &mut Canvas) {
    const INTERVALS: [scalar; 4] = [10.0, 5.0, 2.0, 5.0];
    let paint = &mut Paint::default();
    paint
        .set_path_effect(DashPathEffect::new(&INTERVALS, 0.0).as_ref())
        .set_style(PaintStyle::Stroke)
        .set_stroke_width(2.0)
        .set_anti_alias(true);
    canvas.clear(Color::WHITE);
    canvas.draw_path(&star(), paint);
}

fn draw_discrete_path_effect(canvas: &mut Canvas) {
    let paint = &mut Paint::default();
    paint
        .set_path_effect(DiscretePathEffect::new(10.0, 4.0, None).as_ref())
        .set_style(PaintStyle::Stroke)
        .set_stroke_width(2.0)
        .set_anti_alias(true);
    canvas.clear(Color::WHITE);
    canvas.draw_path(&star(), paint);
}

fn draw_compose_path_effect(canvas: &mut Canvas) {
    const INTERVALS: [scalar; 4] = [10.0, 5.0, 2.0, 5.0];
    let paint = &mut Paint::default();
    paint
        .set_path_effect(&PathEffect::compose(
            &DashPathEffect::new(&INTERVALS, 0.0).unwrap(),
            &DiscretePathEffect::new(10.0, 4.0, None).unwrap(),
        ))
        .set_style(PaintStyle::Stroke)
        .set_stroke_width(2.0)
        .set_anti_alias(true);
    canvas.clear(Color::WHITE);
    canvas.draw_path(&star(), paint);
}

fn draw_sum_path_effect(canvas: &mut Canvas) {
    let paint = &mut Paint::default();
    paint
        .set_path_effect(&PathEffect::sum(
            &DiscretePathEffect::new(10.0, 4.0, None).unwrap(),
            &DiscretePathEffect::new(10.0, 4.0, Some(1245)).unwrap(),
        ))
        .set_style(PaintStyle::Stroke)
        .set_stroke_width(2.0)
        .set_anti_alias(true);
    canvas.clear(Color::WHITE);
    canvas.draw_path(&star(), paint);
}
