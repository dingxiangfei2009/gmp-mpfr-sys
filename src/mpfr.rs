// Copyright © 2017 University of Malta

// This program is free software: you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public License
// as published by the Free Software Foundation, either version 3 of
// the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License and a copy of the GNU General Public License along with
// this program. If not, see <http://www.gnu.org/licenses/>.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use gmp;
use std::os::raw::{c_char, c_int, c_long, c_ulong, c_void};

/// See: [`mpfr_rnd_t`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005frnd_005ft)
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum rnd_t {
    RNDN = 0,
    RNDZ = 1,
    RNDU = 2,
    RNDD = 3,
    RNDA = 4,
    RNDF = 5,
    RNDNA = -1,
}

/// See: [`mpfr_prec_t`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fprec_005ft)
pub type prec_t = c_long;
/// See: [Exception Related Functions](http://www.mpfr.org/mpfr-current/mpfr.html#Exception-Related-Functions)
pub type exp_t = c_long;

/// See: [Nomenclature and Types](http://www.mpfr.org/mpfr-current/mpfr.html#Nomenclature-and-Types)
pub const PREC_MIN: prec_t = 2;
/// See: [Nomenclature and Types](http://www.mpfr.org/mpfr-current/mpfr.html#Nomenclature-and-Types)
pub const PREC_MAX: prec_t = (!(0 as uprec_t) >> 1) as prec_t;

/// See: [`mpfr_t`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005ft)
/// and [Internals](http://www.mpfr.org/mpfr-current/mpfr.html#Internals)
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct mpfr_t {
    pub prec: prec_t,
    pub sign: c_int,
    pub exp: exp_t,
    pub d: *mut gmp::limb_t,
}

/// See: [`mpfr_custom_init_set`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fcustom_005finit_005fset)
pub const NAN_KIND: c_int = 0;
/// See: [`mpfr_custom_init_set`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fcustom_005finit_005fset)
pub const INF_KIND: c_int = 1;
/// See: [`mpfr_custom_init_set`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fcustom_005finit_005fset)
pub const ZERO_KIND: c_int = 2;
/// See: [`mpfr_custom_init_set`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fcustom_005finit_005fset)
pub const REGULAR_KIND: c_int = 3;

// Types for function declarations in this file.

type uprec_t = c_ulong;
type mpz_srcptr = *const gmp::mpz_t;
type mpz_ptr = *mut gmp::mpz_t;
type mpq_srcptr = *const gmp::mpq_t;
type mpf_srcptr = *const gmp::mpf_t;
type mpf_ptr = *mut gmp::mpf_t;
type randstate_ptr = *mut gmp::randstate_t;
type mpfr_ptr = *mut mpfr_t;
type mpfr_srcptr = *const mpfr_t;

extern "C" {
    // Initialization Functions

    /// See: [`mpfr_init2`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005finit2)
    #[link_name = "mpfr_init2"]
    pub fn init2(x: mpfr_ptr, prec: prec_t);
    /// See: [`mpfr_inits2`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005finits2)
    #[link_name = "mpfr_inits2"]
    pub fn inits2(prec: prec_t, x: mpfr_ptr, ...);
    /// See: [`mpfr_clear`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fclear)
    #[link_name = "mpfr_clear"]
    pub fn clear(x: mpfr_ptr);
    /// See: [`mpfr_clears`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fclears)
    #[link_name = "mpfr_clears"]
    pub fn clears(x: mpfr_ptr, ...);
    /// See: [`mpfr_init`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005finit)
    #[link_name = "mpfr_init"]
    pub fn init(x: mpfr_ptr);
    /// See: [`mpfr_inits`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005finits)
    #[link_name = "mpfr_inits"]
    pub fn inits(x: mpfr_ptr, ...);
    /// See: [`mpfr_set_default_prec`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fset_005fdefault_005fprec)
    #[link_name = "mpfr_set_default_prec"]
    pub fn set_default_prec(prec: prec_t);
    /// See: [`mpfr_get_default_prec`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fget_005fdefault_005fprec)
    #[link_name = "mpfr_get_default_prec"]
    pub fn get_default_prec() -> prec_t;
    /// See: [`mpfr_set_prec`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fset_005fprec)
    #[link_name = "mpfr_set_prec"]
    pub fn set_prec(x: mpfr_ptr, prec: prec_t);
    /// See: [`mpfr_get_prec`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fget_005fprec)
    #[link_name = "mpfr_get_prec"]
    pub fn get_prec(x: mpfr_srcptr) -> prec_t;

    // Assignment Functions

    /// See: [`mpfr_set`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fset)
    #[link_name = "mpfr_set"]
    pub fn set(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_set_ui`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fset_005fui)
    #[link_name = "mpfr_set_ui"]
    pub fn set_ui(rop: mpfr_ptr, op: c_ulong, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_set_si`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fset_005fsi)
    #[link_name = "mpfr_set_si"]
    pub fn set_si(rop: mpfr_ptr, op: c_long, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_set_flt`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fset_005fflt)
    #[link_name = "mpfr_set_flt"]
    pub fn set_flt(rop: mpfr_ptr, op: f32, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_set_d`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fset_005fd)
    #[link_name = "mpfr_set_d"]
    pub fn set_d(rop: mpfr_ptr, op: f64, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_set_ld`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fset_005fld)
    #[link_name = "mpfr_set_ld"]
    pub fn set_ld(rop: mpfr_ptr, op: f64, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_set_z`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fset_005fz)
    #[link_name = "mpfr_set_z"]
    pub fn set_z(rop: mpfr_ptr, op: mpz_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_set_q`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fset_005fq)
    #[link_name = "mpfr_set_q"]
    pub fn set_q(rop: mpfr_ptr, op: mpq_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_set_f`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fset_005ff)
    #[link_name = "mpfr_set_f"]
    pub fn set_f(rop: mpfr_ptr, op: mpf_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_set_ui_2exp`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fset_005fui_005f2exp)
    #[link_name = "mpfr_set_ui_2exp"]
    pub fn set_ui_2exp(rop: mpfr_ptr,
                       op: c_ulong,
                       e: exp_t,
                       rnd: rnd_t)
                       -> c_int;
    /// See: [`mpfr_set_si_2exp`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fset_005fsi_005f2exp)
    #[link_name = "mpfr_set_si_2exp"]
    pub fn set_si_2exp(rop: mpfr_ptr,
                       op: c_long,
                       e: exp_t,
                       rnd: rnd_t)
                       -> c_int;
    /// See: [`mpfr_set_z_2exp`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fset_005fz_005f2exp)
    #[link_name = "mpfr_set_z_2exp"]
    pub fn set_z_2exp(rop: mpfr_ptr,
                      op: mpz_srcptr,
                      e: exp_t,
                      rnd: rnd_t)
                      -> c_int;
    /// See: [`mpfr_set_str`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fset_005fstr)
    #[link_name = "mpfr_set_str"]
    pub fn set_str(rop: mpfr_ptr,
                   s: *const c_char,
                   base: c_int,
                   rnd: rnd_t)
                   -> c_int;
    /// See: [`mpfr_strtofr`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fstrtofr)
    #[link_name = "mpfr_strtofr"]
    pub fn strtofr(rop: mpfr_ptr,
                   nptr: *const c_char,
                   endptr: *mut *mut c_char,
                   base: c_int,
                   rnd: rnd_t)
                   -> c_int;
    /// See: [`mpfr_set_nan`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fset_005fnan)
    #[link_name = "mpfr_set_nan"]
    pub fn set_nan(x: mpfr_ptr);
    /// See: [`mpfr_set_inf`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fset_005finf)
    #[link_name = "mpfr_set_inf"]
    pub fn set_inf(x: mpfr_ptr, sign: c_int);
    /// See: [`mpfr_set_zero`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fset_005fzero)
    #[link_name = "mpfr_set_zero"]
    pub fn set_zero(x: mpfr_ptr, sign: c_int);
    /// See: [`mpfr_swap`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fswap)
    #[link_name = "mpfr_swap"]
    pub fn swap(x: mpfr_ptr, y: mpfr_ptr);
}

// Combined Initialization and Assignment Functions

/// See: [`mpfr_init_set`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005finit_005fset)
#[inline]
pub unsafe fn init_set(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int {
    init(rop);
    set(rop, op, rnd)
}
/// See: [`mpfr_init_set_ui`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005finit_005fset_005fui)
#[inline]
pub unsafe fn init_set_ui(rop: mpfr_ptr, op: c_ulong, rnd: rnd_t) -> c_int {
    init(rop);
    set_ui(rop, op, rnd)
}
/// See: [`mpfr_init_set_si`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005finit_005fset_005fsi)
#[inline]
pub unsafe fn init_set_si(rop: mpfr_ptr, op: c_long, rnd: rnd_t) -> c_int {
    init(rop);
    set_si(rop, op, rnd)
}
/// See: [`mpfr_init_set_d`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005finit_005fset_005fd)
#[inline]
pub unsafe fn init_set_d(rop: mpfr_ptr, op: f64, rnd: rnd_t) -> c_int {
    init(rop);
    set_d(rop, op, rnd)
}
/// See: [`mpfr_init_set_ld`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005finit_005fset_005fld)
#[inline]
pub unsafe fn init_set_ld(rop: mpfr_ptr, op: f64, rnd: rnd_t) -> c_int {
    init(rop);
    set_ld(rop, op, rnd)
}
/// See: [`mpfr_init_set_z`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005finit_005fset_005fz)
#[inline]
pub unsafe fn init_set_z(rop: mpfr_ptr, op: mpz_srcptr, rnd: rnd_t) -> c_int {
    init(rop);
    set_z(rop, op, rnd)
}
/// See: [`mpfr_init_set_q`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005finit_005fset_005fq)
#[inline]
pub unsafe fn init_set_q(rop: mpfr_ptr, op: mpq_srcptr, rnd: rnd_t) -> c_int {
    init(rop);
    set_q(rop, op, rnd)
}
/// See: [`mpfr_init_set_f`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005finit_005fset_005ff)
#[inline]
pub unsafe fn init_set_f(rop: mpfr_ptr, op: mpf_srcptr, rnd: rnd_t) -> c_int {
    init(rop);
    set_f(rop, op, rnd)
}
extern "C" {
    /// See: [`mpfr_init_set_str`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005finit_005fset_005fstr)
    #[link_name = "mpfr_init_set_str"]
    pub fn init_set_str(x: mpfr_ptr,
                        s: *const c_char,
                        base: c_int,
                        rnd: rnd_t)
                        -> c_int;

    // Conversion Functions

    /// See: [`mpfr_get_flt`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fget_005fflt)
    #[link_name = "mpfr_get_flt"]
    pub fn get_flt(op: mpfr_srcptr, rnd: rnd_t) -> f32;
    /// See: [`mpfr_get_d`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fget_005fd)
    #[link_name = "mpfr_get_d"]
    pub fn get_d(op: mpfr_srcptr, rnd: rnd_t) -> f64;
    /// See: [`mpfr_get_ld`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fget_005fld)
    #[link_name = "mpfr_get_ld"]
    pub fn get_ld(op: mpfr_srcptr, rnd: rnd_t) -> f64;
    /// See: [`mpfr_get_si`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fget_005fsi)
    #[link_name = "mpfr_get_si"]
    pub fn get_si(op: mpfr_srcptr, rnd: rnd_t) -> c_long;
    /// See: [`mpfr_get_ui`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fget_005fui)
    #[link_name = "mpfr_get_ui"]
    pub fn get_ui(op: mpfr_srcptr, rnd: rnd_t) -> c_ulong;
    /// See: [`mpfr_get_d_2exp`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fget_005fd_005f2exp)
    #[link_name = "mpfr_get_d_2exp"]
    pub fn get_d_2exp(exp: *mut c_long, op: mpfr_srcptr, rnd: rnd_t) -> f64;
    /// See: [`mpfr_get_ld_2exp`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fget_005fld_005f2exp)
    #[link_name = "mpfr_get_ld_2exp"]
    pub fn get_ld_2exp(exp: *mut c_long, op: mpfr_srcptr, rnd: rnd_t) -> f64;
    /// See: [`mpfr_frexp`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005ffrexp)
    #[link_name = "mpfr_frexp"]
    pub fn frexp(exp: *mut exp_t,
                 y: mpfr_ptr,
                 x: mpfr_srcptr,
                 rnd: rnd_t)
                 -> c_int;
    /// See: [`mpfr_get_z_2exp`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fget_005fz_005f2exp)
    #[link_name = "mpfr_get_z_2exp"]
    pub fn get_z_2exp(rop: mpz_ptr, op: mpfr_srcptr) -> exp_t;
    /// See: [`mpfr_get_z`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fget_005fz)
    #[link_name = "mpfr_get_z"]
    pub fn get_z(z: mpz_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_get_f`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fget_005ff)
    #[link_name = "mpfr_get_f"]
    pub fn get_f(rop: mpf_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_get_str`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fget_005fstr)
    #[link_name = "mpfr_get_str"]
    pub fn get_str(str: *mut c_char,
                   expptr: *mut exp_t,
                   b: c_int,
                   n: usize,
                   op: mpfr_srcptr,
                   rnd: rnd_t)
                   -> *mut c_char;
    /// See: [`mpfr_free_str`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005ffree_005fstr)
    #[link_name = "mpfr_free_str"]
    pub fn free_str(str: *mut c_char);
    /// See: [`mpfr_fits_ulong_p`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005ffits_005fulong_005fp)
    #[link_name = "mpfr_fits_ulong_p"]
    pub fn fits_ulong_p(op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_fits_slong_p`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005ffits_005fslong_005fp)
    #[link_name = "mpfr_fits_slong_p"]
    pub fn fits_slong_p(op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_fits_uint_p`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005ffits_005fuint_005fp)
    #[link_name = "mpfr_fits_uint_p"]
    pub fn fits_uint_p(op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_fits_sint_p`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005ffits_005fsint_005fp)
    #[link_name = "mpfr_fits_sint_p"]
    pub fn fits_sint_p(op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_fits_ushort_p`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005ffits_005fushort_005fp)
    #[link_name = "mpfr_fits_ushort_p"]
    pub fn fits_ushort_p(op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_fits_sshort_p`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005ffits_005fsshort_005fp)
    #[link_name = "mpfr_fits_sshort_p"]
    pub fn fits_sshort_p(op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_fits_uintmax_p`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005ffits_005fuintmax_005fp)
    #[link_name = "mpfr_fits_uintmax_p"]
    pub fn fits_uintmax_p(op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_fits_intmax_p`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005ffits_005fintmax_005fp)
    #[link_name = "mpfr_fits_intmax_p"]
    pub fn fits_intmax_p(op: mpfr_srcptr, rnd: rnd_t) -> c_int;

    // Basic Arithmetic Functions

    /// See: [`mpfr_add`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fadd)
    #[link_name = "mpfr_add"]
    pub fn add(rop: mpfr_ptr,
               op1: mpfr_srcptr,
               op2: mpfr_srcptr,
               rnd: rnd_t)
               -> c_int;
    /// See: [`mpfr_add_ui`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fadd_005fui)
    #[link_name = "mpfr_add_ui"]
    pub fn add_ui(rop: mpfr_ptr,
                  op1: mpfr_srcptr,
                  op2: c_ulong,
                  rnd: rnd_t)
                  -> c_int;
    /// See: [`mpfr_add_si`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fadd_005fsi)
    #[link_name = "mpfr_add_si"]
    pub fn add_si(rop: mpfr_ptr,
                  op1: mpfr_srcptr,
                  op2: c_long,
                  rnd: rnd_t)
                  -> c_int;
    /// See: [`mpfr_add_d`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fadd_005fd)
    #[link_name = "mpfr_add_d"]
    pub fn add_d(rop: mpfr_ptr,
                 op1: mpfr_srcptr,
                 op2: f64,
                 rnd: rnd_t)
                 -> c_int;
    /// See: [`mpfr_add_z`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fadd_005fz)
    #[link_name = "mpfr_add_z"]
    pub fn add_z(rop: mpfr_ptr,
                 op1: mpfr_srcptr,
                 op2: mpz_srcptr,
                 rnd: rnd_t)
                 -> c_int;
    /// See: [`mpfr_add_q`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fadd_005fq)
    #[link_name = "mpfr_add_q"]
    pub fn add_q(rop: mpfr_ptr,
                 op1: mpfr_srcptr,
                 op2: mpq_srcptr,
                 rnd: rnd_t)
                 -> c_int;
    /// See: [`mpfr_sub`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fsub)
    #[link_name = "mpfr_sub"]
    pub fn sub(rop: mpfr_ptr,
               op1: mpfr_srcptr,
               op2: mpfr_srcptr,
               rnd: rnd_t)
               -> c_int;
    /// See: [`mpfr_ui_sub`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fui_005fsub)
    #[link_name = "mpfr_ui_sub"]
    pub fn ui_sub(rop: mpfr_ptr,
                  op1: c_ulong,
                  op2: mpfr_srcptr,
                  rnd: rnd_t)
                  -> c_int;
    /// See: [`mpfr_sub_ui`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fsub_005fui)
    #[link_name = "mpfr_sub_ui"]
    pub fn sub_ui(rop: mpfr_ptr,
                  op1: mpfr_srcptr,
                  op2: c_ulong,
                  rnd: rnd_t)
                  -> c_int;
    /// See: [`mpfr_si_sub`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fsi_005fsub)
    #[link_name = "mpfr_si_sub"]
    pub fn si_sub(rop: mpfr_ptr,
                  op1: c_long,
                  op2: mpfr_srcptr,
                  rnd: rnd_t)
                  -> c_int;
    /// See: [`mpfr_sub_si`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fsub_005fsi)
    #[link_name = "mpfr_sub_si"]
    pub fn sub_si(rop: mpfr_ptr,
                  op1: mpfr_srcptr,
                  op2: c_long,
                  rnd: rnd_t)
                  -> c_int;
    /// See: [`mpfr_d_sub`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fd_005fsub)
    #[link_name = "mpfr_d_sub"]
    pub fn d_sub(rop: mpfr_ptr,
                 op1: f64,
                 op2: mpfr_srcptr,
                 rnd: rnd_t)
                 -> c_int;
    /// See: [`mpfr_sub_d`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fsub_005fd)
    #[link_name = "mpfr_sub_d"]
    pub fn sub_d(rop: mpfr_ptr,
                 op1: mpfr_srcptr,
                 op2: f64,
                 rnd: rnd_t)
                 -> c_int;
    /// See: [`mpfr_z_sub`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fz_005fsub)
    #[link_name = "mpfr_z_sub"]
    pub fn z_sub(rop: mpfr_ptr,
                 op1: mpz_srcptr,
                 op2: mpfr_srcptr,
                 rnd: rnd_t)
                 -> c_int;
    /// See: [`mpfr_sub_z`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fsub_005fz)
    #[link_name = "mpfr_sub_z"]
    pub fn sub_z(rop: mpfr_ptr,
                 op1: mpfr_srcptr,
                 op2: mpz_srcptr,
                 rnd: rnd_t)
                 -> c_int;
    /// See: [`mpfr_sub_q`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fsub_005fq)
    #[link_name = "mpfr_sub_q"]
    pub fn sub_q(rop: mpfr_ptr,
                 op1: mpfr_srcptr,
                 op2: mpq_srcptr,
                 rnd: rnd_t)
                 -> c_int;
    /// See: [`mpfr_mul`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fmul)
    #[link_name = "mpfr_mul"]
    pub fn mul(rop: mpfr_ptr,
               op1: mpfr_srcptr,
               op2: mpfr_srcptr,
               rnd: rnd_t)
               -> c_int;
    /// See: [`mpfr_mul_ui`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fmul_005fui)
    #[link_name = "mpfr_mul_ui"]
    pub fn mul_ui(rop: mpfr_ptr,
                  op1: mpfr_srcptr,
                  op2: c_ulong,
                  rnd: rnd_t)
                  -> c_int;
    /// See: [`mpfr_mul_si`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fmul_005fsi)
    #[link_name = "mpfr_mul_si"]
    pub fn mul_si(rop: mpfr_ptr,
                  op1: mpfr_srcptr,
                  op2: c_long,
                  rnd: rnd_t)
                  -> c_int;
    /// See: [`mpfr_mul_d`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fmul_005fd)
    #[link_name = "mpfr_mul_d"]
    pub fn mul_d(rop: mpfr_ptr,
                 op1: mpfr_srcptr,
                 op2: f64,
                 rnd: rnd_t)
                 -> c_int;
    /// See: [`mpfr_mul_z`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fmul_005fz)
    #[link_name = "mpfr_mul_z"]
    pub fn mul_z(rop: mpfr_ptr,
                 op1: mpfr_srcptr,
                 op2: mpz_srcptr,
                 rnd: rnd_t)
                 -> c_int;
    /// See: [`mpfr_mul_q`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fmul_005fq)
    #[link_name = "mpfr_mul_q"]
    pub fn mul_q(rop: mpfr_ptr,
                 op1: mpfr_srcptr,
                 op2: mpq_srcptr,
                 rnd: rnd_t)
                 -> c_int;
    /// See: [`mpfr_sqr`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fsqr)
    #[link_name = "mpfr_sqr"]
    pub fn sqr(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_div`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fdiv)
    #[link_name = "mpfr_div"]
    pub fn div(rop: mpfr_ptr,
               op1: mpfr_srcptr,
               op2: mpfr_srcptr,
               rnd: rnd_t)
               -> c_int;
    /// See: [`mpfr_ui_div`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fui_005fdiv)
    #[link_name = "mpfr_ui_div"]
    pub fn ui_div(rop: mpfr_ptr,
                  op1: c_ulong,
                  op2: mpfr_srcptr,
                  rnd: rnd_t)
                  -> c_int;
    /// See: [`mpfr_div_ui`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fdiv_005fui)
    #[link_name = "mpfr_div_ui"]
    pub fn div_ui(rop: mpfr_ptr,
                  op1: mpfr_srcptr,
                  op2: c_ulong,
                  rnd: rnd_t)
                  -> c_int;
    /// See: [`mpfr_si_div`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fsi_005fdiv)
    #[link_name = "mpfr_si_div"]
    pub fn si_div(rop: mpfr_ptr,
                  op1: c_long,
                  op2: mpfr_srcptr,
                  rnd: rnd_t)
                  -> c_int;
    /// See: [`mpfr_div_si`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fdiv_005fsi)
    #[link_name = "mpfr_div_si"]
    pub fn div_si(rop: mpfr_ptr,
                  op1: mpfr_srcptr,
                  op2: c_long,
                  rnd: rnd_t)
                  -> c_int;
    /// See: [`mpfr_d_div`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fd_005fdiv)
    #[link_name = "mpfr_d_div"]
    pub fn d_div(rop: mpfr_ptr,
                 op1: f64,
                 op2: mpfr_srcptr,
                 rnd: rnd_t)
                 -> c_int;
    /// See: [`mpfr_div_d`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fdiv_005fd)
    #[link_name = "mpfr_div_d"]
    pub fn div_d(rop: mpfr_ptr,
                 op1: mpfr_srcptr,
                 op2: f64,
                 rnd: rnd_t)
                 -> c_int;
    /// See: [`mpfr_div_z`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fdiv_005fz)
    #[link_name = "mpfr_div_z"]
    pub fn div_z(rop: mpfr_ptr,
                 op1: mpfr_srcptr,
                 op2: mpz_srcptr,
                 rnd: rnd_t)
                 -> c_int;
    /// See: [`mpfr_div_q`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fdiv_005fq)
    #[link_name = "mpfr_div_q"]
    pub fn div_q(rop: mpfr_ptr,
                 op1: mpfr_srcptr,
                 op2: mpq_srcptr,
                 rnd: rnd_t)
                 -> c_int;
    /// See: [`mpfr_rec_sqrt`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005frec_005fsqrt)
    #[link_name = "mpfr_sqrt"]
    pub fn sqrt(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_sqrt_ui`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fsqrt_005fui)
    #[link_name = "mpfr_sqrt_ui"]
    pub fn sqrt_ui(rop: mpfr_ptr, op: c_ulong, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_rec_sqrt`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005frec_005fsqrt)
    #[link_name = "mpfr_rec_sqrt"]
    pub fn rec_sqrt(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_cbrt`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fcbrt)
    #[link_name = "mpfr_cbrt"]
    pub fn cbrt(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_root`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005froot)
    #[link_name = "mpfr_root"]
    pub fn root(rop: mpfr_ptr,
                op: mpfr_srcptr,
                k: c_ulong,
                rnd: rnd_t)
                -> c_int;
    /// See: [`mpfr_pow`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fpow)
    #[link_name = "mpfr_pow"]
    pub fn pow(rop: mpfr_ptr,
               op1: mpfr_srcptr,
               op2: mpfr_srcptr,
               rnd: rnd_t)
               -> c_int;
    /// See: [`mpfr_pow_ui`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fpow_005fui)
    #[link_name = "mpfr_pow_ui"]
    pub fn pow_ui(rop: mpfr_ptr,
                  op1: mpfr_srcptr,
                  op2: c_ulong,
                  rnd: rnd_t)
                  -> c_int;
    /// See: [`mpfr_pow_si`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fpow_005fsi)
    #[link_name = "mpfr_pow_si"]
    pub fn pow_si(rop: mpfr_ptr,
                  op1: mpfr_srcptr,
                  op2: c_long,
                  rnd: rnd_t)
                  -> c_int;
    /// See: [`mpfr_pow_z`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fpow_005fz)
    #[link_name = "mpfr_pow_z"]
    pub fn pow_z(rop: mpfr_ptr,
                 op1: mpfr_srcptr,
                 op2: mpz_srcptr,
                 rnd: rnd_t)
                 -> c_int;
    /// See: [`mpfr_ui_pow_ui`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fui_005fpow_005fui)
    #[link_name = "mpfr_ui_pow_ui"]
    pub fn ui_pow_ui(rop: mpfr_ptr,
                     op1: c_ulong,
                     op2: c_ulong,
                     rnd: rnd_t)
                     -> c_int;
    /// See: [`mpfr_ui_pow`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fui_005fpow)
    #[link_name = "mpfr_ui_pow"]
    pub fn ui_pow(rop: mpfr_ptr,
                  op1: c_ulong,
                  op2: mpfr_srcptr,
                  rnd: rnd_t)
                  -> c_int;
    /// See: [`mpfr_neg`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fneg)
    #[link_name = "mpfr_neg"]
    pub fn neg(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_abs`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fabs)
    #[link_name = "mpfr_abs"]
    pub fn abs(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_dim`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fdim)
    #[link_name = "mpfr_dim"]
    pub fn dim(rop: mpfr_ptr,
               op1: mpfr_srcptr,
               op2: mpfr_srcptr,
               rnd: rnd_t)
               -> c_int;
    /// See: [`mpfr_mul_2ui`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fmul_005f2ui)
    #[link_name = "mpfr_mul_2ui"]
    pub fn mul_2ui(rop: mpfr_ptr,
                   op1: mpfr_srcptr,
                   op2: c_ulong,
                   rnd: rnd_t)
                   -> c_int;
    /// See: [`mpfr_mul_2si`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fmul_005f2si)
    #[link_name = "mpfr_mul_2si"]
    pub fn mul_2si(rop: mpfr_ptr,
                   op1: mpfr_srcptr,
                   op2: c_long,
                   rnd: rnd_t)
                   -> c_int;
    /// See: [`mpfr_div_2ui`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fdiv_005f2ui)
    #[link_name = "mpfr_div_2ui"]
    pub fn div_2ui(rop: mpfr_ptr,
                   op1: mpfr_srcptr,
                   op2: c_ulong,
                   rnd: rnd_t)
                   -> c_int;
    /// See: [`mpfr_div_2si`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fdiv_005f2si)
    #[link_name = "mpfr_div_2si"]
    pub fn div_2si(rop: mpfr_ptr,
                   op1: mpfr_srcptr,
                   op2: c_long,
                   rnd: rnd_t)
                   -> c_int;

    // Comparison Functions

    /// See: [`mpfr_cmp`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fcmp)
    #[link_name = "mpfr_cmp"]
    pub fn cmp(op1: mpfr_srcptr, op2: mpfr_srcptr) -> c_int;
    /// See: [`mpfr_cmp_ui`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fcmp_005fui)
    #[link_name = "mpfr_cmp_ui"]
    pub fn cmp_ui(op1: mpfr_srcptr, op2: c_ulong) -> c_int;
    /// See: [`mpfr_cmp_si`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fcmp_005fsi)
    #[link_name = "mpfr_cmp_si"]
    pub fn cmp_si(op1: mpfr_srcptr, op2: c_long) -> c_int;
    /// See: [`mpfr_cmp_d`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fcmp_005fd)
    #[link_name = "mpfr_cmp_d"]
    pub fn cmp_d(op1: mpfr_srcptr, op2: f64) -> c_int;
    /// See: [`mpfr_cmp_ld`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fcmp_005fld)
    #[link_name = "mpfr_cmp_ld"]
    pub fn cmp_ld(op1: mpfr_srcptr, op2: f64) -> c_int;
    /// See: [`mpfr_cmp_z`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fcmp_005fz)
    #[link_name = "mpfr_cmp_z"]
    pub fn cmp_z(op1: mpfr_srcptr, op2: mpz_srcptr) -> c_int;
    /// See: [`mpfr_cmp_q`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fcmp_005fq)
    #[link_name = "mpfr_cmp_q"]
    pub fn cmp_q(op1: mpfr_srcptr, op2: mpq_srcptr) -> c_int;
    /// See: [`mpfr_cmp_f`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fcmp_005ff)
    #[link_name = "mpfr_cmp_f"]
    pub fn cmp_f(op1: mpfr_srcptr, op2: mpf_srcptr) -> c_int;
    /// See: [`mpfr_cmp_ui_2exp`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fcmp_005fui_005f2exp)
    #[link_name = "mpfr_cmp_ui_2exp"]
    pub fn cmp_ui_2exp(op1: mpfr_srcptr, op2: c_ulong, e: exp_t) -> c_int;
    /// See: [`mpfr_cmp_si_2exp`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fcmp_005fsi_005f2exp)
    #[link_name = "mpfr_cmp_si_2exp"]
    pub fn cmp_si_2exp(op1: mpfr_srcptr, op2: c_long, e: exp_t) -> c_int;
    /// See: [`mpfr_cmpabs`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fcmpabs)
    #[link_name = "mpfr_cmpabs"]
    pub fn cmpabs(op1: mpfr_srcptr, op2: mpfr_srcptr) -> c_int;
    /// See: [`mpfr_nan_p`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fnan_005fp)
    #[link_name = "mpfr_nan_p"]
    pub fn nan_p(op: mpfr_srcptr) -> c_int;
    /// See: [`mpfr_inf_p`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005finf_005fp)
    #[link_name = "mpfr_inf_p"]
    pub fn inf_p(op: mpfr_srcptr) -> c_int;
    /// See: [`mpfr_number_p`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fnumber_005fp)
    #[link_name = "mpfr_number_p"]
    pub fn number_p(op: mpfr_srcptr) -> c_int;
    /// See: [`mpfr_zero_p`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fzero_005fp)
    #[link_name = "mpfr_zero_p"]
    pub fn zero_p(op: mpfr_srcptr) -> c_int;
    /// See: [`mpfr_regular_p`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fregular_005fp)
    #[link_name = "mpfr_regular_p"]
    pub fn regular_p(op: mpfr_srcptr) -> c_int;
    /// See: [`mpfr_sgn`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fsgn)
    #[link_name = "mpfr_sgn"]
    pub fn sgn(op: mpfr_srcptr) -> c_int;
    /// See: [`mpfr_greater_p`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fgreater_005fp)
    #[link_name = "mpfr_greater_p"]
    pub fn greater_p(op1: mpfr_srcptr, op2: mpfr_srcptr) -> c_int;
    /// See: [`mpfr_greaterequal_p`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fgreaterequal_005fp)
    #[link_name = "mpfr_greaterequal_p"]
    pub fn greaterequal_p(op1: mpfr_srcptr, op2: mpfr_srcptr) -> c_int;
    /// See: [`mpfr_less_p`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fless_005fp)
    #[link_name = "mpfr_less_p"]
    pub fn less_p(op1: mpfr_srcptr, op2: mpfr_srcptr) -> c_int;
    /// See: [`mpfr_lessequal_p`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005flessequal_005fp)
    #[link_name = "mpfr_lessequal_p"]
    pub fn lessequal_p(op1: mpfr_srcptr, op2: mpfr_srcptr) -> c_int;
    /// See: [`mpfr_equal_p`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fequal_005fp)
    #[link_name = "mpfr_equal_p"]
    pub fn equal_p(op1: mpfr_srcptr, op2: mpfr_srcptr) -> c_int;
    /// See: [`mpfr_lessgreater_p`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005flessgreater_005fp)
    #[link_name = "mpfr_lessgreater_p"]
    pub fn lessgreater_p(op1: mpfr_srcptr, op2: mpfr_srcptr) -> c_int;
    /// See: [`mpfr_unordered_p`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005funordered_005fp)
    #[link_name = "mpfr_unordered_p"]
    pub fn unordered_p(op1: mpfr_srcptr, op2: mpfr_srcptr) -> c_int;

    // Special Functions

    /// See: [`mpfr_log`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005flog)
    #[link_name = "mpfr_log"]
    pub fn log(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_log2`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005flog2)
    #[link_name = "mpfr_log2"]
    pub fn log2(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_log10`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005flog10)
    #[link_name = "mpfr_log10"]
    pub fn log10(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_exp`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fexp)
    #[link_name = "mpfr_exp"]
    pub fn exp(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_exp2`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fexp2)
    #[link_name = "mpfr_exp2"]
    pub fn exp2(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_exp10`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fexp10)
    #[link_name = "mpfr_exp10"]
    pub fn exp10(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_cos`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fcos)
    #[link_name = "mpfr_cos"]
    pub fn cos(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_sin`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fsin)
    #[link_name = "mpfr_sin"]
    pub fn sin(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_tan`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005ftan)
    #[link_name = "mpfr_tan"]
    pub fn tan(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_sin_cos`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fsin_005fcos)
    #[link_name = "mpfr_sin_cos"]
    pub fn sin_cos(sop: mpfr_ptr,
                   cop: mpfr_ptr,
                   op: mpfr_srcptr,
                   rnd: rnd_t)
                   -> c_int;
    /// See: [`mpfr_sec`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fsec)
    #[link_name = "mpfr_sec"]
    pub fn sec(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_csc`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fcsc)
    #[link_name = "mpfr_csc"]
    pub fn csc(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_cot`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fcot)
    #[link_name = "mpfr_cot"]
    pub fn cot(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_acos`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005facos)
    #[link_name = "mpfr_acos"]
    pub fn acos(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_asin`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fasin)
    #[link_name = "mpfr_asin"]
    pub fn asin(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_atan`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fatan)
    #[link_name = "mpfr_atan"]
    pub fn atan(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_atan2`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fatan2)
    #[link_name = "mpfr_atan2"]
    pub fn atan2(rop: mpfr_ptr,
                 y: mpfr_srcptr,
                 x: mpfr_srcptr,
                 rnd: rnd_t)
                 -> c_int;
    /// See: [`mpfr_cosh`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fcosh)
    #[link_name = "mpfr_cosh"]
    pub fn cosh(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_sinh`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fsinh)
    #[link_name = "mpfr_sinh"]
    pub fn sinh(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_tanh`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005ftanh)
    #[link_name = "mpfr_tanh"]
    pub fn tanh(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_sinh_cosh`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fsinh_005fcosh)
    #[link_name = "mpfr_sinh_cosh"]
    pub fn sinh_cosh(sop: mpfr_ptr,
                     cop: mpfr_ptr,
                     op: mpfr_srcptr,
                     rnd: rnd_t)
                     -> c_int;
    /// See: [`mpfr_sech`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fsech)
    #[link_name = "mpfr_sech"]
    pub fn sech(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_csch`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fcsch)
    #[link_name = "mpfr_csch"]
    pub fn csch(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_coth`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fcoth)
    #[link_name = "mpfr_coth"]
    pub fn coth(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_acosh`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005facosh)
    #[link_name = "mpfr_acosh"]
    pub fn acosh(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_asinh`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fasinh)
    #[link_name = "mpfr_asinh"]
    pub fn asinh(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_atanh`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fatanh)
    #[link_name = "mpfr_atanh"]
    pub fn atanh(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_fac_ui`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005ffac_005fui)
    #[link_name = "mpfr_fac_ui"]
    pub fn fac_ui(rop: mpfr_ptr, op: c_ulong, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_log1p`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005flog1p)
    #[link_name = "mpfr_log1p"]
    pub fn log1p(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_expm1`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fexpm1)
    #[link_name = "mpfr_expm1"]
    pub fn expm1(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_eint`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005feint)
    #[link_name = "mpfr_eint"]
    pub fn eint(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_li2`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fli2)
    #[link_name = "mpfr_li2"]
    pub fn li2(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_gamma`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fgamma)
    #[link_name = "mpfr_gamma"]
    pub fn gamma(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_lngamma`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005flngamma)
    #[link_name = "mpfr_lngamma"]
    pub fn lngamma(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_lgamma`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005flgamma)
    #[link_name = "mpfr_lgamma"]
    pub fn lgamma(rop: mpfr_ptr,
                  signp: *mut c_int,
                  op: mpfr_srcptr,
                  rnd: rnd_t)
                  -> c_int;
    /// See: [`mpfr_digamma`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fdigamma)
    #[link_name = "mpfr_digamma"]
    pub fn digamma(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_zeta`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fzeta)
    #[link_name = "mpfr_zeta"]
    pub fn zeta(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_zeta_ui`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fzeta_005fui)
    #[link_name = "mpfr_zeta_ui"]
    pub fn zeta_ui(rop: mpfr_ptr, op: c_ulong, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_erf`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005ferf)
    #[link_name = "mpfr_erf"]
    pub fn erf(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_erfc`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005ferfc)
    #[link_name = "mpfr_erfc"]
    pub fn erfc(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_j0`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fj0)
    #[link_name = "mpfr_j0"]
    pub fn j0(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_j1`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fj1)
    #[link_name = "mpfr_j1"]
    pub fn j1(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_jn`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fjn)
    #[link_name = "mpfr_jn"]
    pub fn jn(rop: mpfr_ptr, n: c_long, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_y0`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fy0)
    #[link_name = "mpfr_y0"]
    pub fn y0(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_y1`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fy1)
    #[link_name = "mpfr_y1"]
    pub fn y1(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_yn`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fyn)
    #[link_name = "mpfr_yn"]
    pub fn yn(rop: mpfr_ptr, n: c_long, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_fma`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005ffma)
    #[link_name = "mpfr_fma"]
    pub fn fma(rop: mpfr_ptr,
               op1: mpfr_srcptr,
               op2: mpfr_srcptr,
               op3: mpfr_srcptr,
               rnd: rnd_t)
               -> c_int;
    /// See: [`mpfr_fms`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005ffms)
    #[link_name = "mpfr_fms"]
    pub fn fms(rop: mpfr_ptr,
               op1: mpfr_srcptr,
               op2: mpfr_srcptr,
               op3: mpfr_srcptr,
               rnd: rnd_t)
               -> c_int;
    /// See: [`mpfr_agm`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fagm)
    #[link_name = "mpfr_agm"]
    pub fn agm(rop: mpfr_ptr,
               op1: mpfr_srcptr,
               op2: mpfr_srcptr,
               rnd: rnd_t)
               -> c_int;
    /// See: [`mpfr_hypot`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fhypot)
    #[link_name = "mpfr_hypot"]
    pub fn hypot(rop: mpfr_ptr,
                 x: mpfr_srcptr,
                 y: mpfr_srcptr,
                 rnd: rnd_t)
                 -> c_int;
    /// See: [`mpfr_ai`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fai)
    #[link_name = "mpfr_ai"]
    pub fn ai(rop: mpfr_ptr, x: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_const_log2`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fconst_005flog2)
    #[link_name = "mpfr_const_log2"]
    pub fn const_log2(rop: mpfr_ptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_const_pi`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fconst_005fpi)
    #[link_name = "mpfr_const_pi"]
    pub fn const_pi(rop: mpfr_ptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_const_euler`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fconst_005feuler)
    #[link_name = "mpfr_const_euler"]
    pub fn const_euler(rop: mpfr_ptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_const_catalan`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fconst_005fcatalan)
    #[link_name = "mpfr_const_catalan"]
    pub fn const_catalan(rop: mpfr_ptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_free_cache`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005ffree_005fcache)
    #[link_name = "mpfr_free_cache"]
    pub fn free_cache();
    /// See: [`mpfr_sum`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fsum)
    #[link_name = "mpfr_sum"]
    pub fn sum(rop: mpfr_ptr,
               tab: *mut mpfr_ptr,
               n: c_ulong,
               rnd: rnd_t)
               -> c_int;

    // Formatted Output Functions

    /// See: [`mpfr_printf`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fprintf)
    #[link_name = "mpfr_printf"]
    pub fn printf(template: *const c_char, ...) -> c_int;
    /// See: [`mpfr_sprintf`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fsprintf)
    #[link_name = "mpfr_sprintf"]
    pub fn sprintf(buf: *mut c_char, template: *const c_char, ...) -> c_int;
    /// See: [`mpfr_snprintf`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fsnprintf)
    #[link_name = "mpfr_snprintf"]
    pub fn snprintf(buf: *mut c_char,
                    n: usize,
                    template: *const c_char,
                    ...)
                    -> c_int;
    /// See: [`mpfr_asprintf`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fasprintf)
    #[link_name = "mpfr_asprintf"]
    pub fn asprintf(str: *mut *mut c_char,
                    template: *const c_char,
                    ...)
                    -> c_int;

    // Integer and Remainder Related Functions

    /// See: [`mpfr_rint`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005frint)
    #[link_name = "mpfr_rint"]
    pub fn rint(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_ceil`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fceil)
    #[link_name = "mpfr_ceil"]
    pub fn ceil(rop: mpfr_ptr, op: mpfr_srcptr) -> c_int;
    /// See: [`mpfr_floor`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005ffloor)
    #[link_name = "mpfr_floor"]
    pub fn floor(rop: mpfr_ptr, op: mpfr_srcptr) -> c_int;
    /// See: [`mpfr_round`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fround)
    #[link_name = "mpfr_round"]
    pub fn round(rop: mpfr_ptr, op: mpfr_srcptr) -> c_int;
    /// See: [`mpfr_trunc`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005ftrunc)
    #[link_name = "mpfr_trunc"]
    pub fn trunc(rop: mpfr_ptr, op: mpfr_srcptr) -> c_int;
    /// See: [`mpfr_rint_ceil`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005frint_005fceil)
    #[link_name = "mpfr_rint_ceil"]
    pub fn rint_ceil(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_rint_floor`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005frint_005ffloor)
    #[link_name = "mpfr_rint_floor"]
    pub fn rint_floor(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_rint_round`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005frint_005fround)
    #[link_name = "mpfr_rint_round"]
    pub fn rint_round(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_rint_trunc`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005frint_005ftrunc)
    #[link_name = "mpfr_rint_trunc"]
    pub fn rint_trunc(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_frac`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005ffrac)
    #[link_name = "mpfr_frac"]
    pub fn frac(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_modf`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fmodf)
    #[link_name = "mpfr_modf"]
    pub fn modf(iop: mpfr_ptr,
                fop: mpfr_ptr,
                op: mpfr_srcptr,
                rnd: rnd_t)
                -> c_int;
    /// See: [`mpfr_fmod`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005ffmod)
    #[link_name = "mpfr_fmod"]
    pub fn fmod(r: mpfr_ptr,
                x: mpfr_srcptr,
                y: mpfr_srcptr,
                rnd: rnd_t)
                -> c_int;
    /// See: [`mpfr_remainder`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fremainder)
    #[link_name = "mpfr_remainder"]
    pub fn remainder(r: mpfr_ptr,
                     x: mpfr_srcptr,
                     y: mpfr_srcptr,
                     rnd: rnd_t)
                     -> c_int;
    /// See: [`mpfr_remquo`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fremquo)
    #[link_name = "mpfr_remquo"]
    pub fn remquo(r: mpfr_ptr,
                  q: *mut c_long,
                  x: mpfr_srcptr,
                  y: mpfr_srcptr,
                  rnd: rnd_t)
                  -> c_int;
    /// See: [`mpfr_integer_p`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005finteger_005fp)
    #[link_name = "mpfr_integer_p"]
    pub fn integer_p(op: mpfr_srcptr) -> c_int;

    // Rounding Related Functions

    /// See: [`mpfr_set_default_rounding_mode`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fset_005fdefault_005frounding_005fmode)
    #[link_name = "mpfr_set_default_rounding_mode"]
    pub fn set_default_rounding_mode(rnd: rnd_t);
    /// See: [`mpfr_get_default_rounding_mode`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fget_005fdefault_005frounding_005fmode)
    #[link_name = "mpfr_get_default_rounding_mode"]
    pub fn get_default_rounding_mode() -> rnd_t;
    /// See: [`mpfr_prec_round`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fprec_005fround)
    #[link_name = "mpfr_prec_round"]
    pub fn prec_round(x: mpfr_ptr, prec: prec_t, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_can_round`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fcan_005fround)
    #[link_name = "mpfr_can_round"]
    pub fn can_round(b: mpfr_srcptr,
                     err: exp_t,
                     rnd1: rnd_t,
                     rnd2: rnd_t,
                     prec: prec_t)
                     -> c_int;
    /// See: [`mpfr_min_prec`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fmin_005fprec)
    #[link_name = "mpfr_min_prec"]
    pub fn min_prec(x: mpfr_srcptr) -> prec_t;
    /// See: [`mpfr_print_rnd_mode`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fprint_005frnd_005fmode)
    #[link_name = "mpfr_print_rnd_mode"]
    pub fn print_rnd_mode(rnd: rnd_t) -> *const c_char;

    // Miscellaneous Functions

    /// See: [`mpfr_nexttoward`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fnexttoward)
    #[link_name = "mpfr_nexttoward"]
    pub fn nexttoward(x: mpfr_ptr, y: mpfr_srcptr);
    /// See: [`mpfr_nextabove`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fnextabove)
    #[link_name = "mpfr_nextabove"]
    pub fn nextabove(x: mpfr_ptr);
    /// See: [`mpfr_nextbelow`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fnextbelow)
    #[link_name = "mpfr_nextbelow"]
    pub fn nextbelow(x: mpfr_ptr);
    /// See: [`mpfr_min`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fmin)
    #[link_name = "mpfr_min"]
    pub fn min(rop: mpfr_ptr,
               op1: mpfr_srcptr,
               op2: mpfr_srcptr,
               rnd: rnd_t)
               -> c_int;
    /// See: [`mpfr_max`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fmax)
    #[link_name = "mpfr_max"]
    pub fn max(rop: mpfr_ptr,
               op1: mpfr_srcptr,
               op2: mpfr_srcptr,
               rnd: rnd_t)
               -> c_int;
    /// See: [`mpfr_urandomb`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005furandomb)
    #[link_name = "mpfr_urandomb"]
    pub fn urandomb(rop: mpfr_ptr, state: randstate_ptr) -> c_int;
    /// See: [`mpfr_urandom`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005furandom)
    #[link_name = "mpfr_urandom"]
    pub fn urandom(rop: mpfr_ptr, state: randstate_ptr, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_grandom`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fgrandom)
    #[link_name = "mpfr_grandom"]
    pub fn grandom(rop1: mpfr_ptr,
                   rop2: mpfr_ptr,
                   state: randstate_ptr,
                   rnd: rnd_t)
                   -> c_int;
    /// See: [`mpfr_get_exp`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fget_005fexp)
    #[link_name = "mpfr_get_exp"]
    pub fn get_exp(x: mpfr_srcptr) -> exp_t;
    /// See: [`mpfr_set_exp`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fset_005fexp)
    #[link_name = "mpfr_set_exp"]
    pub fn set_exp(x: mpfr_ptr, e: exp_t) -> c_int;
    /// See: [`mpfr_signbit`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fsignbit)
    #[link_name = "mpfr_signbit"]
    pub fn signbit(op: mpfr_srcptr) -> c_int;
    /// See: [`mpfr_setsign`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fsetsign)
    #[link_name = "mpfr_setsign"]
    pub fn setsign(rop: mpfr_ptr,
                   op: mpfr_srcptr,
                   s: c_int,
                   rnd: rnd_t)
                   -> c_int;
    /// See: [`mpfr_copysign`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fcopysign)
    #[link_name = "mpfr_copysign"]
    pub fn copysign(rop: mpfr_ptr,
                    op1: mpfr_srcptr,
                    op2: mpfr_srcptr,
                    rnd: rnd_t)
                    -> c_int;
    /// See: [`mpfr_get_version`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fget_005fversion)
    #[link_name = "mpfr_get_version"]
    pub fn get_version() -> *const c_char;
}
/// See: [`MPFR_VERSION`](http://www.mpfr.org/mpfr-current/mpfr.html#index-MPFR_005fVERSION)
pub const VERSION: c_int = (VERSION_MAJOR << 16) | (VERSION_MINOR << 8) |
                           VERSION_PATCHLEVEL;
/// See: [`MPFR_VERSION_MAJOR`](http://www.mpfr.org/mpfr-current/mpfr.html#index-MPFR_005fVERSION_005fMAJOR)
pub const VERSION_MAJOR: c_int = 3;
/// See: [`MPFR_VERSION_MINOR`](http://www.mpfr.org/mpfr-current/mpfr.html#index-MPFR_005fVERSION_005fMINOR)
pub const VERSION_MINOR: c_int = 1;
/// See: [`MPFR_VERSION_PATCHLEVEL`](http://www.mpfr.org/mpfr-current/mpfr.html#index-MPFR_005fVERSION_005fPATCHLEVEL)
pub const VERSION_PATCHLEVEL: c_int = 5;
/// See: [`MPFR_VERSION_STRING`](http://www.mpfr.org/mpfr-current/mpfr.html#index-MPFR_005fVERSION_005fSTRING)
pub const VERSION_STRING: *const c_char = b"3.1.5\0" as *const u8 as
                                          *const c_char;
/// See: [`MPFR_VERSION_NUM`](http://www.mpfr.org/mpfr-current/mpfr.html#index-MPFR_005fVERSION_005fNUM)
#[inline]
pub fn VERSION_NUM(major: c_int, minor: c_int, patchlevel: c_int) -> c_int {
    (major << 16) | (minor << 8) | patchlevel
}
extern "C" {
    /// See: [`mpfr_get_patches`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fget_005fpatches)
    #[link_name = "mpfr_get_patches"]
    pub fn get_patches() -> *const c_char;
    /// See: [`mpfr_buildopt_tls_p`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fbuildopt_005ftls_005fp)
    #[link_name = "mpfr_buildopt_tls_p"]
    pub fn buildopt_tls_p() -> c_int;
    /// See: [`mpfr_buildopt_decimal_p`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fbuildopt_005fdecimal_005fp)
    #[link_name = "mpfr_buildopt_decimal_p"]
    pub fn buildopt_decimal_p() -> c_int;
    /// See: [`mpfr_buildopt_gmpinternals_p`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fbuildopt_005fgmpinternals_005fp)
    #[link_name = "mpfr_buildopt_gmpinternals_p"]
    pub fn buildopt_gmpinternals_p() -> c_int;
    /// See: [`mpfr_buildopt_tune_case`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fbuildopt_005ftune_005fcase)
    #[link_name = "mpfr_buildopt_tune_case"]
    pub fn buildopt_tune_case() -> *const c_char;

    // Exception Related Functions

    /// See: [`mpfr_get_emin`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fget_005femin)
    #[link_name = "mpfr_get_emin"]
    pub fn get_emin() -> exp_t;
    /// See: [`mpfr_get_emax`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fget_005femax)
    #[link_name = "mpfr_get_emax"]
    pub fn get_emax() -> exp_t;
    /// See: [`mpfr_set_emin`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fset_005femin)
    #[link_name = "mpfr_set_emin"]
    pub fn set_emin(exp: exp_t) -> c_int;
    /// See: [`mpfr_set_emax`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fset_005femax)
    #[link_name = "mpfr_set_emax"]
    pub fn set_emax(exp: exp_t) -> c_int;
    /// See: [`mpfr_get_emin_min`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fget_005femin_005fmin)
    #[link_name = "mpfr_get_emin_min"]
    pub fn get_emin_min() -> exp_t;
    /// See: [`mpfr_get_emin_max`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fget_005femin_005fmax)
    #[link_name = "mpfr_get_emin_max"]
    pub fn get_emin_max() -> exp_t;
    /// See: [`mpfr_get_emax_min`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fget_005femax_005fmin)
    #[link_name = "mpfr_get_emax_min"]
    pub fn get_emax_min() -> exp_t;
    /// See: [`mpfr_get_emax_max`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fget_005femax_005fmax)
    #[link_name = "mpfr_get_emax_max"]
    pub fn get_emax_max() -> exp_t;
    /// See: [`mpfr_check_range`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fcheck_005frange)
    #[link_name = "mpfr_check_range"]
    pub fn check_range(x: mpfr_ptr, t: c_int, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_subnormalize`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fsubnormalize)
    #[link_name = "mpfr_subnormalize"]
    pub fn subnormalize(x: mpfr_ptr, t: c_int, rnd: rnd_t) -> c_int;
    /// See: [`mpfr_clear_underflow`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fclear_005funderflow)
    #[link_name = "mpfr_clear_underflow"]
    pub fn clear_underflow();
    /// See: [`mpfr_clear_overflow`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fclear_005foverflow)
    #[link_name = "mpfr_clear_overflow"]
    pub fn clear_overflow();
    /// See: [`mpfr_clear_divby0`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fclear_005fdivby0)
    #[link_name = "mpfr_clear_divby0"]
    pub fn clear_divby0();
    /// See: [`mpfr_clear_nanflag`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fclear_005fnanflag)
    #[link_name = "mpfr_clear_nanflag"]
    pub fn clear_nanflag();
    /// See: [`mpfr_clear_inexflag`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fclear_005finexflag)
    #[link_name = "mpfr_clear_inexflag"]
    pub fn clear_inexflag();
    /// See: [`mpfr_clear_erangeflag`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fclear_005ferangeflag)
    #[link_name = "mpfr_clear_erangeflag"]
    pub fn clear_erangeflag();
    /// See: [`mpfr_set_underflow`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fset_005funderflow)
    #[link_name = "mpfr_set_underflow"]
    pub fn set_underflow();
    /// See: [`mpfr_set_overflow`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fset_005foverflow)
    #[link_name = "mpfr_set_overflow"]
    pub fn set_overflow();
    /// See: [`mpfr_set_divby0`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fset_005fdivby0)
    #[link_name = "mpfr_set_divby0"]
    pub fn set_divby0();
    /// See: [`mpfr_set_nanflag`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fset_005fnanflag)
    #[link_name = "mpfr_set_nanflag"]
    pub fn set_nanflag();
    /// See: [`mpfr_set_inexflag`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fset_005finexflag)
    #[link_name = "mpfr_set_inexflag"]
    pub fn set_inexflag();
    /// See: [`mpfr_set_erangeflag`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fset_005ferangeflag)
    #[link_name = "mpfr_set_erangeflag"]
    pub fn set_erangeflag();
    /// See: [`mpfr_clear_flags`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fclear_005fflags)
    #[link_name = "mpfr_clear_flags"]
    pub fn clear_flags();
    /// See: [`mpfr_underflow_p`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005funderflow_005fp)
    #[link_name = "mpfr_underflow_p"]
    pub fn underflow_p() -> c_int;
    /// See: [`mpfr_overflow_p`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005foverflow_005fp)
    #[link_name = "mpfr_overflow_p"]
    pub fn overflow_p() -> c_int;
    /// See: [`mpfr_divby0_p`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fdivby0_005fp)
    #[link_name = "mpfr_divby0_p"]
    pub fn divby0_p() -> c_int;
    /// See: [`mpfr_nanflag_p`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fnanflag_005fp)
    #[link_name = "mpfr_nanflag_p"]
    pub fn nanflag_p() -> c_int;
    /// See: [`mpfr_inexflag_p`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005finexflag_005fp)
    #[link_name = "mpfr_inexflag_p"]
    pub fn inexflag_p() -> c_int;
    /// See: [`mpfr_erangeflag_p`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005ferangeflag_005fp)
    #[link_name = "mpfr_erangeflag_p"]
    pub fn erangeflag_p() -> c_int;

    // Compatibility with MPF

    /// See: [`mpfr_set_prec_raw`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fset_005fprec_005fraw)
    #[link_name = "mpfr_set_prec_raw"]
    pub fn set_prec_raw(x: mpfr_ptr, prec: prec_t);
    /// See: [`mpfr_eq`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005feq)
    #[link_name = "mpfr_eq"]
    pub fn eq(op1: mpfr_srcptr, op2: mpfr_srcptr, op3: c_ulong) -> c_int;
    /// See: [`mpfr_reldiff`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005freldiff)
    #[link_name = "mpfr_reldiff"]
    pub fn reldiff(rop: mpfr_ptr,
                   op1: mpfr_srcptr,
                   op2: mpfr_srcptr,
                   rnd: rnd_t);
    /// See: [`mpfr_mul_2exp`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fmul_005f2exp)
    #[link_name = "mpfr_mul_2exp"]
    pub fn mul_2exp(rop: mpfr_ptr,
                    op1: mpfr_srcptr,
                    op2: c_ulong,
                    rnd: rnd_t)
                    -> c_int;
    /// See: [`mpfr_div_2exp`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fdiv_005f2exp)
    #[link_name = "mpfr_div_2exp"]
    pub fn div_2exp(rop: mpfr_ptr,
                    op1: mpfr_srcptr,
                    op2: c_ulong,
                    rnd: rnd_t)
                    -> c_int;

    // Custom Interface

    /// See: [`mpfr_custom_get_size`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fcustom_005fget_005fsize)
    #[link_name = "mpfr_custom_get_size"]
    pub fn custom_get_size(prec: prec_t) -> usize;
    /// See: [`mpfr_custom_init`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fcustom_005finit)
    #[link_name = "mpfr_custom_init"]
    pub fn custom_init(significand: *mut c_void, prec: prec_t);
    /// See: [`mpfr_custom_init_set`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fcustom_005finit_005fset)
    #[link_name = "mpfr_custom_init_set"]
    pub fn custom_init_set(x: mpfr_ptr,
                           kind: c_int,
                           exp: exp_t,
                           prec: prec_t,
                           significand: *mut c_void);
    /// See: [`mpfr_custom_get_kind`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fcustom_005fget_005fkind)
    #[link_name = "mpfr_custom_get_kind"]
    pub fn custom_get_kind(x: mpfr_srcptr) -> c_int;
    /// See: [`mpfr_custom_get_significand`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fcustom_005fget_005fsignificand)
    #[link_name = "mpfr_custom_get_significand"]
    pub fn custom_get_significand(x: mpfr_srcptr) -> *mut c_void;
    /// See: [`mpfr_custom_get_exp`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fcustom_005fget_005fexp)
    #[link_name = "mpfr_custom_get_exp"]
    pub fn custom_get_exp(x: mpfr_srcptr) -> exp_t;
    /// See: [`mpfr_custom_move`](http://www.mpfr.org/mpfr-current/mpfr.html#index-mpfr_005fcustom_005fmove)
    #[link_name = "mpfr_custom_move"]
    pub fn custom_move(x: mpfr_ptr, new_position: *mut c_void);
}

#[cfg(test)]
mod tests {
    use mpfr;
    use std::ffi::CStr;
    use std::mem;

    #[test]
    fn check_version() {
        let version = "3.1.5";
        let from_fn = unsafe { CStr::from_ptr(mpfr::get_version()) };
        let from_constants = format!("{}.{}.{}",
                                     mpfr::VERSION_MAJOR,
                                     mpfr::VERSION_MINOR,
                                     mpfr::VERSION_PATCHLEVEL);
        let from_const_string = unsafe { CStr::from_ptr(mpfr::VERSION_STRING) };
        assert!(from_fn.to_str().unwrap() == version);
        assert!(from_constants == version);
        assert!(from_const_string.to_str().unwrap() == version);
    }

    #[test]
    fn it_runs() {
        let d: f64 = 1.0 / 3.0;
        unsafe {
            let mut fr: mpfr::mpfr_t = mem::uninitialized();
            let ptr = &mut fr as *mut _;
            mpfr::init2(ptr, 53);
            assert!(mpfr::set_d(ptr, d, mpfr::rnd_t::RNDN) == 0);
            assert!(mpfr::get_d(ptr, mpfr::rnd_t::RNDN) == d);
            mpfr::clear(ptr);
        }
    }
}
