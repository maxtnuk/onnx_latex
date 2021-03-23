use crate::frame::mmm::*;
use crate::frame::sigmoid::*;
use crate::frame::tanh::*;

extern "C" {
    fn arm64simd_mmm_f32_8x8_a53(op: *const MatMatMulKerSpec<f32>) -> isize;
    fn arm64simd_mmm_f32_8x8_gen(op: *const MatMatMulKerSpec<f32>) -> isize;
    fn arm64simd_mmm_f32_12x8_a53(op: *const MatMatMulKerSpec<f32>) -> isize;
    fn arm64simd_mmm_f32_12x8_gen(op: *const MatMatMulKerSpec<f32>) -> isize;
    fn arm64simd_mmm_f32_64x1_a53(op: *const MatMatMulKerSpec<f32>) -> isize;
    fn arm64simd_mmm_f32_64x1_gen(op: *const MatMatMulKerSpec<f32>) -> isize;
    fn arm64simd_mmm_i8_8x8(op: *const MatMatMulKerSpec<i32>) -> isize;
    fn arm64simd_sigmoid_f32_4n(ptr: *mut f32, count: usize);
    fn arm64simd_tanh_f32_4n(ptr: *mut f32, count: usize);
}

#[derive(Copy, Clone, Debug)]
pub struct MatMatMulF32x8x8A53;

impl MatMatMulKer<f32> for MatMatMulF32x8x8A53 {
    #[inline(always)]
    fn name() -> &'static str {
        "arm64simd (cortex A53)"
    }
    #[inline(always)]
    fn mr() -> usize {
        8
    }
    #[inline(always)]
    fn nr() -> usize {
        8
    }
    fn alignment_bytes_packed_a() -> usize {
        16
    }
    fn alignment_bytes_packed_b() -> usize {
        16
    }
    fn end_padding_packed_a() -> usize {
        1
    }
    fn end_padding_packed_b() -> usize {
        1
    }
    #[inline(never)]
    fn kernel(op: &MatMatMulKerSpec<f32>) -> isize {
        unsafe { arm64simd_mmm_f32_8x8_a53(op) }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MatMatMulF32x12x8A53;

impl MatMatMulKer<f32> for MatMatMulF32x12x8A53 {
    #[inline(always)]
    fn name() -> &'static str {
        "arm64simd (cortex A53)"
    }
    #[inline(always)]
    fn mr() -> usize {
        12
    }
    #[inline(always)]
    fn nr() -> usize {
        8
    }
    fn alignment_bytes_packed_a() -> usize {
        16
    }
    fn alignment_bytes_packed_b() -> usize {
        16
    }
    fn end_padding_packed_a() -> usize {
        1
    }
    fn end_padding_packed_b() -> usize {
        1
    }
    #[inline(never)]
    fn kernel(op: &MatMatMulKerSpec<f32>) -> isize {
        unsafe { arm64simd_mmm_f32_12x8_a53(op) }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MatMatMulF32x64x1A53;

impl MatMatMulKer<f32> for MatMatMulF32x64x1A53 {
    #[inline(always)]
    fn name() -> &'static str {
        "arm64simd (cortex A53)"
    }
    #[inline(always)]
    fn mr() -> usize {
        64
    }
    #[inline(always)]
    fn nr() -> usize {
        1
    }
    fn alignment_bytes_packed_a() -> usize {
        16
    }
    fn alignment_bytes_packed_b() -> usize {
        16
    }
    fn end_padding_packed_a() -> usize {
        1
    }
    fn end_padding_packed_b() -> usize {
        1
    }
    #[inline(never)]
    fn kernel(op: &MatMatMulKerSpec<f32>) -> isize {
        unsafe { arm64simd_mmm_f32_64x1_a53(op) }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MatMatMulF32x8x8;

impl MatMatMulKer<f32> for MatMatMulF32x8x8 {
    #[inline(always)]
    fn name() -> &'static str {
        "arm64simd"
    }
    #[inline(always)]
    fn mr() -> usize {
        8
    }
    #[inline(always)]
    fn nr() -> usize {
        8
    }
    fn alignment_bytes_packed_a() -> usize {
        16
    }
    fn alignment_bytes_packed_b() -> usize {
        16
    }
    fn end_padding_packed_a() -> usize {
        1
    }
    fn end_padding_packed_b() -> usize {
        1
    }
    #[inline(never)]
    fn kernel(op: &MatMatMulKerSpec<f32>) -> isize {
        unsafe { arm64simd_mmm_f32_8x8_gen(op) }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MatMatMulF32x12x8;

impl MatMatMulKer<f32> for MatMatMulF32x12x8 {
    #[inline(always)]
    fn name() -> &'static str {
        "arm64simd"
    }
    #[inline(always)]
    fn mr() -> usize {
        12
    }
    #[inline(always)]
    fn nr() -> usize {
        8
    }
    fn alignment_bytes_packed_a() -> usize {
        16
    }
    fn alignment_bytes_packed_b() -> usize {
        16
    }
    fn end_padding_packed_a() -> usize {
        1
    }
    fn end_padding_packed_b() -> usize {
        1
    }
    #[inline(never)]
    fn kernel(op: &MatMatMulKerSpec<f32>) -> isize {
        unsafe { arm64simd_mmm_f32_12x8_gen(op) }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MatMatMulF32x64x1;

impl MatMatMulKer<f32> for MatMatMulF32x64x1 {
    #[inline(always)]
    fn name() -> &'static str {
        "arm64simd"
    }
    #[inline(always)]
    fn mr() -> usize {
        64
    }
    #[inline(always)]
    fn nr() -> usize {
        1
    }
    fn alignment_bytes_packed_a() -> usize {
        16
    }
    fn alignment_bytes_packed_b() -> usize {
        16
    }
    fn end_padding_packed_a() -> usize {
        1
    }
    fn end_padding_packed_b() -> usize {
        1
    }
    #[inline(never)]
    fn kernel(op: &MatMatMulKerSpec<f32>) -> isize {
        unsafe { arm64simd_mmm_f32_64x1_gen(op) }
    }
}


#[derive(Copy, Clone, Debug)]
pub struct MatMatMulI8x8x8;

impl MatMatMulKer<i32> for MatMatMulI8x8x8 {
    #[inline(always)]
    fn name() -> &'static str {
        "arm64simd"
    }
    #[inline(always)]
    fn mr() -> usize {
        8
    }
    #[inline(always)]
    fn nr() -> usize {
        8
    }
    fn alignment_bytes_packed_a() -> usize {
        16
    }
    fn alignment_bytes_packed_b() -> usize {
        16
    }
    fn end_padding_packed_a() -> usize {
        0
    }
    fn end_padding_packed_b() -> usize {
        0
    }
    #[inline(never)]
    fn kernel(op: &MatMatMulKerSpec<i32>) -> isize {
        unsafe { arm64simd_mmm_i8_8x8(op) }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MatMatMulI8xI32x8x8;

impl MatMatMulKer<i32> for MatMatMulI8xI32x8x8 {
    #[inline(always)]
    fn name() -> &'static str {
        "arm64simd"
    }
    #[inline(always)]
    fn mr() -> usize {
        8
    }
    #[inline(always)]
    fn nr() -> usize {
        8
    }
    fn alignment_bytes_packed_a() -> usize {
        16
    }
    fn alignment_bytes_packed_b() -> usize {
        16
    }
    fn end_padding_packed_a() -> usize {
        0
    }
    fn end_padding_packed_b() -> usize {
        0
    }
    #[inline(never)]
    fn kernel(op: &MatMatMulKerSpec<i32>) -> isize {
        unsafe { arm64simd_mmm_i8_8x8(op as *const _ as _) }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct SigmoidF32x4n;

impl SigmoidKer<f32> for SigmoidF32x4n {
    #[inline(always)]
    fn name() -> &'static str {
        "arm64simd"
    }
    #[inline(always)]
    fn nr() -> usize {
        4
    }
    #[inline(always)]
    fn alignment_bytes() -> usize {
        16
    }
    #[inline(never)]
    fn run(buf: &mut [f32]) {
        unsafe { arm64simd_sigmoid_f32_4n(buf.as_mut_ptr(), buf.len()) }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct TanhF32x4n;

impl TanhKer<f32> for TanhF32x4n {
    #[inline(always)]
    fn name() -> &'static str {
        "arm64simd"
    }
    #[inline(always)]
    fn nr() -> usize {
        4
    }
    #[inline(always)]
    fn alignment_bytes() -> usize {
        16
    }
    #[inline(never)]
    fn run(buf: &mut [f32]) {
        unsafe { arm64simd_tanh_f32_4n(buf.as_mut_ptr(), buf.len()) }
    }
}

test_mmm_kernel_f32!(crate::arm64::arm64simd::MatMatMulF32x8x8A53, test_MatMatMulF32x8x8a53, true);
test_mmm_kernel_f32!(crate::arm64::arm64simd::MatMatMulF32x8x8, test_MatMatMulF32x8x8, true);
test_mmm_kernel_f32!(crate::arm64::arm64simd::MatMatMulF32x12x8A53, test_MatMatMulF32x12x8a53, true);
test_mmm_kernel_f32!(crate::arm64::arm64simd::MatMatMulF32x12x8, test_MatMatMulF32x12x8, true);
test_mmm_kernel_f32!(crate::arm64::arm64simd::MatMatMulF32x64x1A53, test_MatMatMulF32x64x1a53, true);
test_mmm_kernel_f32!(crate::arm64::arm64simd::MatMatMulF32x64x1, test_MatMatMulF32x64x1, true);
test_mmm_kernel_i8!(crate::arm64::arm64simd::MatMatMulI8x8x8, test_MatMatMulI8x8x8, true);
test_mmm_kernel_i8_i32!(
    crate::arm64::arm64simd::MatMatMulI8xI32x8x8,
    test_MatMatMulI8xI32x8x8,
    true
);

#[cfg(test)]
mod test_simd {
    sigmoid_frame_tests!(true, crate::arm64::arm64simd::SigmoidF32x4n);
    tanh_frame_tests!(true, crate::arm64::arm64simd::TanhF32x4n);
}
