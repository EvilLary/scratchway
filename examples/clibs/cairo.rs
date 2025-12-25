#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]

type cairo_surface_t = CairoSurface;
type cairo_t = Cairo;

type cairo_format_t = libc::c_int;
type cairo_bool_t = libc::c_int;
type cairo_font_weight_t = libc::c_int;
type cairo_font_slant_t = libc::c_int;

#[derive(Debug, Copy, Clone)]
#[repr(i32)]
pub enum CairoFontSlant {
    Normal,
    Italic,
    Oblique,
}

#[derive(Debug, Copy, Clone)]
#[repr(i32)]
pub enum CairoFormat {
    INVALID = -1,
    ARGB32 = 0,
    RGB24 = 1,
    A8 = 2,
    A1 = 3,
    RGB16_565 = 4,
    RGB30 = 5,
    RGB96F = 6,
    RGBA128F = 7,
}

#[derive(Debug, Copy, Clone)]
#[repr(i32)]
pub enum CairoFontWeight {
    Normal = 0,
    Bold = 1,
}

#[derive(Debug, Copy, Clone)]
pub enum CairoSurface {}

#[derive(Debug, Copy, Clone)]
pub enum Cairo {}

#[derive(Debug)]
#[repr(C)]
pub struct cairo_text_extents_t {
    pub x_bearing: libc::c_double,
    pub y_bearing: libc::c_double,
    pub width:     libc::c_double,
    pub height:    libc::c_double,
    pub x_advance: libc::c_double,
    pub y_advance: libc::c_double,
}

#[link(name = "cairo", kind = "dylib")]
unsafe extern "C" {
    pub fn cairo_format_stride_for_width(format: cairo_format_t, width: libc::c_int)
    -> libc::c_int;

    pub fn cairo_image_surface_create(
        format: cairo_format_t, width: libc::c_int, height: libc::c_int,
    ) -> cairo_surface_t;

    pub fn cairo_image_surface_create_for_data(
        data: *mut u8, format: cairo_format_t, width: libc::c_int, height: libc::c_int,
        stride: libc::c_int,
    ) -> *mut cairo_surface_t;

    pub fn cairo_set_source_rgb(
        cr: *mut cairo_t, red: libc::c_double, green: libc::c_double, blue: libc::c_double,
    );

    pub fn cairo_set_source_rgba(
        cr: *mut cairo_t, red: libc::c_double, green: libc::c_double, blue: libc::c_double,
        alpha: libc::c_double,
    );

    pub fn cairo_create(target: *mut cairo_surface_t) -> *mut cairo_t;

    pub fn cairo_select_font_face(
        cr: *mut cairo_t, family: *const libc::c_char, slant: cairo_font_slant_t,
        weight: cairo_font_weight_t,
    );

    pub fn cairo_set_font_size(cr: *mut cairo_t, size: libc::c_double);

    pub fn cairo_text_extents(
        cr: *mut cairo_t, utf8: *const libc::c_char, extents: *mut cairo_text_extents_t,
    );

    pub fn cairo_move_to(cr: *mut cairo_t, x: libc::c_double, y: libc::c_double);

    pub fn cairo_show_text(cr: *mut cairo_t, utf8: *const libc::c_char);

    pub fn cairo_set_line_width(cr: *mut cairo_t, width: libc::c_double);

    pub fn cairo_rectangle(
        cr: *mut cairo_t, x: libc::c_double, y: libc::c_double, width: libc::c_double,
        height: libc::c_double,
    );

    pub fn cairo_stroke(cr: *mut cairo_t);

    pub fn cairo_fill(cr: *mut cairo_t);

    pub fn cairo_line_to(cr: *mut cairo_t, x: libc::c_double, y: libc::c_double);

    pub fn cairo_rel_line_to(cr: *mut cairo_t, dx: libc::c_double, dy: libc::c_double);

    pub fn cairo_surface_destroy(surface: *mut cairo_surface_t);

    pub fn cairo_destroy(cr: *mut cairo_t);
}
