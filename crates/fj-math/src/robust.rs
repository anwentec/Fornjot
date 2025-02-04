//! Robust geometric primitives
//!
//! This is an implementation of the geometric primitives that are required by
//! Fornjot and not already provided by [`robust`]. They are auto-transpiled and
//! then manually modified.
//!
//! Original source (public domain):
//! <http://www.cs.cmu.edu/~quake/robust.html>
//!
//! The following tools were used for translation to Rust:
//!
//! - Clang: <https://clang.llvm.org/>
//! - Bear: <https://github.com/rizsotto/Bear>
//! - C2Rust: <https://c2rust.com/>
//!
//! The following steps are used to translate original C code to Rust:
//!
//! 1. Prepare a C file that contains the functions you want to translate.
//! 2. Create compile commands file: `bear -- clang -c predicates.c`
//! 3. Transpile C code to Rust: `c2rust transpile compile_commands.json`
//! 4. Copy code from transpiled file here.

use std::mem;

const SPLITTER: f64 = 134217729.0;
const RESULTERRBOUND: f64 = 3.3306690738754706e-16;
const O3DERRBOUNDA: f64 = 7.771561172376103e-16;
const O3DERRBOUNDB: f64 = 3.330669073875473e-16;
const O3DERRBOUNDC: f64 = 3.2047474274603644e-31;

/// Test a point's orientation against a plane
pub fn orient3d(pa: [f64; 3], pb: [f64; 3], pc: [f64; 3], pd: [f64; 3]) -> f64 {
    let adx: f64 = pa[0] - pd[0];
    let bdx: f64 = pb[0] - pd[0];
    let cdx: f64 = pc[0] - pd[0];
    let ady: f64 = pa[1] - pd[1];
    let bdy: f64 = pb[1] - pd[1];
    let cdy: f64 = pc[1] - pd[1];
    let adz: f64 = pa[2] - pd[2];
    let bdz: f64 = pb[2] - pd[2];
    let cdz: f64 = pc[2] - pd[2];
    let bdxcdy: f64 = bdx * cdy;
    let cdxbdy: f64 = cdx * bdy;
    let cdxady: f64 = cdx * ady;
    let adxcdy: f64 = adx * cdy;
    let adxbdy: f64 = adx * bdy;
    let bdxady: f64 = bdx * ady;
    let det: f64 = adz * (bdxcdy - cdxbdy)
        + bdz * (cdxady - adxcdy)
        + cdz * (adxbdy - bdxady);
    let permanent: f64 = ((if bdxcdy >= 0.0f64 { bdxcdy } else { -bdxcdy })
        + (if cdxbdy >= 0.0f64 { cdxbdy } else { -cdxbdy }))
        * (if adz >= 0.0f64 { adz } else { -adz })
        + ((if cdxady >= 0.0f64 { cdxady } else { -cdxady })
            + (if adxcdy >= 0.0f64 { adxcdy } else { -adxcdy }))
            * (if bdz >= 0.0f64 { bdz } else { -bdz })
        + ((if adxbdy >= 0.0f64 { adxbdy } else { -adxbdy })
            + (if bdxady >= 0.0f64 { bdxady } else { -bdxady }))
            * (if cdz >= 0.0f64 { cdz } else { -cdz });
    let errbound: f64 = O3DERRBOUNDA * permanent;
    if det > errbound || -det > errbound {
        return det;
    }
    orient3dadapt(pa, pb, pc, pd, permanent)
}

fn orient3dadapt(
    pa: [f64; 3],
    pb: [f64; 3],
    pc: [f64; 3],
    pd: [f64; 3],
    permanent: f64,
) -> f64 {
    let mut det: f64;
    let mut errbound: f64;
    let mut bc: [f64; 4] = [0.; 4];
    let mut ca: [f64; 4] = [0.; 4];
    let mut ab: [f64; 4] = [0.; 4];
    let mut adet: [f64; 8] = [0.; 8];
    let mut bdet: [f64; 8] = [0.; 8];
    let mut cdet: [f64; 8] = [0.; 8];
    let mut abdet: [f64; 16] = [0.; 16];
    let mut fin1: [f64; 192] = [0.; 192];
    let mut fin2: [f64; 192] = [0.; 192];
    let mut finlength: i32;
    let at_blarge: f64;
    let at_clarge: f64;
    let bt_clarge: f64;
    let bt_alarge: f64;
    let ct_alarge: f64;
    let ct_blarge: f64;
    let mut at_b: [f64; 4] = [0.; 4];
    let mut at_c: [f64; 4] = [0.; 4];
    let mut bt_c: [f64; 4] = [0.; 4];
    let mut bt_a: [f64; 4] = [0.; 4];
    let mut ct_a: [f64; 4] = [0.; 4];
    let mut ct_b: [f64; 4] = [0.; 4];
    let at_blen: i32;
    let at_clen: i32;
    let bt_clen: i32;
    let bt_alen: i32;
    let ct_alen: i32;
    let ct_blen: i32;
    let bdxt_cdy1: f64;
    let cdxt_bdy1: f64;
    let cdxt_ady1: f64;
    let adxt_cdy1: f64;
    let adxt_bdy1: f64;
    let bdxt_ady1: f64;
    let bdxt_cdy0: f64;
    let cdxt_bdy0: f64;
    let cdxt_ady0: f64;
    let adxt_cdy0: f64;
    let adxt_bdy0: f64;
    let bdxt_ady0: f64;
    let bdyt_cdx1: f64;
    let cdyt_bdx1: f64;
    let cdyt_adx1: f64;
    let adyt_cdx1: f64;
    let adyt_bdx1: f64;
    let bdyt_adx1: f64;
    let bdyt_cdx0: f64;
    let cdyt_bdx0: f64;
    let cdyt_adx0: f64;
    let adyt_cdx0: f64;
    let adyt_bdx0: f64;
    let bdyt_adx0: f64;
    let mut bct: [f64; 8] = [0.; 8];
    let mut cat: [f64; 8] = [0.; 8];
    let mut abt: [f64; 8] = [0.; 8];
    let bdxt_cdyt1: f64;
    let cdxt_bdyt1: f64;
    let cdxt_adyt1: f64;
    let adxt_cdyt1: f64;
    let adxt_bdyt1: f64;
    let bdxt_adyt1: f64;
    let bdxt_cdyt0: f64;
    let cdxt_bdyt0: f64;
    let cdxt_adyt0: f64;
    let adxt_cdyt0: f64;
    let adxt_bdyt0: f64;
    let bdxt_adyt0: f64;
    let mut u: [f64; 4] = [0.; 4];
    let mut v: [f64; 12] = [0.; 12];
    let mut w: [f64; 16] = [0.; 16];
    let mut u3: f64;
    let mut vlength: i32;
    let mut wlength: i32;
    let mut negate: f64;
    let mut bvirt: f64;
    let mut avirt: f64;
    let mut bround: f64;
    let mut around: f64;
    let mut c: f64;
    let mut abig: f64;
    let mut ahi: f64;
    let mut alo: f64;
    let mut bhi: f64;
    let mut blo: f64;
    let mut err1: f64;
    let mut err2: f64;
    let mut err3: f64;
    let mut i: f64;
    let mut j: f64;
    let mut k: f64;
    let mut z: f64;
    let adx: f64 = pa[0] - pd[0];
    let bdx: f64 = pb[0] - pd[0];
    let cdx: f64 = pc[0] - pd[0];
    let ady: f64 = pa[1] - pd[1];
    let bdy: f64 = pb[1] - pd[1];
    let cdy: f64 = pc[1] - pd[1];
    let adz: f64 = pa[2] - pd[2];
    let bdz: f64 = pb[2] - pd[2];
    let cdz: f64 = pc[2] - pd[2];
    let bdxcdy1: f64 = bdx * cdy;
    c = SPLITTER * bdx;
    abig = c - bdx;
    ahi = c - abig;
    alo = bdx - ahi;
    c = SPLITTER * cdy;
    abig = c - cdy;
    bhi = c - abig;
    blo = cdy - bhi;
    err1 = bdxcdy1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    let bdxcdy0: f64 = alo * blo - err3;
    let cdxbdy1: f64 = cdx * bdy;
    c = SPLITTER * cdx;
    abig = c - cdx;
    ahi = c - abig;
    alo = cdx - ahi;
    c = SPLITTER * bdy;
    abig = c - bdy;
    bhi = c - abig;
    blo = bdy - bhi;
    err1 = cdxbdy1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    let cdxbdy0: f64 = alo * blo - err3;
    i = bdxcdy0 - cdxbdy0;
    bvirt = bdxcdy0 - i;
    avirt = i + bvirt;
    bround = bvirt - cdxbdy0;
    around = bdxcdy0 - avirt;
    bc[0] = around + bround;
    j = bdxcdy1 + i;
    bvirt = j - bdxcdy1;
    avirt = j - bvirt;
    bround = i - bvirt;
    around = bdxcdy1 - avirt;
    z = around + bround;
    i = z - cdxbdy1;
    bvirt = z - i;
    avirt = i + bvirt;
    bround = bvirt - cdxbdy1;
    around = z - avirt;
    bc[1] = around + bround;
    let bc3: f64 = j + i;
    bvirt = bc3 - j;
    avirt = bc3 - bvirt;
    bround = i - bvirt;
    around = j - avirt;
    bc[2] = around + bround;
    bc[3] = bc3;
    let alen: i32 = scale_expansion_zeroelim(4, &bc, adz, &mut adet);
    let cdxady1: f64 = cdx * ady;
    c = SPLITTER * cdx;
    abig = c - cdx;
    ahi = c - abig;
    alo = cdx - ahi;
    c = SPLITTER * ady;
    abig = c - ady;
    bhi = c - abig;
    blo = ady - bhi;
    err1 = cdxady1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    let cdxady0: f64 = alo * blo - err3;
    let adxcdy1: f64 = adx * cdy;
    c = SPLITTER * adx;
    abig = c - adx;
    ahi = c - abig;
    alo = adx - ahi;
    c = SPLITTER * cdy;
    abig = c - cdy;
    bhi = c - abig;
    blo = cdy - bhi;
    err1 = adxcdy1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    let adxcdy0: f64 = alo * blo - err3;
    i = cdxady0 - adxcdy0;
    bvirt = cdxady0 - i;
    avirt = i + bvirt;
    bround = bvirt - adxcdy0;
    around = cdxady0 - avirt;
    ca[0] = around + bround;
    j = cdxady1 + i;
    bvirt = j - cdxady1;
    avirt = j - bvirt;
    bround = i - bvirt;
    around = cdxady1 - avirt;
    z = around + bround;
    i = z - adxcdy1;
    bvirt = z - i;
    avirt = i + bvirt;
    bround = bvirt - adxcdy1;
    around = z - avirt;
    ca[1] = around + bround;
    let ca3: f64 = j + i;
    bvirt = ca3 - j;
    avirt = ca3 - bvirt;
    bround = i - bvirt;
    around = j - avirt;
    ca[2] = around + bround;
    ca[3] = ca3;
    let blen: i32 = scale_expansion_zeroelim(4, &ca, bdz, &mut bdet);
    let adxbdy1: f64 = adx * bdy;
    c = SPLITTER * adx;
    abig = c - adx;
    ahi = c - abig;
    alo = adx - ahi;
    c = SPLITTER * bdy;
    abig = c - bdy;
    bhi = c - abig;
    blo = bdy - bhi;
    err1 = adxbdy1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    let adxbdy0: f64 = alo * blo - err3;
    let bdxady1: f64 = bdx * ady;
    c = SPLITTER * bdx;
    abig = c - bdx;
    ahi = c - abig;
    alo = bdx - ahi;
    c = SPLITTER * ady;
    abig = c - ady;
    bhi = c - abig;
    blo = ady - bhi;
    err1 = bdxady1 - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    let bdxady0: f64 = alo * blo - err3;
    i = adxbdy0 - bdxady0;
    bvirt = adxbdy0 - i;
    avirt = i + bvirt;
    bround = bvirt - bdxady0;
    around = adxbdy0 - avirt;
    ab[0] = around + bround;
    j = adxbdy1 + i;
    bvirt = j - adxbdy1;
    avirt = j - bvirt;
    bround = i - bvirt;
    around = adxbdy1 - avirt;
    z = around + bround;
    i = z - bdxady1;
    bvirt = z - i;
    avirt = i + bvirt;
    bround = bvirt - bdxady1;
    around = z - avirt;
    ab[1] = around + bround;
    let ab3: f64 = j + i;
    bvirt = ab3 - j;
    avirt = ab3 - bvirt;
    bround = i - bvirt;
    around = j - avirt;
    ab[2] = around + bround;
    ab[3] = ab3;
    let clen: i32 = scale_expansion_zeroelim(4, &ab, cdz, &mut cdet);
    let ablen: i32 =
        fast_expansion_sum_zeroelim(alen, &adet, blen, &bdet, &mut abdet);
    finlength =
        fast_expansion_sum_zeroelim(ablen, &abdet, clen, &cdet, &mut fin1);
    det = estimate(&fin1[..finlength as usize]);
    errbound = O3DERRBOUNDB * permanent;
    if det >= errbound || -det >= errbound {
        return det;
    }
    bvirt = pa[0] - adx;
    avirt = adx + bvirt;
    bround = bvirt - pd[0];
    around = pa[0] - avirt;
    let adxtail: f64 = around + bround;
    bvirt = pb[0] - bdx;
    avirt = bdx + bvirt;
    bround = bvirt - pd[0];
    around = pb[0] - avirt;
    let bdxtail: f64 = around + bround;
    bvirt = pc[0] - cdx;
    avirt = cdx + bvirt;
    bround = bvirt - pd[0];
    around = pc[0] - avirt;
    let cdxtail: f64 = around + bround;
    bvirt = pa[1] - ady;
    avirt = ady + bvirt;
    bround = bvirt - pd[1];
    around = pa[1] - avirt;
    let adytail: f64 = around + bround;
    bvirt = pb[1] - bdy;
    avirt = bdy + bvirt;
    bround = bvirt - pd[1];
    around = pb[1] - avirt;
    let bdytail: f64 = around + bround;
    bvirt = pc[1] - cdy;
    avirt = cdy + bvirt;
    bround = bvirt - pd[1];
    around = pc[1] - avirt;
    let cdytail: f64 = around + bround;
    bvirt = pa[2] - adz;
    avirt = adz + bvirt;
    bround = bvirt - pd[2];
    around = pa[2] - avirt;
    let adztail: f64 = around + bround;
    bvirt = pb[2] - bdz;
    avirt = bdz + bvirt;
    bround = bvirt - pd[2];
    around = pb[2] - avirt;
    let bdztail: f64 = around + bround;
    bvirt = pc[2] - cdz;
    avirt = cdz + bvirt;
    bround = bvirt - pd[2];
    around = pc[2] - avirt;
    let cdztail: f64 = around + bround;
    if adxtail == 0.0f64
        && bdxtail == 0.0f64
        && cdxtail == 0.0f64
        && adytail == 0.0f64
        && bdytail == 0.0f64
        && cdytail == 0.0f64
        && adztail == 0.0f64
        && bdztail == 0.0f64
        && cdztail == 0.0f64
    {
        return det;
    }
    errbound = O3DERRBOUNDC * permanent
        + RESULTERRBOUND * (if det >= 0.0f64 { det } else { -det });
    det += adz
        * (bdx * cdytail + cdy * bdxtail - (bdy * cdxtail + cdx * bdytail))
        + adztail * (bdx * cdy - bdy * cdx)
        + (bdz
            * (cdx * adytail + ady * cdxtail
                - (cdy * adxtail + adx * cdytail))
            + bdztail * (cdx * ady - cdy * adx))
        + (cdz
            * (adx * bdytail + bdy * adxtail
                - (ady * bdxtail + bdx * adytail))
            + cdztail * (adx * bdy - ady * bdx));
    if det >= errbound || -det >= errbound {
        return det;
    }
    let mut finnow = &mut fin1;
    let mut finother = &mut fin2;
    if adxtail == 0.0f64 {
        if adytail == 0.0f64 {
            at_b[0] = 0.0f64;
            at_blen = 1;
            at_c[0] = 0.0f64;
            at_clen = 1;
        } else {
            negate = -adytail;
            at_blarge = negate * bdx;
            c = SPLITTER * negate;
            abig = c - negate;
            ahi = c - abig;
            alo = negate - ahi;
            c = SPLITTER * bdx;
            abig = c - bdx;
            bhi = c - abig;
            blo = bdx - bhi;
            err1 = at_blarge - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            at_b[0] = alo * blo - err3;
            at_b[1] = at_blarge;
            at_blen = 2;
            at_clarge = adytail * cdx;
            c = SPLITTER * adytail;
            abig = c - adytail;
            ahi = c - abig;
            alo = adytail - ahi;
            c = SPLITTER * cdx;
            abig = c - cdx;
            bhi = c - abig;
            blo = cdx - bhi;
            err1 = at_clarge - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            at_c[0] = alo * blo - err3;
            at_c[1] = at_clarge;
            at_clen = 2;
        }
    } else if adytail == 0.0f64 {
        at_blarge = adxtail * bdy;
        c = SPLITTER * adxtail;
        abig = c - adxtail;
        ahi = c - abig;
        alo = adxtail - ahi;
        c = SPLITTER * bdy;
        abig = c - bdy;
        bhi = c - abig;
        blo = bdy - bhi;
        err1 = at_blarge - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        at_b[0] = alo * blo - err3;
        at_b[1] = at_blarge;
        at_blen = 2;
        negate = -adxtail;
        at_clarge = negate * cdy;
        c = SPLITTER * negate;
        abig = c - negate;
        ahi = c - abig;
        alo = negate - ahi;
        c = SPLITTER * cdy;
        abig = c - cdy;
        bhi = c - abig;
        blo = cdy - bhi;
        err1 = at_clarge - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        at_c[0] = alo * blo - err3;
        at_c[1] = at_clarge;
        at_clen = 2;
    } else {
        adxt_bdy1 = adxtail * bdy;
        c = SPLITTER * adxtail;
        abig = c - adxtail;
        ahi = c - abig;
        alo = adxtail - ahi;
        c = SPLITTER * bdy;
        abig = c - bdy;
        bhi = c - abig;
        blo = bdy - bhi;
        err1 = adxt_bdy1 - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        adxt_bdy0 = alo * blo - err3;
        adyt_bdx1 = adytail * bdx;
        c = SPLITTER * adytail;
        abig = c - adytail;
        ahi = c - abig;
        alo = adytail - ahi;
        c = SPLITTER * bdx;
        abig = c - bdx;
        bhi = c - abig;
        blo = bdx - bhi;
        err1 = adyt_bdx1 - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        adyt_bdx0 = alo * blo - err3;
        i = adxt_bdy0 - adyt_bdx0;
        bvirt = adxt_bdy0 - i;
        avirt = i + bvirt;
        bround = bvirt - adyt_bdx0;
        around = adxt_bdy0 - avirt;
        at_b[0] = around + bround;
        j = adxt_bdy1 + i;
        bvirt = j - adxt_bdy1;
        avirt = j - bvirt;
        bround = i - bvirt;
        around = adxt_bdy1 - avirt;
        z = around + bround;
        i = z - adyt_bdx1;
        bvirt = z - i;
        avirt = i + bvirt;
        bround = bvirt - adyt_bdx1;
        around = z - avirt;
        at_b[1] = around + bround;
        at_blarge = j + i;
        bvirt = at_blarge - j;
        avirt = at_blarge - bvirt;
        bround = i - bvirt;
        around = j - avirt;
        at_b[2] = around + bround;
        at_b[3] = at_blarge;
        at_blen = 4;
        adyt_cdx1 = adytail * cdx;
        c = SPLITTER * adytail;
        abig = c - adytail;
        ahi = c - abig;
        alo = adytail - ahi;
        c = SPLITTER * cdx;
        abig = c - cdx;
        bhi = c - abig;
        blo = cdx - bhi;
        err1 = adyt_cdx1 - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        adyt_cdx0 = alo * blo - err3;
        adxt_cdy1 = adxtail * cdy;
        c = SPLITTER * adxtail;
        abig = c - adxtail;
        ahi = c - abig;
        alo = adxtail - ahi;
        c = SPLITTER * cdy;
        abig = c - cdy;
        bhi = c - abig;
        blo = cdy - bhi;
        err1 = adxt_cdy1 - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        adxt_cdy0 = alo * blo - err3;
        i = adyt_cdx0 - adxt_cdy0;
        bvirt = adyt_cdx0 - i;
        avirt = i + bvirt;
        bround = bvirt - adxt_cdy0;
        around = adyt_cdx0 - avirt;
        at_c[0] = around + bround;
        j = adyt_cdx1 + i;
        bvirt = j - adyt_cdx1;
        avirt = j - bvirt;
        bround = i - bvirt;
        around = adyt_cdx1 - avirt;
        z = around + bround;
        i = z - adxt_cdy1;
        bvirt = z - i;
        avirt = i + bvirt;
        bround = bvirt - adxt_cdy1;
        around = z - avirt;
        at_c[1] = around + bround;
        at_clarge = j + i;
        bvirt = at_clarge - j;
        avirt = at_clarge - bvirt;
        bround = i - bvirt;
        around = j - avirt;
        at_c[2] = around + bround;
        at_c[3] = at_clarge;
        at_clen = 4;
    }
    if bdxtail == 0.0f64 {
        if bdytail == 0.0f64 {
            bt_c[0] = 0.0f64;
            bt_clen = 1;
            bt_a[0] = 0.0f64;
            bt_alen = 1;
        } else {
            negate = -bdytail;
            bt_clarge = negate * cdx;
            c = SPLITTER * negate;
            abig = c - negate;
            ahi = c - abig;
            alo = negate - ahi;
            c = SPLITTER * cdx;
            abig = c - cdx;
            bhi = c - abig;
            blo = cdx - bhi;
            err1 = bt_clarge - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            bt_c[0] = alo * blo - err3;
            bt_c[1] = bt_clarge;
            bt_clen = 2;
            bt_alarge = bdytail * adx;
            c = SPLITTER * bdytail;
            abig = c - bdytail;
            ahi = c - abig;
            alo = bdytail - ahi;
            c = SPLITTER * adx;
            abig = c - adx;
            bhi = c - abig;
            blo = adx - bhi;
            err1 = bt_alarge - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            bt_a[0] = alo * blo - err3;
            bt_a[1] = bt_alarge;
            bt_alen = 2;
        }
    } else if bdytail == 0.0f64 {
        bt_clarge = bdxtail * cdy;
        c = SPLITTER * bdxtail;
        abig = c - bdxtail;
        ahi = c - abig;
        alo = bdxtail - ahi;
        c = SPLITTER * cdy;
        abig = c - cdy;
        bhi = c - abig;
        blo = cdy - bhi;
        err1 = bt_clarge - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        bt_c[0] = alo * blo - err3;
        bt_c[1] = bt_clarge;
        bt_clen = 2;
        negate = -bdxtail;
        bt_alarge = negate * ady;
        c = SPLITTER * negate;
        abig = c - negate;
        ahi = c - abig;
        alo = negate - ahi;
        c = SPLITTER * ady;
        abig = c - ady;
        bhi = c - abig;
        blo = ady - bhi;
        err1 = bt_alarge - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        bt_a[0] = alo * blo - err3;
        bt_a[1] = bt_alarge;
        bt_alen = 2;
    } else {
        bdxt_cdy1 = bdxtail * cdy;
        c = SPLITTER * bdxtail;
        abig = c - bdxtail;
        ahi = c - abig;
        alo = bdxtail - ahi;
        c = SPLITTER * cdy;
        abig = c - cdy;
        bhi = c - abig;
        blo = cdy - bhi;
        err1 = bdxt_cdy1 - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        bdxt_cdy0 = alo * blo - err3;
        bdyt_cdx1 = bdytail * cdx;
        c = SPLITTER * bdytail;
        abig = c - bdytail;
        ahi = c - abig;
        alo = bdytail - ahi;
        c = SPLITTER * cdx;
        abig = c - cdx;
        bhi = c - abig;
        blo = cdx - bhi;
        err1 = bdyt_cdx1 - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        bdyt_cdx0 = alo * blo - err3;
        i = bdxt_cdy0 - bdyt_cdx0;
        bvirt = bdxt_cdy0 - i;
        avirt = i + bvirt;
        bround = bvirt - bdyt_cdx0;
        around = bdxt_cdy0 - avirt;
        bt_c[0] = around + bround;
        j = bdxt_cdy1 + i;
        bvirt = j - bdxt_cdy1;
        avirt = j - bvirt;
        bround = i - bvirt;
        around = bdxt_cdy1 - avirt;
        z = around + bround;
        i = z - bdyt_cdx1;
        bvirt = z - i;
        avirt = i + bvirt;
        bround = bvirt - bdyt_cdx1;
        around = z - avirt;
        bt_c[1] = around + bround;
        bt_clarge = j + i;
        bvirt = bt_clarge - j;
        avirt = bt_clarge - bvirt;
        bround = i - bvirt;
        around = j - avirt;
        bt_c[2] = around + bround;
        bt_c[3] = bt_clarge;
        bt_clen = 4;
        bdyt_adx1 = bdytail * adx;
        c = SPLITTER * bdytail;
        abig = c - bdytail;
        ahi = c - abig;
        alo = bdytail - ahi;
        c = SPLITTER * adx;
        abig = c - adx;
        bhi = c - abig;
        blo = adx - bhi;
        err1 = bdyt_adx1 - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        bdyt_adx0 = alo * blo - err3;
        bdxt_ady1 = bdxtail * ady;
        c = SPLITTER * bdxtail;
        abig = c - bdxtail;
        ahi = c - abig;
        alo = bdxtail - ahi;
        c = SPLITTER * ady;
        abig = c - ady;
        bhi = c - abig;
        blo = ady - bhi;
        err1 = bdxt_ady1 - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        bdxt_ady0 = alo * blo - err3;
        i = bdyt_adx0 - bdxt_ady0;
        bvirt = bdyt_adx0 - i;
        avirt = i + bvirt;
        bround = bvirt - bdxt_ady0;
        around = bdyt_adx0 - avirt;
        bt_a[0] = around + bround;
        j = bdyt_adx1 + i;
        bvirt = j - bdyt_adx1;
        avirt = j - bvirt;
        bround = i - bvirt;
        around = bdyt_adx1 - avirt;
        z = around + bround;
        i = z - bdxt_ady1;
        bvirt = z - i;
        avirt = i + bvirt;
        bround = bvirt - bdxt_ady1;
        around = z - avirt;
        bt_a[1] = around + bround;
        bt_alarge = j + i;
        bvirt = bt_alarge - j;
        avirt = bt_alarge - bvirt;
        bround = i - bvirt;
        around = j - avirt;
        bt_a[2] = around + bround;
        bt_a[3] = bt_alarge;
        bt_alen = 4;
    }
    if cdxtail == 0.0f64 {
        if cdytail == 0.0f64 {
            ct_a[0] = 0.0f64;
            ct_alen = 1;
            ct_b[0] = 0.0f64;
            ct_blen = 1;
        } else {
            negate = -cdytail;
            ct_alarge = negate * adx;
            c = SPLITTER * negate;
            abig = c - negate;
            ahi = c - abig;
            alo = negate - ahi;
            c = SPLITTER * adx;
            abig = c - adx;
            bhi = c - abig;
            blo = adx - bhi;
            err1 = ct_alarge - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            ct_a[0] = alo * blo - err3;
            ct_a[1] = ct_alarge;
            ct_alen = 2;
            ct_blarge = cdytail * bdx;
            c = SPLITTER * cdytail;
            abig = c - cdytail;
            ahi = c - abig;
            alo = cdytail - ahi;
            c = SPLITTER * bdx;
            abig = c - bdx;
            bhi = c - abig;
            blo = bdx - bhi;
            err1 = ct_blarge - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            ct_b[0] = alo * blo - err3;
            ct_b[1] = ct_blarge;
            ct_blen = 2;
        }
    } else if cdytail == 0.0f64 {
        ct_alarge = cdxtail * ady;
        c = SPLITTER * cdxtail;
        abig = c - cdxtail;
        ahi = c - abig;
        alo = cdxtail - ahi;
        c = SPLITTER * ady;
        abig = c - ady;
        bhi = c - abig;
        blo = ady - bhi;
        err1 = ct_alarge - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        ct_a[0] = alo * blo - err3;
        ct_a[1] = ct_alarge;
        ct_alen = 2;
        negate = -cdxtail;
        ct_blarge = negate * bdy;
        c = SPLITTER * negate;
        abig = c - negate;
        ahi = c - abig;
        alo = negate - ahi;
        c = SPLITTER * bdy;
        abig = c - bdy;
        bhi = c - abig;
        blo = bdy - bhi;
        err1 = ct_blarge - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        ct_b[0] = alo * blo - err3;
        ct_b[1] = ct_blarge;
        ct_blen = 2;
    } else {
        cdxt_ady1 = cdxtail * ady;
        c = SPLITTER * cdxtail;
        abig = c - cdxtail;
        ahi = c - abig;
        alo = cdxtail - ahi;
        c = SPLITTER * ady;
        abig = c - ady;
        bhi = c - abig;
        blo = ady - bhi;
        err1 = cdxt_ady1 - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        cdxt_ady0 = alo * blo - err3;
        cdyt_adx1 = cdytail * adx;
        c = SPLITTER * cdytail;
        abig = c - cdytail;
        ahi = c - abig;
        alo = cdytail - ahi;
        c = SPLITTER * adx;
        abig = c - adx;
        bhi = c - abig;
        blo = adx - bhi;
        err1 = cdyt_adx1 - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        cdyt_adx0 = alo * blo - err3;
        i = cdxt_ady0 - cdyt_adx0;
        bvirt = cdxt_ady0 - i;
        avirt = i + bvirt;
        bround = bvirt - cdyt_adx0;
        around = cdxt_ady0 - avirt;
        ct_a[0] = around + bround;
        j = cdxt_ady1 + i;
        bvirt = j - cdxt_ady1;
        avirt = j - bvirt;
        bround = i - bvirt;
        around = cdxt_ady1 - avirt;
        z = around + bround;
        i = z - cdyt_adx1;
        bvirt = z - i;
        avirt = i + bvirt;
        bround = bvirt - cdyt_adx1;
        around = z - avirt;
        ct_a[1] = around + bround;
        ct_alarge = j + i;
        bvirt = ct_alarge - j;
        avirt = ct_alarge - bvirt;
        bround = i - bvirt;
        around = j - avirt;
        ct_a[2] = around + bround;
        ct_a[3] = ct_alarge;
        ct_alen = 4;
        cdyt_bdx1 = cdytail * bdx;
        c = SPLITTER * cdytail;
        abig = c - cdytail;
        ahi = c - abig;
        alo = cdytail - ahi;
        c = SPLITTER * bdx;
        abig = c - bdx;
        bhi = c - abig;
        blo = bdx - bhi;
        err1 = cdyt_bdx1 - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        cdyt_bdx0 = alo * blo - err3;
        cdxt_bdy1 = cdxtail * bdy;
        c = SPLITTER * cdxtail;
        abig = c - cdxtail;
        ahi = c - abig;
        alo = cdxtail - ahi;
        c = SPLITTER * bdy;
        abig = c - bdy;
        bhi = c - abig;
        blo = bdy - bhi;
        err1 = cdxt_bdy1 - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        cdxt_bdy0 = alo * blo - err3;
        i = cdyt_bdx0 - cdxt_bdy0;
        bvirt = cdyt_bdx0 - i;
        avirt = i + bvirt;
        bround = bvirt - cdxt_bdy0;
        around = cdyt_bdx0 - avirt;
        ct_b[0] = around + bround;
        j = cdyt_bdx1 + i;
        bvirt = j - cdyt_bdx1;
        avirt = j - bvirt;
        bround = i - bvirt;
        around = cdyt_bdx1 - avirt;
        z = around + bround;
        i = z - cdxt_bdy1;
        bvirt = z - i;
        avirt = i + bvirt;
        bround = bvirt - cdxt_bdy1;
        around = z - avirt;
        ct_b[1] = around + bround;
        ct_blarge = j + i;
        bvirt = ct_blarge - j;
        avirt = ct_blarge - bvirt;
        bround = i - bvirt;
        around = j - avirt;
        ct_b[2] = around + bround;
        ct_b[3] = ct_blarge;
        ct_blen = 4;
    }
    let bctlen: i32 =
        fast_expansion_sum_zeroelim(bt_clen, &bt_c, ct_blen, &ct_b, &mut bct);
    wlength = scale_expansion_zeroelim(bctlen, &bct, adz, &mut w);
    finlength =
        fast_expansion_sum_zeroelim(finlength, finnow, wlength, &w, finother);
    mem::swap(&mut finnow, &mut finother);
    let catlen: i32 =
        fast_expansion_sum_zeroelim(ct_alen, &ct_a, at_clen, &at_c, &mut cat);
    wlength = scale_expansion_zeroelim(catlen, &cat, bdz, &mut w);
    finlength =
        fast_expansion_sum_zeroelim(finlength, finnow, wlength, &w, finother);
    mem::swap(&mut finnow, &mut finother);
    let abtlen: i32 =
        fast_expansion_sum_zeroelim(at_blen, &at_b, bt_alen, &bt_a, &mut abt);
    wlength = scale_expansion_zeroelim(abtlen, &abt, cdz, &mut w);
    finlength =
        fast_expansion_sum_zeroelim(finlength, finnow, wlength, &w, finother);
    mem::swap(&mut finnow, &mut finother);
    if adztail != 0.0f64 {
        vlength = scale_expansion_zeroelim(4, &bc, adztail, &mut v);
        finlength = fast_expansion_sum_zeroelim(
            finlength, finnow, vlength, &v, finother,
        );
        mem::swap(&mut finnow, &mut finother);
    }
    if bdztail != 0.0f64 {
        vlength = scale_expansion_zeroelim(4, &ca, bdztail, &mut v);
        finlength = fast_expansion_sum_zeroelim(
            finlength, finnow, vlength, &v, finother,
        );
        mem::swap(&mut finnow, &mut finother);
    }
    if cdztail != 0.0f64 {
        vlength = scale_expansion_zeroelim(4, &ab, cdztail, &mut v);
        finlength = fast_expansion_sum_zeroelim(
            finlength, finnow, vlength, &v, finother,
        );
        mem::swap(&mut finnow, &mut finother);
    }
    if adxtail != 0.0f64 {
        if bdytail != 0.0f64 {
            adxt_bdyt1 = adxtail * bdytail;
            c = SPLITTER * adxtail;
            abig = c - adxtail;
            ahi = c - abig;
            alo = adxtail - ahi;
            c = SPLITTER * bdytail;
            abig = c - bdytail;
            bhi = c - abig;
            blo = bdytail - bhi;
            err1 = adxt_bdyt1 - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            adxt_bdyt0 = alo * blo - err3;
            c = SPLITTER * cdz;
            abig = c - cdz;
            bhi = c - abig;
            blo = cdz - bhi;
            i = adxt_bdyt0 * cdz;
            c = SPLITTER * adxt_bdyt0;
            abig = c - adxt_bdyt0;
            ahi = c - abig;
            alo = adxt_bdyt0 - ahi;
            err1 = i - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            u[0] = alo * blo - err3;
            j = adxt_bdyt1 * cdz;
            c = SPLITTER * adxt_bdyt1;
            abig = c - adxt_bdyt1;
            ahi = c - abig;
            alo = adxt_bdyt1 - ahi;
            err1 = j - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            z = alo * blo - err3;
            k = i + z;
            bvirt = k - i;
            avirt = k - bvirt;
            bround = z - bvirt;
            around = i - avirt;
            u[1] = around + bround;
            u3 = j + k;
            bvirt = u3 - j;
            u[2] = k - bvirt;
            u[3] = u3;
            finlength =
                fast_expansion_sum_zeroelim(finlength, finnow, 4, &u, finother);
            mem::swap(&mut finnow, &mut finother);
            if cdztail != 0.0f64 {
                c = SPLITTER * cdztail;
                abig = c - cdztail;
                bhi = c - abig;
                blo = cdztail - bhi;
                i = adxt_bdyt0 * cdztail;
                c = SPLITTER * adxt_bdyt0;
                abig = c - adxt_bdyt0;
                ahi = c - abig;
                alo = adxt_bdyt0 - ahi;
                err1 = i - ahi * bhi;
                err2 = err1 - alo * bhi;
                err3 = err2 - ahi * blo;
                u[0] = alo * blo - err3;
                j = adxt_bdyt1 * cdztail;
                c = SPLITTER * adxt_bdyt1;
                abig = c - adxt_bdyt1;
                ahi = c - abig;
                alo = adxt_bdyt1 - ahi;
                err1 = j - ahi * bhi;
                err2 = err1 - alo * bhi;
                err3 = err2 - ahi * blo;
                z = alo * blo - err3;
                k = i + z;
                bvirt = k - i;
                avirt = k - bvirt;
                bround = z - bvirt;
                around = i - avirt;
                u[1] = around + bround;
                u3 = j + k;
                bvirt = u3 - j;
                u[2] = k - bvirt;
                u[3] = u3;
                finlength = fast_expansion_sum_zeroelim(
                    finlength, finnow, 4, &u, finother,
                );
                mem::swap(&mut finnow, &mut finother);
            }
        }
        if cdytail != 0.0f64 {
            negate = -adxtail;
            adxt_cdyt1 = negate * cdytail;
            c = SPLITTER * negate;
            abig = c - negate;
            ahi = c - abig;
            alo = negate - ahi;
            c = SPLITTER * cdytail;
            abig = c - cdytail;
            bhi = c - abig;
            blo = cdytail - bhi;
            err1 = adxt_cdyt1 - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            adxt_cdyt0 = alo * blo - err3;
            c = SPLITTER * bdz;
            abig = c - bdz;
            bhi = c - abig;
            blo = bdz - bhi;
            i = adxt_cdyt0 * bdz;
            c = SPLITTER * adxt_cdyt0;
            abig = c - adxt_cdyt0;
            ahi = c - abig;
            alo = adxt_cdyt0 - ahi;
            err1 = i - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            u[0] = alo * blo - err3;
            j = adxt_cdyt1 * bdz;
            c = SPLITTER * adxt_cdyt1;
            abig = c - adxt_cdyt1;
            ahi = c - abig;
            alo = adxt_cdyt1 - ahi;
            err1 = j - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            z = alo * blo - err3;
            k = i + z;
            bvirt = k - i;
            avirt = k - bvirt;
            bround = z - bvirt;
            around = i - avirt;
            u[1] = around + bround;
            u3 = j + k;
            bvirt = u3 - j;
            u[2] = k - bvirt;
            u[3] = u3;
            finlength =
                fast_expansion_sum_zeroelim(finlength, finnow, 4, &u, finother);
            mem::swap(&mut finnow, &mut finother);
            if bdztail != 0.0f64 {
                c = SPLITTER * bdztail;
                abig = c - bdztail;
                bhi = c - abig;
                blo = bdztail - bhi;
                i = adxt_cdyt0 * bdztail;
                c = SPLITTER * adxt_cdyt0;
                abig = c - adxt_cdyt0;
                ahi = c - abig;
                alo = adxt_cdyt0 - ahi;
                err1 = i - ahi * bhi;
                err2 = err1 - alo * bhi;
                err3 = err2 - ahi * blo;
                u[0] = alo * blo - err3;
                j = adxt_cdyt1 * bdztail;
                c = SPLITTER * adxt_cdyt1;
                abig = c - adxt_cdyt1;
                ahi = c - abig;
                alo = adxt_cdyt1 - ahi;
                err1 = j - ahi * bhi;
                err2 = err1 - alo * bhi;
                err3 = err2 - ahi * blo;
                z = alo * blo - err3;
                k = i + z;
                bvirt = k - i;
                avirt = k - bvirt;
                bround = z - bvirt;
                around = i - avirt;
                u[1] = around + bround;
                u3 = j + k;
                bvirt = u3 - j;
                u[2] = k - bvirt;
                u[3] = u3;
                finlength = fast_expansion_sum_zeroelim(
                    finlength, finnow, 4, &u, finother,
                );
                mem::swap(&mut finnow, &mut finother);
            }
        }
    }
    if bdxtail != 0.0f64 {
        if cdytail != 0.0f64 {
            bdxt_cdyt1 = bdxtail * cdytail;
            c = SPLITTER * bdxtail;
            abig = c - bdxtail;
            ahi = c - abig;
            alo = bdxtail - ahi;
            c = SPLITTER * cdytail;
            abig = c - cdytail;
            bhi = c - abig;
            blo = cdytail - bhi;
            err1 = bdxt_cdyt1 - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            bdxt_cdyt0 = alo * blo - err3;
            c = SPLITTER * adz;
            abig = c - adz;
            bhi = c - abig;
            blo = adz - bhi;
            i = bdxt_cdyt0 * adz;
            c = SPLITTER * bdxt_cdyt0;
            abig = c - bdxt_cdyt0;
            ahi = c - abig;
            alo = bdxt_cdyt0 - ahi;
            err1 = i - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            u[0] = alo * blo - err3;
            j = bdxt_cdyt1 * adz;
            c = SPLITTER * bdxt_cdyt1;
            abig = c - bdxt_cdyt1;
            ahi = c - abig;
            alo = bdxt_cdyt1 - ahi;
            err1 = j - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            z = alo * blo - err3;
            k = i + z;
            bvirt = k - i;
            avirt = k - bvirt;
            bround = z - bvirt;
            around = i - avirt;
            u[1] = around + bround;
            u3 = j + k;
            bvirt = u3 - j;
            u[2] = k - bvirt;
            u[3] = u3;
            finlength =
                fast_expansion_sum_zeroelim(finlength, finnow, 4, &u, finother);
            mem::swap(&mut finnow, &mut finother);
            if adztail != 0.0f64 {
                c = SPLITTER * adztail;
                abig = c - adztail;
                bhi = c - abig;
                blo = adztail - bhi;
                i = bdxt_cdyt0 * adztail;
                c = SPLITTER * bdxt_cdyt0;
                abig = c - bdxt_cdyt0;
                ahi = c - abig;
                alo = bdxt_cdyt0 - ahi;
                err1 = i - ahi * bhi;
                err2 = err1 - alo * bhi;
                err3 = err2 - ahi * blo;
                u[0] = alo * blo - err3;
                j = bdxt_cdyt1 * adztail;
                c = SPLITTER * bdxt_cdyt1;
                abig = c - bdxt_cdyt1;
                ahi = c - abig;
                alo = bdxt_cdyt1 - ahi;
                err1 = j - ahi * bhi;
                err2 = err1 - alo * bhi;
                err3 = err2 - ahi * blo;
                z = alo * blo - err3;
                k = i + z;
                bvirt = k - i;
                avirt = k - bvirt;
                bround = z - bvirt;
                around = i - avirt;
                u[1] = around + bround;
                u3 = j + k;
                bvirt = u3 - j;
                u[2] = k - bvirt;
                u[3] = u3;
                finlength = fast_expansion_sum_zeroelim(
                    finlength, finnow, 4, &u, finother,
                );
                mem::swap(&mut finnow, &mut finother);
            }
        }
        if adytail != 0.0f64 {
            negate = -bdxtail;
            bdxt_adyt1 = negate * adytail;
            c = SPLITTER * negate;
            abig = c - negate;
            ahi = c - abig;
            alo = negate - ahi;
            c = SPLITTER * adytail;
            abig = c - adytail;
            bhi = c - abig;
            blo = adytail - bhi;
            err1 = bdxt_adyt1 - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            bdxt_adyt0 = alo * blo - err3;
            c = SPLITTER * cdz;
            abig = c - cdz;
            bhi = c - abig;
            blo = cdz - bhi;
            i = bdxt_adyt0 * cdz;
            c = SPLITTER * bdxt_adyt0;
            abig = c - bdxt_adyt0;
            ahi = c - abig;
            alo = bdxt_adyt0 - ahi;
            err1 = i - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            u[0] = alo * blo - err3;
            j = bdxt_adyt1 * cdz;
            c = SPLITTER * bdxt_adyt1;
            abig = c - bdxt_adyt1;
            ahi = c - abig;
            alo = bdxt_adyt1 - ahi;
            err1 = j - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            z = alo * blo - err3;
            k = i + z;
            bvirt = k - i;
            avirt = k - bvirt;
            bround = z - bvirt;
            around = i - avirt;
            u[1] = around + bround;
            u3 = j + k;
            bvirt = u3 - j;
            u[2] = k - bvirt;
            u[3] = u3;
            finlength =
                fast_expansion_sum_zeroelim(finlength, finnow, 4, &u, finother);
            mem::swap(&mut finnow, &mut finother);
            if cdztail != 0.0f64 {
                c = SPLITTER * cdztail;
                abig = c - cdztail;
                bhi = c - abig;
                blo = cdztail - bhi;
                i = bdxt_adyt0 * cdztail;
                c = SPLITTER * bdxt_adyt0;
                abig = c - bdxt_adyt0;
                ahi = c - abig;
                alo = bdxt_adyt0 - ahi;
                err1 = i - ahi * bhi;
                err2 = err1 - alo * bhi;
                err3 = err2 - ahi * blo;
                u[0] = alo * blo - err3;
                j = bdxt_adyt1 * cdztail;
                c = SPLITTER * bdxt_adyt1;
                abig = c - bdxt_adyt1;
                ahi = c - abig;
                alo = bdxt_adyt1 - ahi;
                err1 = j - ahi * bhi;
                err2 = err1 - alo * bhi;
                err3 = err2 - ahi * blo;
                z = alo * blo - err3;
                k = i + z;
                bvirt = k - i;
                avirt = k - bvirt;
                bround = z - bvirt;
                around = i - avirt;
                u[1] = around + bround;
                u3 = j + k;
                bvirt = u3 - j;
                u[2] = k - bvirt;
                u[3] = u3;
                finlength = fast_expansion_sum_zeroelim(
                    finlength, finnow, 4, &u, finother,
                );
                mem::swap(&mut finnow, &mut finother);
            }
        }
    }
    if cdxtail != 0.0f64 {
        if adytail != 0.0f64 {
            cdxt_adyt1 = cdxtail * adytail;
            c = SPLITTER * cdxtail;
            abig = c - cdxtail;
            ahi = c - abig;
            alo = cdxtail - ahi;
            c = SPLITTER * adytail;
            abig = c - adytail;
            bhi = c - abig;
            blo = adytail - bhi;
            err1 = cdxt_adyt1 - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            cdxt_adyt0 = alo * blo - err3;
            c = SPLITTER * bdz;
            abig = c - bdz;
            bhi = c - abig;
            blo = bdz - bhi;
            i = cdxt_adyt0 * bdz;
            c = SPLITTER * cdxt_adyt0;
            abig = c - cdxt_adyt0;
            ahi = c - abig;
            alo = cdxt_adyt0 - ahi;
            err1 = i - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            u[0] = alo * blo - err3;
            j = cdxt_adyt1 * bdz;
            c = SPLITTER * cdxt_adyt1;
            abig = c - cdxt_adyt1;
            ahi = c - abig;
            alo = cdxt_adyt1 - ahi;
            err1 = j - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            z = alo * blo - err3;
            k = i + z;
            bvirt = k - i;
            avirt = k - bvirt;
            bround = z - bvirt;
            around = i - avirt;
            u[1] = around + bround;
            u3 = j + k;
            bvirt = u3 - j;
            u[2] = k - bvirt;
            u[3] = u3;
            finlength =
                fast_expansion_sum_zeroelim(finlength, finnow, 4, &u, finother);
            mem::swap(&mut finnow, &mut finother);
            if bdztail != 0.0f64 {
                c = SPLITTER * bdztail;
                abig = c - bdztail;
                bhi = c - abig;
                blo = bdztail - bhi;
                i = cdxt_adyt0 * bdztail;
                c = SPLITTER * cdxt_adyt0;
                abig = c - cdxt_adyt0;
                ahi = c - abig;
                alo = cdxt_adyt0 - ahi;
                err1 = i - ahi * bhi;
                err2 = err1 - alo * bhi;
                err3 = err2 - ahi * blo;
                u[0] = alo * blo - err3;
                j = cdxt_adyt1 * bdztail;
                c = SPLITTER * cdxt_adyt1;
                abig = c - cdxt_adyt1;
                ahi = c - abig;
                alo = cdxt_adyt1 - ahi;
                err1 = j - ahi * bhi;
                err2 = err1 - alo * bhi;
                err3 = err2 - ahi * blo;
                z = alo * blo - err3;
                k = i + z;
                bvirt = k - i;
                avirt = k - bvirt;
                bround = z - bvirt;
                around = i - avirt;
                u[1] = around + bround;
                u3 = j + k;
                bvirt = u3 - j;
                u[2] = k - bvirt;
                u[3] = u3;
                finlength = fast_expansion_sum_zeroelim(
                    finlength, finnow, 4, &u, finother,
                );
                mem::swap(&mut finnow, &mut finother);
            }
        }
        if bdytail != 0.0f64 {
            negate = -cdxtail;
            cdxt_bdyt1 = negate * bdytail;
            c = SPLITTER * negate;
            abig = c - negate;
            ahi = c - abig;
            alo = negate - ahi;
            c = SPLITTER * bdytail;
            abig = c - bdytail;
            bhi = c - abig;
            blo = bdytail - bhi;
            err1 = cdxt_bdyt1 - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            cdxt_bdyt0 = alo * blo - err3;
            c = SPLITTER * adz;
            abig = c - adz;
            bhi = c - abig;
            blo = adz - bhi;
            i = cdxt_bdyt0 * adz;
            c = SPLITTER * cdxt_bdyt0;
            abig = c - cdxt_bdyt0;
            ahi = c - abig;
            alo = cdxt_bdyt0 - ahi;
            err1 = i - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            u[0] = alo * blo - err3;
            j = cdxt_bdyt1 * adz;
            c = SPLITTER * cdxt_bdyt1;
            abig = c - cdxt_bdyt1;
            ahi = c - abig;
            alo = cdxt_bdyt1 - ahi;
            err1 = j - ahi * bhi;
            err2 = err1 - alo * bhi;
            err3 = err2 - ahi * blo;
            z = alo * blo - err3;
            k = i + z;
            bvirt = k - i;
            avirt = k - bvirt;
            bround = z - bvirt;
            around = i - avirt;
            u[1] = around + bround;
            u3 = j + k;
            bvirt = u3 - j;
            u[2] = k - bvirt;
            u[3] = u3;
            finlength =
                fast_expansion_sum_zeroelim(finlength, finnow, 4, &u, finother);
            mem::swap(&mut finnow, &mut finother);
            if adztail != 0.0f64 {
                c = SPLITTER * adztail;
                abig = c - adztail;
                bhi = c - abig;
                blo = adztail - bhi;
                i = cdxt_bdyt0 * adztail;
                c = SPLITTER * cdxt_bdyt0;
                abig = c - cdxt_bdyt0;
                ahi = c - abig;
                alo = cdxt_bdyt0 - ahi;
                err1 = i - ahi * bhi;
                err2 = err1 - alo * bhi;
                err3 = err2 - ahi * blo;
                u[0] = alo * blo - err3;
                j = cdxt_bdyt1 * adztail;
                c = SPLITTER * cdxt_bdyt1;
                abig = c - cdxt_bdyt1;
                ahi = c - abig;
                alo = cdxt_bdyt1 - ahi;
                err1 = j - ahi * bhi;
                err2 = err1 - alo * bhi;
                err3 = err2 - ahi * blo;
                z = alo * blo - err3;
                k = i + z;
                bvirt = k - i;
                avirt = k - bvirt;
                bround = z - bvirt;
                around = i - avirt;
                u[1] = around + bround;
                u3 = j + k;
                bvirt = u3 - j;
                u[2] = k - bvirt;
                u[3] = u3;
                finlength = fast_expansion_sum_zeroelim(
                    finlength, finnow, 4, &u, finother,
                );
                mem::swap(&mut finnow, &mut finother);
            }
        }
    }
    if adztail != 0.0f64 {
        wlength = scale_expansion_zeroelim(bctlen, &bct, adztail, &mut w);
        finlength = fast_expansion_sum_zeroelim(
            finlength, finnow, wlength, &w, finother,
        );
        mem::swap(&mut finnow, &mut finother);
    }
    if bdztail != 0.0f64 {
        wlength = scale_expansion_zeroelim(catlen, &cat, bdztail, &mut w);
        finlength = fast_expansion_sum_zeroelim(
            finlength, finnow, wlength, &w, finother,
        );
        mem::swap(&mut finnow, &mut finother);
    }
    if cdztail != 0.0f64 {
        wlength = scale_expansion_zeroelim(abtlen, &abt, cdztail, &mut w);
        finlength = fast_expansion_sum_zeroelim(
            finlength, finnow, wlength, &w, finother,
        );
        finnow = finother;
    }
    finnow[(finlength - 1) as usize]
}

fn scale_expansion_zeroelim(
    elen: i32,
    e: &[f64],
    b: f64,
    h: &mut [f64],
) -> i32 {
    let mut q: f64;
    let mut sum: f64;
    let mut hh: f64;
    let mut product1: f64;
    let mut product0: f64;
    let mut eindex: i32;
    let mut hindex: i32;
    let mut enow: f64;
    let mut bvirt: f64;
    let mut avirt: f64;
    let mut bround: f64;
    let mut around: f64;
    let mut c: f64;
    let mut abig: f64;
    let mut ahi: f64;
    let mut alo: f64;
    let mut err1: f64;
    let mut err2: f64;
    let mut err3: f64;
    c = SPLITTER * b;
    abig = c - b;
    let bhi: f64 = c - abig;
    let blo: f64 = b - bhi;
    q = e[0] * b;
    c = SPLITTER * e[0];
    abig = c - e[0];
    ahi = c - abig;
    alo = e[0] - ahi;
    err1 = q - ahi * bhi;
    err2 = err1 - alo * bhi;
    err3 = err2 - ahi * blo;
    hh = alo * blo - err3;
    hindex = 0;
    if hh != 0 as f64 {
        let fresh12 = hindex;
        hindex += 1;
        h[fresh12 as usize] = hh;
    }
    eindex = 1;
    while eindex < elen {
        enow = e[eindex as usize];
        product1 = enow * b;
        c = SPLITTER * enow;
        abig = c - enow;
        ahi = c - abig;
        alo = enow - ahi;
        err1 = product1 - ahi * bhi;
        err2 = err1 - alo * bhi;
        err3 = err2 - ahi * blo;
        product0 = alo * blo - err3;
        sum = q + product0;
        bvirt = sum - q;
        avirt = sum - bvirt;
        bround = product0 - bvirt;
        around = q - avirt;
        hh = around + bround;
        if hh != 0 as f64 {
            let fresh13 = hindex;
            hindex += 1;
            h[fresh13 as usize] = hh;
        }
        q = product1 + sum;
        bvirt = q - product1;
        hh = sum - bvirt;
        if hh != 0 as f64 {
            let fresh14 = hindex;
            hindex += 1;
            h[fresh14 as usize] = hh;
        }
        eindex += 1;
    }
    if q != 0.0f64 || hindex == 0 {
        let fresh15 = hindex;
        hindex += 1;
        h[fresh15 as usize] = q;
    }
    hindex
}

fn fast_expansion_sum_zeroelim(
    elen: i32,
    e: &[f64],
    flen: i32,
    f: &[f64],
    h: &mut [f64],
) -> i32 {
    let mut q: f64;
    let mut q_new: f64;
    let mut hh: f64;
    let mut bvirt: f64;
    let mut avirt: f64;
    let mut bround: f64;
    let mut around: f64;
    let mut eindex: i32;
    let mut findex: i32;
    let mut hindex: i32;
    let mut enow: f64;
    let mut fnow: f64;
    enow = e[0];
    fnow = f[0];
    findex = 0;
    eindex = findex;
    if (fnow > enow) as i32 == (fnow > -enow) as i32 {
        q = enow;
        eindex += 1;
        enow = e[eindex as usize];
    } else {
        q = fnow;
        findex += 1;
        fnow = f[findex as usize];
    }
    hindex = 0;
    if eindex < elen && findex < flen {
        if (fnow > enow) as i32 == (fnow > -enow) as i32 {
            q_new = enow + q;
            bvirt = q_new - enow;
            hh = q - bvirt;
            eindex += 1;
            enow = e[eindex as usize];
        } else {
            q_new = fnow + q;
            bvirt = q_new - fnow;
            hh = q - bvirt;
            findex += 1;
            fnow = f[findex as usize];
        }
        q = q_new;
        if hh != 0.0f64 {
            let fresh4 = hindex;
            hindex += 1;
            h[fresh4 as usize] = hh;
        }
        while eindex < elen && findex < flen {
            if (fnow > enow) as i32 == (fnow > -enow) as i32 {
                q_new = q + enow;
                bvirt = q_new - q;
                avirt = q_new - bvirt;
                bround = enow - bvirt;
                around = q - avirt;
                hh = around + bround;
                eindex += 1;
                enow = e[eindex as usize];
            } else {
                q_new = q + fnow;
                bvirt = q_new - q;
                avirt = q_new - bvirt;
                bround = fnow - bvirt;
                around = q - avirt;
                hh = around + bround;
                findex += 1;
                fnow = f[findex as usize];
            }
            q = q_new;
            if hh != 0.0f64 {
                let fresh5 = hindex;
                hindex += 1;
                h[fresh5 as usize] = hh;
            }
        }
    }
    while eindex < elen {
        q_new = q + enow;
        bvirt = q_new - q;
        avirt = q_new - bvirt;
        bround = enow - bvirt;
        around = q - avirt;
        hh = around + bround;
        eindex += 1;
        enow = e[eindex as usize];
        q = q_new;
        if hh != 0.0f64 {
            let fresh6 = hindex;
            hindex += 1;
            h[fresh6 as usize] = hh;
        }
    }
    while findex < flen {
        q_new = q + fnow;
        bvirt = q_new - q;
        avirt = q_new - bvirt;
        bround = fnow - bvirt;
        around = q - avirt;
        hh = around + bround;
        findex += 1;
        fnow = f[findex as usize];
        q = q_new;
        if hh != 0.0f64 {
            let fresh7 = hindex;
            hindex += 1;
            h[fresh7 as usize] = hh;
        }
    }
    if q != 0.0f64 || hindex == 0 {
        let fresh8 = hindex;
        hindex += 1;
        h[fresh8 as usize] = q;
    }
    hindex
}

fn estimate(e: &[f64]) -> f64 {
    let mut q = e[0];

    for e in &e[1..] {
        q += e;
    }

    q
}
