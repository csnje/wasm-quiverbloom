/// Return pointer to allocated memory of specified size.
#[unsafe(no_mangle)]
pub extern "C" fn create_array(size: usize) -> *mut f64 {
    let mut data = Vec::with_capacity(size);
    let ptr = data.as_mut_ptr();
    std::mem::forget(data);
    ptr
}

/// Free pointer to previously allocated memory of specific size.
///
/// # Safety
///
/// The memory pointer must previously have been allocated for the specified size.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_array(ptr: *mut f64, size: usize) {
    unsafe {
        let _ = Vec::from_raw_parts(ptr, size, size);
    }
}

/// Get the number of algorithms.
#[unsafe(no_mangle)]
pub extern "C" fn num_algorithms() -> usize {
    8
}

/// Get the canvas width for the specified algorithm.
#[unsafe(no_mangle)]
pub extern "C" fn width(_algorithm: usize) -> usize {
    400
}

/// Get the canvas height for the specified algorithm.
#[unsafe(no_mangle)]
pub extern "C" fn height(_algorithm: usize) -> usize {
    400
}

/// Get the recommended number of points for the specified algorithm.
#[unsafe(no_mangle)]
pub extern "C" fn num_points(algorithm: usize) -> usize {
    match algorithm {
        6 => 20000,
        7 | 8 => 40000,
        _ => 10000,
    }
}

/// Get the points for a frame of the specified algorithm.
///
/// # Safety
///
/// The memory pointers must previously have been allocated for the specified size.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn frame_points(
    algorithm: usize,
    t: f64,
    xpts_ptr: *mut f64,
    ypts_ptr: *mut f64,
    size: usize,
) {
    let xpts = unsafe { std::slice::from_raw_parts_mut(xpts_ptr, size) };
    let ypts = unsafe { std::slice::from_raw_parts_mut(ypts_ptr, size) };

    match algorithm {
        1 => algo1_frame_points(t, xpts, ypts),
        2 => algo2_frame_points(t, xpts, ypts),
        3 => algo3_frame_points(t, xpts, ypts),
        4 => algo4_frame_points(t, xpts, ypts),
        5 => algo5_frame_points(t, xpts, ypts),
        6 => algo6_frame_points(t, xpts, ypts),
        7 => algo7_frame_points(t, xpts, ypts),
        8 => algo8_frame_points(t, xpts, ypts),
        _ => panic!(),
    }
}

/// Algorithm by [yuruyurau](https://x.com/yuruyurau) at
/// https://x.com/yuruyurau/status/1942231466446057727.
fn algo1_frame_points(t: f64, xpts: &mut [f64], ypts: &mut [f64]) {
    let t = std::f64::consts::TAU * (t % 1.0);
    for (i, (xpt, ypt)) in std::iter::zip(xpts, ypts).enumerate() {
        let (x, y) = (i as f64, i as f64 / 235.0);
        let k = (4.0 + (x / 11.0 + t * 8.0).sin()) * (x / 14.0).cos();
        let e = y / 8.0 - 19.0;
        let d = (k * k + e * e).sqrt() + (y / 9.0 + t * 2.0).sin();
        let q = 2.0 * (k * 2.0).sin() + (y / 17.0).sin() * k * (9.0 + 2.0 * (y - d * 3.0).sin());
        let c = d * d / 49.0 - t;
        *xpt = q + 50.0 * c.cos() + 200.0;
        *ypt = q * c.sin() + 39.0 * d - 440.0;
    }
}

/// Algorithm by [yuruyurau](https://x.com/yuruyurau) at
/// https://x.com/yuruyurau/status/1943684973199819135.
fn algo2_frame_points(t: f64, xpts: &mut [f64], ypts: &mut [f64]) {
    let t = std::f64::consts::TAU * (t % 1.0);
    for (i, (xpt, ypt)) in std::iter::zip(xpts, ypts).enumerate() {
        let (x, y) = (i as f64, i as f64 / 41.0);
        let k = 5.0 * (x / 19.0).cos() * (y / 30.0).cos();
        let e = y / 8.0 - 12.0;
        let d = (k * k + e * e) / 59.0 + 2.0;
        let q = 4.0 * (k.atan2(e) * 9.0).sin() + 9.0 * (d - t).sin()
            - k / d * (9.0 + (d * 9.0 - t * 16.0).sin() * 3.0);
        let c = d * d / 7.0 - t;
        *xpt = q + 50.0 * c.cos() + 200.0;
        *ypt = q * c.sin() + d * 45.0 - 9.0;
    }
}

/// Algorithm by [yuruyurau](https://x.com/yuruyurau) at
/// https://x.com/yuruyurau/status/1936449579161092595.
fn algo3_frame_points(t: f64, xpts: &mut [f64], ypts: &mut [f64]) {
    let t: f64 = std::f64::consts::TAU * (t * 4.0 % 4.0);
    for (i, (xpt, ypt)) in std::iter::zip(xpts, ypts).enumerate() {
        let (x, y) = (i as f64, i as f64 / 235.0);
        let k = (4.0 + y.cos()) * (x / 4.0).cos();
        let e = y / 8.0 - 20.0;
        let d = (k * k + e * e).sqrt();
        let q = (k * 3.0).sin() + (y / 19.0 + 9.0).sin() * k * (6.0 + (e * 14.0 - d).sin());
        let c = d - t;
        *xpt = q * (d / 8.0 + t / 4.0).cos() + 50.0 * c.cos() + 200.0;
        *ypt = q * c.sin() + d * 7.0 * (c / 4.0).sin() + 200.0;
    }
}

/// Algorithm by [yuruyurau](https://x.com/yuruyurau) at
/// https://x.com/yuruyurau/status/1933629116575855091.
fn algo4_frame_points(t: f64, xpts: &mut [f64], ypts: &mut [f64]) {
    let t: f64 = std::f64::consts::TAU * (t % 1.0);
    for (i, (xpt, ypt)) in std::iter::zip(xpts, ypts).enumerate() {
        let (x, y) = (i as f64, i as f64 / 235.0);
        let k = (4.0 + (y * 2.0 - t).sin() * 3.0) * (x / 29.0).cos();
        let e = y / 8.0 - 13.0;
        let d = (k * k + e * e).sqrt();
        let q = 3.0 * (k * 2.0).sin()
            + 0.3 / k
            + (y / 25.0).sin() * k * (9.0 + 4.0 * (e * 9.0 - d * 3.0 + t * 2.0).sin());
        let c = d - t;
        *xpt = q + 30.0 * c.cos() + 200.0;
        *ypt = q * c.sin() + d * 39.0 - 220.0;
    }
}

/// Algorithm by [yuruyurau](https://x.com/yuruyurau) at
/// https://x.com/yuruyurau/status/1927373647125119025.
fn algo5_frame_points(t: f64, xpts: &mut [f64], ypts: &mut [f64]) {
    let t: f64 = std::f64::consts::TAU * (t * 16.0 % 16.0);
    for (i, (xpt, ypt)) in std::iter::zip(xpts, ypts).enumerate() {
        let (x, y) = (i as f64 % 200.0, i as f64 / 55.0);
        let k = 9.0 * (x / 8.0).cos();
        let e = y / 8.0 - 12.5;
        let d = (k * k + e * e) / 99.0 + t.sin() / 6.0 + 0.5;
        let q = 99.0 - e * (k.atan2(e) * 7.0).sin() / d + k * (3.0 + (d * d - t).cos() * 2.0);
        let c = d / 2.0 + e / 69.0 - t / 16.0;
        *xpt = q * c.sin() + 200.0;
        *ypt = (q + 19.0 * d) * c.cos() + 200.0;
    }
}

/// Algorithm by [yuruyurau](https://x.com/yuruyurau) at
/// https://x.com/yuruyurau/status/1925557708817932636.
fn algo6_frame_points(t: f64, xpts: &mut [f64], ypts: &mut [f64]) {
    let t: f64 = std::f64::consts::TAU * (t * 16.0 % 16.0);
    for (i, (xpt, ypt)) in std::iter::zip(xpts, ypts).enumerate() {
        let (x, y) = (i as f64 % 100.0, i as f64 / 350.0);
        let k = x / 4.0 - 12.5;
        let e = y / 9.0;
        let o = (k * k + e * e).sqrt() / 9.0;
        let q = 99.0
            + 3.0 * ((y / 2.0).tan() / 2.0 + y.cos()) / k
            + k * (3.0 + y.cos() / 3.0 + (e + o * 4.0 - t * 2.0).sin());
        let c = o / 4.0 + e / 4.0 - t / 8.0;
        *xpt = q * c.cos() * (c / 2.0 - e / 3.0 + t / 8.0).cos() + 200.0;
        *ypt = q * c.sin() + 200.0;
    }
}

/// Algorithm by [yuruyurau](https://x.com/yuruyurau) at
/// https://x.com/yuruyurau/status/1877743319205433558.
fn algo7_frame_points(t: f64, xpts: &mut [f64], ypts: &mut [f64]) {
    let t: f64 = std::f64::consts::TAU * (t % 1.0);
    for (i, (xpt, ypt)) in std::iter::zip(xpts, ypts).enumerate() {
        let (x, y) = (i as f64 % 200.0, i as f64 / 200.0);
        let k = x / 8.0 - 12.5;
        let e = y / 8.0 - 12.0;
        let o = 3.0 - (k * k + e * e).sqrt() / 3.0;
        let d = -4.0 * ((k / 2.0).sin() * e.cos());
        *xpt = (x + e * t.cos() + d * k * (d + t).sin()) * 0.7 + k * o + 130.0;
        *ypt = (y - d * 19.0 + d * e * (d + t).cos()) * 0.7 + 130.0;
    }
}

/// Algorithm by [yuruyurau](https://x.com/yuruyurau) at
/// https://x.com/yuruyurau/status/1877743319205433558.
fn algo8_frame_points(t: f64, xpts: &mut [f64], ypts: &mut [f64]) {
    let t: f64 = std::f64::consts::TAU * (t % 1.0);
    for (i, (xpt, ypt)) in std::iter::zip(xpts, ypts).enumerate() {
        let (x, y) = (i as f64 % 200.0, i as f64 / 200.0);
        let k = x / 8.0 - 12.5;
        let e = y / 8.0 - 12.5;
        let o = (k * k + e * e).sqrt() / 12.0 * ((k / 2.0).sin() * (e / 2.0).cos()).cos();
        let d = 5.0 * o.cos();
        *xpt = (x + d * k * ((d * 2.0 + t).sin() + (y * o * o).sin() / 9.0)) / 1.5 + 133.0;
        *ypt = (y / 3.0 - d * 40.0 + 19.0 * (d + t).cos()) * 1.5 + 300.0;
    }
}
