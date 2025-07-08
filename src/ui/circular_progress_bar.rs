use core::f32;
use eframe::egui;
use egui::{Color32, Pos2, Shape, Stroke, Vec2};
use std::f32::consts::{FRAC_PI_2, TAU};

pub fn draw(
    ui: &mut egui::Ui,
    width: f32,
    height: f32,
    color: Color32,
    frac: f32,
    text: Option<&str>,
) {
    let desired_size = Vec2 {
        x: width,
        y: height,
    };

    let (response, mut painter) = ui.allocate_painter(desired_size, egui::Sense::hover());

    let center = response.rect.center();
    let radius = response.rect.width().min(response.rect.height()) / 2.0 - 4.0;

    if frac >= 1.0 {
        painter.circle_filled(center, radius, color);
    } else {
        draw_segments(&mut painter, center, radius, color, frac);
    }

    if let Some(text) = text {
        painter.text(
            center,
            egui::Align2::CENTER_CENTER,
            text,
            egui::FontId::proportional(32.0),
            Color32::WHITE,
        );
    }

    let stroke_color = color.linear_multiply(0.05);
    painter.circle_stroke(center, radius, Stroke::new(2.0, stroke_color));
}

pub fn draw_segments(
    painter: &mut egui::Painter,
    center: Pos2,
    radius: f32,
    color: Color32,
    frac: f32,
) {
    let bg_color = color.linear_multiply(0.05);
    let error_margin = 0.01;

    if frac < error_margin || frac > 1.0 - error_margin {
        painter.circle_filled(center, radius, color);
        return;
    }

    if frac.abs() < error_margin {
        painter.circle_filled(center, radius, bg_color);
    }

    const MAX_SEGMENTS: usize = 100;

    let filled_segments = ((MAX_SEGMENTS as f32) * frac).ceil() as usize;
    let empty_segments = MAX_SEGMENTS - filled_segments;

    let mut filled = vec![center];
    let mut empty = vec![center];

    let filled_angle = frac * TAU;
    let empty_angle = TAU - filled_angle;

    for i in 0..=filled_segments {
        let segment = make_segment(-FRAC_PI_2, filled_angle, i, filled_segments, center, radius);
        filled.push(segment);
    }

    for i in 0..=empty_segments {
        let segment = make_segment(
            -FRAC_PI_2 + filled_angle,
            empty_angle,
            i,
            empty_segments,
            center,
            radius,
        );
        empty.push(segment);
    }

    painter.add(Shape::convex_polygon(filled, color, Stroke::NONE));
    painter.add(Shape::convex_polygon(empty, bg_color, Stroke::NONE));
}

fn make_segment(
    start_angle: f32,
    sweep_angle: f32,
    i: usize,
    total: usize,
    center: Pos2,
    radius: f32,
) -> Pos2 {
    let t = i as f32 / total as f32;
    let theta = start_angle + sweep_angle * t;
    Pos2 {
        x: center.x + radius * theta.cos(),
        y: center.y + radius * theta.sin(),
    }
}
