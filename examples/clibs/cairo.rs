#![allow(non_upper_case_globals, non_snake_case, non_camel_case_types, unsafe_op_in_unsafe_fn, unused)]

#[doc = " cairo_bool_t:\n\n #cairo_bool_t is used for boolean values. Returns of type\n #cairo_bool_t will always be either 0 or 1, but testing against\n these values explicitly is not encouraged; just use the\n value as a boolean condition.\n\n <informalexample><programlisting>\n  if (cairo_in_stroke (cr, x, y)) {\n      /<!-- -->* do something *<!-- -->/\n  }\n </programlisting></informalexample>\n\n Since: 1.0"]
pub type cairo_bool_t = ::core::ffi::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _cairo {
    _unused: [u8; 0],
}
#[doc = " cairo_t:\n\n A #cairo_t contains the current state of the rendering device,\n including coordinates of yet to be drawn shapes.\n\n Cairo contexts, as #cairo_t objects are named, are central to\n cairo and all drawing with cairo is always done to a #cairo_t\n object.\n\n Memory management of #cairo_t is done with\n cairo_reference() and cairo_destroy().\n\n Since: 1.0"]
pub type cairo_t = _cairo;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _cairo_surface {
    _unused: [u8; 0],
}
#[doc = " cairo_surface_t:\n\n A #cairo_surface_t represents an image, either as the destination\n of a drawing operation or as source when drawing onto another\n surface.  To draw to a #cairo_surface_t, create a cairo context\n with the surface as the target, using cairo_create().\n\n There are different subtypes of #cairo_surface_t for\n different drawing backends; for example, cairo_image_surface_create()\n creates a bitmap image in memory.\n The type of a surface can be queried with cairo_surface_get_type().\n\n The initial contents of a surface after creation depend upon the manner\n of its creation. If cairo creates the surface and backing storage for\n the user, it will be initially cleared; for example,\n cairo_image_surface_create() and cairo_surface_create_similar().\n Alternatively, if the user passes in a reference to some backing storage\n and asks cairo to wrap that in a #cairo_surface_t, then the contents are\n not modified; for example, cairo_image_surface_create_for_data() and\n cairo_xlib_surface_create().\n\n Memory management of #cairo_surface_t is done with\n cairo_surface_reference() and cairo_surface_destroy().\n\n Since: 1.0"]
pub type cairo_surface_t = _cairo_surface;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _cairo_device {
    _unused: [u8; 0],
}
#[doc = " cairo_device_t:\n\n A #cairo_device_t represents the driver interface for drawing\n operations to a #cairo_surface_t.  There are different subtypes of\n #cairo_device_t for different drawing backends.\n\n The type of a device can be queried with cairo_device_get_type().\n\n Memory management of #cairo_device_t is done with\n cairo_device_reference() and cairo_device_destroy().\n\n Since: 1.10"]
pub type cairo_device_t = _cairo_device;
#[doc = " cairo_matrix_t:\n @xx: xx component of the affine transformation\n @yx: yx component of the affine transformation\n @xy: xy component of the affine transformation\n @yy: yy component of the affine transformation\n @x0: X translation component of the affine transformation\n @y0: Y translation component of the affine transformation\n\n A #cairo_matrix_t holds an affine transformation, such as a scale,\n rotation, shear, or a combination of those. The transformation of\n a point (x, y) is given by:\n <programlisting>\n     x_new = xx * x + xy * y + x0;\n     y_new = yx * x + yy * y + y0;\n </programlisting>\n\n Since: 1.0"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _cairo_matrix {
    pub xx: f64,
    pub yx: f64,
    pub xy: f64,
    pub yy: f64,
    pub x0: f64,
    pub y0: f64,
}
#[doc = " cairo_matrix_t:\n @xx: xx component of the affine transformation\n @yx: yx component of the affine transformation\n @xy: xy component of the affine transformation\n @yy: yy component of the affine transformation\n @x0: X translation component of the affine transformation\n @y0: Y translation component of the affine transformation\n\n A #cairo_matrix_t holds an affine transformation, such as a scale,\n rotation, shear, or a combination of those. The transformation of\n a point (x, y) is given by:\n <programlisting>\n     x_new = xx * x + xy * y + x0;\n     y_new = yx * x + yy * y + y0;\n </programlisting>\n\n Since: 1.0"]
pub type cairo_matrix_t = _cairo_matrix;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _cairo_pattern {
    _unused: [u8; 0],
}
#[doc = " cairo_pattern_t:\n\n A #cairo_pattern_t represents a source when drawing onto a\n surface. There are different subtypes of #cairo_pattern_t,\n for different types of sources; for example,\n cairo_pattern_create_rgb() creates a pattern for a solid\n opaque color.\n\n Other than various\n <function>cairo_pattern_create_<emphasis>type</emphasis>()</function>\n functions, some of the pattern types can be implicitly created using various\n <function>cairo_set_source_<emphasis>type</emphasis>()</function> functions;\n for example cairo_set_source_rgb().\n\n The type of a pattern can be queried with cairo_pattern_get_type().\n\n Memory management of #cairo_pattern_t is done with\n cairo_pattern_reference() and cairo_pattern_destroy().\n\n Since: 1.0"]
pub type cairo_pattern_t = _cairo_pattern;
#[doc = " cairo_destroy_func_t:\n @data: The data element being destroyed.\n\n #cairo_destroy_func_t the type of function which is called when a\n data element is destroyed. It is passed the pointer to the data\n element and should free any memory and resources allocated for it.\n\n Since: 1.0"]
pub type cairo_destroy_func_t =
    ::core::option::Option<unsafe extern "C" fn(data: *mut ::core::ffi::c_void)>;
#[doc = " cairo_user_data_key_t:\n @unused: not used; ignore.\n\n #cairo_user_data_key_t is used for attaching user data to cairo\n data structures.  The actual contents of the struct is never used,\n and there is no need to initialize the object; only the unique\n address of a #cairo_data_key_t object is used.  Typically, you\n would just use the address of a static #cairo_data_key_t object.\n\n Since: 1.0"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _cairo_user_data_key {
    pub unused: ::core::ffi::c_int,
}
#[doc = " cairo_user_data_key_t:\n @unused: not used; ignore.\n\n #cairo_user_data_key_t is used for attaching user data to cairo\n data structures.  The actual contents of the struct is never used,\n and there is no need to initialize the object; only the unique\n address of a #cairo_data_key_t object is used.  Typically, you\n would just use the address of a static #cairo_data_key_t object.\n\n Since: 1.0"]
pub type cairo_user_data_key_t = _cairo_user_data_key;
pub const CAIRO_STATUS_SUCCESS: _cairo_status = 0;
pub const CAIRO_STATUS_NO_MEMORY: _cairo_status = 1;
pub const CAIRO_STATUS_INVALID_RESTORE: _cairo_status = 2;
pub const CAIRO_STATUS_INVALID_POP_GROUP: _cairo_status = 3;
pub const CAIRO_STATUS_NO_CURRENT_POINT: _cairo_status = 4;
pub const CAIRO_STATUS_INVALID_MATRIX: _cairo_status = 5;
pub const CAIRO_STATUS_INVALID_STATUS: _cairo_status = 6;
pub const CAIRO_STATUS_NULL_POINTER: _cairo_status = 7;
pub const CAIRO_STATUS_INVALID_STRING: _cairo_status = 8;
pub const CAIRO_STATUS_INVALID_PATH_DATA: _cairo_status = 9;
pub const CAIRO_STATUS_READ_ERROR: _cairo_status = 10;
pub const CAIRO_STATUS_WRITE_ERROR: _cairo_status = 11;
pub const CAIRO_STATUS_SURFACE_FINISHED: _cairo_status = 12;
pub const CAIRO_STATUS_SURFACE_TYPE_MISMATCH: _cairo_status = 13;
pub const CAIRO_STATUS_PATTERN_TYPE_MISMATCH: _cairo_status = 14;
pub const CAIRO_STATUS_INVALID_CONTENT: _cairo_status = 15;
pub const CAIRO_STATUS_INVALID_FORMAT: _cairo_status = 16;
pub const CAIRO_STATUS_INVALID_VISUAL: _cairo_status = 17;
pub const CAIRO_STATUS_FILE_NOT_FOUND: _cairo_status = 18;
pub const CAIRO_STATUS_INVALID_DASH: _cairo_status = 19;
pub const CAIRO_STATUS_INVALID_DSC_COMMENT: _cairo_status = 20;
pub const CAIRO_STATUS_INVALID_INDEX: _cairo_status = 21;
pub const CAIRO_STATUS_CLIP_NOT_REPRESENTABLE: _cairo_status = 22;
pub const CAIRO_STATUS_TEMP_FILE_ERROR: _cairo_status = 23;
pub const CAIRO_STATUS_INVALID_STRIDE: _cairo_status = 24;
pub const CAIRO_STATUS_FONT_TYPE_MISMATCH: _cairo_status = 25;
pub const CAIRO_STATUS_USER_FONT_IMMUTABLE: _cairo_status = 26;
pub const CAIRO_STATUS_USER_FONT_ERROR: _cairo_status = 27;
pub const CAIRO_STATUS_NEGATIVE_COUNT: _cairo_status = 28;
pub const CAIRO_STATUS_INVALID_CLUSTERS: _cairo_status = 29;
pub const CAIRO_STATUS_INVALID_SLANT: _cairo_status = 30;
pub const CAIRO_STATUS_INVALID_WEIGHT: _cairo_status = 31;
pub const CAIRO_STATUS_INVALID_SIZE: _cairo_status = 32;
pub const CAIRO_STATUS_USER_FONT_NOT_IMPLEMENTED: _cairo_status = 33;
pub const CAIRO_STATUS_DEVICE_TYPE_MISMATCH: _cairo_status = 34;
pub const CAIRO_STATUS_DEVICE_ERROR: _cairo_status = 35;
pub const CAIRO_STATUS_INVALID_MESH_CONSTRUCTION: _cairo_status = 36;
pub const CAIRO_STATUS_DEVICE_FINISHED: _cairo_status = 37;
pub const CAIRO_STATUS_JBIG2_GLOBAL_MISSING: _cairo_status = 38;
pub const CAIRO_STATUS_PNG_ERROR: _cairo_status = 39;
pub const CAIRO_STATUS_FREETYPE_ERROR: _cairo_status = 40;
pub const CAIRO_STATUS_WIN32_GDI_ERROR: _cairo_status = 41;
pub const CAIRO_STATUS_TAG_ERROR: _cairo_status = 42;
pub const CAIRO_STATUS_DWRITE_ERROR: _cairo_status = 43;
pub const CAIRO_STATUS_SVG_FONT_ERROR: _cairo_status = 44;
pub const CAIRO_STATUS_LAST_STATUS: _cairo_status = 45;
#[doc = " cairo_status_t:\n @CAIRO_STATUS_SUCCESS: no error has occurred (Since 1.0)\n @CAIRO_STATUS_NO_MEMORY: out of memory (Since 1.0)\n @CAIRO_STATUS_INVALID_RESTORE: cairo_restore() called without matching cairo_save() (Since 1.0)\n @CAIRO_STATUS_INVALID_POP_GROUP: no saved group to pop, i.e. cairo_pop_group() without matching cairo_push_group() (Since 1.0)\n @CAIRO_STATUS_NO_CURRENT_POINT: no current point defined (Since 1.0)\n @CAIRO_STATUS_INVALID_MATRIX: invalid matrix (not invertible) (Since 1.0)\n @CAIRO_STATUS_INVALID_STATUS: invalid value for an input #cairo_status_t (Since 1.0)\n @CAIRO_STATUS_NULL_POINTER: %NULL pointer (Since 1.0)\n @CAIRO_STATUS_INVALID_STRING: input string not valid UTF-8 (Since 1.0)\n @CAIRO_STATUS_INVALID_PATH_DATA: input path data not valid (Since 1.0)\n @CAIRO_STATUS_READ_ERROR: error while reading from input stream (Since 1.0)\n @CAIRO_STATUS_WRITE_ERROR: error while writing to output stream (Since 1.0)\n @CAIRO_STATUS_SURFACE_FINISHED: target surface has been finished (Since 1.0)\n @CAIRO_STATUS_SURFACE_TYPE_MISMATCH: the surface type is not appropriate for the operation (Since 1.0)\n @CAIRO_STATUS_PATTERN_TYPE_MISMATCH: the pattern type is not appropriate for the operation (Since 1.0)\n @CAIRO_STATUS_INVALID_CONTENT: invalid value for an input #cairo_content_t (Since 1.0)\n @CAIRO_STATUS_INVALID_FORMAT: invalid value for an input #cairo_format_t (Since 1.0)\n @CAIRO_STATUS_INVALID_VISUAL: invalid value for an input Visual* (Since 1.0)\n @CAIRO_STATUS_FILE_NOT_FOUND: file not found (Since 1.0)\n @CAIRO_STATUS_INVALID_DASH: invalid value for a dash setting (Since 1.0)\n @CAIRO_STATUS_INVALID_DSC_COMMENT: invalid value for a DSC comment (Since 1.2)\n @CAIRO_STATUS_INVALID_INDEX: invalid index passed to getter (Since 1.4)\n @CAIRO_STATUS_CLIP_NOT_REPRESENTABLE: clip region not representable in desired format (Since 1.4)\n @CAIRO_STATUS_TEMP_FILE_ERROR: error creating or writing to a temporary file (Since 1.6)\n @CAIRO_STATUS_INVALID_STRIDE: invalid value for stride (Since 1.6)\n @CAIRO_STATUS_FONT_TYPE_MISMATCH: the font type is not appropriate for the operation (Since 1.8)\n @CAIRO_STATUS_USER_FONT_IMMUTABLE: the user-font is immutable (Since 1.8)\n @CAIRO_STATUS_USER_FONT_ERROR: error occurred in a user-font callback function (Since 1.8)\n @CAIRO_STATUS_NEGATIVE_COUNT: negative number used where it is not allowed (Since 1.8)\n @CAIRO_STATUS_INVALID_CLUSTERS: input clusters do not represent the accompanying text and glyph array (Since 1.8)\n @CAIRO_STATUS_INVALID_SLANT: invalid value for an input #cairo_font_slant_t (Since 1.8)\n @CAIRO_STATUS_INVALID_WEIGHT: invalid value for an input #cairo_font_weight_t (Since 1.8)\n @CAIRO_STATUS_INVALID_SIZE: invalid value (typically too big) for the size of the input (surface, pattern, etc.) (Since 1.10)\n @CAIRO_STATUS_USER_FONT_NOT_IMPLEMENTED: user-font method not implemented (Since 1.10)\n @CAIRO_STATUS_DEVICE_TYPE_MISMATCH: the device type is not appropriate for the operation (Since 1.10)\n @CAIRO_STATUS_DEVICE_ERROR: an operation to the device caused an unspecified error (Since 1.10)\n @CAIRO_STATUS_INVALID_MESH_CONSTRUCTION: a mesh pattern\n   construction operation was used outside of a\n   cairo_mesh_pattern_begin_patch()/cairo_mesh_pattern_end_patch()\n   pair (Since 1.12)\n @CAIRO_STATUS_DEVICE_FINISHED: target device has been finished (Since 1.12)\n @CAIRO_STATUS_JBIG2_GLOBAL_MISSING: %CAIRO_MIME_TYPE_JBIG2_GLOBAL_ID has been used on at least one image\n   but no image provided %CAIRO_MIME_TYPE_JBIG2_GLOBAL (Since 1.14)\n @CAIRO_STATUS_PNG_ERROR: error occurred in libpng while reading from or writing to a PNG file (Since 1.16)\n @CAIRO_STATUS_FREETYPE_ERROR: error occurred in libfreetype (Since 1.16)\n @CAIRO_STATUS_WIN32_GDI_ERROR: error occurred in the Windows Graphics Device Interface (Since 1.16)\n @CAIRO_STATUS_TAG_ERROR: invalid tag name, attributes, or nesting (Since 1.16)\n @CAIRO_STATUS_DWRITE_ERROR: error occurred in the Windows Direct Write API (Since 1.18)\n @CAIRO_STATUS_SVG_FONT_ERROR: error occurred in OpenType-SVG font rendering (Since 1.18)\n @CAIRO_STATUS_LAST_STATUS: this is a special value indicating the number of\n   status values defined in this enumeration.  When using this value, note\n   that the version of cairo at run-time may have additional status values\n   defined than the value of this symbol at compile-time. (Since 1.10)\n\n #cairo_status_t is used to indicate errors that can occur when\n using Cairo. In some cases it is returned directly by functions.\n but when using #cairo_t, the last error, if any, is stored in\n the context and can be retrieved with cairo_status().\n\n New entries may be added in future versions.  Use cairo_status_to_string()\n to get a human-readable representation of an error message.\n\n Since: 1.0"]
pub type _cairo_status = u32;
#[doc = " cairo_status_t:\n @CAIRO_STATUS_SUCCESS: no error has occurred (Since 1.0)\n @CAIRO_STATUS_NO_MEMORY: out of memory (Since 1.0)\n @CAIRO_STATUS_INVALID_RESTORE: cairo_restore() called without matching cairo_save() (Since 1.0)\n @CAIRO_STATUS_INVALID_POP_GROUP: no saved group to pop, i.e. cairo_pop_group() without matching cairo_push_group() (Since 1.0)\n @CAIRO_STATUS_NO_CURRENT_POINT: no current point defined (Since 1.0)\n @CAIRO_STATUS_INVALID_MATRIX: invalid matrix (not invertible) (Since 1.0)\n @CAIRO_STATUS_INVALID_STATUS: invalid value for an input #cairo_status_t (Since 1.0)\n @CAIRO_STATUS_NULL_POINTER: %NULL pointer (Since 1.0)\n @CAIRO_STATUS_INVALID_STRING: input string not valid UTF-8 (Since 1.0)\n @CAIRO_STATUS_INVALID_PATH_DATA: input path data not valid (Since 1.0)\n @CAIRO_STATUS_READ_ERROR: error while reading from input stream (Since 1.0)\n @CAIRO_STATUS_WRITE_ERROR: error while writing to output stream (Since 1.0)\n @CAIRO_STATUS_SURFACE_FINISHED: target surface has been finished (Since 1.0)\n @CAIRO_STATUS_SURFACE_TYPE_MISMATCH: the surface type is not appropriate for the operation (Since 1.0)\n @CAIRO_STATUS_PATTERN_TYPE_MISMATCH: the pattern type is not appropriate for the operation (Since 1.0)\n @CAIRO_STATUS_INVALID_CONTENT: invalid value for an input #cairo_content_t (Since 1.0)\n @CAIRO_STATUS_INVALID_FORMAT: invalid value for an input #cairo_format_t (Since 1.0)\n @CAIRO_STATUS_INVALID_VISUAL: invalid value for an input Visual* (Since 1.0)\n @CAIRO_STATUS_FILE_NOT_FOUND: file not found (Since 1.0)\n @CAIRO_STATUS_INVALID_DASH: invalid value for a dash setting (Since 1.0)\n @CAIRO_STATUS_INVALID_DSC_COMMENT: invalid value for a DSC comment (Since 1.2)\n @CAIRO_STATUS_INVALID_INDEX: invalid index passed to getter (Since 1.4)\n @CAIRO_STATUS_CLIP_NOT_REPRESENTABLE: clip region not representable in desired format (Since 1.4)\n @CAIRO_STATUS_TEMP_FILE_ERROR: error creating or writing to a temporary file (Since 1.6)\n @CAIRO_STATUS_INVALID_STRIDE: invalid value for stride (Since 1.6)\n @CAIRO_STATUS_FONT_TYPE_MISMATCH: the font type is not appropriate for the operation (Since 1.8)\n @CAIRO_STATUS_USER_FONT_IMMUTABLE: the user-font is immutable (Since 1.8)\n @CAIRO_STATUS_USER_FONT_ERROR: error occurred in a user-font callback function (Since 1.8)\n @CAIRO_STATUS_NEGATIVE_COUNT: negative number used where it is not allowed (Since 1.8)\n @CAIRO_STATUS_INVALID_CLUSTERS: input clusters do not represent the accompanying text and glyph array (Since 1.8)\n @CAIRO_STATUS_INVALID_SLANT: invalid value for an input #cairo_font_slant_t (Since 1.8)\n @CAIRO_STATUS_INVALID_WEIGHT: invalid value for an input #cairo_font_weight_t (Since 1.8)\n @CAIRO_STATUS_INVALID_SIZE: invalid value (typically too big) for the size of the input (surface, pattern, etc.) (Since 1.10)\n @CAIRO_STATUS_USER_FONT_NOT_IMPLEMENTED: user-font method not implemented (Since 1.10)\n @CAIRO_STATUS_DEVICE_TYPE_MISMATCH: the device type is not appropriate for the operation (Since 1.10)\n @CAIRO_STATUS_DEVICE_ERROR: an operation to the device caused an unspecified error (Since 1.10)\n @CAIRO_STATUS_INVALID_MESH_CONSTRUCTION: a mesh pattern\n   construction operation was used outside of a\n   cairo_mesh_pattern_begin_patch()/cairo_mesh_pattern_end_patch()\n   pair (Since 1.12)\n @CAIRO_STATUS_DEVICE_FINISHED: target device has been finished (Since 1.12)\n @CAIRO_STATUS_JBIG2_GLOBAL_MISSING: %CAIRO_MIME_TYPE_JBIG2_GLOBAL_ID has been used on at least one image\n   but no image provided %CAIRO_MIME_TYPE_JBIG2_GLOBAL (Since 1.14)\n @CAIRO_STATUS_PNG_ERROR: error occurred in libpng while reading from or writing to a PNG file (Since 1.16)\n @CAIRO_STATUS_FREETYPE_ERROR: error occurred in libfreetype (Since 1.16)\n @CAIRO_STATUS_WIN32_GDI_ERROR: error occurred in the Windows Graphics Device Interface (Since 1.16)\n @CAIRO_STATUS_TAG_ERROR: invalid tag name, attributes, or nesting (Since 1.16)\n @CAIRO_STATUS_DWRITE_ERROR: error occurred in the Windows Direct Write API (Since 1.18)\n @CAIRO_STATUS_SVG_FONT_ERROR: error occurred in OpenType-SVG font rendering (Since 1.18)\n @CAIRO_STATUS_LAST_STATUS: this is a special value indicating the number of\n   status values defined in this enumeration.  When using this value, note\n   that the version of cairo at run-time may have additional status values\n   defined than the value of this symbol at compile-time. (Since 1.10)\n\n #cairo_status_t is used to indicate errors that can occur when\n using Cairo. In some cases it is returned directly by functions.\n but when using #cairo_t, the last error, if any, is stored in\n the context and can be retrieved with cairo_status().\n\n New entries may be added in future versions.  Use cairo_status_to_string()\n to get a human-readable representation of an error message.\n\n Since: 1.0"]
pub use self::_cairo_status as cairo_status_t;
pub const CAIRO_CONTENT_COLOR: _cairo_content = 4096;
pub const CAIRO_CONTENT_ALPHA: _cairo_content = 8192;
pub const CAIRO_CONTENT_COLOR_ALPHA: _cairo_content = 12288;
#[doc = " cairo_content_t:\n @CAIRO_CONTENT_COLOR: The surface will hold color content only. (Since 1.0)\n @CAIRO_CONTENT_ALPHA: The surface will hold alpha content only. (Since 1.0)\n @CAIRO_CONTENT_COLOR_ALPHA: The surface will hold color and alpha content. (Since 1.0)\n\n #cairo_content_t is used to describe the content that a surface will\n contain, whether color information, alpha information (translucence\n vs. opacity), or both.\n\n Note: The large values here are designed to keep #cairo_content_t\n values distinct from #cairo_format_t values so that the\n implementation can detect the error if users confuse the two types.\n\n Since: 1.0"]
pub type _cairo_content = u32;
#[doc = " cairo_content_t:\n @CAIRO_CONTENT_COLOR: The surface will hold color content only. (Since 1.0)\n @CAIRO_CONTENT_ALPHA: The surface will hold alpha content only. (Since 1.0)\n @CAIRO_CONTENT_COLOR_ALPHA: The surface will hold color and alpha content. (Since 1.0)\n\n #cairo_content_t is used to describe the content that a surface will\n contain, whether color information, alpha information (translucence\n vs. opacity), or both.\n\n Note: The large values here are designed to keep #cairo_content_t\n values distinct from #cairo_format_t values so that the\n implementation can detect the error if users confuse the two types.\n\n Since: 1.0"]
pub use self::_cairo_content as cairo_content_t;
pub const CAIRO_FORMAT_INVALID: _cairo_format = -1;
pub const CAIRO_FORMAT_ARGB32: _cairo_format = 0;
pub const CAIRO_FORMAT_RGB24: _cairo_format = 1;
pub const CAIRO_FORMAT_A8: _cairo_format = 2;
pub const CAIRO_FORMAT_A1: _cairo_format = 3;
pub const CAIRO_FORMAT_RGB16_565: _cairo_format = 4;
pub const CAIRO_FORMAT_RGB30: _cairo_format = 5;
pub const CAIRO_FORMAT_RGB96F: _cairo_format = 6;
pub const CAIRO_FORMAT_RGBA128F: _cairo_format = 7;
#[doc = " cairo_format_t:\n @CAIRO_FORMAT_INVALID: no such format exists or is supported.\n @CAIRO_FORMAT_ARGB32: each pixel is a 32-bit quantity, with\n   alpha in the upper 8 bits, then red, then green, then blue.\n   The 32-bit quantities are stored native-endian. Pre-multiplied\n   alpha is used. (That is, 50% transparent red is 0x80800000,\n   not 0x80ff0000.) (Since 1.0)\n @CAIRO_FORMAT_RGB24: each pixel is a 32-bit quantity, with\n   the upper 8 bits unused. Red, Green, and Blue are stored\n   in the remaining 24 bits in that order. (Since 1.0)\n @CAIRO_FORMAT_A8: each pixel is a 8-bit quantity holding\n   an alpha value. (Since 1.0)\n @CAIRO_FORMAT_A1: each pixel is a 1-bit quantity holding\n   an alpha value. Pixels are packed together into 32-bit\n   quantities. The ordering of the bits matches the\n   endianness of the platform. On a big-endian machine, the\n   first pixel is in the uppermost bit, on a little-endian\n   machine the first pixel is in the least-significant bit. (Since 1.0)\n @CAIRO_FORMAT_RGB16_565: each pixel is a 16-bit quantity\n   with red in the upper 5 bits, then green in the middle\n   6 bits, and blue in the lower 5 bits. (Since 1.2)\n @CAIRO_FORMAT_RGB30: like RGB24 but with 10bpc. (Since 1.12)\n @CAIRO_FORMAT_RGB96F: 3 floats, R, G, B. (Since 1.17.2)\n @CAIRO_FORMAT_RGBA128F: 4 floats, R, G, B, A. (Since 1.17.2)\n\n #cairo_format_t is used to identify the memory format of\n image data.\n\n New entries may be added in future versions.\n\n Since: 1.0"]
pub type _cairo_format = i32;
#[doc = " cairo_format_t:\n @CAIRO_FORMAT_INVALID: no such format exists or is supported.\n @CAIRO_FORMAT_ARGB32: each pixel is a 32-bit quantity, with\n   alpha in the upper 8 bits, then red, then green, then blue.\n   The 32-bit quantities are stored native-endian. Pre-multiplied\n   alpha is used. (That is, 50% transparent red is 0x80800000,\n   not 0x80ff0000.) (Since 1.0)\n @CAIRO_FORMAT_RGB24: each pixel is a 32-bit quantity, with\n   the upper 8 bits unused. Red, Green, and Blue are stored\n   in the remaining 24 bits in that order. (Since 1.0)\n @CAIRO_FORMAT_A8: each pixel is a 8-bit quantity holding\n   an alpha value. (Since 1.0)\n @CAIRO_FORMAT_A1: each pixel is a 1-bit quantity holding\n   an alpha value. Pixels are packed together into 32-bit\n   quantities. The ordering of the bits matches the\n   endianness of the platform. On a big-endian machine, the\n   first pixel is in the uppermost bit, on a little-endian\n   machine the first pixel is in the least-significant bit. (Since 1.0)\n @CAIRO_FORMAT_RGB16_565: each pixel is a 16-bit quantity\n   with red in the upper 5 bits, then green in the middle\n   6 bits, and blue in the lower 5 bits. (Since 1.2)\n @CAIRO_FORMAT_RGB30: like RGB24 but with 10bpc. (Since 1.12)\n @CAIRO_FORMAT_RGB96F: 3 floats, R, G, B. (Since 1.17.2)\n @CAIRO_FORMAT_RGBA128F: 4 floats, R, G, B, A. (Since 1.17.2)\n\n #cairo_format_t is used to identify the memory format of\n image data.\n\n New entries may be added in future versions.\n\n Since: 1.0"]
pub use self::_cairo_format as cairo_format_t;
pub const CAIRO_DITHER_NONE: _cairo_dither = 0;
pub const CAIRO_DITHER_DEFAULT: _cairo_dither = 1;
pub const CAIRO_DITHER_FAST: _cairo_dither = 2;
pub const CAIRO_DITHER_GOOD: _cairo_dither = 3;
pub const CAIRO_DITHER_BEST: _cairo_dither = 4;
#[doc = " cairo_dither_t:\n @CAIRO_DITHER_NONE: No dithering.\n @CAIRO_DITHER_DEFAULT: Default choice at cairo compile time. Currently NONE.\n @CAIRO_DITHER_FAST: Fastest dithering algorithm supported by the backend\n @CAIRO_DITHER_GOOD: An algorithm with smoother dithering than FAST\n @CAIRO_DITHER_BEST: Best algorithm available in the backend\n\n Dither is an intentionally applied form of noise used to randomize\n quantization error, preventing large-scale patterns such as color banding\n in images (e.g. for gradients). Ordered dithering applies a precomputed\n threshold matrix to spread the errors smoothly.\n\n  #cairo_dither_t is modeled on pixman dithering algorithm choice.\n As of Pixman 0.40, FAST corresponds to a 8x8 ordered bayer noise and GOOD\n and BEST use an ordered 64x64 precomputed blue noise.\n\n Since: 1.18"]
pub type _cairo_dither = u32;
#[doc = " cairo_dither_t:\n @CAIRO_DITHER_NONE: No dithering.\n @CAIRO_DITHER_DEFAULT: Default choice at cairo compile time. Currently NONE.\n @CAIRO_DITHER_FAST: Fastest dithering algorithm supported by the backend\n @CAIRO_DITHER_GOOD: An algorithm with smoother dithering than FAST\n @CAIRO_DITHER_BEST: Best algorithm available in the backend\n\n Dither is an intentionally applied form of noise used to randomize\n quantization error, preventing large-scale patterns such as color banding\n in images (e.g. for gradients). Ordered dithering applies a precomputed\n threshold matrix to spread the errors smoothly.\n\n  #cairo_dither_t is modeled on pixman dithering algorithm choice.\n As of Pixman 0.40, FAST corresponds to a 8x8 ordered bayer noise and GOOD\n and BEST use an ordered 64x64 precomputed blue noise.\n\n Since: 1.18"]
pub use self::_cairo_dither as cairo_dither_t;
#[doc = " cairo_write_func_t:\n @closure: the output closure\n @data: the buffer containing the data to write\n @length: the amount of data to write\n\n #cairo_write_func_t is the type of function which is called when a\n backend needs to write data to an output stream.  It is passed the\n closure which was specified by the user at the time the write\n function was registered, the data to write and the length of the\n data in bytes.  The write function should return\n %CAIRO_STATUS_SUCCESS if all the data was successfully written,\n %CAIRO_STATUS_WRITE_ERROR otherwise.\n\n Returns: the status code of the write operation\n\n Since: 1.0"]
pub type cairo_write_func_t = ::core::option::Option<
    unsafe extern "C" fn(
        closure: *mut ::core::ffi::c_void,
        data: *const ::core::ffi::c_uchar,
        length: ::core::ffi::c_uint,
    ) -> cairo_status_t,
>;
#[doc = " cairo_read_func_t:\n @closure: the input closure\n @data: the buffer into which to read the data\n @length: the amount of data to read\n\n #cairo_read_func_t is the type of function which is called when a\n backend needs to read data from an input stream.  It is passed the\n closure which was specified by the user at the time the read\n function was registered, the buffer to read the data into and the\n length of the data in bytes.  The read function should return\n %CAIRO_STATUS_SUCCESS if all the data was successfully read,\n %CAIRO_STATUS_READ_ERROR otherwise.\n\n Returns: the status code of the read operation\n\n Since: 1.0"]
pub type cairo_read_func_t = ::core::option::Option<
    unsafe extern "C" fn(
        closure: *mut ::core::ffi::c_void,
        data: *mut ::core::ffi::c_uchar,
        length: ::core::ffi::c_uint,
    ) -> cairo_status_t,
>;
#[doc = " cairo_rectangle_int_t:\n @x: X coordinate of the left side of the rectangle\n @y: Y coordinate of the top side of the rectangle\n @width: width of the rectangle\n @height: height of the rectangle\n\n A data structure for holding a rectangle with integer coordinates.\n\n Since: 1.10"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _cairo_rectangle_int {
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
}
#[doc = " cairo_rectangle_int_t:\n @x: X coordinate of the left side of the rectangle\n @y: Y coordinate of the top side of the rectangle\n @width: width of the rectangle\n @height: height of the rectangle\n\n A data structure for holding a rectangle with integer coordinates.\n\n Since: 1.10"]
pub type cairo_rectangle_int_t = _cairo_rectangle_int;
pub const CAIRO_OPERATOR_CLEAR: _cairo_operator = 0;
pub const CAIRO_OPERATOR_SOURCE: _cairo_operator = 1;
pub const CAIRO_OPERATOR_OVER: _cairo_operator = 2;
pub const CAIRO_OPERATOR_IN: _cairo_operator = 3;
pub const CAIRO_OPERATOR_OUT: _cairo_operator = 4;
pub const CAIRO_OPERATOR_ATOP: _cairo_operator = 5;
pub const CAIRO_OPERATOR_DEST: _cairo_operator = 6;
pub const CAIRO_OPERATOR_DEST_OVER: _cairo_operator = 7;
pub const CAIRO_OPERATOR_DEST_IN: _cairo_operator = 8;
pub const CAIRO_OPERATOR_DEST_OUT: _cairo_operator = 9;
pub const CAIRO_OPERATOR_DEST_ATOP: _cairo_operator = 10;
pub const CAIRO_OPERATOR_XOR: _cairo_operator = 11;
pub const CAIRO_OPERATOR_ADD: _cairo_operator = 12;
pub const CAIRO_OPERATOR_SATURATE: _cairo_operator = 13;
pub const CAIRO_OPERATOR_MULTIPLY: _cairo_operator = 14;
pub const CAIRO_OPERATOR_SCREEN: _cairo_operator = 15;
pub const CAIRO_OPERATOR_OVERLAY: _cairo_operator = 16;
pub const CAIRO_OPERATOR_DARKEN: _cairo_operator = 17;
pub const CAIRO_OPERATOR_LIGHTEN: _cairo_operator = 18;
pub const CAIRO_OPERATOR_COLOR_DODGE: _cairo_operator = 19;
pub const CAIRO_OPERATOR_COLOR_BURN: _cairo_operator = 20;
pub const CAIRO_OPERATOR_HARD_LIGHT: _cairo_operator = 21;
pub const CAIRO_OPERATOR_SOFT_LIGHT: _cairo_operator = 22;
pub const CAIRO_OPERATOR_DIFFERENCE: _cairo_operator = 23;
pub const CAIRO_OPERATOR_EXCLUSION: _cairo_operator = 24;
pub const CAIRO_OPERATOR_HSL_HUE: _cairo_operator = 25;
pub const CAIRO_OPERATOR_HSL_SATURATION: _cairo_operator = 26;
pub const CAIRO_OPERATOR_HSL_COLOR: _cairo_operator = 27;
pub const CAIRO_OPERATOR_HSL_LUMINOSITY: _cairo_operator = 28;
#[doc = " cairo_operator_t:\n @CAIRO_OPERATOR_CLEAR: clear destination layer (bounded) (Since 1.0)\n @CAIRO_OPERATOR_SOURCE: replace destination layer (bounded) (Since 1.0)\n @CAIRO_OPERATOR_OVER: draw source layer on top of destination layer\n (bounded) (Since 1.0)\n @CAIRO_OPERATOR_IN: draw source where there was destination content\n (unbounded) (Since 1.0)\n @CAIRO_OPERATOR_OUT: draw source where there was no destination\n content (unbounded) (Since 1.0)\n @CAIRO_OPERATOR_ATOP: draw source on top of destination content and\n only there (Since 1.0)\n @CAIRO_OPERATOR_DEST: ignore the source (Since 1.0)\n @CAIRO_OPERATOR_DEST_OVER: draw destination on top of source (Since 1.0)\n @CAIRO_OPERATOR_DEST_IN: leave destination only where there was\n source content (unbounded) (Since 1.0)\n @CAIRO_OPERATOR_DEST_OUT: leave destination only where there was no\n source content (Since 1.0)\n @CAIRO_OPERATOR_DEST_ATOP: leave destination on top of source content\n and only there (unbounded) (Since 1.0)\n @CAIRO_OPERATOR_XOR: source and destination are shown where there is only\n one of them (Since 1.0)\n @CAIRO_OPERATOR_ADD: source and destination layers are accumulated (Since 1.0)\n @CAIRO_OPERATOR_SATURATE: like over, but assuming source and dest are\n disjoint geometries (Since 1.0)\n @CAIRO_OPERATOR_MULTIPLY: source and destination layers are multiplied.\n This causes the result to be at least as dark as the darker inputs. (Since 1.10)\n @CAIRO_OPERATOR_SCREEN: source and destination are complemented and\n multiplied. This causes the result to be at least as light as the lighter\n inputs. (Since 1.10)\n @CAIRO_OPERATOR_OVERLAY: multiplies or screens, depending on the\n lightness of the destination color. (Since 1.10)\n @CAIRO_OPERATOR_DARKEN: replaces the destination with the source if it\n is darker, otherwise keeps the source. (Since 1.10)\n @CAIRO_OPERATOR_LIGHTEN: replaces the destination with the source if it\n is lighter, otherwise keeps the source. (Since 1.10)\n @CAIRO_OPERATOR_COLOR_DODGE: brightens the destination color to reflect\n the source color. (Since 1.10)\n @CAIRO_OPERATOR_COLOR_BURN: darkens the destination color to reflect\n the source color. (Since 1.10)\n @CAIRO_OPERATOR_HARD_LIGHT: Multiplies or screens, dependent on source\n color. (Since 1.10)\n @CAIRO_OPERATOR_SOFT_LIGHT: Darkens or lightens, dependent on source\n color. (Since 1.10)\n @CAIRO_OPERATOR_DIFFERENCE: Takes the difference of the source and\n destination color. (Since 1.10)\n @CAIRO_OPERATOR_EXCLUSION: Produces an effect similar to difference, but\n with lower contrast. (Since 1.10)\n @CAIRO_OPERATOR_HSL_HUE: Creates a color with the hue of the source\n and the saturation and luminosity of the target. (Since 1.10)\n @CAIRO_OPERATOR_HSL_SATURATION: Creates a color with the saturation\n of the source and the hue and luminosity of the target. Painting with\n this mode onto a gray area produces no change. (Since 1.10)\n @CAIRO_OPERATOR_HSL_COLOR: Creates a color with the hue and saturation\n of the source and the luminosity of the target. This preserves the gray\n levels of the target and is useful for coloring monochrome images or\n tinting color images. (Since 1.10)\n @CAIRO_OPERATOR_HSL_LUMINOSITY: Creates a color with the luminosity of\n the source and the hue and saturation of the target. This produces an\n inverse effect to @CAIRO_OPERATOR_HSL_COLOR. (Since 1.10)\n\n #cairo_operator_t is used to set the compositing operator for all cairo\n drawing operations.\n\n The default operator is %CAIRO_OPERATOR_OVER.\n\n The operators marked as <firstterm>unbounded</firstterm> modify their\n destination even outside of the mask layer (that is, their effect is not\n bound by the mask layer).  However, their effect can still be limited by\n way of clipping.\n\n To keep things simple, the operator descriptions here\n document the behavior for when both source and destination are either fully\n transparent or fully opaque.  The actual implementation works for\n translucent layers too.\n For a more detailed explanation of the effects of each operator, including\n the mathematical definitions, see\n <ulink url=\"https://cairographics.org/operators/\">https://cairographics.org/operators/</ulink>.\n\n Since: 1.0"]
pub type _cairo_operator = u32;
#[doc = " cairo_operator_t:\n @CAIRO_OPERATOR_CLEAR: clear destination layer (bounded) (Since 1.0)\n @CAIRO_OPERATOR_SOURCE: replace destination layer (bounded) (Since 1.0)\n @CAIRO_OPERATOR_OVER: draw source layer on top of destination layer\n (bounded) (Since 1.0)\n @CAIRO_OPERATOR_IN: draw source where there was destination content\n (unbounded) (Since 1.0)\n @CAIRO_OPERATOR_OUT: draw source where there was no destination\n content (unbounded) (Since 1.0)\n @CAIRO_OPERATOR_ATOP: draw source on top of destination content and\n only there (Since 1.0)\n @CAIRO_OPERATOR_DEST: ignore the source (Since 1.0)\n @CAIRO_OPERATOR_DEST_OVER: draw destination on top of source (Since 1.0)\n @CAIRO_OPERATOR_DEST_IN: leave destination only where there was\n source content (unbounded) (Since 1.0)\n @CAIRO_OPERATOR_DEST_OUT: leave destination only where there was no\n source content (Since 1.0)\n @CAIRO_OPERATOR_DEST_ATOP: leave destination on top of source content\n and only there (unbounded) (Since 1.0)\n @CAIRO_OPERATOR_XOR: source and destination are shown where there is only\n one of them (Since 1.0)\n @CAIRO_OPERATOR_ADD: source and destination layers are accumulated (Since 1.0)\n @CAIRO_OPERATOR_SATURATE: like over, but assuming source and dest are\n disjoint geometries (Since 1.0)\n @CAIRO_OPERATOR_MULTIPLY: source and destination layers are multiplied.\n This causes the result to be at least as dark as the darker inputs. (Since 1.10)\n @CAIRO_OPERATOR_SCREEN: source and destination are complemented and\n multiplied. This causes the result to be at least as light as the lighter\n inputs. (Since 1.10)\n @CAIRO_OPERATOR_OVERLAY: multiplies or screens, depending on the\n lightness of the destination color. (Since 1.10)\n @CAIRO_OPERATOR_DARKEN: replaces the destination with the source if it\n is darker, otherwise keeps the source. (Since 1.10)\n @CAIRO_OPERATOR_LIGHTEN: replaces the destination with the source if it\n is lighter, otherwise keeps the source. (Since 1.10)\n @CAIRO_OPERATOR_COLOR_DODGE: brightens the destination color to reflect\n the source color. (Since 1.10)\n @CAIRO_OPERATOR_COLOR_BURN: darkens the destination color to reflect\n the source color. (Since 1.10)\n @CAIRO_OPERATOR_HARD_LIGHT: Multiplies or screens, dependent on source\n color. (Since 1.10)\n @CAIRO_OPERATOR_SOFT_LIGHT: Darkens or lightens, dependent on source\n color. (Since 1.10)\n @CAIRO_OPERATOR_DIFFERENCE: Takes the difference of the source and\n destination color. (Since 1.10)\n @CAIRO_OPERATOR_EXCLUSION: Produces an effect similar to difference, but\n with lower contrast. (Since 1.10)\n @CAIRO_OPERATOR_HSL_HUE: Creates a color with the hue of the source\n and the saturation and luminosity of the target. (Since 1.10)\n @CAIRO_OPERATOR_HSL_SATURATION: Creates a color with the saturation\n of the source and the hue and luminosity of the target. Painting with\n this mode onto a gray area produces no change. (Since 1.10)\n @CAIRO_OPERATOR_HSL_COLOR: Creates a color with the hue and saturation\n of the source and the luminosity of the target. This preserves the gray\n levels of the target and is useful for coloring monochrome images or\n tinting color images. (Since 1.10)\n @CAIRO_OPERATOR_HSL_LUMINOSITY: Creates a color with the luminosity of\n the source and the hue and saturation of the target. This produces an\n inverse effect to @CAIRO_OPERATOR_HSL_COLOR. (Since 1.10)\n\n #cairo_operator_t is used to set the compositing operator for all cairo\n drawing operations.\n\n The default operator is %CAIRO_OPERATOR_OVER.\n\n The operators marked as <firstterm>unbounded</firstterm> modify their\n destination even outside of the mask layer (that is, their effect is not\n bound by the mask layer).  However, their effect can still be limited by\n way of clipping.\n\n To keep things simple, the operator descriptions here\n document the behavior for when both source and destination are either fully\n transparent or fully opaque.  The actual implementation works for\n translucent layers too.\n For a more detailed explanation of the effects of each operator, including\n the mathematical definitions, see\n <ulink url=\"https://cairographics.org/operators/\">https://cairographics.org/operators/</ulink>.\n\n Since: 1.0"]
pub use self::_cairo_operator as cairo_operator_t;
pub const CAIRO_ANTIALIAS_DEFAULT: _cairo_antialias = 0;
pub const CAIRO_ANTIALIAS_NONE: _cairo_antialias = 1;
pub const CAIRO_ANTIALIAS_GRAY: _cairo_antialias = 2;
pub const CAIRO_ANTIALIAS_SUBPIXEL: _cairo_antialias = 3;
pub const CAIRO_ANTIALIAS_FAST: _cairo_antialias = 4;
pub const CAIRO_ANTIALIAS_GOOD: _cairo_antialias = 5;
pub const CAIRO_ANTIALIAS_BEST: _cairo_antialias = 6;
#[doc = " cairo_antialias_t:\n @CAIRO_ANTIALIAS_DEFAULT: Use the default antialiasing for\n   the subsystem and target device, since 1.0\n @CAIRO_ANTIALIAS_NONE: Use a bilevel alpha mask, since 1.0\n @CAIRO_ANTIALIAS_GRAY: Perform single-color antialiasing (using\n  shades of gray for black text on a white background, for example), since 1.0\n @CAIRO_ANTIALIAS_SUBPIXEL: Perform antialiasing by taking\n  advantage of the order of subpixel elements on devices\n  such as LCD panels, since 1.0\n @CAIRO_ANTIALIAS_FAST: Hint that the backend should perform some\n antialiasing but prefer speed over quality, since 1.12\n @CAIRO_ANTIALIAS_GOOD: The backend should balance quality against\n performance, since 1.12\n @CAIRO_ANTIALIAS_BEST: Hint that the backend should render at the highest\n quality, sacrificing speed if necessary, since 1.12\n\n Specifies the type of antialiasing to do when rendering text or shapes.\n\n As it is not necessarily clear from the above what advantages a particular\n antialias method provides, since 1.12, there is also a set of hints:\n @CAIRO_ANTIALIAS_FAST: Allow the backend to degrade raster quality for speed\n @CAIRO_ANTIALIAS_GOOD: A balance between speed and quality\n @CAIRO_ANTIALIAS_BEST: A high-fidelity, but potentially slow, raster mode\n\n These make no guarantee on how the backend will perform its rasterisation\n (if it even rasterises!), nor that they have any differing effect other\n than to enable some form of antialiasing. In the case of glyph rendering,\n @CAIRO_ANTIALIAS_FAST and @CAIRO_ANTIALIAS_GOOD will be mapped to\n @CAIRO_ANTIALIAS_GRAY, with @CAIRO_ANTALIAS_BEST being equivalent to\n @CAIRO_ANTIALIAS_SUBPIXEL.\n\n The interpretation of @CAIRO_ANTIALIAS_DEFAULT is left entirely up to\n the backend, typically this will be similar to @CAIRO_ANTIALIAS_GOOD.\n\n Since: 1.0"]
pub type _cairo_antialias = u32;
#[doc = " cairo_antialias_t:\n @CAIRO_ANTIALIAS_DEFAULT: Use the default antialiasing for\n   the subsystem and target device, since 1.0\n @CAIRO_ANTIALIAS_NONE: Use a bilevel alpha mask, since 1.0\n @CAIRO_ANTIALIAS_GRAY: Perform single-color antialiasing (using\n  shades of gray for black text on a white background, for example), since 1.0\n @CAIRO_ANTIALIAS_SUBPIXEL: Perform antialiasing by taking\n  advantage of the order of subpixel elements on devices\n  such as LCD panels, since 1.0\n @CAIRO_ANTIALIAS_FAST: Hint that the backend should perform some\n antialiasing but prefer speed over quality, since 1.12\n @CAIRO_ANTIALIAS_GOOD: The backend should balance quality against\n performance, since 1.12\n @CAIRO_ANTIALIAS_BEST: Hint that the backend should render at the highest\n quality, sacrificing speed if necessary, since 1.12\n\n Specifies the type of antialiasing to do when rendering text or shapes.\n\n As it is not necessarily clear from the above what advantages a particular\n antialias method provides, since 1.12, there is also a set of hints:\n @CAIRO_ANTIALIAS_FAST: Allow the backend to degrade raster quality for speed\n @CAIRO_ANTIALIAS_GOOD: A balance between speed and quality\n @CAIRO_ANTIALIAS_BEST: A high-fidelity, but potentially slow, raster mode\n\n These make no guarantee on how the backend will perform its rasterisation\n (if it even rasterises!), nor that they have any differing effect other\n than to enable some form of antialiasing. In the case of glyph rendering,\n @CAIRO_ANTIALIAS_FAST and @CAIRO_ANTIALIAS_GOOD will be mapped to\n @CAIRO_ANTIALIAS_GRAY, with @CAIRO_ANTALIAS_BEST being equivalent to\n @CAIRO_ANTIALIAS_SUBPIXEL.\n\n The interpretation of @CAIRO_ANTIALIAS_DEFAULT is left entirely up to\n the backend, typically this will be similar to @CAIRO_ANTIALIAS_GOOD.\n\n Since: 1.0"]
pub use self::_cairo_antialias as cairo_antialias_t;
pub const CAIRO_FILL_RULE_WINDING: _cairo_fill_rule = 0;
pub const CAIRO_FILL_RULE_EVEN_ODD: _cairo_fill_rule = 1;
#[doc = " cairo_fill_rule_t:\n @CAIRO_FILL_RULE_WINDING: If the path crosses the ray from\n left-to-right, counts +1. If the path crosses the ray\n from right to left, counts -1. (Left and right are determined\n from the perspective of looking along the ray from the starting\n point.) If the total count is non-zero, the point will be filled. (Since 1.0)\n @CAIRO_FILL_RULE_EVEN_ODD: Counts the total number of\n intersections, without regard to the orientation of the contour. If\n the total number of intersections is odd, the point will be\n filled. (Since 1.0)\n\n #cairo_fill_rule_t is used to select how paths are filled. For both\n fill rules, whether or not a point is included in the fill is\n determined by taking a ray from that point to infinity and looking\n at intersections with the path. The ray can be in any direction,\n as long as it doesn't pass through the end point of a segment\n or have a tricky intersection such as intersecting tangent to the path.\n (Note that filling is not actually implemented in this way. This\n is just a description of the rule that is applied.)\n\n The default fill rule is %CAIRO_FILL_RULE_WINDING.\n\n New entries may be added in future versions.\n\n Since: 1.0"]
pub type _cairo_fill_rule = u32;
#[doc = " cairo_fill_rule_t:\n @CAIRO_FILL_RULE_WINDING: If the path crosses the ray from\n left-to-right, counts +1. If the path crosses the ray\n from right to left, counts -1. (Left and right are determined\n from the perspective of looking along the ray from the starting\n point.) If the total count is non-zero, the point will be filled. (Since 1.0)\n @CAIRO_FILL_RULE_EVEN_ODD: Counts the total number of\n intersections, without regard to the orientation of the contour. If\n the total number of intersections is odd, the point will be\n filled. (Since 1.0)\n\n #cairo_fill_rule_t is used to select how paths are filled. For both\n fill rules, whether or not a point is included in the fill is\n determined by taking a ray from that point to infinity and looking\n at intersections with the path. The ray can be in any direction,\n as long as it doesn't pass through the end point of a segment\n or have a tricky intersection such as intersecting tangent to the path.\n (Note that filling is not actually implemented in this way. This\n is just a description of the rule that is applied.)\n\n The default fill rule is %CAIRO_FILL_RULE_WINDING.\n\n New entries may be added in future versions.\n\n Since: 1.0"]
pub use self::_cairo_fill_rule as cairo_fill_rule_t;
pub const CAIRO_LINE_CAP_BUTT: _cairo_line_cap = 0;
pub const CAIRO_LINE_CAP_ROUND: _cairo_line_cap = 1;
pub const CAIRO_LINE_CAP_SQUARE: _cairo_line_cap = 2;
#[doc = " cairo_line_cap_t:\n @CAIRO_LINE_CAP_BUTT: start(stop) the line exactly at the start(end) point (Since 1.0)\n @CAIRO_LINE_CAP_ROUND: use a round ending, the center of the circle is the end point (Since 1.0)\n @CAIRO_LINE_CAP_SQUARE: use squared ending, the center of the square is the end point (Since 1.0)\n\n Specifies how to render the endpoints of the path when stroking.\n\n The default line cap style is %CAIRO_LINE_CAP_BUTT.\n\n Since: 1.0"]
pub type _cairo_line_cap = u32;
#[doc = " cairo_line_cap_t:\n @CAIRO_LINE_CAP_BUTT: start(stop) the line exactly at the start(end) point (Since 1.0)\n @CAIRO_LINE_CAP_ROUND: use a round ending, the center of the circle is the end point (Since 1.0)\n @CAIRO_LINE_CAP_SQUARE: use squared ending, the center of the square is the end point (Since 1.0)\n\n Specifies how to render the endpoints of the path when stroking.\n\n The default line cap style is %CAIRO_LINE_CAP_BUTT.\n\n Since: 1.0"]
pub use self::_cairo_line_cap as cairo_line_cap_t;
pub const CAIRO_LINE_JOIN_MITER: _cairo_line_join = 0;
pub const CAIRO_LINE_JOIN_ROUND: _cairo_line_join = 1;
pub const CAIRO_LINE_JOIN_BEVEL: _cairo_line_join = 2;
#[doc = " cairo_line_join_t:\n @CAIRO_LINE_JOIN_MITER: use a sharp (angled) corner, see\n cairo_set_miter_limit() (Since 1.0)\n @CAIRO_LINE_JOIN_ROUND: use a rounded join, the center of the circle is the\n joint point (Since 1.0)\n @CAIRO_LINE_JOIN_BEVEL: use a cut-off join, the join is cut off at half\n the line width from the joint point (Since 1.0)\n\n Specifies how to render the junction of two lines when stroking.\n\n The default line join style is %CAIRO_LINE_JOIN_MITER.\n\n Since: 1.0"]
pub type _cairo_line_join = u32;
#[doc = " cairo_line_join_t:\n @CAIRO_LINE_JOIN_MITER: use a sharp (angled) corner, see\n cairo_set_miter_limit() (Since 1.0)\n @CAIRO_LINE_JOIN_ROUND: use a rounded join, the center of the circle is the\n joint point (Since 1.0)\n @CAIRO_LINE_JOIN_BEVEL: use a cut-off join, the join is cut off at half\n the line width from the joint point (Since 1.0)\n\n Specifies how to render the junction of two lines when stroking.\n\n The default line join style is %CAIRO_LINE_JOIN_MITER.\n\n Since: 1.0"]
pub use self::_cairo_line_join as cairo_line_join_t;
#[doc = " cairo_rectangle_t:\n @x: X coordinate of the left side of the rectangle\n @y: Y coordinate of the top side of the rectangle\n @width: width of the rectangle\n @height: height of the rectangle\n\n A data structure for holding a rectangle.\n\n Since: 1.4"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _cairo_rectangle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}
#[doc = " cairo_rectangle_t:\n @x: X coordinate of the left side of the rectangle\n @y: Y coordinate of the top side of the rectangle\n @width: width of the rectangle\n @height: height of the rectangle\n\n A data structure for holding a rectangle.\n\n Since: 1.4"]
pub type cairo_rectangle_t = _cairo_rectangle;
#[doc = " cairo_rectangle_list_t:\n @status: Error status of the rectangle list\n @rectangles: Array containing the rectangles\n @num_rectangles: Number of rectangles in this list\n\n A data structure for holding a dynamically allocated\n array of rectangles.\n\n Since: 1.4"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _cairo_rectangle_list {
    pub status: cairo_status_t,
    pub rectangles: *mut cairo_rectangle_t,
    pub num_rectangles: ::core::ffi::c_int,
}
#[doc = " cairo_rectangle_list_t:\n @status: Error status of the rectangle list\n @rectangles: Array containing the rectangles\n @num_rectangles: Number of rectangles in this list\n\n A data structure for holding a dynamically allocated\n array of rectangles.\n\n Since: 1.4"]
pub type cairo_rectangle_list_t = _cairo_rectangle_list;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _cairo_scaled_font {
    _unused: [u8; 0],
}
#[doc = " cairo_scaled_font_t:\n\n A #cairo_scaled_font_t is a font scaled to a particular size and device\n resolution. A #cairo_scaled_font_t is most useful for low-level font\n usage where a library or application wants to cache a reference\n to a scaled font to speed up the computation of metrics.\n\n There are various types of scaled fonts, depending on the\n <firstterm>font backend</firstterm> they use. The type of a\n scaled font can be queried using cairo_scaled_font_get_type().\n\n Memory management of #cairo_scaled_font_t is done with\n cairo_scaled_font_reference() and cairo_scaled_font_destroy().\n\n Since: 1.0"]
pub type cairo_scaled_font_t = _cairo_scaled_font;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _cairo_font_face {
    _unused: [u8; 0],
}
#[doc = " cairo_font_face_t:\n\n A #cairo_font_face_t specifies all aspects of a font other\n than the size or font matrix (a font matrix is used to distort\n a font by shearing it or scaling it unequally in the two\n directions) . A font face can be set on a #cairo_t by using\n cairo_set_font_face(); the size and font matrix are set with\n cairo_set_font_size() and cairo_set_font_matrix().\n\n There are various types of font faces, depending on the\n <firstterm>font backend</firstterm> they use. The type of a\n font face can be queried using cairo_font_face_get_type().\n\n Memory management of #cairo_font_face_t is done with\n cairo_font_face_reference() and cairo_font_face_destroy().\n\n Since: 1.0"]
pub type cairo_font_face_t = _cairo_font_face;
#[doc = " cairo_glyph_t:\n @index: glyph index in the font. The exact interpretation of the\n      glyph index depends on the font technology being used.\n @x: the offset in the X direction between the origin used for\n     drawing or measuring the string and the origin of this glyph.\n @y: the offset in the Y direction between the origin used for\n     drawing or measuring the string and the origin of this glyph.\n\n The #cairo_glyph_t structure holds information about a single glyph\n when drawing or measuring text. A font is (in simple terms) a\n collection of shapes used to draw text. A glyph is one of these\n shapes. There can be multiple glyphs for a single character\n (alternates to be used in different contexts, for example), or a\n glyph can be a <firstterm>ligature</firstterm> of multiple\n characters. Cairo doesn't expose any way of converting input text\n into glyphs, so in order to use the Cairo interfaces that take\n arrays of glyphs, you must directly access the appropriate\n underlying font system.\n\n Note that the offsets given by @x and @y are not cumulative. When\n drawing or measuring text, each glyph is individually positioned\n with respect to the overall origin\n\n Since: 1.0"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cairo_glyph_t {
    pub index: ::core::ffi::c_ulong,
    pub x: f64,
    pub y: f64,
}
#[doc = " cairo_text_cluster_t:\n @num_bytes: the number of bytes of UTF-8 text covered by cluster\n @num_glyphs: the number of glyphs covered by cluster\n\n The #cairo_text_cluster_t structure holds information about a single\n <firstterm>text cluster</firstterm>.  A text cluster is a minimal\n mapping of some glyphs corresponding to some UTF-8 text.\n\n For a cluster to be valid, both @num_bytes and @num_glyphs should\n be non-negative, and at least one should be non-zero.\n Note that clusters with zero glyphs are not as well supported as\n normal clusters.  For example, PDF rendering applications typically\n ignore those clusters when PDF text is being selected.\n\n See cairo_show_text_glyphs() for how clusters are used in advanced\n text operations.\n\n Since: 1.8"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cairo_text_cluster_t {
    pub num_bytes: ::core::ffi::c_int,
    pub num_glyphs: ::core::ffi::c_int,
}
pub const CAIRO_TEXT_CLUSTER_FLAG_BACKWARD: _cairo_text_cluster_flags = 1;
#[doc = " cairo_text_cluster_flags_t:\n @CAIRO_TEXT_CLUSTER_FLAG_BACKWARD: The clusters in the cluster array\n map to glyphs in the glyph array from end to start. (Since 1.8)\n\n Specifies properties of a text cluster mapping.\n\n Since: 1.8"]
pub type _cairo_text_cluster_flags = u32;
#[doc = " cairo_text_cluster_flags_t:\n @CAIRO_TEXT_CLUSTER_FLAG_BACKWARD: The clusters in the cluster array\n map to glyphs in the glyph array from end to start. (Since 1.8)\n\n Specifies properties of a text cluster mapping.\n\n Since: 1.8"]
pub use self::_cairo_text_cluster_flags as cairo_text_cluster_flags_t;
#[doc = " cairo_text_extents_t:\n @x_bearing: the horizontal distance from the origin to the\n   leftmost part of the glyphs as drawn. Positive if the\n   glyphs lie entirely to the right of the origin.\n @y_bearing: the vertical distance from the origin to the\n   topmost part of the glyphs as drawn. Positive only if the\n   glyphs lie completely below the origin; will usually be\n   negative.\n @width: width of the glyphs as drawn\n @height: height of the glyphs as drawn\n @x_advance:distance to advance in the X direction\n    after drawing these glyphs\n @y_advance: distance to advance in the Y direction\n   after drawing these glyphs. Will typically be zero except\n   for vertical text layout as found in East-Asian languages.\n\n The #cairo_text_extents_t structure stores the extents of a single\n glyph or a string of glyphs in user-space coordinates. Because text\n extents are in user-space coordinates, they are mostly, but not\n entirely, independent of the current transformation matrix. If you call\n <literal>cairo_scale(cr, 2.0, 2.0)</literal>, text will\n be drawn twice as big, but the reported text extents will not be\n doubled. They will change slightly due to hinting (so you can't\n assume that metrics are independent of the transformation matrix),\n but otherwise will remain unchanged.\n\n Since: 1.0"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cairo_text_extents_t {
    pub x_bearing: f64,
    pub y_bearing: f64,
    pub width: f64,
    pub height: f64,
    pub x_advance: f64,
    pub y_advance: f64,
}
#[doc = " cairo_font_extents_t:\n @ascent: the distance that the font extends above the baseline.\n          Note that this is not always exactly equal to the maximum\n          of the extents of all the glyphs in the font, but rather\n          is picked to express the font designer's intent as to\n          how the font should align with elements above it.\n @descent: the distance that the font extends below the baseline.\n           This value is positive for typical fonts that include\n           portions below the baseline. Note that this is not always\n           exactly equal to the maximum of the extents of all the\n           glyphs in the font, but rather is picked to express the\n           font designer's intent as to how the font should\n           align with elements below it.\n @height: the recommended vertical distance between baselines when\n          setting consecutive lines of text with the font. This\n          is greater than @ascent+@descent by a\n          quantity known as the <firstterm>line spacing</firstterm>\n          or <firstterm>external leading</firstterm>. When space\n          is at a premium, most fonts can be set with only\n          a distance of @ascent+@descent between lines.\n @max_x_advance: the maximum distance in the X direction that\n         the origin is advanced for any glyph in the font.\n @max_y_advance: the maximum distance in the Y direction that\n         the origin is advanced for any glyph in the font.\n         This will be zero for normal fonts used for horizontal\n         writing. (The scripts of East Asia are sometimes written\n         vertically.)\n\n The #cairo_font_extents_t structure stores metric information for\n a font. Values are given in the current user-space coordinate\n system.\n\n Because font metrics are in user-space coordinates, they are\n mostly, but not entirely, independent of the current transformation\n matrix. If you call <literal>cairo_scale(cr, 2.0, 2.0)</literal>,\n text will be drawn twice as big, but the reported text extents will\n not be doubled. They will change slightly due to hinting (so you\n can't assume that metrics are independent of the transformation\n matrix), but otherwise will remain unchanged.\n\n Since: 1.0"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cairo_font_extents_t {
    pub ascent: f64,
    pub descent: f64,
    pub height: f64,
    pub max_x_advance: f64,
    pub max_y_advance: f64,
}
pub const CAIRO_FONT_SLANT_NORMAL: _cairo_font_slant = 0;
pub const CAIRO_FONT_SLANT_ITALIC: _cairo_font_slant = 1;
pub const CAIRO_FONT_SLANT_OBLIQUE: _cairo_font_slant = 2;
#[doc = " cairo_font_slant_t:\n @CAIRO_FONT_SLANT_NORMAL: Upright font style, since 1.0\n @CAIRO_FONT_SLANT_ITALIC: Italic font style, since 1.0\n @CAIRO_FONT_SLANT_OBLIQUE: Oblique font style, since 1.0\n\n Specifies variants of a font face based on their slant.\n\n Since: 1.0"]
pub type _cairo_font_slant = u32;
#[doc = " cairo_font_slant_t:\n @CAIRO_FONT_SLANT_NORMAL: Upright font style, since 1.0\n @CAIRO_FONT_SLANT_ITALIC: Italic font style, since 1.0\n @CAIRO_FONT_SLANT_OBLIQUE: Oblique font style, since 1.0\n\n Specifies variants of a font face based on their slant.\n\n Since: 1.0"]
pub use self::_cairo_font_slant as cairo_font_slant_t;
pub const CAIRO_FONT_WEIGHT_NORMAL: _cairo_font_weight = 0;
pub const CAIRO_FONT_WEIGHT_BOLD: _cairo_font_weight = 1;
#[doc = " cairo_font_weight_t:\n @CAIRO_FONT_WEIGHT_NORMAL: Normal font weight, since 1.0\n @CAIRO_FONT_WEIGHT_BOLD: Bold font weight, since 1.0\n\n Specifies variants of a font face based on their weight.\n\n Since: 1.0"]
pub type _cairo_font_weight = u32;
#[doc = " cairo_font_weight_t:\n @CAIRO_FONT_WEIGHT_NORMAL: Normal font weight, since 1.0\n @CAIRO_FONT_WEIGHT_BOLD: Bold font weight, since 1.0\n\n Specifies variants of a font face based on their weight.\n\n Since: 1.0"]
pub use self::_cairo_font_weight as cairo_font_weight_t;
pub const CAIRO_SUBPIXEL_ORDER_DEFAULT: _cairo_subpixel_order = 0;
pub const CAIRO_SUBPIXEL_ORDER_RGB: _cairo_subpixel_order = 1;
pub const CAIRO_SUBPIXEL_ORDER_BGR: _cairo_subpixel_order = 2;
pub const CAIRO_SUBPIXEL_ORDER_VRGB: _cairo_subpixel_order = 3;
pub const CAIRO_SUBPIXEL_ORDER_VBGR: _cairo_subpixel_order = 4;
#[doc = " cairo_subpixel_order_t:\n @CAIRO_SUBPIXEL_ORDER_DEFAULT: Use the default subpixel order for\n   for the target device, since 1.0\n @CAIRO_SUBPIXEL_ORDER_RGB: Subpixel elements are arranged horizontally\n   with red at the left, since 1.0\n @CAIRO_SUBPIXEL_ORDER_BGR:  Subpixel elements are arranged horizontally\n   with blue at the left, since 1.0\n @CAIRO_SUBPIXEL_ORDER_VRGB: Subpixel elements are arranged vertically\n   with red at the top, since 1.0\n @CAIRO_SUBPIXEL_ORDER_VBGR: Subpixel elements are arranged vertically\n   with blue at the top, since 1.0\n\n The subpixel order specifies the order of color elements within\n each pixel on the display device when rendering with an\n antialiasing mode of %CAIRO_ANTIALIAS_SUBPIXEL.\n\n Since: 1.0"]
pub type _cairo_subpixel_order = u32;
#[doc = " cairo_subpixel_order_t:\n @CAIRO_SUBPIXEL_ORDER_DEFAULT: Use the default subpixel order for\n   for the target device, since 1.0\n @CAIRO_SUBPIXEL_ORDER_RGB: Subpixel elements are arranged horizontally\n   with red at the left, since 1.0\n @CAIRO_SUBPIXEL_ORDER_BGR:  Subpixel elements are arranged horizontally\n   with blue at the left, since 1.0\n @CAIRO_SUBPIXEL_ORDER_VRGB: Subpixel elements are arranged vertically\n   with red at the top, since 1.0\n @CAIRO_SUBPIXEL_ORDER_VBGR: Subpixel elements are arranged vertically\n   with blue at the top, since 1.0\n\n The subpixel order specifies the order of color elements within\n each pixel on the display device when rendering with an\n antialiasing mode of %CAIRO_ANTIALIAS_SUBPIXEL.\n\n Since: 1.0"]
pub use self::_cairo_subpixel_order as cairo_subpixel_order_t;
pub const CAIRO_HINT_STYLE_DEFAULT: _cairo_hint_style = 0;
pub const CAIRO_HINT_STYLE_NONE: _cairo_hint_style = 1;
pub const CAIRO_HINT_STYLE_SLIGHT: _cairo_hint_style = 2;
pub const CAIRO_HINT_STYLE_MEDIUM: _cairo_hint_style = 3;
pub const CAIRO_HINT_STYLE_FULL: _cairo_hint_style = 4;
#[doc = " cairo_hint_style_t:\n @CAIRO_HINT_STYLE_DEFAULT: Use the default hint style for\n   font backend and target device, since 1.0\n @CAIRO_HINT_STYLE_NONE: Do not hint outlines, since 1.0\n @CAIRO_HINT_STYLE_SLIGHT: Hint outlines slightly to improve\n   contrast while retaining good fidelity to the original\n   shapes, since 1.0\n @CAIRO_HINT_STYLE_MEDIUM: Hint outlines with medium strength\n   giving a compromise between fidelity to the original shapes\n   and contrast, since 1.0\n @CAIRO_HINT_STYLE_FULL: Hint outlines to maximize contrast, since 1.0\n\n Specifies the type of hinting to do on font outlines. Hinting\n is the process of fitting outlines to the pixel grid in order\n to improve the appearance of the result. Since hinting outlines\n involves distorting them, it also reduces the faithfulness\n to the original outline shapes. Not all of the outline hinting\n styles are supported by all font backends.\n\n New entries may be added in future versions.\n\n Since: 1.0"]
pub type _cairo_hint_style = u32;
#[doc = " cairo_hint_style_t:\n @CAIRO_HINT_STYLE_DEFAULT: Use the default hint style for\n   font backend and target device, since 1.0\n @CAIRO_HINT_STYLE_NONE: Do not hint outlines, since 1.0\n @CAIRO_HINT_STYLE_SLIGHT: Hint outlines slightly to improve\n   contrast while retaining good fidelity to the original\n   shapes, since 1.0\n @CAIRO_HINT_STYLE_MEDIUM: Hint outlines with medium strength\n   giving a compromise between fidelity to the original shapes\n   and contrast, since 1.0\n @CAIRO_HINT_STYLE_FULL: Hint outlines to maximize contrast, since 1.0\n\n Specifies the type of hinting to do on font outlines. Hinting\n is the process of fitting outlines to the pixel grid in order\n to improve the appearance of the result. Since hinting outlines\n involves distorting them, it also reduces the faithfulness\n to the original outline shapes. Not all of the outline hinting\n styles are supported by all font backends.\n\n New entries may be added in future versions.\n\n Since: 1.0"]
pub use self::_cairo_hint_style as cairo_hint_style_t;
pub const CAIRO_HINT_METRICS_DEFAULT: _cairo_hint_metrics = 0;
pub const CAIRO_HINT_METRICS_OFF: _cairo_hint_metrics = 1;
pub const CAIRO_HINT_METRICS_ON: _cairo_hint_metrics = 2;
#[doc = " cairo_hint_metrics_t:\n @CAIRO_HINT_METRICS_DEFAULT: Hint metrics in the default\n  manner for the font backend and target device, since 1.0\n @CAIRO_HINT_METRICS_OFF: Do not hint font metrics, since 1.0\n @CAIRO_HINT_METRICS_ON: Hint font metrics, since 1.0\n\n Specifies whether to hint font metrics; hinting font metrics\n means quantizing them so that they are integer values in\n device space. Doing this improves the consistency of\n letter and line spacing, however it also means that text\n will be laid out differently at different zoom factors.\n\n Since: 1.0"]
pub type _cairo_hint_metrics = u32;
#[doc = " cairo_hint_metrics_t:\n @CAIRO_HINT_METRICS_DEFAULT: Hint metrics in the default\n  manner for the font backend and target device, since 1.0\n @CAIRO_HINT_METRICS_OFF: Do not hint font metrics, since 1.0\n @CAIRO_HINT_METRICS_ON: Hint font metrics, since 1.0\n\n Specifies whether to hint font metrics; hinting font metrics\n means quantizing them so that they are integer values in\n device space. Doing this improves the consistency of\n letter and line spacing, however it also means that text\n will be laid out differently at different zoom factors.\n\n Since: 1.0"]
pub use self::_cairo_hint_metrics as cairo_hint_metrics_t;
pub const CAIRO_COLOR_MODE_DEFAULT: _cairo_color_mode = 0;
pub const CAIRO_COLOR_MODE_NO_COLOR: _cairo_color_mode = 1;
pub const CAIRO_COLOR_MODE_COLOR: _cairo_color_mode = 2;
#[doc = " cairo_color_mode_t:\n @CAIRO_COLOR_MODE_DEFAULT: Use the default color mode for\n font backend and target device, since 1.18.\n @CAIRO_COLOR_MODE_NO_COLOR: Disable rendering color glyphs. Glyphs are\n always rendered as outline glyphs, since 1.18.\n @CAIRO_COLOR_MODE_COLOR: Enable rendering color glyphs. If the font\n contains a color presentation for a glyph, and when supported by\n the font backend, the glyph will be rendered in color, since 1.18.\n\n Specifies if color fonts are to be rendered using the color\n glyphs or outline glyphs. Glyphs that do not have a color\n presentation, and non-color fonts are not affected by this font\n option.\n\n Since: 1.18"]
pub type _cairo_color_mode = u32;
#[doc = " cairo_color_mode_t:\n @CAIRO_COLOR_MODE_DEFAULT: Use the default color mode for\n font backend and target device, since 1.18.\n @CAIRO_COLOR_MODE_NO_COLOR: Disable rendering color glyphs. Glyphs are\n always rendered as outline glyphs, since 1.18.\n @CAIRO_COLOR_MODE_COLOR: Enable rendering color glyphs. If the font\n contains a color presentation for a glyph, and when supported by\n the font backend, the glyph will be rendered in color, since 1.18.\n\n Specifies if color fonts are to be rendered using the color\n glyphs or outline glyphs. Glyphs that do not have a color\n presentation, and non-color fonts are not affected by this font\n option.\n\n Since: 1.18"]
pub use self::_cairo_color_mode as cairo_color_mode_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _cairo_font_options {
    _unused: [u8; 0],
}
#[doc = " cairo_font_options_t:\n\n An opaque structure holding all options that are used when\n rendering fonts.\n\n Individual features of a #cairo_font_options_t can be set or\n accessed using functions named\n <function>cairo_font_options_set_<emphasis>feature_name</emphasis>()</function> and\n <function>cairo_font_options_get_<emphasis>feature_name</emphasis>()</function>, like\n cairo_font_options_set_antialias() and\n cairo_font_options_get_antialias().\n\n New features may be added to a #cairo_font_options_t in the\n future.  For this reason, cairo_font_options_copy(),\n cairo_font_options_equal(), cairo_font_options_merge(), and\n cairo_font_options_hash() should be used to copy, check\n for equality, merge, or compute a hash value of\n #cairo_font_options_t objects.\n\n Since: 1.0"]
pub type cairo_font_options_t = _cairo_font_options;
pub const CAIRO_FONT_TYPE_TOY: _cairo_font_type = 0;
pub const CAIRO_FONT_TYPE_FT: _cairo_font_type = 1;
pub const CAIRO_FONT_TYPE_WIN32: _cairo_font_type = 2;
pub const CAIRO_FONT_TYPE_QUARTZ: _cairo_font_type = 3;
pub const CAIRO_FONT_TYPE_USER: _cairo_font_type = 4;
pub const CAIRO_FONT_TYPE_DWRITE: _cairo_font_type = 5;
#[doc = " cairo_font_type_t:\n @CAIRO_FONT_TYPE_TOY: The font was created using cairo's toy font api (Since: 1.2)\n @CAIRO_FONT_TYPE_FT: The font is of type FreeType (Since: 1.2)\n @CAIRO_FONT_TYPE_WIN32: The font is of type Win32 (Since: 1.2)\n @CAIRO_FONT_TYPE_QUARTZ: The font is of type Quartz (Since: 1.6, in 1.2 and\n 1.4 it was named CAIRO_FONT_TYPE_ATSUI)\n @CAIRO_FONT_TYPE_USER: The font was create using cairo's user font api (Since: 1.8)\n @CAIRO_FONT_TYPE_DWRITE: The font is of type Win32 DWrite (Since: 1.18)\n\n #cairo_font_type_t is used to describe the type of a given font\n face or scaled font. The font types are also known as \"font\n backends\" within cairo.\n\n The type of a font face is determined by the function used to\n create it, which will generally be of the form\n <function>cairo_<emphasis>type</emphasis>_font_face_create(<!-- -->)</function>.\n The font face type can be queried with cairo_font_face_get_type()\n\n The various #cairo_font_face_t functions can be used with a font face\n of any type.\n\n The type of a scaled font is determined by the type of the font\n face passed to cairo_scaled_font_create(). The scaled font type can\n be queried with cairo_scaled_font_get_type()\n\n The various #cairo_scaled_font_t functions can be used with scaled\n fonts of any type, but some font backends also provide\n type-specific functions that must only be called with a scaled font\n of the appropriate type. These functions have names that begin with\n <function>cairo_<emphasis>type</emphasis>_scaled_font(<!-- -->)</function>\n such as cairo_ft_scaled_font_lock_face().\n\n The behavior of calling a type-specific function with a scaled font\n of the wrong type is undefined.\n\n New entries may be added in future versions.\n\n Since: 1.2"]
pub type _cairo_font_type = u32;
#[doc = " cairo_font_type_t:\n @CAIRO_FONT_TYPE_TOY: The font was created using cairo's toy font api (Since: 1.2)\n @CAIRO_FONT_TYPE_FT: The font is of type FreeType (Since: 1.2)\n @CAIRO_FONT_TYPE_WIN32: The font is of type Win32 (Since: 1.2)\n @CAIRO_FONT_TYPE_QUARTZ: The font is of type Quartz (Since: 1.6, in 1.2 and\n 1.4 it was named CAIRO_FONT_TYPE_ATSUI)\n @CAIRO_FONT_TYPE_USER: The font was create using cairo's user font api (Since: 1.8)\n @CAIRO_FONT_TYPE_DWRITE: The font is of type Win32 DWrite (Since: 1.18)\n\n #cairo_font_type_t is used to describe the type of a given font\n face or scaled font. The font types are also known as \"font\n backends\" within cairo.\n\n The type of a font face is determined by the function used to\n create it, which will generally be of the form\n <function>cairo_<emphasis>type</emphasis>_font_face_create(<!-- -->)</function>.\n The font face type can be queried with cairo_font_face_get_type()\n\n The various #cairo_font_face_t functions can be used with a font face\n of any type.\n\n The type of a scaled font is determined by the type of the font\n face passed to cairo_scaled_font_create(). The scaled font type can\n be queried with cairo_scaled_font_get_type()\n\n The various #cairo_scaled_font_t functions can be used with scaled\n fonts of any type, but some font backends also provide\n type-specific functions that must only be called with a scaled font\n of the appropriate type. These functions have names that begin with\n <function>cairo_<emphasis>type</emphasis>_scaled_font(<!-- -->)</function>\n such as cairo_ft_scaled_font_lock_face().\n\n The behavior of calling a type-specific function with a scaled font\n of the wrong type is undefined.\n\n New entries may be added in future versions.\n\n Since: 1.2"]
pub use self::_cairo_font_type as cairo_font_type_t;
#[doc = " cairo_user_scaled_font_init_func_t:\n @scaled_font: the scaled-font being created\n @cr: a cairo context, in font space\n @extents: font extents to fill in, in font space\n\n #cairo_user_scaled_font_init_func_t is the type of function which is\n called when a scaled-font needs to be created for a user font-face.\n\n The cairo context @cr is not used by the caller, but is prepared in font\n space, similar to what the cairo contexts passed to the render_glyph\n method will look like.  The callback can use this context for extents\n computation for example.  After the callback is called, @cr is checked\n for any error status.\n\n The @extents argument is where the user font sets the font extents for\n @scaled_font.  It is in font space, which means that for most cases its\n ascent and descent members should add to 1.0.  @extents is preset to\n hold a value of 1.0 for ascent, height, and max_x_advance, and 0.0 for\n descent and max_y_advance members.\n\n The callback is optional.  If not set, default font extents as described\n in the previous paragraph will be used.\n\n Note that @scaled_font is not fully initialized at this\n point and trying to use it for text operations in the callback will result\n in deadlock.\n\n Returns: %CAIRO_STATUS_SUCCESS upon success, or an error status on error.\n\n Since: 1.8"]
pub type cairo_user_scaled_font_init_func_t = ::core::option::Option<
    unsafe extern "C" fn(
        scaled_font: *mut cairo_scaled_font_t,
        cr: *mut cairo_t,
        extents: *mut cairo_font_extents_t,
    ) -> cairo_status_t,
>;
#[doc = " cairo_user_scaled_font_render_glyph_func_t:\n @scaled_font: user scaled-font\n @glyph: glyph code to render\n @cr: cairo context to draw to, in font space\n @extents: glyph extents to fill in, in font space\n\n #cairo_user_scaled_font_render_glyph_func_t is the type of function which\n is called when a user scaled-font needs to render a glyph.\n\n The callback is mandatory, and expected to draw the glyph with code @glyph to\n the cairo context @cr.  @cr is prepared such that the glyph drawing is done in\n font space.  That is, the matrix set on @cr is the scale matrix of @scaled_font.\n The @extents argument is where the user font sets the font extents for\n @scaled_font.  However, if user prefers to draw in user space, they can\n achieve that by changing the matrix on @cr.\n\n All cairo rendering operations to @cr are permitted. However, when\n this callback is set with\n cairo_user_font_face_set_render_glyph_func(), the result is\n undefined if any source other than the default source on @cr is\n used.  That means, glyph bitmaps should be rendered using\n cairo_mask() instead of cairo_paint().\n\n When this callback is set with\n cairo_user_font_face_set_render_color_glyph_func(), the default\n source is black. Setting the source is a valid\n operation. cairo_user_scaled_font_get_foreground_marker() or\n cairo_user_scaled_font_get_foreground_source() may be called to\n obtain the current source at the time the glyph is rendered.\n\n Other non-default settings on @cr include a font size of 1.0 (given that\n it is set up to be in font space), and font options corresponding to\n @scaled_font.\n\n The @extents argument is preset to have <literal>x_bearing</literal>,\n <literal>width</literal>, and <literal>y_advance</literal> of zero,\n <literal>y_bearing</literal> set to <literal>-font_extents.ascent</literal>,\n <literal>height</literal> to <literal>font_extents.ascent+font_extents.descent</literal>,\n and <literal>x_advance</literal> to <literal>font_extents.max_x_advance</literal>.\n The only field user needs to set in majority of cases is\n <literal>x_advance</literal>.\n If the <literal>width</literal> field is zero upon the callback returning\n (which is its preset value), the glyph extents are automatically computed\n based on the drawings done to @cr.  This is in most cases exactly what the\n desired behavior is.  However, if for any reason the callback sets the\n extents, it must be ink extents, and include the extents of all drawing\n done to @cr in the callback.\n\n Where both color and non-color callbacks has been set using\n cairo_user_font_face_set_render_color_glyph_func(), and\n cairo_user_font_face_set_render_glyph_func(), the color glyph\n callback will be called first. If the color glyph callback returns\n %CAIRO_STATUS_USER_FONT_NOT_IMPLEMENTED, any drawing operations are\n discarded and the non-color callback will be called. This is the\n only case in which the %CAIRO_STATUS_USER_FONT_NOT_IMPLEMENTED may\n be returned from a render callback. This fallback sequence allows a\n user font face to contain a combination of both color and non-color\n glyphs.\n\n Returns: %CAIRO_STATUS_SUCCESS upon success,\n %CAIRO_STATUS_USER_FONT_NOT_IMPLEMENTED if fallback options should be tried,\n or %CAIRO_STATUS_USER_FONT_ERROR or any other error status on error.\n\n Since: 1.8"]
pub type cairo_user_scaled_font_render_glyph_func_t = ::core::option::Option<
    unsafe extern "C" fn(
        scaled_font: *mut cairo_scaled_font_t,
        glyph: ::core::ffi::c_ulong,
        cr: *mut cairo_t,
        extents: *mut cairo_text_extents_t,
    ) -> cairo_status_t,
>;
#[doc = " cairo_user_scaled_font_text_to_glyphs_func_t:\n @scaled_font: the scaled-font being created\n @utf8: a string of text encoded in UTF-8\n @utf8_len: length of @utf8 in bytes\n @glyphs: pointer to array of glyphs to fill, in font space\n @num_glyphs: pointer to number of glyphs\n @clusters: pointer to array of cluster mapping information to fill, or %NULL\n @num_clusters: pointer to number of clusters\n @cluster_flags: pointer to location to store cluster flags corresponding to the\n                 output @clusters\n\n #cairo_user_scaled_font_text_to_glyphs_func_t is the type of function which\n is called to convert input text to an array of glyphs.  This is used by the\n cairo_show_text() operation.\n\n Using this callback the user-font has full control on glyphs and their\n positions.  That means, it allows for features like ligatures and kerning,\n as well as complex <firstterm>shaping</firstterm> required for scripts like\n Arabic and Indic.\n\n The @num_glyphs argument is preset to the number of glyph entries available\n in the @glyphs buffer. If the @glyphs buffer is %NULL, the value of\n @num_glyphs will be zero.  If the provided glyph array is too short for\n the conversion (or for convenience), a new glyph array may be allocated\n using cairo_glyph_allocate() and placed in @glyphs.  Upon return,\n @num_glyphs should contain the number of generated glyphs.  If the value\n @glyphs points at has changed after the call, the caller will free the\n allocated glyph array using cairo_glyph_free().  The caller will also free\n the original value of @glyphs, so the callback shouldn't do so.\n The callback should populate the glyph indices and positions (in font space)\n assuming that the text is to be shown at the origin.\n\n If @clusters is not %NULL, @num_clusters and @cluster_flags are also\n non-%NULL, and cluster mapping should be computed. The semantics of how\n cluster array allocation works is similar to the glyph array.  That is,\n if @clusters initially points to a non-%NULL value, that array may be used\n as a cluster buffer, and @num_clusters points to the number of cluster\n entries available there.  If the provided cluster array is too short for\n the conversion (or for convenience), a new cluster array may be allocated\n using cairo_text_cluster_allocate() and placed in @clusters.  In this case,\n the original value of @clusters will still be freed by the caller.  Upon\n return, @num_clusters should contain the number of generated clusters.\n If the value @clusters points at has changed after the call, the caller\n will free the allocated cluster array using cairo_text_cluster_free().\n\n The callback is optional.  If @num_glyphs is negative upon\n the callback returning or if the return value\n is %CAIRO_STATUS_USER_FONT_NOT_IMPLEMENTED, the unicode_to_glyph callback\n is tried.  See #cairo_user_scaled_font_unicode_to_glyph_func_t.\n\n Note: While cairo does not impose any limitation on glyph indices,\n some applications may assume that a glyph index fits in a 16-bit\n unsigned integer.  As such, it is advised that user-fonts keep their\n glyphs in the 0 to 65535 range.  Furthermore, some applications may\n assume that glyph 0 is a special glyph-not-found glyph.  User-fonts\n are advised to use glyph 0 for such purposes and do not use that\n glyph value for other purposes.\n\n Returns: %CAIRO_STATUS_SUCCESS upon success,\n %CAIRO_STATUS_USER_FONT_NOT_IMPLEMENTED if fallback options should be tried,\n or %CAIRO_STATUS_USER_FONT_ERROR or any other error status on error.\n\n Since: 1.8"]
pub type cairo_user_scaled_font_text_to_glyphs_func_t = ::core::option::Option<
    unsafe extern "C" fn(
        scaled_font: *mut cairo_scaled_font_t,
        utf8: *const ::core::ffi::c_char,
        utf8_len: ::core::ffi::c_int,
        glyphs: *mut *mut cairo_glyph_t,
        num_glyphs: *mut ::core::ffi::c_int,
        clusters: *mut *mut cairo_text_cluster_t,
        num_clusters: *mut ::core::ffi::c_int,
        cluster_flags: *mut cairo_text_cluster_flags_t,
    ) -> cairo_status_t,
>;
#[doc = " cairo_user_scaled_font_unicode_to_glyph_func_t:\n @scaled_font: the scaled-font being created\n @unicode: input unicode character code-point\n @glyph_index: output glyph index\n\n #cairo_user_scaled_font_unicode_to_glyph_func_t is the type of function which\n is called to convert an input Unicode character to a single glyph.\n This is used by the cairo_show_text() operation.\n\n This callback is used to provide the same functionality as the\n text_to_glyphs callback does (see #cairo_user_scaled_font_text_to_glyphs_func_t)\n but has much less control on the output,\n in exchange for increased ease of use.  The inherent assumption to using\n this callback is that each character maps to one glyph, and that the\n mapping is context independent.  It also assumes that glyphs are positioned\n according to their advance width.  These mean no ligatures, kerning, or\n complex scripts can be implemented using this callback.\n\n The callback is optional, and only used if text_to_glyphs callback is not\n set or fails to return glyphs.  If this callback is not set or if it returns\n %CAIRO_STATUS_USER_FONT_NOT_IMPLEMENTED, an identity mapping from Unicode\n code-points to glyph indices is assumed.\n\n Note: While cairo does not impose any limitation on glyph indices,\n some applications may assume that a glyph index fits in a 16-bit\n unsigned integer.  As such, it is advised that user-fonts keep their\n glyphs in the 0 to 65535 range.  Furthermore, some applications may\n assume that glyph 0 is a special glyph-not-found glyph.  User-fonts\n are advised to use glyph 0 for such purposes and do not use that\n glyph value for other purposes.\n\n Returns: %CAIRO_STATUS_SUCCESS upon success,\n %CAIRO_STATUS_USER_FONT_NOT_IMPLEMENTED if fallback options should be tried,\n or %CAIRO_STATUS_USER_FONT_ERROR or any other error status on error.\n\n Since: 1.8"]
pub type cairo_user_scaled_font_unicode_to_glyph_func_t = ::core::option::Option<
    unsafe extern "C" fn(
        scaled_font: *mut cairo_scaled_font_t,
        unicode: ::core::ffi::c_ulong,
        glyph_index: *mut ::core::ffi::c_ulong,
    ) -> cairo_status_t,
>;
pub const CAIRO_PATH_MOVE_TO: _cairo_path_data_type = 0;
pub const CAIRO_PATH_LINE_TO: _cairo_path_data_type = 1;
pub const CAIRO_PATH_CURVE_TO: _cairo_path_data_type = 2;
pub const CAIRO_PATH_CLOSE_PATH: _cairo_path_data_type = 3;
#[doc = " cairo_path_data_type_t:\n @CAIRO_PATH_MOVE_TO: A move-to operation, since 1.0\n @CAIRO_PATH_LINE_TO: A line-to operation, since 1.0\n @CAIRO_PATH_CURVE_TO: A curve-to operation, since 1.0\n @CAIRO_PATH_CLOSE_PATH: A close-path operation, since 1.0\n\n #cairo_path_data_t is used to describe the type of one portion\n of a path when represented as a #cairo_path_t.\n See #cairo_path_data_t for details.\n\n Since: 1.0"]
pub type _cairo_path_data_type = u32;
#[doc = " cairo_path_data_type_t:\n @CAIRO_PATH_MOVE_TO: A move-to operation, since 1.0\n @CAIRO_PATH_LINE_TO: A line-to operation, since 1.0\n @CAIRO_PATH_CURVE_TO: A curve-to operation, since 1.0\n @CAIRO_PATH_CLOSE_PATH: A close-path operation, since 1.0\n\n #cairo_path_data_t is used to describe the type of one portion\n of a path when represented as a #cairo_path_t.\n See #cairo_path_data_t for details.\n\n Since: 1.0"]
pub use self::_cairo_path_data_type as cairo_path_data_type_t;
#[doc = " cairo_path_data_t:\n\n #cairo_path_data_t is used to represent the path data inside a\n #cairo_path_t.\n\n The data structure is designed to try to balance the demands of\n efficiency and ease-of-use. A path is represented as an array of\n #cairo_path_data_t, which is a union of headers and points.\n\n Each portion of the path is represented by one or more elements in\n the array, (one header followed by 0 or more points). The length\n value of the header is the number of array elements for the current\n portion including the header, (ie. length == 1 + # of points), and\n where the number of points for each element type is as follows:\n\n <programlisting>\n     %CAIRO_PATH_MOVE_TO:     1 point\n     %CAIRO_PATH_LINE_TO:     1 point\n     %CAIRO_PATH_CURVE_TO:    3 points\n     %CAIRO_PATH_CLOSE_PATH:  0 points\n </programlisting>\n\n The semantics and ordering of the coordinate values are consistent\n with cairo_move_to(), cairo_line_to(), cairo_curve_to(), and\n cairo_close_path().\n\n Here is sample code for iterating through a #cairo_path_t:\n\n <informalexample><programlisting>\n      int i;\n      cairo_path_t *path;\n      cairo_path_data_t *data;\n &nbsp;\n      path = cairo_copy_path (cr);\n &nbsp;\n      for (i=0; i < path->num_data; i += path->data[i].header.length) {\n          data = &amp;path->data[i];\n          switch (data->header.type) {\n          case CAIRO_PATH_MOVE_TO:\n              do_move_to_things (data[1].point.x, data[1].point.y);\n              break;\n          case CAIRO_PATH_LINE_TO:\n              do_line_to_things (data[1].point.x, data[1].point.y);\n              break;\n          case CAIRO_PATH_CURVE_TO:\n              do_curve_to_things (data[1].point.x, data[1].point.y,\n                                  data[2].point.x, data[2].point.y,\n                                  data[3].point.x, data[3].point.y);\n              break;\n          case CAIRO_PATH_CLOSE_PATH:\n              do_close_path_things ();\n              break;\n          }\n      }\n      cairo_path_destroy (path);\n </programlisting></informalexample>\n\n As of cairo 1.4, cairo does not mind if there are more elements in\n a portion of the path than needed.  Such elements can be used by\n users of the cairo API to hold extra values in the path data\n structure.  For this reason, it is recommended that applications\n always use <literal>data->header.length</literal> to\n iterate over the path data, instead of hardcoding the number of\n elements for each element type.\n\n Since: 1.0"]
pub type cairo_path_data_t = _cairo_path_data_t;
#[repr(C)]
#[derive(Copy, Clone)]
pub union _cairo_path_data_t {
    pub header: _cairo_path_data_t__bindgen_ty_1,
    pub point: _cairo_path_data_t__bindgen_ty_2,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _cairo_path_data_t__bindgen_ty_1 {
    pub type_: cairo_path_data_type_t,
    pub length: ::core::ffi::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _cairo_path_data_t__bindgen_ty_2 {
    pub x: f64,
    pub y: f64,
}
#[doc = " cairo_path_t:\n @status: the current error status\n @data: the elements in the path\n @num_data: the number of elements in the data array\n\n A data structure for holding a path. This data structure serves as\n the return value for cairo_copy_path() and\n cairo_copy_path_flat() as well the input value for\n cairo_append_path().\n\n See #cairo_path_data_t for hints on how to iterate over the\n actual data within the path.\n\n The num_data member gives the number of elements in the data\n array. This number is larger than the number of independent path\n portions (defined in #cairo_path_data_type_t), since the data\n includes both headers and coordinates for each portion.\n\n Since: 1.0"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cairo_path {
    pub status: cairo_status_t,
    pub data: *mut cairo_path_data_t,
    pub num_data: ::core::ffi::c_int,
}
#[doc = " cairo_path_t:\n @status: the current error status\n @data: the elements in the path\n @num_data: the number of elements in the data array\n\n A data structure for holding a path. This data structure serves as\n the return value for cairo_copy_path() and\n cairo_copy_path_flat() as well the input value for\n cairo_append_path().\n\n See #cairo_path_data_t for hints on how to iterate over the\n actual data within the path.\n\n The num_data member gives the number of elements in the data\n array. This number is larger than the number of independent path\n portions (defined in #cairo_path_data_type_t), since the data\n includes both headers and coordinates for each portion.\n\n Since: 1.0"]
pub type cairo_path_t = cairo_path;
pub const CAIRO_DEVICE_TYPE_DRM: _cairo_device_type = 0;
pub const CAIRO_DEVICE_TYPE_GL: _cairo_device_type = 1;
pub const CAIRO_DEVICE_TYPE_SCRIPT: _cairo_device_type = 2;
pub const CAIRO_DEVICE_TYPE_XCB: _cairo_device_type = 3;
pub const CAIRO_DEVICE_TYPE_XLIB: _cairo_device_type = 4;
pub const CAIRO_DEVICE_TYPE_XML: _cairo_device_type = 5;
pub const CAIRO_DEVICE_TYPE_COGL: _cairo_device_type = 6;
pub const CAIRO_DEVICE_TYPE_WIN32: _cairo_device_type = 7;
pub const CAIRO_DEVICE_TYPE_INVALID: _cairo_device_type = -1;
#[doc = " cairo_device_type_t:\n @CAIRO_DEVICE_TYPE_DRM: The device is of type Direct Render Manager, since 1.10\n @CAIRO_DEVICE_TYPE_GL: The device is of type OpenGL, since 1.10\n @CAIRO_DEVICE_TYPE_SCRIPT: The device is of type script, since 1.10\n @CAIRO_DEVICE_TYPE_XCB: The device is of type xcb, since 1.10\n @CAIRO_DEVICE_TYPE_XLIB: The device is of type xlib, since 1.10\n @CAIRO_DEVICE_TYPE_XML: The device is of type XML, since 1.10\n @CAIRO_DEVICE_TYPE_COGL: The device is of type cogl, since 1.12\n @CAIRO_DEVICE_TYPE_WIN32: The device is of type win32, since 1.12\n @CAIRO_DEVICE_TYPE_INVALID: The device is invalid, since 1.10\n\n #cairo_device_type_t is used to describe the type of a given\n device. The devices types are also known as \"backends\" within cairo.\n\n The device type can be queried with cairo_device_get_type()\n\n The various #cairo_device_t functions can be used with devices of\n any type, but some backends also provide type-specific functions\n that must only be called with a device of the appropriate\n type. These functions have names that begin with\n <literal>cairo_<emphasis>type</emphasis>_device</literal> such as\n cairo_xcb_device_debug_cap_xrender_version().\n\n The behavior of calling a type-specific function with a device of\n the wrong type is undefined.\n\n New entries may be added in future versions.\n\n Since: 1.10"]
pub type _cairo_device_type = i32;
#[doc = " cairo_device_type_t:\n @CAIRO_DEVICE_TYPE_DRM: The device is of type Direct Render Manager, since 1.10\n @CAIRO_DEVICE_TYPE_GL: The device is of type OpenGL, since 1.10\n @CAIRO_DEVICE_TYPE_SCRIPT: The device is of type script, since 1.10\n @CAIRO_DEVICE_TYPE_XCB: The device is of type xcb, since 1.10\n @CAIRO_DEVICE_TYPE_XLIB: The device is of type xlib, since 1.10\n @CAIRO_DEVICE_TYPE_XML: The device is of type XML, since 1.10\n @CAIRO_DEVICE_TYPE_COGL: The device is of type cogl, since 1.12\n @CAIRO_DEVICE_TYPE_WIN32: The device is of type win32, since 1.12\n @CAIRO_DEVICE_TYPE_INVALID: The device is invalid, since 1.10\n\n #cairo_device_type_t is used to describe the type of a given\n device. The devices types are also known as \"backends\" within cairo.\n\n The device type can be queried with cairo_device_get_type()\n\n The various #cairo_device_t functions can be used with devices of\n any type, but some backends also provide type-specific functions\n that must only be called with a device of the appropriate\n type. These functions have names that begin with\n <literal>cairo_<emphasis>type</emphasis>_device</literal> such as\n cairo_xcb_device_debug_cap_xrender_version().\n\n The behavior of calling a type-specific function with a device of\n the wrong type is undefined.\n\n New entries may be added in future versions.\n\n Since: 1.10"]
pub use self::_cairo_device_type as cairo_device_type_t;
pub const CAIRO_SURFACE_OBSERVER_NORMAL: cairo_surface_observer_mode_t = 0;
pub const CAIRO_SURFACE_OBSERVER_RECORD_OPERATIONS: cairo_surface_observer_mode_t = 1;
#[doc = " cairo_surface_observer_mode_t:\n @CAIRO_SURFACE_OBSERVER_NORMAL: no recording is done\n @CAIRO_SURFACE_OBSERVER_RECORD_OPERATIONS: operations are recorded\n\n Whether operations should be recorded.\n\n Since: 1.12"]
pub type cairo_surface_observer_mode_t = u32;
#[doc = " cairo_surface_observer_callback_t:\n @observer: the #cairo_surface_observer_t\n @target: the observed surface\n @data: closure used when adding the callback\n\n A generic callback function for surface operations.\n\n Since: 1.12"]
pub type cairo_surface_observer_callback_t = ::core::option::Option<
    unsafe extern "C" fn(
        observer: *mut cairo_surface_t,
        target: *mut cairo_surface_t,
        data: *mut ::core::ffi::c_void,
    ),
>;
pub const CAIRO_SURFACE_TYPE_IMAGE: _cairo_surface_type = 0;
pub const CAIRO_SURFACE_TYPE_PDF: _cairo_surface_type = 1;
pub const CAIRO_SURFACE_TYPE_PS: _cairo_surface_type = 2;
pub const CAIRO_SURFACE_TYPE_XLIB: _cairo_surface_type = 3;
pub const CAIRO_SURFACE_TYPE_XCB: _cairo_surface_type = 4;
pub const CAIRO_SURFACE_TYPE_GLITZ: _cairo_surface_type = 5;
pub const CAIRO_SURFACE_TYPE_QUARTZ: _cairo_surface_type = 6;
pub const CAIRO_SURFACE_TYPE_WIN32: _cairo_surface_type = 7;
pub const CAIRO_SURFACE_TYPE_BEOS: _cairo_surface_type = 8;
pub const CAIRO_SURFACE_TYPE_DIRECTFB: _cairo_surface_type = 9;
pub const CAIRO_SURFACE_TYPE_SVG: _cairo_surface_type = 10;
pub const CAIRO_SURFACE_TYPE_OS2: _cairo_surface_type = 11;
pub const CAIRO_SURFACE_TYPE_WIN32_PRINTING: _cairo_surface_type = 12;
pub const CAIRO_SURFACE_TYPE_QUARTZ_IMAGE: _cairo_surface_type = 13;
pub const CAIRO_SURFACE_TYPE_SCRIPT: _cairo_surface_type = 14;
pub const CAIRO_SURFACE_TYPE_QT: _cairo_surface_type = 15;
pub const CAIRO_SURFACE_TYPE_RECORDING: _cairo_surface_type = 16;
pub const CAIRO_SURFACE_TYPE_VG: _cairo_surface_type = 17;
pub const CAIRO_SURFACE_TYPE_GL: _cairo_surface_type = 18;
pub const CAIRO_SURFACE_TYPE_DRM: _cairo_surface_type = 19;
pub const CAIRO_SURFACE_TYPE_TEE: _cairo_surface_type = 20;
pub const CAIRO_SURFACE_TYPE_XML: _cairo_surface_type = 21;
pub const CAIRO_SURFACE_TYPE_SKIA: _cairo_surface_type = 22;
pub const CAIRO_SURFACE_TYPE_SUBSURFACE: _cairo_surface_type = 23;
pub const CAIRO_SURFACE_TYPE_COGL: _cairo_surface_type = 24;
#[doc = " cairo_surface_type_t:\n @CAIRO_SURFACE_TYPE_IMAGE: The surface is of type image, since 1.2\n @CAIRO_SURFACE_TYPE_PDF: The surface is of type pdf, since 1.2\n @CAIRO_SURFACE_TYPE_PS: The surface is of type ps, since 1.2\n @CAIRO_SURFACE_TYPE_XLIB: The surface is of type xlib, since 1.2\n @CAIRO_SURFACE_TYPE_XCB: The surface is of type xcb, since 1.2\n @CAIRO_SURFACE_TYPE_GLITZ: The surface is of type glitz, since 1.2, deprecated 1.18\n   (glitz support have been removed, this surface type will never be set by cairo)\n @CAIRO_SURFACE_TYPE_QUARTZ: The surface is of type quartz, since 1.2\n @CAIRO_SURFACE_TYPE_WIN32: The surface is of type win32, since 1.2\n @CAIRO_SURFACE_TYPE_BEOS: The surface is of type beos, since 1.2, deprecated 1.18\n   (beos support have been removed, this surface type will never be set by cairo)\n @CAIRO_SURFACE_TYPE_DIRECTFB: The surface is of type directfb, since 1.2, deprecated 1.18\n   (directfb support have been removed, this surface type will never be set by cairo)\n @CAIRO_SURFACE_TYPE_SVG: The surface is of type svg, since 1.2\n @CAIRO_SURFACE_TYPE_OS2: The surface is of type os2, since 1.4, deprecated 1.18\n   (os2 support have been removed, this surface type will never be set by cairo)\n @CAIRO_SURFACE_TYPE_WIN32_PRINTING: The surface is a win32 printing surface, since 1.6\n @CAIRO_SURFACE_TYPE_QUARTZ_IMAGE: The surface is of type quartz_image, since 1.6\n @CAIRO_SURFACE_TYPE_SCRIPT: The surface is of type script, since 1.10\n @CAIRO_SURFACE_TYPE_QT: The surface is of type Qt, since 1.10, deprecated 1.18\n   (Ot support have been removed, this surface type will never be set by cairo)\n @CAIRO_SURFACE_TYPE_RECORDING: The surface is of type recording, since 1.10\n @CAIRO_SURFACE_TYPE_VG: The surface is a OpenVG surface, since 1.10, deprecated 1.18\n   (OpenVG support have been removed, this surface type will never be set by cairo)\n @CAIRO_SURFACE_TYPE_GL: The surface is of type OpenGL, since 1.10, deprecated 1.18\n   (OpenGL support have been removed, this surface type will never be set by cairo)\n @CAIRO_SURFACE_TYPE_DRM: The surface is of type Direct Render Manager, since 1.10, deprecated 1.18\n   (DRM support have been removed, this surface type will never be set by cairo)\n @CAIRO_SURFACE_TYPE_TEE: The surface is of type 'tee' (a multiplexing surface), since 1.10\n @CAIRO_SURFACE_TYPE_XML: The surface is of type XML (for debugging), since 1.10\n @CAIRO_SURFACE_TYPE_SKIA: The surface is of type Skia, since 1.10, deprecated 1.18\n   (Skia support have been removed, this surface type will never be set by cairo)\n @CAIRO_SURFACE_TYPE_SUBSURFACE: The surface is a subsurface created with\n   cairo_surface_create_for_rectangle(), since 1.10\n @CAIRO_SURFACE_TYPE_COGL: This surface is of type Cogl, since 1.12, deprecated 1.18\n   (Cogl support have been removed, this surface type will never be set by cairo)\n\n #cairo_surface_type_t is used to describe the type of a given\n surface. The surface types are also known as \"backends\" or \"surface\n backends\" within cairo.\n\n The type of a surface is determined by the function used to create\n it, which will generally be of the form\n <function>cairo_<emphasis>type</emphasis>_surface_create(<!-- -->)</function>,\n (though see cairo_surface_create_similar() as well).\n\n The surface type can be queried with cairo_surface_get_type()\n\n The various #cairo_surface_t functions can be used with surfaces of\n any type, but some backends also provide type-specific functions\n that must only be called with a surface of the appropriate\n type. These functions have names that begin with\n <literal>cairo_<emphasis>type</emphasis>_surface</literal> such as cairo_image_surface_get_width().\n\n The behavior of calling a type-specific function with a surface of\n the wrong type is undefined.\n\n New entries may be added in future versions.\n\n Since: 1.2"]
pub type _cairo_surface_type = u32;
#[doc = " cairo_surface_type_t:\n @CAIRO_SURFACE_TYPE_IMAGE: The surface is of type image, since 1.2\n @CAIRO_SURFACE_TYPE_PDF: The surface is of type pdf, since 1.2\n @CAIRO_SURFACE_TYPE_PS: The surface is of type ps, since 1.2\n @CAIRO_SURFACE_TYPE_XLIB: The surface is of type xlib, since 1.2\n @CAIRO_SURFACE_TYPE_XCB: The surface is of type xcb, since 1.2\n @CAIRO_SURFACE_TYPE_GLITZ: The surface is of type glitz, since 1.2, deprecated 1.18\n   (glitz support have been removed, this surface type will never be set by cairo)\n @CAIRO_SURFACE_TYPE_QUARTZ: The surface is of type quartz, since 1.2\n @CAIRO_SURFACE_TYPE_WIN32: The surface is of type win32, since 1.2\n @CAIRO_SURFACE_TYPE_BEOS: The surface is of type beos, since 1.2, deprecated 1.18\n   (beos support have been removed, this surface type will never be set by cairo)\n @CAIRO_SURFACE_TYPE_DIRECTFB: The surface is of type directfb, since 1.2, deprecated 1.18\n   (directfb support have been removed, this surface type will never be set by cairo)\n @CAIRO_SURFACE_TYPE_SVG: The surface is of type svg, since 1.2\n @CAIRO_SURFACE_TYPE_OS2: The surface is of type os2, since 1.4, deprecated 1.18\n   (os2 support have been removed, this surface type will never be set by cairo)\n @CAIRO_SURFACE_TYPE_WIN32_PRINTING: The surface is a win32 printing surface, since 1.6\n @CAIRO_SURFACE_TYPE_QUARTZ_IMAGE: The surface is of type quartz_image, since 1.6\n @CAIRO_SURFACE_TYPE_SCRIPT: The surface is of type script, since 1.10\n @CAIRO_SURFACE_TYPE_QT: The surface is of type Qt, since 1.10, deprecated 1.18\n   (Ot support have been removed, this surface type will never be set by cairo)\n @CAIRO_SURFACE_TYPE_RECORDING: The surface is of type recording, since 1.10\n @CAIRO_SURFACE_TYPE_VG: The surface is a OpenVG surface, since 1.10, deprecated 1.18\n   (OpenVG support have been removed, this surface type will never be set by cairo)\n @CAIRO_SURFACE_TYPE_GL: The surface is of type OpenGL, since 1.10, deprecated 1.18\n   (OpenGL support have been removed, this surface type will never be set by cairo)\n @CAIRO_SURFACE_TYPE_DRM: The surface is of type Direct Render Manager, since 1.10, deprecated 1.18\n   (DRM support have been removed, this surface type will never be set by cairo)\n @CAIRO_SURFACE_TYPE_TEE: The surface is of type 'tee' (a multiplexing surface), since 1.10\n @CAIRO_SURFACE_TYPE_XML: The surface is of type XML (for debugging), since 1.10\n @CAIRO_SURFACE_TYPE_SKIA: The surface is of type Skia, since 1.10, deprecated 1.18\n   (Skia support have been removed, this surface type will never be set by cairo)\n @CAIRO_SURFACE_TYPE_SUBSURFACE: The surface is a subsurface created with\n   cairo_surface_create_for_rectangle(), since 1.10\n @CAIRO_SURFACE_TYPE_COGL: This surface is of type Cogl, since 1.12, deprecated 1.18\n   (Cogl support have been removed, this surface type will never be set by cairo)\n\n #cairo_surface_type_t is used to describe the type of a given\n surface. The surface types are also known as \"backends\" or \"surface\n backends\" within cairo.\n\n The type of a surface is determined by the function used to create\n it, which will generally be of the form\n <function>cairo_<emphasis>type</emphasis>_surface_create(<!-- -->)</function>,\n (though see cairo_surface_create_similar() as well).\n\n The surface type can be queried with cairo_surface_get_type()\n\n The various #cairo_surface_t functions can be used with surfaces of\n any type, but some backends also provide type-specific functions\n that must only be called with a surface of the appropriate\n type. These functions have names that begin with\n <literal>cairo_<emphasis>type</emphasis>_surface</literal> such as cairo_image_surface_get_width().\n\n The behavior of calling a type-specific function with a surface of\n the wrong type is undefined.\n\n New entries may be added in future versions.\n\n Since: 1.2"]
pub use self::_cairo_surface_type as cairo_surface_type_t;
#[doc = " cairo_raster_source_acquire_func_t:\n @pattern: the pattern being rendered from\n @callback_data: the user data supplied during creation\n @target: the rendering target surface\n @extents: rectangular region of interest in pixels in sample space\n\n #cairo_raster_source_acquire_func_t is the type of function which is\n called when a pattern is being rendered from. It should create a surface\n that provides the pixel data for the region of interest as defined by\n extents, though the surface itself does not have to be limited to that\n area. For convenience the surface should probably be of image type,\n created with cairo_surface_create_similar_image() for the target (which\n enables the number of copies to be reduced during transfer to the\n device). Another option, might be to return a similar surface to the\n target for explicit handling by the application of a set of cached sources\n on the device. The region of sample data provided should be defined using\n cairo_surface_set_device_offset() to specify the top-left corner of the\n sample data (along with width and height of the surface).\n\n Returns: a #cairo_surface_t\n\n Since: 1.12"]
pub type cairo_raster_source_acquire_func_t = ::core::option::Option<
    unsafe extern "C" fn(
        pattern: *mut cairo_pattern_t,
        callback_data: *mut ::core::ffi::c_void,
        target: *mut cairo_surface_t,
        extents: *const cairo_rectangle_int_t,
    ) -> *mut cairo_surface_t,
>;
#[doc = " cairo_raster_source_release_func_t:\n @pattern: the pattern being rendered from\n @callback_data: the user data supplied during creation\n @surface: the surface created during acquire\n\n #cairo_raster_source_release_func_t is the type of function which is\n called when the pixel data is no longer being access by the pattern\n for the rendering operation. Typically this function will simply\n destroy the surface created during acquire.\n\n Since: 1.12"]
pub type cairo_raster_source_release_func_t = ::core::option::Option<
    unsafe extern "C" fn(
        pattern: *mut cairo_pattern_t,
        callback_data: *mut ::core::ffi::c_void,
        surface: *mut cairo_surface_t,
    ),
>;
#[doc = " cairo_raster_source_snapshot_func_t:\n @pattern: the pattern being rendered from\n @callback_data: the user data supplied during creation\n\n #cairo_raster_source_snapshot_func_t is the type of function which is\n called when the pixel data needs to be preserved for later use\n during printing. This pattern will be accessed again later, and it\n is expected to provide the pixel data that was current at the time\n of snapshotting.\n\n Return value: CAIRO_STATUS_SUCCESS on success, or one of the\n #cairo_status_t error codes for failure.\n\n Since: 1.12"]
pub type cairo_raster_source_snapshot_func_t = ::core::option::Option<
    unsafe extern "C" fn(
        pattern: *mut cairo_pattern_t,
        callback_data: *mut ::core::ffi::c_void,
    ) -> cairo_status_t,
>;
#[doc = " cairo_raster_source_copy_func_t:\n @pattern: the #cairo_pattern_t that was copied to\n @callback_data: the user data supplied during creation\n @other: the #cairo_pattern_t being used as the source for the copy\n\n #cairo_raster_source_copy_func_t is the type of function which is\n called when the pattern gets copied as a normal part of rendering.\n\n Return value: CAIRO_STATUS_SUCCESS on success, or one of the\n #cairo_status_t error codes for failure.\n\n Since: 1.12"]
pub type cairo_raster_source_copy_func_t = ::core::option::Option<
    unsafe extern "C" fn(
        pattern: *mut cairo_pattern_t,
        callback_data: *mut ::core::ffi::c_void,
        other: *const cairo_pattern_t,
    ) -> cairo_status_t,
>;
#[doc = " cairo_raster_source_finish_func_t:\n @pattern: the pattern being rendered from\n @callback_data: the user data supplied during creation\n\n #cairo_raster_source_finish_func_t is the type of function which is\n called when the pattern (or a copy thereof) is no longer required.\n\n Since: 1.12"]
pub type cairo_raster_source_finish_func_t = ::core::option::Option<
    unsafe extern "C" fn(pattern: *mut cairo_pattern_t, callback_data: *mut ::core::ffi::c_void),
>;
pub const CAIRO_PATTERN_TYPE_SOLID: _cairo_pattern_type = 0;
pub const CAIRO_PATTERN_TYPE_SURFACE: _cairo_pattern_type = 1;
pub const CAIRO_PATTERN_TYPE_LINEAR: _cairo_pattern_type = 2;
pub const CAIRO_PATTERN_TYPE_RADIAL: _cairo_pattern_type = 3;
pub const CAIRO_PATTERN_TYPE_MESH: _cairo_pattern_type = 4;
pub const CAIRO_PATTERN_TYPE_RASTER_SOURCE: _cairo_pattern_type = 5;
#[doc = " cairo_pattern_type_t:\n @CAIRO_PATTERN_TYPE_SOLID: The pattern is a solid (uniform)\n color. It may be opaque or translucent, since 1.2.\n @CAIRO_PATTERN_TYPE_SURFACE: The pattern is a based on a surface (an image), since 1.2.\n @CAIRO_PATTERN_TYPE_LINEAR: The pattern is a linear gradient, since 1.2.\n @CAIRO_PATTERN_TYPE_RADIAL: The pattern is a radial gradient, since 1.2.\n @CAIRO_PATTERN_TYPE_MESH: The pattern is a mesh, since 1.12.\n @CAIRO_PATTERN_TYPE_RASTER_SOURCE: The pattern is a user pattern providing raster data, since 1.12.\n\n #cairo_pattern_type_t is used to describe the type of a given pattern.\n\n The type of a pattern is determined by the function used to create\n it. The cairo_pattern_create_rgb() and cairo_pattern_create_rgba()\n functions create SOLID patterns. The remaining\n cairo_pattern_create<!-- --> functions map to pattern types in obvious\n ways.\n\n The pattern type can be queried with cairo_pattern_get_type()\n\n Most #cairo_pattern_t functions can be called with a pattern of any\n type, (though trying to change the extend or filter for a solid\n pattern will have no effect). A notable exception is\n cairo_pattern_add_color_stop_rgb() and\n cairo_pattern_add_color_stop_rgba() which must only be called with\n gradient patterns (either LINEAR or RADIAL). Otherwise the pattern\n will be shutdown and put into an error state.\n\n New entries may be added in future versions.\n\n Since: 1.2"]
pub type _cairo_pattern_type = u32;
#[doc = " cairo_pattern_type_t:\n @CAIRO_PATTERN_TYPE_SOLID: The pattern is a solid (uniform)\n color. It may be opaque or translucent, since 1.2.\n @CAIRO_PATTERN_TYPE_SURFACE: The pattern is a based on a surface (an image), since 1.2.\n @CAIRO_PATTERN_TYPE_LINEAR: The pattern is a linear gradient, since 1.2.\n @CAIRO_PATTERN_TYPE_RADIAL: The pattern is a radial gradient, since 1.2.\n @CAIRO_PATTERN_TYPE_MESH: The pattern is a mesh, since 1.12.\n @CAIRO_PATTERN_TYPE_RASTER_SOURCE: The pattern is a user pattern providing raster data, since 1.12.\n\n #cairo_pattern_type_t is used to describe the type of a given pattern.\n\n The type of a pattern is determined by the function used to create\n it. The cairo_pattern_create_rgb() and cairo_pattern_create_rgba()\n functions create SOLID patterns. The remaining\n cairo_pattern_create<!-- --> functions map to pattern types in obvious\n ways.\n\n The pattern type can be queried with cairo_pattern_get_type()\n\n Most #cairo_pattern_t functions can be called with a pattern of any\n type, (though trying to change the extend or filter for a solid\n pattern will have no effect). A notable exception is\n cairo_pattern_add_color_stop_rgb() and\n cairo_pattern_add_color_stop_rgba() which must only be called with\n gradient patterns (either LINEAR or RADIAL). Otherwise the pattern\n will be shutdown and put into an error state.\n\n New entries may be added in future versions.\n\n Since: 1.2"]
pub use self::_cairo_pattern_type as cairo_pattern_type_t;
pub const CAIRO_EXTEND_NONE: _cairo_extend = 0;
pub const CAIRO_EXTEND_REPEAT: _cairo_extend = 1;
pub const CAIRO_EXTEND_REFLECT: _cairo_extend = 2;
pub const CAIRO_EXTEND_PAD: _cairo_extend = 3;
#[doc = " cairo_extend_t:\n @CAIRO_EXTEND_NONE: pixels outside of the source pattern\n   are fully transparent (Since 1.0)\n @CAIRO_EXTEND_REPEAT: the pattern is tiled by repeating (Since 1.0)\n @CAIRO_EXTEND_REFLECT: the pattern is tiled by reflecting\n   at the edges (Since 1.0; but only implemented for surface patterns since 1.6)\n @CAIRO_EXTEND_PAD: pixels outside of the pattern copy\n   the closest pixel from the source (Since 1.2; but only\n   implemented for surface patterns since 1.6)\n\n #cairo_extend_t is used to describe how pattern color/alpha will be\n determined for areas \"outside\" the pattern's natural area, (for\n example, outside the surface bounds or outside the gradient\n geometry).\n\n Mesh patterns are not affected by the extend mode.\n\n The default extend mode is %CAIRO_EXTEND_NONE for surface patterns\n and %CAIRO_EXTEND_PAD for gradient patterns.\n\n New entries may be added in future versions.\n\n Since: 1.0"]
pub type _cairo_extend = u32;
#[doc = " cairo_extend_t:\n @CAIRO_EXTEND_NONE: pixels outside of the source pattern\n   are fully transparent (Since 1.0)\n @CAIRO_EXTEND_REPEAT: the pattern is tiled by repeating (Since 1.0)\n @CAIRO_EXTEND_REFLECT: the pattern is tiled by reflecting\n   at the edges (Since 1.0; but only implemented for surface patterns since 1.6)\n @CAIRO_EXTEND_PAD: pixels outside of the pattern copy\n   the closest pixel from the source (Since 1.2; but only\n   implemented for surface patterns since 1.6)\n\n #cairo_extend_t is used to describe how pattern color/alpha will be\n determined for areas \"outside\" the pattern's natural area, (for\n example, outside the surface bounds or outside the gradient\n geometry).\n\n Mesh patterns are not affected by the extend mode.\n\n The default extend mode is %CAIRO_EXTEND_NONE for surface patterns\n and %CAIRO_EXTEND_PAD for gradient patterns.\n\n New entries may be added in future versions.\n\n Since: 1.0"]
pub use self::_cairo_extend as cairo_extend_t;
pub const CAIRO_FILTER_FAST: _cairo_filter = 0;
pub const CAIRO_FILTER_GOOD: _cairo_filter = 1;
pub const CAIRO_FILTER_BEST: _cairo_filter = 2;
pub const CAIRO_FILTER_NEAREST: _cairo_filter = 3;
pub const CAIRO_FILTER_BILINEAR: _cairo_filter = 4;
pub const CAIRO_FILTER_GAUSSIAN: _cairo_filter = 5;
#[doc = " cairo_filter_t:\n @CAIRO_FILTER_FAST: A high-performance filter, with quality similar\n     to %CAIRO_FILTER_NEAREST (Since 1.0)\n @CAIRO_FILTER_GOOD: A reasonable-performance filter, with quality\n     similar to %CAIRO_FILTER_BILINEAR (Since 1.0)\n @CAIRO_FILTER_BEST: The highest-quality available, performance may\n     not be suitable for interactive use. (Since 1.0)\n @CAIRO_FILTER_NEAREST: Nearest-neighbor filtering (Since 1.0)\n @CAIRO_FILTER_BILINEAR: Linear interpolation in two dimensions (Since 1.0)\n @CAIRO_FILTER_GAUSSIAN: This filter value is currently\n     unimplemented, and should not be used in current code. (Since 1.0)\n\n #cairo_filter_t is used to indicate what filtering should be\n applied when reading pixel values from patterns. See\n cairo_pattern_set_filter() for indicating the desired filter to be\n used with a particular pattern.\n\n Since: 1.0"]
pub type _cairo_filter = u32;
#[doc = " cairo_filter_t:\n @CAIRO_FILTER_FAST: A high-performance filter, with quality similar\n     to %CAIRO_FILTER_NEAREST (Since 1.0)\n @CAIRO_FILTER_GOOD: A reasonable-performance filter, with quality\n     similar to %CAIRO_FILTER_BILINEAR (Since 1.0)\n @CAIRO_FILTER_BEST: The highest-quality available, performance may\n     not be suitable for interactive use. (Since 1.0)\n @CAIRO_FILTER_NEAREST: Nearest-neighbor filtering (Since 1.0)\n @CAIRO_FILTER_BILINEAR: Linear interpolation in two dimensions (Since 1.0)\n @CAIRO_FILTER_GAUSSIAN: This filter value is currently\n     unimplemented, and should not be used in current code. (Since 1.0)\n\n #cairo_filter_t is used to indicate what filtering should be\n applied when reading pixel values from patterns. See\n cairo_pattern_set_filter() for indicating the desired filter to be\n used with a particular pattern.\n\n Since: 1.0"]
pub use self::_cairo_filter as cairo_filter_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _cairo_region {
    _unused: [u8; 0],
}
#[doc = " cairo_region_t:\n\n A #cairo_region_t represents a set of integer-aligned rectangles.\n\n It allows set-theoretical operations like cairo_region_union() and\n cairo_region_intersect() to be performed on them.\n\n Memory management of #cairo_region_t is done with\n cairo_region_reference() and cairo_region_destroy().\n\n Since: 1.10"]
pub type cairo_region_t = _cairo_region;
pub const CAIRO_REGION_OVERLAP_IN: _cairo_region_overlap = 0;
pub const CAIRO_REGION_OVERLAP_OUT: _cairo_region_overlap = 1;
pub const CAIRO_REGION_OVERLAP_PART: _cairo_region_overlap = 2;
#[doc = " cairo_region_overlap_t:\n @CAIRO_REGION_OVERLAP_IN: The contents are entirely inside the region. (Since 1.10)\n @CAIRO_REGION_OVERLAP_OUT: The contents are entirely outside the region. (Since 1.10)\n @CAIRO_REGION_OVERLAP_PART: The contents are partially inside and\n     partially outside the region. (Since 1.10)\n\n Used as the return value for cairo_region_contains_rectangle().\n\n Since: 1.10"]
pub type _cairo_region_overlap = u32;
#[doc = " cairo_region_overlap_t:\n @CAIRO_REGION_OVERLAP_IN: The contents are entirely inside the region. (Since 1.10)\n @CAIRO_REGION_OVERLAP_OUT: The contents are entirely outside the region. (Since 1.10)\n @CAIRO_REGION_OVERLAP_PART: The contents are partially inside and\n     partially outside the region. (Since 1.10)\n\n Used as the return value for cairo_region_contains_rectangle().\n\n Since: 1.10"]
pub use self::_cairo_region_overlap as cairo_region_overlap_t;
#[link(name = "cairo")]
unsafe extern "C" {
    pub fn cairo_version() -> ::core::ffi::c_int;
    pub fn cairo_version_string() -> *const ::core::ffi::c_char;
    pub fn cairo_pattern_set_dither(pattern: *mut cairo_pattern_t, dither: cairo_dither_t);
    pub fn cairo_pattern_get_dither(pattern: *mut cairo_pattern_t) -> cairo_dither_t;
    pub fn cairo_create(target: *mut cairo_surface_t) -> *mut cairo_t;
    pub fn cairo_reference(cr: *mut cairo_t) -> *mut cairo_t;
    pub fn cairo_destroy(cr: *mut cairo_t);
    pub fn cairo_get_reference_count(cr: *mut cairo_t) -> ::core::ffi::c_uint;
    pub fn cairo_get_user_data(
        cr: *mut cairo_t, key: *const cairo_user_data_key_t,
    ) -> *mut ::core::ffi::c_void;
    pub fn cairo_set_user_data(
        cr: *mut cairo_t, key: *const cairo_user_data_key_t, user_data: *mut ::core::ffi::c_void,
        destroy: cairo_destroy_func_t,
    ) -> cairo_status_t;
    pub fn cairo_save(cr: *mut cairo_t);
    pub fn cairo_restore(cr: *mut cairo_t);
    pub fn cairo_push_group(cr: *mut cairo_t);
    pub fn cairo_push_group_with_content(cr: *mut cairo_t, content: cairo_content_t);
    pub fn cairo_pop_group(cr: *mut cairo_t) -> *mut cairo_pattern_t;
    pub fn cairo_pop_group_to_source(cr: *mut cairo_t);
    pub fn cairo_set_operator(cr: *mut cairo_t, op: cairo_operator_t);
    pub fn cairo_set_source(cr: *mut cairo_t, source: *mut cairo_pattern_t);
    pub fn cairo_set_source_rgb(cr: *mut cairo_t, red: f64, green: f64, blue: f64);
    pub fn cairo_set_source_rgba(cr: *mut cairo_t, red: f64, green: f64, blue: f64, alpha: f64);
    pub fn cairo_set_source_surface(
        cr: *mut cairo_t, surface: *mut cairo_surface_t, x: f64, y: f64,
    );
    pub fn cairo_set_tolerance(cr: *mut cairo_t, tolerance: f64);
    pub fn cairo_set_antialias(cr: *mut cairo_t, antialias: cairo_antialias_t);
    pub fn cairo_set_fill_rule(cr: *mut cairo_t, fill_rule: cairo_fill_rule_t);
    pub fn cairo_set_line_width(cr: *mut cairo_t, width: f64);
    pub fn cairo_set_hairline(cr: *mut cairo_t, set_hairline: cairo_bool_t);
    pub fn cairo_set_line_cap(cr: *mut cairo_t, line_cap: cairo_line_cap_t);
    pub fn cairo_set_line_join(cr: *mut cairo_t, line_join: cairo_line_join_t);
    pub fn cairo_set_dash(
        cr: *mut cairo_t, dashes: *const f64, num_dashes: ::core::ffi::c_int, offset: f64,
    );
    pub fn cairo_set_miter_limit(cr: *mut cairo_t, limit: f64);
    pub fn cairo_translate(cr: *mut cairo_t, tx: f64, ty: f64);
    pub fn cairo_scale(cr: *mut cairo_t, sx: f64, sy: f64);
    pub fn cairo_rotate(cr: *mut cairo_t, angle: f64);
    pub fn cairo_transform(cr: *mut cairo_t, matrix: *const cairo_matrix_t);
    pub fn cairo_set_matrix(cr: *mut cairo_t, matrix: *const cairo_matrix_t);
    pub fn cairo_identity_matrix(cr: *mut cairo_t);
    pub fn cairo_user_to_device(cr: *mut cairo_t, x: *mut f64, y: *mut f64);
    pub fn cairo_user_to_device_distance(cr: *mut cairo_t, dx: *mut f64, dy: *mut f64);
    pub fn cairo_device_to_user(cr: *mut cairo_t, x: *mut f64, y: *mut f64);
    pub fn cairo_device_to_user_distance(cr: *mut cairo_t, dx: *mut f64, dy: *mut f64);
    pub fn cairo_new_path(cr: *mut cairo_t);
    pub fn cairo_move_to(cr: *mut cairo_t, x: f64, y: f64);
    pub fn cairo_new_sub_path(cr: *mut cairo_t);
    pub fn cairo_line_to(cr: *mut cairo_t, x: f64, y: f64);
    pub fn cairo_curve_to(cr: *mut cairo_t, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64);
    pub fn cairo_arc(cr: *mut cairo_t, xc: f64, yc: f64, radius: f64, angle1: f64, angle2: f64);
    pub fn cairo_arc_negative(
        cr: *mut cairo_t, xc: f64, yc: f64, radius: f64, angle1: f64, angle2: f64,
    );
    pub fn cairo_rel_move_to(cr: *mut cairo_t, dx: f64, dy: f64);
    pub fn cairo_rel_line_to(cr: *mut cairo_t, dx: f64, dy: f64);
    pub fn cairo_rel_curve_to(
        cr: *mut cairo_t, dx1: f64, dy1: f64, dx2: f64, dy2: f64, dx3: f64, dy3: f64,
    );
    pub fn cairo_rectangle(cr: *mut cairo_t, x: f64, y: f64, width: f64, height: f64);
    pub fn cairo_close_path(cr: *mut cairo_t);
    pub fn cairo_path_extents(
        cr: *mut cairo_t, x1: *mut f64, y1: *mut f64, x2: *mut f64, y2: *mut f64,
    );
    pub fn cairo_paint(cr: *mut cairo_t);
    pub fn cairo_paint_with_alpha(cr: *mut cairo_t, alpha: f64);
    pub fn cairo_mask(cr: *mut cairo_t, pattern: *mut cairo_pattern_t);
    pub fn cairo_mask_surface(
        cr: *mut cairo_t, surface: *mut cairo_surface_t, surface_x: f64, surface_y: f64,
    );
    pub fn cairo_stroke(cr: *mut cairo_t);
    pub fn cairo_stroke_preserve(cr: *mut cairo_t);
    pub fn cairo_fill(cr: *mut cairo_t);
    pub fn cairo_fill_preserve(cr: *mut cairo_t);
    pub fn cairo_copy_page(cr: *mut cairo_t);
    pub fn cairo_show_page(cr: *mut cairo_t);
    pub fn cairo_in_stroke(cr: *mut cairo_t, x: f64, y: f64) -> cairo_bool_t;
    pub fn cairo_in_fill(cr: *mut cairo_t, x: f64, y: f64) -> cairo_bool_t;
    pub fn cairo_in_clip(cr: *mut cairo_t, x: f64, y: f64) -> cairo_bool_t;
    pub fn cairo_stroke_extents(
        cr: *mut cairo_t, x1: *mut f64, y1: *mut f64, x2: *mut f64, y2: *mut f64,
    );
    pub fn cairo_fill_extents(
        cr: *mut cairo_t, x1: *mut f64, y1: *mut f64, x2: *mut f64, y2: *mut f64,
    );
    pub fn cairo_reset_clip(cr: *mut cairo_t);
    pub fn cairo_clip(cr: *mut cairo_t);
    pub fn cairo_clip_preserve(cr: *mut cairo_t);
    pub fn cairo_clip_extents(
        cr: *mut cairo_t, x1: *mut f64, y1: *mut f64, x2: *mut f64, y2: *mut f64,
    );
    pub fn cairo_copy_clip_rectangle_list(cr: *mut cairo_t) -> *mut cairo_rectangle_list_t;
    pub fn cairo_rectangle_list_destroy(rectangle_list: *mut cairo_rectangle_list_t);
    pub fn cairo_tag_begin(
        cr: *mut cairo_t, tag_name: *const ::core::ffi::c_char,
        attributes: *const ::core::ffi::c_char,
    );
    pub fn cairo_tag_end(cr: *mut cairo_t, tag_name: *const ::core::ffi::c_char);
    pub fn cairo_glyph_allocate(num_glyphs: ::core::ffi::c_int) -> *mut cairo_glyph_t;
    pub fn cairo_glyph_free(glyphs: *mut cairo_glyph_t);
    pub fn cairo_text_cluster_allocate(
        num_clusters: ::core::ffi::c_int,
    ) -> *mut cairo_text_cluster_t;
    pub fn cairo_text_cluster_free(clusters: *mut cairo_text_cluster_t);
    pub fn cairo_font_options_create() -> *mut cairo_font_options_t;
    pub fn cairo_font_options_copy(
        original: *const cairo_font_options_t,
    ) -> *mut cairo_font_options_t;
    pub fn cairo_font_options_destroy(options: *mut cairo_font_options_t);
    pub fn cairo_font_options_status(options: *mut cairo_font_options_t) -> cairo_status_t;
    pub fn cairo_font_options_merge(
        options: *mut cairo_font_options_t, other: *const cairo_font_options_t,
    );
    pub fn cairo_font_options_equal(
        options: *const cairo_font_options_t, other: *const cairo_font_options_t,
    ) -> cairo_bool_t;
    pub fn cairo_font_options_hash(options: *const cairo_font_options_t) -> ::core::ffi::c_ulong;
    pub fn cairo_font_options_set_antialias(
        options: *mut cairo_font_options_t, antialias: cairo_antialias_t,
    );
    pub fn cairo_font_options_get_antialias(
        options: *const cairo_font_options_t,
    ) -> cairo_antialias_t;
    pub fn cairo_font_options_set_subpixel_order(
        options: *mut cairo_font_options_t, subpixel_order: cairo_subpixel_order_t,
    );
    pub fn cairo_font_options_get_subpixel_order(
        options: *const cairo_font_options_t,
    ) -> cairo_subpixel_order_t;
    pub fn cairo_font_options_set_hint_style(
        options: *mut cairo_font_options_t, hint_style: cairo_hint_style_t,
    );
    pub fn cairo_font_options_get_hint_style(
        options: *const cairo_font_options_t,
    ) -> cairo_hint_style_t;
    pub fn cairo_font_options_set_hint_metrics(
        options: *mut cairo_font_options_t, hint_metrics: cairo_hint_metrics_t,
    );
    pub fn cairo_font_options_get_hint_metrics(
        options: *const cairo_font_options_t,
    ) -> cairo_hint_metrics_t;
    pub fn cairo_font_options_get_variations(
        options: *mut cairo_font_options_t,
    ) -> *const ::core::ffi::c_char;
    pub fn cairo_font_options_set_variations(
        options: *mut cairo_font_options_t, variations: *const ::core::ffi::c_char,
    );
    pub fn cairo_font_options_set_color_mode(
        options: *mut cairo_font_options_t, color_mode: cairo_color_mode_t,
    );
    pub fn cairo_font_options_get_color_mode(
        options: *const cairo_font_options_t,
    ) -> cairo_color_mode_t;
    pub fn cairo_font_options_get_color_palette(
        options: *const cairo_font_options_t,
    ) -> ::core::ffi::c_uint;
    pub fn cairo_font_options_set_color_palette(
        options: *mut cairo_font_options_t, palette_index: ::core::ffi::c_uint,
    );
    pub fn cairo_font_options_set_custom_palette_color(
        options: *mut cairo_font_options_t, index: ::core::ffi::c_uint, red: f64, green: f64,
        blue: f64, alpha: f64,
    );
    pub fn cairo_font_options_get_custom_palette_color(
        options: *mut cairo_font_options_t, index: ::core::ffi::c_uint, red: *mut f64,
        green: *mut f64, blue: *mut f64, alpha: *mut f64,
    ) -> cairo_status_t;
    pub fn cairo_select_font_face(
        cr: *mut cairo_t, family: *const ::core::ffi::c_char, slant: cairo_font_slant_t,
        weight: cairo_font_weight_t,
    );
    pub fn cairo_set_font_size(cr: *mut cairo_t, size: f64);
    pub fn cairo_set_font_matrix(cr: *mut cairo_t, matrix: *const cairo_matrix_t);
    pub fn cairo_get_font_matrix(cr: *mut cairo_t, matrix: *mut cairo_matrix_t);
    pub fn cairo_set_font_options(cr: *mut cairo_t, options: *const cairo_font_options_t);
    pub fn cairo_get_font_options(cr: *mut cairo_t, options: *mut cairo_font_options_t);
    pub fn cairo_set_font_face(cr: *mut cairo_t, font_face: *mut cairo_font_face_t);
    pub fn cairo_get_font_face(cr: *mut cairo_t) -> *mut cairo_font_face_t;
    pub fn cairo_set_scaled_font(cr: *mut cairo_t, scaled_font: *const cairo_scaled_font_t);
    pub fn cairo_get_scaled_font(cr: *mut cairo_t) -> *mut cairo_scaled_font_t;
    pub fn cairo_show_text(cr: *mut cairo_t, utf8: *const ::core::ffi::c_char);
    pub fn cairo_show_glyphs(
        cr: *mut cairo_t, glyphs: *const cairo_glyph_t, num_glyphs: ::core::ffi::c_int,
    );
    pub fn cairo_show_text_glyphs(
        cr: *mut cairo_t, utf8: *const ::core::ffi::c_char, utf8_len: ::core::ffi::c_int,
        glyphs: *const cairo_glyph_t, num_glyphs: ::core::ffi::c_int,
        clusters: *const cairo_text_cluster_t, num_clusters: ::core::ffi::c_int,
        cluster_flags: cairo_text_cluster_flags_t,
    );
    pub fn cairo_text_path(cr: *mut cairo_t, utf8: *const ::core::ffi::c_char);
    pub fn cairo_glyph_path(
        cr: *mut cairo_t, glyphs: *const cairo_glyph_t, num_glyphs: ::core::ffi::c_int,
    );
    pub fn cairo_text_extents(
        cr: *mut cairo_t, utf8: *const ::core::ffi::c_char, extents: *mut cairo_text_extents_t,
    );
    pub fn cairo_glyph_extents(
        cr: *mut cairo_t, glyphs: *const cairo_glyph_t, num_glyphs: ::core::ffi::c_int,
        extents: *mut cairo_text_extents_t,
    );
    pub fn cairo_font_extents(cr: *mut cairo_t, extents: *mut cairo_font_extents_t);
    pub fn cairo_font_face_reference(font_face: *mut cairo_font_face_t) -> *mut cairo_font_face_t;
    pub fn cairo_font_face_destroy(font_face: *mut cairo_font_face_t);
    pub fn cairo_font_face_get_reference_count(
        font_face: *mut cairo_font_face_t,
    ) -> ::core::ffi::c_uint;
    pub fn cairo_font_face_status(font_face: *mut cairo_font_face_t) -> cairo_status_t;
    pub fn cairo_font_face_get_type(font_face: *mut cairo_font_face_t) -> cairo_font_type_t;
    pub fn cairo_font_face_get_user_data(
        font_face: *mut cairo_font_face_t, key: *const cairo_user_data_key_t,
    ) -> *mut ::core::ffi::c_void;
    pub fn cairo_font_face_set_user_data(
        font_face: *mut cairo_font_face_t, key: *const cairo_user_data_key_t,
        user_data: *mut ::core::ffi::c_void, destroy: cairo_destroy_func_t,
    ) -> cairo_status_t;
    pub fn cairo_scaled_font_create(
        font_face: *mut cairo_font_face_t, font_matrix: *const cairo_matrix_t,
        ctm: *const cairo_matrix_t, options: *const cairo_font_options_t,
    ) -> *mut cairo_scaled_font_t;
    pub fn cairo_scaled_font_reference(
        scaled_font: *mut cairo_scaled_font_t,
    ) -> *mut cairo_scaled_font_t;
    pub fn cairo_scaled_font_destroy(scaled_font: *mut cairo_scaled_font_t);
    pub fn cairo_scaled_font_get_reference_count(
        scaled_font: *mut cairo_scaled_font_t,
    ) -> ::core::ffi::c_uint;
    pub fn cairo_scaled_font_status(scaled_font: *mut cairo_scaled_font_t) -> cairo_status_t;
    pub fn cairo_scaled_font_get_type(scaled_font: *mut cairo_scaled_font_t) -> cairo_font_type_t;
    pub fn cairo_scaled_font_get_user_data(
        scaled_font: *mut cairo_scaled_font_t, key: *const cairo_user_data_key_t,
    ) -> *mut ::core::ffi::c_void;
    pub fn cairo_scaled_font_set_user_data(
        scaled_font: *mut cairo_scaled_font_t, key: *const cairo_user_data_key_t,
        user_data: *mut ::core::ffi::c_void, destroy: cairo_destroy_func_t,
    ) -> cairo_status_t;
    pub fn cairo_scaled_font_extents(
        scaled_font: *mut cairo_scaled_font_t, extents: *mut cairo_font_extents_t,
    );
    pub fn cairo_scaled_font_text_extents(
        scaled_font: *mut cairo_scaled_font_t, utf8: *const ::core::ffi::c_char,
        extents: *mut cairo_text_extents_t,
    );
    pub fn cairo_scaled_font_glyph_extents(
        scaled_font: *mut cairo_scaled_font_t, glyphs: *const cairo_glyph_t,
        num_glyphs: ::core::ffi::c_int, extents: *mut cairo_text_extents_t,
    );
    pub fn cairo_scaled_font_text_to_glyphs(
        scaled_font: *mut cairo_scaled_font_t, x: f64, y: f64, utf8: *const ::core::ffi::c_char,
        utf8_len: ::core::ffi::c_int, glyphs: *mut *mut cairo_glyph_t,
        num_glyphs: *mut ::core::ffi::c_int, clusters: *mut *mut cairo_text_cluster_t,
        num_clusters: *mut ::core::ffi::c_int, cluster_flags: *mut cairo_text_cluster_flags_t,
    ) -> cairo_status_t;
    pub fn cairo_scaled_font_get_font_face(
        scaled_font: *mut cairo_scaled_font_t,
    ) -> *mut cairo_font_face_t;
    pub fn cairo_scaled_font_get_font_matrix(
        scaled_font: *mut cairo_scaled_font_t, font_matrix: *mut cairo_matrix_t,
    );
    pub fn cairo_scaled_font_get_ctm(
        scaled_font: *mut cairo_scaled_font_t, ctm: *mut cairo_matrix_t,
    );
    pub fn cairo_scaled_font_get_scale_matrix(
        scaled_font: *mut cairo_scaled_font_t, scale_matrix: *mut cairo_matrix_t,
    );
    pub fn cairo_scaled_font_get_font_options(
        scaled_font: *mut cairo_scaled_font_t, options: *mut cairo_font_options_t,
    );
    pub fn cairo_toy_font_face_create(
        family: *const ::core::ffi::c_char, slant: cairo_font_slant_t, weight: cairo_font_weight_t,
    ) -> *mut cairo_font_face_t;
    pub fn cairo_toy_font_face_get_family(
        font_face: *mut cairo_font_face_t,
    ) -> *const ::core::ffi::c_char;
    pub fn cairo_toy_font_face_get_slant(font_face: *mut cairo_font_face_t) -> cairo_font_slant_t;
    pub fn cairo_toy_font_face_get_weight(font_face: *mut cairo_font_face_t)
    -> cairo_font_weight_t;
    pub fn cairo_user_font_face_create() -> *mut cairo_font_face_t;
    pub fn cairo_user_font_face_set_init_func(
        font_face: *mut cairo_font_face_t, init_func: cairo_user_scaled_font_init_func_t,
    );
    pub fn cairo_user_font_face_set_render_glyph_func(
        font_face: *mut cairo_font_face_t,
        render_glyph_func: cairo_user_scaled_font_render_glyph_func_t,
    );
    pub fn cairo_user_font_face_set_render_color_glyph_func(
        font_face: *mut cairo_font_face_t,
        render_glyph_func: cairo_user_scaled_font_render_glyph_func_t,
    );
    pub fn cairo_user_font_face_set_text_to_glyphs_func(
        font_face: *mut cairo_font_face_t,
        text_to_glyphs_func: cairo_user_scaled_font_text_to_glyphs_func_t,
    );
    pub fn cairo_user_font_face_set_unicode_to_glyph_func(
        font_face: *mut cairo_font_face_t,
        unicode_to_glyph_func: cairo_user_scaled_font_unicode_to_glyph_func_t,
    );
    pub fn cairo_user_font_face_get_init_func(
        font_face: *mut cairo_font_face_t,
    ) -> cairo_user_scaled_font_init_func_t;
    pub fn cairo_user_font_face_get_render_glyph_func(
        font_face: *mut cairo_font_face_t,
    ) -> cairo_user_scaled_font_render_glyph_func_t;
    pub fn cairo_user_font_face_get_render_color_glyph_func(
        font_face: *mut cairo_font_face_t,
    ) -> cairo_user_scaled_font_render_glyph_func_t;
    pub fn cairo_user_font_face_get_text_to_glyphs_func(
        font_face: *mut cairo_font_face_t,
    ) -> cairo_user_scaled_font_text_to_glyphs_func_t;
    pub fn cairo_user_font_face_get_unicode_to_glyph_func(
        font_face: *mut cairo_font_face_t,
    ) -> cairo_user_scaled_font_unicode_to_glyph_func_t;
    pub fn cairo_user_scaled_font_get_foreground_marker(
        scaled_font: *mut cairo_scaled_font_t,
    ) -> *mut cairo_pattern_t;
    pub fn cairo_user_scaled_font_get_foreground_source(
        scaled_font: *mut cairo_scaled_font_t,
    ) -> *mut cairo_pattern_t;
    pub fn cairo_get_operator(cr: *mut cairo_t) -> cairo_operator_t;
    pub fn cairo_get_source(cr: *mut cairo_t) -> *mut cairo_pattern_t;
    pub fn cairo_get_tolerance(cr: *mut cairo_t) -> f64;
    pub fn cairo_get_antialias(cr: *mut cairo_t) -> cairo_antialias_t;
    pub fn cairo_has_current_point(cr: *mut cairo_t) -> cairo_bool_t;
    pub fn cairo_get_current_point(cr: *mut cairo_t, x: *mut f64, y: *mut f64);
    pub fn cairo_get_fill_rule(cr: *mut cairo_t) -> cairo_fill_rule_t;
    pub fn cairo_get_line_width(cr: *mut cairo_t) -> f64;
    pub fn cairo_get_hairline(cr: *mut cairo_t) -> cairo_bool_t;
    pub fn cairo_get_line_cap(cr: *mut cairo_t) -> cairo_line_cap_t;
    pub fn cairo_get_line_join(cr: *mut cairo_t) -> cairo_line_join_t;
    pub fn cairo_get_miter_limit(cr: *mut cairo_t) -> f64;
    pub fn cairo_get_dash_count(cr: *mut cairo_t) -> ::core::ffi::c_int;
    pub fn cairo_get_dash(cr: *mut cairo_t, dashes: *mut f64, offset: *mut f64);
    pub fn cairo_get_matrix(cr: *mut cairo_t, matrix: *mut cairo_matrix_t);
    pub fn cairo_get_target(cr: *mut cairo_t) -> *mut cairo_surface_t;
    pub fn cairo_get_group_target(cr: *mut cairo_t) -> *mut cairo_surface_t;
    pub fn cairo_copy_path(cr: *mut cairo_t) -> *mut cairo_path_t;
    pub fn cairo_copy_path_flat(cr: *mut cairo_t) -> *mut cairo_path_t;
    pub fn cairo_append_path(cr: *mut cairo_t, path: *const cairo_path_t);
    pub fn cairo_path_destroy(path: *mut cairo_path_t);
    pub fn cairo_status(cr: *mut cairo_t) -> cairo_status_t;
    pub fn cairo_status_to_string(status: cairo_status_t) -> *const ::core::ffi::c_char;
    pub fn cairo_device_reference(device: *mut cairo_device_t) -> *mut cairo_device_t;
    pub fn cairo_device_get_type(device: *mut cairo_device_t) -> cairo_device_type_t;
    pub fn cairo_device_status(device: *mut cairo_device_t) -> cairo_status_t;
    pub fn cairo_device_acquire(device: *mut cairo_device_t) -> cairo_status_t;
    pub fn cairo_device_release(device: *mut cairo_device_t);
    pub fn cairo_device_flush(device: *mut cairo_device_t);
    pub fn cairo_device_finish(device: *mut cairo_device_t);
    pub fn cairo_device_destroy(device: *mut cairo_device_t);
    pub fn cairo_device_get_reference_count(device: *mut cairo_device_t) -> ::core::ffi::c_uint;
    pub fn cairo_device_get_user_data(
        device: *mut cairo_device_t, key: *const cairo_user_data_key_t,
    ) -> *mut ::core::ffi::c_void;
    pub fn cairo_device_set_user_data(
        device: *mut cairo_device_t, key: *const cairo_user_data_key_t,
        user_data: *mut ::core::ffi::c_void, destroy: cairo_destroy_func_t,
    ) -> cairo_status_t;
    pub fn cairo_surface_create_similar(
        other: *mut cairo_surface_t, content: cairo_content_t, width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
    ) -> *mut cairo_surface_t;
    pub fn cairo_surface_create_similar_image(
        other: *mut cairo_surface_t, format: cairo_format_t, width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
    ) -> *mut cairo_surface_t;
    pub fn cairo_surface_map_to_image(
        surface: *mut cairo_surface_t, extents: *const cairo_rectangle_int_t,
    ) -> *mut cairo_surface_t;
    pub fn cairo_surface_unmap_image(surface: *mut cairo_surface_t, image: *mut cairo_surface_t);
    pub fn cairo_surface_create_for_rectangle(
        target: *mut cairo_surface_t, x: f64, y: f64, width: f64, height: f64,
    ) -> *mut cairo_surface_t;
    pub fn cairo_surface_create_observer(
        target: *mut cairo_surface_t, mode: cairo_surface_observer_mode_t,
    ) -> *mut cairo_surface_t;
    pub fn cairo_surface_observer_add_paint_callback(
        abstract_surface: *mut cairo_surface_t, func: cairo_surface_observer_callback_t,
        data: *mut ::core::ffi::c_void,
    ) -> cairo_status_t;
    pub fn cairo_surface_observer_add_mask_callback(
        abstract_surface: *mut cairo_surface_t, func: cairo_surface_observer_callback_t,
        data: *mut ::core::ffi::c_void,
    ) -> cairo_status_t;
    pub fn cairo_surface_observer_add_fill_callback(
        abstract_surface: *mut cairo_surface_t, func: cairo_surface_observer_callback_t,
        data: *mut ::core::ffi::c_void,
    ) -> cairo_status_t;
    pub fn cairo_surface_observer_add_stroke_callback(
        abstract_surface: *mut cairo_surface_t, func: cairo_surface_observer_callback_t,
        data: *mut ::core::ffi::c_void,
    ) -> cairo_status_t;
    pub fn cairo_surface_observer_add_glyphs_callback(
        abstract_surface: *mut cairo_surface_t, func: cairo_surface_observer_callback_t,
        data: *mut ::core::ffi::c_void,
    ) -> cairo_status_t;
    pub fn cairo_surface_observer_add_flush_callback(
        abstract_surface: *mut cairo_surface_t, func: cairo_surface_observer_callback_t,
        data: *mut ::core::ffi::c_void,
    ) -> cairo_status_t;
    pub fn cairo_surface_observer_add_finish_callback(
        abstract_surface: *mut cairo_surface_t, func: cairo_surface_observer_callback_t,
        data: *mut ::core::ffi::c_void,
    ) -> cairo_status_t;
    pub fn cairo_surface_observer_print(
        abstract_surface: *mut cairo_surface_t, write_func: cairo_write_func_t,
        closure: *mut ::core::ffi::c_void,
    ) -> cairo_status_t;
    pub fn cairo_surface_observer_elapsed(abstract_surface: *mut cairo_surface_t) -> f64;
    pub fn cairo_device_observer_print(
        abstract_device: *mut cairo_device_t, write_func: cairo_write_func_t,
        closure: *mut ::core::ffi::c_void,
    ) -> cairo_status_t;
    pub fn cairo_device_observer_elapsed(abstract_device: *mut cairo_device_t) -> f64;
    pub fn cairo_device_observer_paint_elapsed(abstract_device: *mut cairo_device_t) -> f64;
    pub fn cairo_device_observer_mask_elapsed(abstract_device: *mut cairo_device_t) -> f64;
    pub fn cairo_device_observer_fill_elapsed(abstract_device: *mut cairo_device_t) -> f64;
    pub fn cairo_device_observer_stroke_elapsed(abstract_device: *mut cairo_device_t) -> f64;
    pub fn cairo_device_observer_glyphs_elapsed(abstract_device: *mut cairo_device_t) -> f64;
    pub fn cairo_surface_reference(surface: *mut cairo_surface_t) -> *mut cairo_surface_t;
    pub fn cairo_surface_finish(surface: *mut cairo_surface_t);
    pub fn cairo_surface_destroy(surface: *mut cairo_surface_t);
    pub fn cairo_surface_get_device(surface: *mut cairo_surface_t) -> *mut cairo_device_t;
    pub fn cairo_surface_get_reference_count(surface: *mut cairo_surface_t) -> ::core::ffi::c_uint;
    pub fn cairo_surface_status(surface: *mut cairo_surface_t) -> cairo_status_t;
    pub fn cairo_surface_get_type(surface: *mut cairo_surface_t) -> cairo_surface_type_t;
    pub fn cairo_surface_get_content(surface: *mut cairo_surface_t) -> cairo_content_t;
    pub fn cairo_surface_write_to_png(
        surface: *mut cairo_surface_t, filename: *const ::core::ffi::c_char,
    ) -> cairo_status_t;
    pub fn cairo_surface_write_to_png_stream(
        surface: *mut cairo_surface_t, write_func: cairo_write_func_t,
        closure: *mut ::core::ffi::c_void,
    ) -> cairo_status_t;
    pub fn cairo_surface_get_user_data(
        surface: *mut cairo_surface_t, key: *const cairo_user_data_key_t,
    ) -> *mut ::core::ffi::c_void;
    pub fn cairo_surface_set_user_data(
        surface: *mut cairo_surface_t, key: *const cairo_user_data_key_t,
        user_data: *mut ::core::ffi::c_void, destroy: cairo_destroy_func_t,
    ) -> cairo_status_t;
    pub fn cairo_surface_get_mime_data(
        surface: *mut cairo_surface_t, mime_type: *const ::core::ffi::c_char,
        data: *mut *const ::core::ffi::c_uchar, length: *mut ::core::ffi::c_ulong,
    );
    pub fn cairo_surface_set_mime_data(
        surface: *mut cairo_surface_t, mime_type: *const ::core::ffi::c_char,
        data: *const ::core::ffi::c_uchar, length: ::core::ffi::c_ulong,
        destroy: cairo_destroy_func_t, closure: *mut ::core::ffi::c_void,
    ) -> cairo_status_t;
    pub fn cairo_surface_supports_mime_type(
        surface: *mut cairo_surface_t, mime_type: *const ::core::ffi::c_char,
    ) -> cairo_bool_t;
    pub fn cairo_surface_get_font_options(
        surface: *mut cairo_surface_t, options: *mut cairo_font_options_t,
    );
    pub fn cairo_surface_flush(surface: *mut cairo_surface_t);
    pub fn cairo_surface_mark_dirty(surface: *mut cairo_surface_t);
    pub fn cairo_surface_mark_dirty_rectangle(
        surface: *mut cairo_surface_t, x: ::core::ffi::c_int, y: ::core::ffi::c_int,
        width: ::core::ffi::c_int, height: ::core::ffi::c_int,
    );
    pub fn cairo_surface_set_device_scale(
        surface: *mut cairo_surface_t, x_scale: f64, y_scale: f64,
    );
    pub fn cairo_surface_get_device_scale(
        surface: *mut cairo_surface_t, x_scale: *mut f64, y_scale: *mut f64,
    );
    pub fn cairo_surface_set_device_offset(
        surface: *mut cairo_surface_t, x_offset: f64, y_offset: f64,
    );
    pub fn cairo_surface_get_device_offset(
        surface: *mut cairo_surface_t, x_offset: *mut f64, y_offset: *mut f64,
    );
    pub fn cairo_surface_set_fallback_resolution(
        surface: *mut cairo_surface_t, x_pixels_per_inch: f64, y_pixels_per_inch: f64,
    );
    pub fn cairo_surface_get_fallback_resolution(
        surface: *mut cairo_surface_t, x_pixels_per_inch: *mut f64, y_pixels_per_inch: *mut f64,
    );
    pub fn cairo_surface_copy_page(surface: *mut cairo_surface_t);
    pub fn cairo_surface_show_page(surface: *mut cairo_surface_t);
    pub fn cairo_surface_has_show_text_glyphs(surface: *mut cairo_surface_t) -> cairo_bool_t;
    pub fn cairo_image_surface_create(
        format: cairo_format_t, width: ::core::ffi::c_int, height: ::core::ffi::c_int,
    ) -> *mut cairo_surface_t;
    pub fn cairo_format_stride_for_width(
        format: cairo_format_t, width: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn cairo_image_surface_create_for_data(
        data: *mut ::core::ffi::c_uchar, format: cairo_format_t, width: ::core::ffi::c_int,
        height: ::core::ffi::c_int, stride: ::core::ffi::c_int,
    ) -> *mut cairo_surface_t;
    pub fn cairo_image_surface_get_data(surface: *mut cairo_surface_t)
    -> *mut ::core::ffi::c_uchar;
    pub fn cairo_image_surface_get_format(surface: *mut cairo_surface_t) -> cairo_format_t;
    pub fn cairo_image_surface_get_width(surface: *mut cairo_surface_t) -> ::core::ffi::c_int;
    pub fn cairo_image_surface_get_height(surface: *mut cairo_surface_t) -> ::core::ffi::c_int;
    pub fn cairo_image_surface_get_stride(surface: *mut cairo_surface_t) -> ::core::ffi::c_int;
    pub fn cairo_image_surface_create_from_png(
        filename: *const ::core::ffi::c_char,
    ) -> *mut cairo_surface_t;
    pub fn cairo_image_surface_create_from_png_stream(
        read_func: cairo_read_func_t, closure: *mut ::core::ffi::c_void,
    ) -> *mut cairo_surface_t;
    pub fn cairo_recording_surface_create(
        content: cairo_content_t, extents: *const cairo_rectangle_t,
    ) -> *mut cairo_surface_t;
    pub fn cairo_recording_surface_ink_extents(
        surface: *mut cairo_surface_t, x0: *mut f64, y0: *mut f64, width: *mut f64,
        height: *mut f64,
    );
    pub fn cairo_recording_surface_get_extents(
        surface: *mut cairo_surface_t, extents: *mut cairo_rectangle_t,
    ) -> cairo_bool_t;
    pub fn cairo_pattern_create_raster_source(
        user_data: *mut ::core::ffi::c_void, content: cairo_content_t, width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
    ) -> *mut cairo_pattern_t;
    pub fn cairo_raster_source_pattern_set_callback_data(
        pattern: *mut cairo_pattern_t, data: *mut ::core::ffi::c_void,
    );
    pub fn cairo_raster_source_pattern_get_callback_data(
        pattern: *mut cairo_pattern_t,
    ) -> *mut ::core::ffi::c_void;
    pub fn cairo_raster_source_pattern_set_acquire(
        pattern: *mut cairo_pattern_t, acquire: cairo_raster_source_acquire_func_t,
        release: cairo_raster_source_release_func_t,
    );
    pub fn cairo_raster_source_pattern_get_acquire(
        pattern: *mut cairo_pattern_t, acquire: *mut cairo_raster_source_acquire_func_t,
        release: *mut cairo_raster_source_release_func_t,
    );
    pub fn cairo_raster_source_pattern_set_snapshot(
        pattern: *mut cairo_pattern_t, snapshot: cairo_raster_source_snapshot_func_t,
    );
    pub fn cairo_raster_source_pattern_get_snapshot(
        pattern: *mut cairo_pattern_t,
    ) -> cairo_raster_source_snapshot_func_t;
    pub fn cairo_raster_source_pattern_set_copy(
        pattern: *mut cairo_pattern_t, copy: cairo_raster_source_copy_func_t,
    );
    pub fn cairo_raster_source_pattern_get_copy(
        pattern: *mut cairo_pattern_t,
    ) -> cairo_raster_source_copy_func_t;
    pub fn cairo_raster_source_pattern_set_finish(
        pattern: *mut cairo_pattern_t, finish: cairo_raster_source_finish_func_t,
    );
    pub fn cairo_raster_source_pattern_get_finish(
        pattern: *mut cairo_pattern_t,
    ) -> cairo_raster_source_finish_func_t;
    pub fn cairo_pattern_create_rgb(red: f64, green: f64, blue: f64) -> *mut cairo_pattern_t;
    pub fn cairo_pattern_create_rgba(
        red: f64, green: f64, blue: f64, alpha: f64,
    ) -> *mut cairo_pattern_t;
    pub fn cairo_pattern_create_for_surface(surface: *mut cairo_surface_t) -> *mut cairo_pattern_t;
    pub fn cairo_pattern_create_linear(x0: f64, y0: f64, x1: f64, y1: f64) -> *mut cairo_pattern_t;
    pub fn cairo_pattern_create_radial(
        cx0: f64, cy0: f64, radius0: f64, cx1: f64, cy1: f64, radius1: f64,
    ) -> *mut cairo_pattern_t;
    pub fn cairo_pattern_create_mesh() -> *mut cairo_pattern_t;
    pub fn cairo_pattern_reference(pattern: *mut cairo_pattern_t) -> *mut cairo_pattern_t;
    pub fn cairo_pattern_destroy(pattern: *mut cairo_pattern_t);
    pub fn cairo_pattern_get_reference_count(pattern: *mut cairo_pattern_t) -> ::core::ffi::c_uint;
    pub fn cairo_pattern_status(pattern: *mut cairo_pattern_t) -> cairo_status_t;
    pub fn cairo_pattern_get_user_data(
        pattern: *mut cairo_pattern_t, key: *const cairo_user_data_key_t,
    ) -> *mut ::core::ffi::c_void;
    pub fn cairo_pattern_set_user_data(
        pattern: *mut cairo_pattern_t, key: *const cairo_user_data_key_t,
        user_data: *mut ::core::ffi::c_void, destroy: cairo_destroy_func_t,
    ) -> cairo_status_t;
    pub fn cairo_pattern_get_type(pattern: *mut cairo_pattern_t) -> cairo_pattern_type_t;
    pub fn cairo_pattern_add_color_stop_rgb(
        pattern: *mut cairo_pattern_t, offset: f64, red: f64, green: f64, blue: f64,
    );
    pub fn cairo_pattern_add_color_stop_rgba(
        pattern: *mut cairo_pattern_t, offset: f64, red: f64, green: f64, blue: f64, alpha: f64,
    );
    pub fn cairo_mesh_pattern_begin_patch(pattern: *mut cairo_pattern_t);
    pub fn cairo_mesh_pattern_end_patch(pattern: *mut cairo_pattern_t);
    pub fn cairo_mesh_pattern_curve_to(
        pattern: *mut cairo_pattern_t, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64,
    );
    pub fn cairo_mesh_pattern_line_to(pattern: *mut cairo_pattern_t, x: f64, y: f64);
    pub fn cairo_mesh_pattern_move_to(pattern: *mut cairo_pattern_t, x: f64, y: f64);
    pub fn cairo_mesh_pattern_set_control_point(
        pattern: *mut cairo_pattern_t, point_num: ::core::ffi::c_uint, x: f64, y: f64,
    );
    pub fn cairo_mesh_pattern_set_corner_color_rgb(
        pattern: *mut cairo_pattern_t, corner_num: ::core::ffi::c_uint, red: f64, green: f64,
        blue: f64,
    );
    pub fn cairo_mesh_pattern_set_corner_color_rgba(
        pattern: *mut cairo_pattern_t, corner_num: ::core::ffi::c_uint, red: f64, green: f64,
        blue: f64, alpha: f64,
    );
    pub fn cairo_pattern_set_matrix(pattern: *mut cairo_pattern_t, matrix: *const cairo_matrix_t);
    pub fn cairo_pattern_get_matrix(pattern: *mut cairo_pattern_t, matrix: *mut cairo_matrix_t);
    pub fn cairo_pattern_set_extend(pattern: *mut cairo_pattern_t, extend: cairo_extend_t);
    pub fn cairo_pattern_get_extend(pattern: *mut cairo_pattern_t) -> cairo_extend_t;
    pub fn cairo_pattern_set_filter(pattern: *mut cairo_pattern_t, filter: cairo_filter_t);
    pub fn cairo_pattern_get_filter(pattern: *mut cairo_pattern_t) -> cairo_filter_t;
    pub fn cairo_pattern_get_rgba(
        pattern: *mut cairo_pattern_t, red: *mut f64, green: *mut f64, blue: *mut f64,
        alpha: *mut f64,
    ) -> cairo_status_t;
    pub fn cairo_pattern_get_surface(
        pattern: *mut cairo_pattern_t, surface: *mut *mut cairo_surface_t,
    ) -> cairo_status_t;
    pub fn cairo_pattern_get_color_stop_rgba(
        pattern: *mut cairo_pattern_t, index: ::core::ffi::c_int, offset: *mut f64, red: *mut f64,
        green: *mut f64, blue: *mut f64, alpha: *mut f64,
    ) -> cairo_status_t;
    pub fn cairo_pattern_get_color_stop_count(
        pattern: *mut cairo_pattern_t, count: *mut ::core::ffi::c_int,
    ) -> cairo_status_t;
    pub fn cairo_pattern_get_linear_points(
        pattern: *mut cairo_pattern_t, x0: *mut f64, y0: *mut f64, x1: *mut f64, y1: *mut f64,
    ) -> cairo_status_t;
    pub fn cairo_pattern_get_radial_circles(
        pattern: *mut cairo_pattern_t, x0: *mut f64, y0: *mut f64, r0: *mut f64, x1: *mut f64,
        y1: *mut f64, r1: *mut f64,
    ) -> cairo_status_t;
    pub fn cairo_mesh_pattern_get_patch_count(
        pattern: *mut cairo_pattern_t, count: *mut ::core::ffi::c_uint,
    ) -> cairo_status_t;
    pub fn cairo_mesh_pattern_get_path(
        pattern: *mut cairo_pattern_t, patch_num: ::core::ffi::c_uint,
    ) -> *mut cairo_path_t;
    pub fn cairo_mesh_pattern_get_corner_color_rgba(
        pattern: *mut cairo_pattern_t, patch_num: ::core::ffi::c_uint,
        corner_num: ::core::ffi::c_uint, red: *mut f64, green: *mut f64, blue: *mut f64,
        alpha: *mut f64,
    ) -> cairo_status_t;
    pub fn cairo_mesh_pattern_get_control_point(
        pattern: *mut cairo_pattern_t, patch_num: ::core::ffi::c_uint,
        point_num: ::core::ffi::c_uint, x: *mut f64, y: *mut f64,
    ) -> cairo_status_t;
    pub fn cairo_matrix_init(
        matrix: *mut cairo_matrix_t, xx: f64, yx: f64, xy: f64, yy: f64, x0: f64, y0: f64,
    );
    pub fn cairo_matrix_init_identity(matrix: *mut cairo_matrix_t);
    pub fn cairo_matrix_init_translate(matrix: *mut cairo_matrix_t, tx: f64, ty: f64);
    pub fn cairo_matrix_init_scale(matrix: *mut cairo_matrix_t, sx: f64, sy: f64);
    pub fn cairo_matrix_init_rotate(matrix: *mut cairo_matrix_t, radians: f64);
    pub fn cairo_matrix_translate(matrix: *mut cairo_matrix_t, tx: f64, ty: f64);
    pub fn cairo_matrix_scale(matrix: *mut cairo_matrix_t, sx: f64, sy: f64);
    pub fn cairo_matrix_rotate(matrix: *mut cairo_matrix_t, radians: f64);
    pub fn cairo_matrix_invert(matrix: *mut cairo_matrix_t) -> cairo_status_t;
    pub fn cairo_matrix_multiply(
        result: *mut cairo_matrix_t, a: *const cairo_matrix_t, b: *const cairo_matrix_t,
    );
    pub fn cairo_matrix_transform_distance(
        matrix: *const cairo_matrix_t, dx: *mut f64, dy: *mut f64,
    );
    pub fn cairo_matrix_transform_point(matrix: *const cairo_matrix_t, x: *mut f64, y: *mut f64);
    pub fn cairo_region_create() -> *mut cairo_region_t;
    pub fn cairo_region_create_rectangle(
        rectangle: *const cairo_rectangle_int_t,
    ) -> *mut cairo_region_t;
    pub fn cairo_region_create_rectangles(
        rects: *const cairo_rectangle_int_t, count: ::core::ffi::c_int,
    ) -> *mut cairo_region_t;
    pub fn cairo_region_copy(original: *const cairo_region_t) -> *mut cairo_region_t;
    pub fn cairo_region_reference(region: *mut cairo_region_t) -> *mut cairo_region_t;
    pub fn cairo_region_destroy(region: *mut cairo_region_t);
    pub fn cairo_region_equal(a: *const cairo_region_t, b: *const cairo_region_t) -> cairo_bool_t;
    pub fn cairo_region_status(region: *const cairo_region_t) -> cairo_status_t;
    pub fn cairo_region_get_extents(
        region: *const cairo_region_t, extents: *mut cairo_rectangle_int_t,
    );
    pub fn cairo_region_num_rectangles(region: *const cairo_region_t) -> ::core::ffi::c_int;
    pub fn cairo_region_get_rectangle(
        region: *const cairo_region_t, nth: ::core::ffi::c_int,
        rectangle: *mut cairo_rectangle_int_t,
    );
    pub fn cairo_region_is_empty(region: *const cairo_region_t) -> cairo_bool_t;
    pub fn cairo_region_contains_rectangle(
        region: *const cairo_region_t, rectangle: *const cairo_rectangle_int_t,
    ) -> cairo_region_overlap_t;
    pub fn cairo_region_contains_point(
        region: *const cairo_region_t, x: ::core::ffi::c_int, y: ::core::ffi::c_int,
    ) -> cairo_bool_t;
    pub fn cairo_region_translate(
        region: *mut cairo_region_t, dx: ::core::ffi::c_int, dy: ::core::ffi::c_int,
    );
    pub fn cairo_region_subtract(
        dst: *mut cairo_region_t, other: *const cairo_region_t,
    ) -> cairo_status_t;
    pub fn cairo_region_subtract_rectangle(
        dst: *mut cairo_region_t, rectangle: *const cairo_rectangle_int_t,
    ) -> cairo_status_t;
    pub fn cairo_region_intersect(
        dst: *mut cairo_region_t, other: *const cairo_region_t,
    ) -> cairo_status_t;
    pub fn cairo_region_intersect_rectangle(
        dst: *mut cairo_region_t, rectangle: *const cairo_rectangle_int_t,
    ) -> cairo_status_t;
    pub fn cairo_region_union(
        dst: *mut cairo_region_t, other: *const cairo_region_t,
    ) -> cairo_status_t;
    pub fn cairo_region_union_rectangle(
        dst: *mut cairo_region_t, rectangle: *const cairo_rectangle_int_t,
    ) -> cairo_status_t;
    pub fn cairo_region_xor(
        dst: *mut cairo_region_t, other: *const cairo_region_t,
    ) -> cairo_status_t;
    pub fn cairo_region_xor_rectangle(
        dst: *mut cairo_region_t, rectangle: *const cairo_rectangle_int_t,
    ) -> cairo_status_t;
    pub fn cairo_debug_reset_static_data();
}
