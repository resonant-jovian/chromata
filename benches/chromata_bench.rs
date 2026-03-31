use criterion::{Criterion, black_box, criterion_group, criterion_main};

use chromata::{Color, Contrast, Theme, Variant};

fn color_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("color");

    group.bench_function("from_hex", |b| {
        b.iter(|| Color::from_hex(black_box(0x1d2021)));
    });

    group.bench_function("from_css_hex_6", |b| {
        b.iter(|| Color::from_css_hex(black_box("#1d2021")));
    });

    group.bench_function("from_css_hex_3", |b| {
        b.iter(|| Color::from_css_hex(black_box("#FFF")));
    });

    let color = Color::from_hex(0x1d2021);

    group.bench_function("to_hex", |b| {
        b.iter(|| black_box(color).to_hex());
    });

    group.bench_function("to_css_hex", |b| {
        b.iter(|| black_box(color).to_css_hex());
    });

    group.bench_function("to_f32", |b| {
        b.iter(|| black_box(color).to_f32());
    });

    group.finish();
}

fn wcag_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("wcag");

    let color = Color::from_hex(0x1d2021);
    let white = Color::new(255, 255, 255);

    group.bench_function("luminance", |b| {
        b.iter(|| black_box(color).luminance());
    });

    group.bench_function("contrast_ratio", |b| {
        b.iter(|| black_box(color).contrast_ratio(black_box(white)));
    });

    group.bench_function("contrast_from_ratio", |b| {
        b.iter(|| Contrast::from_ratio(black_box(7.5)));
    });

    group.finish();
}

fn interpolation_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("interpolation");

    let black = Color::new(0, 0, 0);
    let white = Color::new(255, 255, 255);

    group.bench_function("lerp", |b| {
        b.iter(|| black_box(black).lerp(black_box(white), black_box(0.5)));
    });

    group.finish();
}

fn theme_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("theme");

    group.bench_function("find_by_name_hit", |b| {
        b.iter(|| chromata::find_by_name(black_box("Gruvbox Dark Hard")));
    });

    group.bench_function("find_by_name_miss", |b| {
        b.iter(|| chromata::find_by_name(black_box("Nonexistent Theme")));
    });

    group.bench_function("collect_all_themes", |b| {
        b.iter(chromata::collect_all_themes);
    });

    group.bench_function("filter_by_variant", |b| {
        b.iter(|| chromata::filter_by_variant(black_box(Variant::Dark)));
    });

    group.bench_function("filter_by_contrast", |b| {
        b.iter(|| chromata::filter_by_contrast(black_box(Contrast::Normal)));
    });

    group.finish();
}

fn builder_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("builder");

    let bg = Color::new(0, 0, 0);
    let fg = Color::new(255, 255, 255);

    group.bench_function("minimal_build", |b| {
        b.iter(|| {
            Theme::builder(
                black_box("Bench Theme"),
                black_box("Author"),
                black_box(bg),
                black_box(fg),
            )
            .build()
        });
    });

    group.bench_function("full_build", |b| {
        b.iter(|| {
            Theme::builder(
                black_box("Bench Theme"),
                black_box("Author"),
                black_box(bg),
                black_box(fg),
            )
            .keyword(Color::from_hex(0xff79c6))
            .string(Color::from_hex(0xf1fa8c))
            .function(Color::from_hex(0x50fa7b))
            .comment(Color::from_hex(0x6272a4))
            .red(Color::from_hex(0xff5555))
            .green(Color::from_hex(0x50fa7b))
            .blue(Color::from_hex(0x6272a4))
            .build()
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    color_benchmarks,
    wcag_benchmarks,
    interpolation_benchmarks,
    theme_benchmarks,
    builder_benchmarks,
);
criterion_main!(benches);
