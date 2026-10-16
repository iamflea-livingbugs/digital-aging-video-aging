//! PAL 视频制式模拟的行为测试。
//!
//! 验证路线 A 的三个核心行为：
//! 1. PAL 模式与 NTSC 模式输出不同（延迟线平均改变了色度）
//! 2. PAL 的相位误差主要表现为饱和度下降，而 NTSC 表现为色调偏移
//! 3. 无相位误差时 PAL 能基本还原原始色彩（调制/解调往返自洽）

use ntsc_rs::ctx::Context;
use ntsc_rs::settings::standard::{NtscEffect, UseField, VideoStandard};
use ntsc_rs::yiq_fielding::Rgb;

const WIDTH: usize = 32;
const HEIGHT: usize = 32;

/// 生成高饱和度测试图：横竖两个方向的强色彩渐变。
fn make_test_image() -> Vec<u8> {
    let mut buf = vec![0u8; WIDTH * HEIGHT * 3];
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let px = (y * WIDTH + x) * 3;
            buf[px] = ((x * 255) / (WIDTH - 1)) as u8;
            buf[px + 1] = ((y * 255) / (HEIGHT - 1)) as u8;
            buf[px + 2] = 200;
        }
    }
    buf
}

/// 构造一个隔离了随机噪声干扰的效果实例，便于行为对比。
fn test_effect(standard: VideoStandard, phase_error: f32) -> NtscEffect {
    let mut effect = NtscEffect::default();
    effect.video_standard = standard;
    // 逐行处理，避免隔行扫描引入额外变量
    effect.use_field = UseField::Both;
    // 关掉所有随机噪声，让相位误差成为唯一变量
    effect.composite_noise.enabled = false;
    effect.luma_noise.enabled = false;
    effect.chroma_noise.enabled = false;
    effect.snow_intensity = 0.0;
    effect.chroma_phase_noise_intensity = 0.0;
    effect.chroma_phase_error = phase_error;
    effect
}

fn render(effect: &NtscEffect, source: &[u8]) -> Vec<u8> {
    let ctx = Context::new();
    let mut buf = source.to_vec();
    effect.apply_effect_to_buffer::<Rgb, u8>(&ctx, (WIDTH, HEIGHT), &mut buf, 0, [1.0, 1.0]);
    buf
}

/// 两个缓冲区之间的平均逐像素绝对差。
fn mean_abs_diff(a: &[u8], b: &[u8]) -> f32 {
    a.iter()
        .zip(b.iter())
        .map(|(&x, &y)| (x as f32 - y as f32).abs())
        .sum::<f32>()
        / a.len() as f32
}

#[test]
fn pal_output_differs_from_ntsc() {
    let image = make_test_image();
    let ntsc = render(&test_effect(VideoStandard::Ntsc, 0.0), &image);
    let pal = render(&test_effect(VideoStandard::Pal, 0.0), &image);
    // PAL 的延迟线平均必然改变色度，输出应与 NTSC 有可测量差异
    assert!(
        mean_abs_diff(&ntsc, &pal) > 0.01,
        "PAL and NTSC outputs should differ"
    );
}

#[test]
fn pal_phase_error_desaturates_instead_of_shifting_hue() {
    let image = make_test_image();
    // 0.1 对应 36° 相位旋转
    let phase_error = 0.1;

    // 用"有误差 vs 无误差"的渲染差异隔离相位误差的边际影响，
    // 排除默认滤镜链本身对画面的基础改造
    let ntsc_clean = render(&test_effect(VideoStandard::Ntsc, 0.0), &image);
    let ntsc_shifted = render(&test_effect(VideoStandard::Ntsc, phase_error), &image);
    let pal_clean = render(&test_effect(VideoStandard::Pal, 0.0), &image);
    let pal_shifted = render(&test_effect(VideoStandard::Pal, phase_error), &image);

    let ntsc_marginal = mean_abs_diff(&ntsc_shifted, &ntsc_clean);
    let pal_marginal = mean_abs_diff(&pal_shifted, &pal_clean);
    eprintln!("ntsc_marginal={ntsc_marginal} pal_marginal={pal_marginal}");

    // NTSC：36° 色调旋转对整个色度向量产生巨大偏移
    assert!(
        ntsc_marginal > 3.0,
        "NTSC phase error should visibly shift hue (marginal={ntsc_marginal})"
    );
    // PAL：色调被逐行倒相+行间平均纠正，只剩 cos(36°)≈0.81 倍的饱和度损失。
    // 色度向量长度缩放产生的 RGB 变化（2·(1-cosφ)）远小于旋转（2·sin(φ/2)）
    assert!(
        pal_marginal < ntsc_marginal * 0.5,
        "PAL phase error should mostly desaturate (pal={pal_marginal}, ntsc={ntsc_marginal})"
    );
}

#[test]
fn pal_clean_signal_roundtrips_as_well_as_ntsc() {
    let image = make_test_image();
    let pal = render(&test_effect(VideoStandard::Pal, 0.0), &image);
    let ntsc = render(&test_effect(VideoStandard::Ntsc, 0.0), &image);
    // 无相位误差时，PAL 调制/解调往返的保真度应与 NTSC 相当
    //（默认滤镜链本身会改造画面，所以比较两种制式对原图的偏差是否一致）
    let pal_diff = mean_abs_diff(&pal, &image);
    let ntsc_diff = mean_abs_diff(&ntsc, &image);
    eprintln!("pal_clean={pal_diff} ntsc_clean={ntsc_diff}");
    assert!(
        (pal_diff - ntsc_diff).abs() < 2.0,
        "PAL roundtrip fidelity should match NTSC (pal={pal_diff}, ntsc={ntsc_diff})"
    );
}
