#![cfg(target_feature = "sse")]
#![cfg(feature = "bytemuck")]

use bytemuck;
use safe_arch::*;

type Point2D = [f32; 2];
const PLAYER_POS: Point2D = [128.0, 128.0];
const MAX_DISTANCE: f32 = 16.0;

/* SIMD operations */
fn simd_sub_and_square(xyxy: m128, player_pos: m128) -> m128 {
    let xyxy = xyxy - player_pos;
    xyxy * xyxy
}

fn simd_is_close(xyxy: m128, max_distance: m128) -> i32 {
    let results = cmp_lt_mask_m128(xyxy, max_distance);
    return move_mask_m128(results);
}

fn simd_objects_close(x: &[Point2D]) -> usize {
    let player_pos: m128 = [PLAYER_POS[0], PLAYER_POS[1], PLAYER_POS[0], PLAYER_POS[1]].into();
    let max_distances = load_f32_splat_m128(&MAX_DISTANCE);
    let max_distances_squared = max_distances * max_distances;

    let (begin, meat, end) = bytemuck::pod_align_to(x);
    let mut it = meat.chunks_exact(2);
    let mut result = objects_close(begin);

    while let Some(chunk) = it.next() {
        let distances_squared = add_horizontal_m128(
            simd_sub_and_square(chunk[0], player_pos),
            simd_sub_and_square(chunk[1], player_pos),
        );
        let results = simd_is_close(distances_squared, max_distances_squared);
        result += results.count_ones() as usize;
    }

    if let Some(remainder) = it.remainder().get(0) {
        let xyxy = simd_sub_and_square(*remainder, player_pos);
        let distances_squared = add_horizontal_m128(xyxy, xyxy);
        let results = simd_is_close(distances_squared, max_distances_squared);
        result += results.count_ones() as usize / 2;
    }

    return result + objects_close(end);
}

/* Normal operations */
fn sub_and_square(xy: Point2D, player_pos: Point2D) -> Point2D {
    let xy = [xy[0] - player_pos[0], xy[1] - player_pos[1]];
    return [xy[0]*xy[0], xy[1]*xy[1]];
}

fn is_close(xy: &Point2D) -> bool {
    let squared = sub_and_square(*xy, PLAYER_POS);
    let distance_squared = squared[0] + squared[1];
    return distance_squared < MAX_DISTANCE * MAX_DISTANCE;
}

fn objects_close(x: &[Point2D]) -> usize {
    x.iter().copied().filter(is_close).count()
}

/* Generation utils */
fn poor_rng(x: u16) -> u16 {
    let x = x ^ 0xC0DE;
    (x >> 3) | (x << 1)
}

fn random_positions(n: usize) -> Vec<Point2D> {
    let mut vec = Vec::with_capacity(n);
    let mut rng = 0;
    let mut pos: Point2D = Default::default();

    for _ in 0..n {
        for i in 0..2 {
            rng = poor_rng(rng);
            pos[i] = rng as f32 / 256.0;
        }

        vec.push(pos);
    }

    return vec;
}

#[test]
fn test_points_pythagoras() {
    let pos = random_positions(1024 + 1);
    assert_eq!(objects_close(&pos), simd_objects_close(&pos));
}

#[test]
fn test_points_pythagoras_zero() {
    let pos = random_positions(0);
    assert_eq!(objects_close(&pos), simd_objects_close(&pos));
}

#[test]
fn test_points_pythagoras_one() {
    let pos = random_positions(1);
    assert_eq!(objects_close(&pos), simd_objects_close(&pos));
}

#[test]
fn test_points_pythagoras_long() {
    let mut rng = 0;
    for _ in 0..128 {
        rng = poor_rng(rng);
        let pos = random_positions(rng as usize);
        assert_eq!(objects_close(&pos), simd_objects_close(&pos));
    }
}
