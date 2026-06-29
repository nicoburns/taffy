//! Regression tests for grid track-count overflows.
//!
//! Grid lines are stored internally as `i16`, so track counts (whether from huge
//! `repeat()` counts in a grid template or from very large explicit grid line
//! indices on grid items) that overflow `i16::MAX` used to cause panics (integer
//! overflow / out-of-range indexing).
//!
//! These cases mirror the following crashes found by Servo's fuzzers:
//!   - <https://github.com/servo/servo/issues/45881>
//!   - <https://github.com/servo/servo/issues/45949>
//!   - <https://github.com/servo/servo/issues/45939>
//!   - <https://github.com/servo/servo/issues/45938>
//!   - <https://github.com/servo/servo/issues/46081>
//!
//! Taffy should clamp the grid to a sane maximum size rather than panicking.
//!
//! Note: a grid only runs the grid layout algorithm when it has at least one
//! child, so every test gives the grid container a child.
#![cfg(feature = "grid")]

use taffy::prelude::*;
use taffy_test_helpers::new_test_tree;

/// <https://github.com/servo/servo/issues/45881>
///
/// A grid template whose combined track count overflows `u16`/`i16`.
/// `repeat(40000, …) repeat(40000, …)` => 80000 tracks (overflows `u16`).
#[test]
fn huge_template_track_count_does_not_panic() {
    let mut taffy = new_test_tree();
    let child = taffy.new_leaf(Style::default()).unwrap();
    let root = taffy
        .new_with_children(
            Style {
                display: Display::Grid,
                grid_template_rows: vec![
                    repeat(40000u16, vec![fr(65.0)]),
                    repeat(40000u16, vec![minmax(percent(0.99), fr(0.0))]),
                ],
                ..Default::default()
            },
            &[child],
        )
        .unwrap();

    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
}

/// A single `repeat()` whose `count * tracks_per_repetition` overflows `u16`.
#[test]
fn huge_repeat_multiplication_does_not_panic() {
    let mut taffy = new_test_tree();
    let child = taffy.new_leaf(Style::default()).unwrap();
    let root = taffy
        .new_with_children(
            Style {
                display: Display::Grid,
                grid_template_columns: vec![repeat(40000u16, vec![length(1.0), length(1.0)])],
                ..Default::default()
            },
            &[child],
        )
        .unwrap();

    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
}

/// <https://github.com/servo/servo/issues/45949>
///
/// `repeat(32768, fit-content(512px)) fit-content(100%)` => 32769 explicit tracks,
/// which overflows `i16::MAX` (32767) when converted to grid-line coordinates.
#[test]
fn explicit_track_count_above_i16_max_does_not_panic() {
    let mut taffy = new_test_tree();
    let child = taffy.new_leaf(Style::default()).unwrap();
    let root = taffy
        .new_with_children(
            Style {
                display: Display::Grid,
                grid_template_columns: vec![
                    repeat(32768u16, vec![fit_content(length(512.0))]),
                    fit_content(percent(1.0)),
                ],
                ..Default::default()
            },
            &[child],
        )
        .unwrap();

    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
}

/// <https://github.com/servo/servo/issues/46081>
///
/// `repeat(auto-fill, 0px)` in a definite-size container: a zero-sized repeated
/// track makes the "number of repetitions that fit" computation infinite.
#[test]
fn auto_fill_zero_sized_track_does_not_panic() {
    let mut taffy = new_test_tree();
    let child = taffy.new_leaf(Style::default()).unwrap();
    let root = taffy
        .new_with_children(
            Style {
                display: Display::Grid,
                size: Size { width: length(100.0), height: length(100.0) },
                grid_template_columns: vec![repeat(RepetitionCount::AutoFill, vec![length(0.0)])],
                ..Default::default()
            },
            &[child],
        )
        .unwrap();

    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
}

/// <https://github.com/servo/servo/issues/45939> and
/// <https://github.com/servo/servo/issues/45938>
///
/// A grid item placed at a very large explicit line index (combined with a span
/// that pushes its end line past `i16::MAX`) forces the implicit grid to grow
/// past `i16::MAX`.
#[test]
fn huge_grid_item_line_index_does_not_panic() {
    let mut taffy = new_test_tree();
    let child =
        taffy.new_leaf(Style { grid_row: Line { start: line(32000), end: span(2000) }, ..Default::default() }).unwrap();
    let root = taffy
        .new_with_children(
            Style { display: Display::Grid, grid_auto_flow: GridAutoFlow::Column, ..Default::default() },
            &[child],
        )
        .unwrap();

    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
}

/// A grid item spanning a huge number of tracks forces the implicit grid past `i16::MAX`.
#[test]
fn huge_grid_item_span_does_not_panic() {
    let mut taffy = new_test_tree();
    let child =
        taffy.new_leaf(Style { grid_column: Line { start: auto(), end: span(40000) }, ..Default::default() }).unwrap();
    let root = taffy.new_with_children(Style { display: Display::Grid, ..Default::default() }, &[child]).unwrap();

    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
}

/// A negative grid line far below `-i16::MAX` (via a large negative end line
/// combined with a backwards span) forces the negative implicit grid past `i16::MAX`.
#[test]
fn huge_negative_grid_item_line_index_does_not_panic() {
    let mut taffy = new_test_tree();
    let child = taffy
        .new_leaf(Style { grid_column: Line { start: span(2000), end: line(-32000) }, ..Default::default() })
        .unwrap();
    let root = taffy.new_with_children(Style { display: Display::Grid, ..Default::default() }, &[child]).unwrap();

    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
}
