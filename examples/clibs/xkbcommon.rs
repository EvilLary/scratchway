#![allow(non_upper_case_globals, non_snake_case, non_camel_case_types, unsafe_op_in_unsafe_fn, unused)]
use libc::FILE;

pub type __uint64_t = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __gnuc_va_list = __builtin_va_list;
pub type va_list = __gnuc_va_list;
#[doc = " @struct xkb_context\n Opaque top level library context object.\n\n The context contains various general library data and state, like\n logging level and include paths.\n\n Objects are created in a specific context, and multiple contexts may\n coexist simultaneously.  Objects from different contexts are completely\n separated and do not share any memory or state."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xkb_context {
    _unused: [u8; 0],
}
#[doc = " @struct xkb_keymap\n Opaque compiled keymap object.\n\n The keymap object holds all of the static keyboard information obtained\n from compiling XKB files.\n\n A keymap is immutable after it is created (besides reference counts, etc.);\n if you need to change it, you must create a new one."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xkb_keymap {
    _unused: [u8; 0],
}
#[doc = " @struct xkb_state\n Opaque keyboard state object.\n\n State objects contain the active state of a keyboard (or keyboards), such\n as the currently effective layout and the active modifiers.  It acts as a\n simple state machine, wherein key presses and releases are the input, and\n key symbols (keysyms) are the output."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xkb_state {
    _unused: [u8; 0],
}
#[doc = " A number used to represent a physical key on a keyboard.\n\n A standard PC-compatible keyboard might have 102 keys.  An appropriate\n keymap would assign each of them a keycode, by which the user should\n refer to the key throughout the library.\n\n Historically, the X11 protocol, and consequentially the XKB protocol,\n assign only 8 bits for keycodes.  This limits the number of different\n keys that can be used simultaneously in a single keymap to 256\n (disregarding other limitations).  This library does not share this limit;\n keycodes beyond 255 (*extended* keycodes) are not treated specially.\n Keymaps and applications which are compatible with X11 should not use\n these keycodes.\n\n The values of specific keycodes are determined by the keymap and the\n underlying input system.  For example, with an X11-compatible keymap\n and Linux evdev scan codes (see `linux/input.h`), a fixed offset is used:\n\n The keymap defines a canonical name for each key, plus possible aliases.\n Historically, the XKB protocol restricts these names to at most 4 (ASCII)\n characters, but this library does not share this limit.\n\n @code\n xkb_keycode_t keycode_A = KEY_A + 8;\n @endcode\n\n @sa xkb_keycode_is_legal_ext() xkb_keycode_is_legal_x11()"]
pub type xkb_keycode_t = u32;
#[doc = " A number used to represent the symbols generated from a key on a keyboard.\n\n A key, represented by a keycode, may generate different symbols according\n to keyboard state.  For example, on a QWERTY keyboard, pressing the key\n labled \\<A\\> generates the symbol ‘a’.  If the Shift key is held, it\n generates the symbol ‘A’.  If a different layout is used, say Greek,\n it generates the symbol ‘α’.  And so on.\n\n Each such symbol is represented by a *keysym* (short for “key symbol”).\n Note that keysyms are somewhat more general, in that they can also represent\n some “function”, such as “Left” or “Right” for the arrow keys.  For more\n information, see: Appendix A [“KEYSYM Encoding”][encoding] of the X Window\n System Protocol.\n\n Specifically named keysyms can be found in the\n xkbcommon/xkbcommon-keysyms.h header file.  Their name does not include\n the `XKB_KEY_` prefix.\n\n Besides those, any Unicode/ISO&nbsp;10646 character in the range U+0100 to\n U+10FFFF can be represented by a keysym value in the range 0x01000100 to\n 0x0110FFFF.  The name of Unicode keysyms is `U<codepoint>`, e.g. `UA1B2`.\n\n The name of other unnamed keysyms is the hexadecimal representation of\n their value, e.g. `0xabcd1234`.\n\n Keysym names are case-sensitive.\n\n @note **Encoding:** Keysyms are 32-bit integers with the 3 most significant\n bits always set to zero.  Thus valid keysyms are in the range\n `0 .. 0x1fffffff` = @ref XKB_KEYSYM_MAX.\n See: Appendix A [“KEYSYM Encoding”][encoding] of the X Window System Protocol.\n\n [encoding]: https://www.x.org/releases/current/doc/xproto/x11protocol.html#keysym_encoding\n\n @ingroup keysyms\n @sa `::XKB_KEYSYM_MAX`"]
pub type xkb_keysym_t = u32;
#[doc = " Index of a keyboard layout.\n\n The layout index is a state component which determines which <em>keyboard\n layout</em> is active.  These may be different alphabets, different key\n arrangements, etc.\n\n Layout indices are consecutive.  The first layout has index 0.\n\n Each layout is not required to have a name, and the names are not\n guaranteed to be unique (though they are usually provided and unique).\n Therefore, it is not safe to use the name as a unique identifier for a\n layout.  Layout names are case-sensitive.\n\n Layout names are specified in the layout’s definition, for example\n “English (US)”.  These are different from the (conventionally) short names\n which are used to locate the layout, for example `us` or `us(intl)`.  These\n names are not present in a compiled keymap.\n\n If the user selects layouts from a list generated from the XKB registry\n (using libxkbregistry or directly), and this metadata is needed later on, it\n is recommended to store it along with the keymap.\n\n Layouts are also called *groups* by XKB.\n\n @sa xkb_keymap::xkb_keymap_num_layouts()\n @sa xkb_keymap::xkb_keymap_num_layouts_for_key()"]
pub type xkb_layout_index_t = u32;
#[doc = " Index of a shift level.\n\n Any key, in any layout, can have several <em>shift levels</em>.  Each\n shift level can assign different keysyms to the key.  The shift level\n to use is chosen according to the current keyboard state; for example,\n if no keys are pressed, the first level may be used; if the Left Shift\n key is pressed, the second; if Num Lock is pressed, the third; and\n many such combinations are possible (see `xkb_mod_index_t`).\n\n Level indices are consecutive.  The first level has index 0."]
pub type xkb_level_index_t = u32;
#[doc = " Index of a modifier.\n\n A @e modifier is a state component which changes the way keys are\n interpreted.  A keymap defines a set of modifiers, such as Alt, Shift,\n Num Lock or Meta, and specifies which keys may @e activate which\n modifiers (in a many-to-many relationship, i.e. a key can activate\n several modifiers, and a modifier may be activated by several keys.\n Different keymaps do this differently).\n\n When retrieving the keysyms for a key, the active modifier set is\n consulted; this determines the correct shift level to use within the\n currently active layout (see `xkb_level_index_t`).\n\n Modifier indices are consecutive.  The first modifier has index 0.\n\n Each modifier must have a name, and the names are unique.  Therefore, it\n is safe to use the name as a unique identifier for a modifier.  The names\n of some common modifiers are provided in the `xkbcommon/xkbcommon-names.h`\n header file.  Modifier names are case-sensitive.\n\n @sa xkb_keymap_num_mods()"]
pub type xkb_mod_index_t = u32;
#[doc = " A mask of modifier indices."]
pub type xkb_mod_mask_t = u32;
#[doc = " Index of a keyboard LED.\n\n LEDs are logical objects which may be @e active or @e inactive.  They\n typically correspond to the lights on the keyboard. Their state is\n determined by the current keyboard state.\n\n LED indices are non-consecutive.  The first LED has index 0.\n\n Each LED must have a name, and the names are unique. Therefore,\n it is safe to use the name as a unique identifier for a LED.  The names\n of some common LEDs are provided in the `xkbcommon/xkbcommon-names.h`\n header file.  LED names are case-sensitive.\n\n @warning A given keymap may specify an exact index for a given LED.\n Therefore, LED indexing is not necessarily sequential, as opposed to\n modifiers and layouts.  This means that when iterating over the LEDs\n in a keymap using e.g. xkb_keymap::xkb_keymap_num_leds(), some indices might\n be invalid.\n Given such an index, functions like xkb_keymap::xkb_keymap_led_get_name()\n will return `NULL`, and `xkb_state::xkb_state_led_index_is_active()` will\n return -1.\n\n LEDs are also called *indicators* by XKB.\n\n @sa `xkb_keymap::xkb_keymap_num_leds()`"]
pub type xkb_led_index_t = u32;
#[doc = " @struct xkb_rmlvo_builder\n Opaque [RMLVO] configuration object.\n\n It denotes the configuration values by which a user picks a keymap.\n\n @see [Introduction to RMLVO][RMLVO]\n @see @ref rules-api \"\"\n @since 1.11.0\n\n [RMLVO]: @ref RMLVO-intro"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xkb_rmlvo_builder {
    _unused: [u8; 0],
}
pub const XKB_RMLVO_BUILDER_NO_FLAGS: xkb_rmlvo_builder_flags = 0;
pub type xkb_rmlvo_builder_flags = u32;
#[doc = " Names to compile a keymap with, also known as [RMLVO].\n\n The names are the common configuration values by which a user picks\n a keymap.\n\n If the entire struct is `NULL`, then each field is taken to be `NULL`.\n You should prefer passing `NULL` instead of choosing your own defaults.\n\n @see [Introduction to RMLVO][RMLVO]\n @see @ref rules-api \"\"\n\n [RMLVO]: @ref RMLVO-intro"]
#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Copy, Clone)]
pub struct xkb_rule_names {
    pub _bindgen_opaque_blob: [u64; 5usize],
}
#[doc = " Keymap components, also known as [KcCGST].\n\n The components are the result of the [RMLVO] resolution.\n\n @see [Introduction to RMLVO][RMLVO]\n @see [Introduction to KcCGST][KcCGST]\n @see @ref rules-api \"\"\n\n [RMLVO]: @ref RMLVO-intro\n [KcCGST]: @ref KcCGST-intro"]
#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Copy, Clone)]
pub struct xkb_component_names {
    pub _bindgen_opaque_blob: [u64; 5usize],
}
#[doc = " Do not apply any flags."]
pub const XKB_KEYSYM_NO_FLAGS: xkb_keysym_flags = 0;
#[doc = " Find keysym by case-insensitive search."]
pub const XKB_KEYSYM_CASE_INSENSITIVE: xkb_keysym_flags = 1;
#[doc = " Flags for xkb_keysym_from_name()."]
pub type xkb_keysym_flags = u32;
#[doc = " Do not apply any context flags."]
pub const XKB_CONTEXT_NO_FLAGS: xkb_context_flags = 0;
#[doc = " Create this context with an empty include path.\n\n This may be useful e.g.:\n - to have full control over the included paths;\n - for clients that do not need to access the XKB directories, e.g.\n   if only retrieving keymap from the Wayland or X server. It avoids\n   potential issues with directory access permissions."]
pub const XKB_CONTEXT_NO_DEFAULT_INCLUDES: xkb_context_flags = 1;
#[doc = " Don’t take RMLVO names from the environment.\n\n @since 0.3.0"]
pub const XKB_CONTEXT_NO_ENVIRONMENT_NAMES: xkb_context_flags = 2;
#[doc = " Disable the use of secure_getenv for this context, so that privileged\n processes can use environment variables. Client uses at their own risk.\n\n @since 1.5.0"]
pub const XKB_CONTEXT_NO_SECURE_GETENV: xkb_context_flags = 4;
#[doc = " Flags for context creation."]
pub type xkb_context_flags = u32;
#[doc = "< Log critical internal errors only."]
pub const XKB_LOG_LEVEL_CRITICAL: xkb_log_level = 10;
#[doc = "< Log all errors."]
pub const XKB_LOG_LEVEL_ERROR: xkb_log_level = 20;
#[doc = "< Log warnings and errors."]
pub const XKB_LOG_LEVEL_WARNING: xkb_log_level = 30;
#[doc = "< Log information, warnings, and errors."]
pub const XKB_LOG_LEVEL_INFO: xkb_log_level = 40;
#[doc = "< Log everything."]
pub const XKB_LOG_LEVEL_DEBUG: xkb_log_level = 50;
#[doc = " Specifies a logging level."]
pub type xkb_log_level = u32;
#[doc = " Do not apply any flags."]
pub const XKB_KEYMAP_COMPILE_NO_FLAGS: xkb_keymap_compile_flags = 0;
#[doc = " Flags for keymap compilation."]
pub type xkb_keymap_compile_flags = u32;
#[doc = " The classic XKB text format, as generated by `xkbcomp -xkb`.\n\n @important This format should *always* be used when *serializing* a\n keymap for **X11**.\n\n @important For the **Wayland** <code>[xkb_v1]</code> format, it is\n advised to use this format as well for serializing, in order to ensure\n maximum compatibility for interchange.\n\n [xkb_v1]: https://wayland.freedesktop.org/docs/html/apa.html#protocol-spec-wl_keyboard-enum-keymap_format"]
pub const XKB_KEYMAP_FORMAT_TEXT_V1: xkb_keymap_format = 1;
#[doc = " Xkbcommon extensions of the classic XKB text format, **incompatible with\n X11**.\n\n @important Do *not* use when *serializing* a keymap for **X11**\n (incompatible).\n\n @important Considered *experimental* when *serializing* for **Wayland**:\n at the time of writing (July 2025), there is only one XKB keymap format\n <code>[xkb_v1]</code> in Wayland and no Wayland API for keymap format\n *negotiation*, so the clients may not be able to parse the keymap if it\n uses v2-specific features. Therefore a compositor may *parse* keymaps\n using `::XKB_KEYMAP_FORMAT_TEXT_V2` but it should serialize them using\n `::XKB_KEYMAP_FORMAT_TEXT_V1` and rely on the automatic *fallback*\n mechanisms.\n\n @since 1.11.0\n\n [xkb_v1]: https://wayland.freedesktop.org/docs/html/apa.html#protocol-spec-wl_keyboard-enum-keymap_format"]
pub const XKB_KEYMAP_FORMAT_TEXT_V2: xkb_keymap_format = 2;
#[doc = " The possible keymap formats.\n\n See @ref keymap-text-format-v1-v2 \"\" for the complete description of the\n formats and @ref keymap-support \"\" for detailed differences between the\n formats.\n\n @remark A keymap can be parsed in one format and serialized in another,\n thanks to automatic fallback mechanisms.\n\n <table>\n <caption>\n Keymap format to use depending on the target protocol\n </caption>\n <thead>\n <tr>\n <th colspan=\"2\">Protocol</th>\n <th colspan=\"2\">libxkbcommon keymap format</th>\n </tr>\n <tr>\n <th>Name</th>\n <th>Keymap format</th>\n <th>Parsing</th>\n <th>Serialization</th>\n </tr>\n </thead>\n <tbody>\n <tr>\n <th>X11</th>\n <td>XKB</td>\n <td>\n `::XKB_KEYMAP_FORMAT_TEXT_V1`\n </td>\n <td>\n *Always* use `::XKB_KEYMAP_FORMAT_TEXT_V1`, since the other formats are\n incompatible.\n </td>\n </tr>\n <tr>\n <th>Wayland</th>\n <td><code>[xkb_v1]</code></td>\n <td>\n <dl>\n <dt>Wayland compositors<dt>\n <dd>\n The format depends on the keyboard layout database (usually [xkeyboard-config]).\n Note that since v2 is a superset of v1, compositors are encouraged to use\n `::XKB_KEYMAP_FORMAT_TEXT_V2` whenever possible.\n </dd>\n <dt>Client apps</dt>\n <dd>\n Clients should use `::XKB_KEYMAP_FORMAT_TEXT_V1` to parse the keymap sent\n by a Wayland compositor, at least until `::XKB_KEYMAP_FORMAT_TEXT_V2`\n stabilizes.\n </dd>\n </td>\n <td>\n At the time of writing (July 2025), the Wayland <code>[xkb_v1]</code> keymap\n format is only defined as “libxkbcommon compatible”. In theory it enables\n flexibility, but the set of supported features varies depending on the\n libxkbcommon version and libxkbcommon keymap format used. Unfortunately there\n is currently no Wayland API for keymap format *negotiation*.\n\n Therefore the **recommended** serialization format is\n `::XKB_KEYMAP_FORMAT_TEXT_V1`, in order to ensure maximum compatibility for\n interchange.\n\n Serializing using `::XKB_KEYMAP_FORMAT_TEXT_V2` should be considered\n **experimental**, as some clients may fail to parse the resulting string.\n </td>\n </tr>\n </tbody>\n </table>\n\n [xkb_v1]: https://wayland.freedesktop.org/docs/html/apa.html#protocol-spec-wl_keyboard-enum-keymap_format\n [xkeyboard-config]: https://gitlab.freedesktop.org/xkeyboard-config/xkeyboard-config"]
pub type xkb_keymap_format = u32;
#[doc = " Do not apply any flags."]
pub const XKB_KEYMAP_SERIALIZE_NO_FLAGS: xkb_keymap_serialize_flags = 0;
#[doc = " Enable pretty-printing"]
pub const XKB_KEYMAP_SERIALIZE_PRETTY: xkb_keymap_serialize_flags = 1;
#[doc = " Do not drop unused bits (key types, compatibility entries)"]
pub const XKB_KEYMAP_SERIALIZE_KEEP_UNUSED: xkb_keymap_serialize_flags = 2;
#[doc = " Flags to control keymap serialization.\n\n @since 1.12.0"]
pub type xkb_keymap_serialize_flags = u32;
#[doc = " The iterator used by `xkb_keymap_key_for_each()`.\n\n @sa xkb_keymap_key_for_each\n @memberof xkb_keymap\n @since 0.3.1"]
pub type xkb_keymap_key_iter_t = u64;
#[doc = "< The key was released."]
pub const XKB_KEY_UP: xkb_key_direction = 0;
#[doc = "< The key was pressed."]
pub const XKB_KEY_DOWN: xkb_key_direction = 1;
#[doc = " Specifies the direction of the key (press / release)."]
pub type xkb_key_direction = u32;
#[doc = " Depressed modifiers, i.e. a key is physically holding them."]
pub const XKB_STATE_MODS_DEPRESSED: xkb_state_component = 1;
#[doc = " Latched modifiers, i.e. will be unset after the next non-modifier\n  key press."]
pub const XKB_STATE_MODS_LATCHED: xkb_state_component = 2;
#[doc = " Locked modifiers, i.e. will be unset after the key provoking the\n  lock has been pressed again."]
pub const XKB_STATE_MODS_LOCKED: xkb_state_component = 4;
#[doc = " Effective modifiers, i.e. currently active and affect key\n  processing (derived from the other state components).\n  Use this unless you explicitly care how the state came about."]
pub const XKB_STATE_MODS_EFFECTIVE: xkb_state_component = 8;
#[doc = " Depressed layout, i.e. a key is physically holding it."]
pub const XKB_STATE_LAYOUT_DEPRESSED: xkb_state_component = 16;
#[doc = " Latched layout, i.e. will be unset after the next non-modifier\n  key press."]
pub const XKB_STATE_LAYOUT_LATCHED: xkb_state_component = 32;
#[doc = " Locked layout, i.e. will be unset after the key provoking the lock\n  has been pressed again."]
pub const XKB_STATE_LAYOUT_LOCKED: xkb_state_component = 64;
#[doc = " Effective layout, i.e. currently active and affects key processing\n  (derived from the other state components).\n  Use this unless you explicitly care how the state came about."]
pub const XKB_STATE_LAYOUT_EFFECTIVE: xkb_state_component = 128;
#[doc = " LEDs (derived from the other state components)."]
pub const XKB_STATE_LEDS: xkb_state_component = 256;
#[doc = " Modifier and layout types for state objects.  This enum is bitmaskable,\n e.g. (`::XKB_STATE_MODS_DEPRESSED` | `::XKB_STATE_MODS_LATCHED`) is valid to\n exclude locked modifiers.\n\n In XKB, the `DEPRESSED` components are also known as *base*."]
pub type xkb_state_component = u32;
#[doc = " Returns true if any of the modifiers are active."]
pub const XKB_STATE_MATCH_ANY: xkb_state_match = 1;
#[doc = " Returns true if all of the modifiers are active."]
pub const XKB_STATE_MATCH_ALL: xkb_state_match = 2;
#[doc = " Makes matching non-exclusive, i.e. will not return false if a\n  modifier not specified in the arguments is active."]
pub const XKB_STATE_MATCH_NON_EXCLUSIVE: xkb_state_match = 65536;
#[doc = " Match flags for `xkb_state::xkb_state_mod_indices_are_active()` and\n `xkb_state::xkb_state_mod_names_are_active()`, specifying the conditions for a\n successful match.  `::XKB_STATE_MATCH_NON_EXCLUSIVE` is bitmaskable with\n the other modes."]
pub type xkb_state_match = u32;
#[doc = " This is the mode defined in the XKB specification and used by libX11.\n\n A modifier is consumed if and only if it *may affect* key translation.\n\n For example, if `Control+Alt+<Backspace>` produces some assigned keysym,\n then when pressing just `<Backspace>`, `Control` and `Alt` are consumed,\n even though they are not active, since if they *were* active they would\n have affected key translation."]
pub const XKB_CONSUMED_MODE_XKB: xkb_consumed_mode = 0;
#[doc = " This is the mode used by the GTK+ toolkit.\n\n The mode consists of the following two independent heuristics:\n\n - The currently active set of modifiers, excluding modifiers which do\n   not affect the key (as described for @ref XKB_CONSUMED_MODE_XKB), are\n   considered consumed, if the keysyms produced when all of them are\n   active are different from the keysyms produced when no modifiers are\n   active.\n\n - A single modifier is considered consumed if the keysyms produced for\n   the key when it is the only active modifier are different from the\n   keysyms produced when no modifiers are active."]
pub const XKB_CONSUMED_MODE_GTK: xkb_consumed_mode = 1;
#[doc = " Consumed modifiers mode.\n\n There are several possible methods for deciding which modifiers are\n consumed and which are not, each applicable for different systems or\n situations. The mode selects the method to use.\n\n Keep in mind that in all methods, the keymap may decide to *preserve*\n a modifier, meaning it is not reported as consumed even if it would\n have otherwise."]
pub type xkb_consumed_mode = u32;
pub type __builtin_va_list = [__va_list_tag; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __va_list_tag {
    pub gp_offset:         ::core::ffi::c_uint,
    pub fp_offset:         ::core::ffi::c_uint,
    pub overflow_arg_area: *mut ::core::ffi::c_void,
    pub reg_save_area:     *mut ::core::ffi::c_void,
}
#[link(name = "xkbcommon")]
unsafe extern "C" {
    #[doc = " Create a new [RMLVO] builder.\n\n @param context The context in which to create the builder.\n @param rules   The ruleset.\n If `NULL` or the empty string `\"\"`, a default value is used.\n If the `XKB_DEFAULT_RULES` environment variable is set, it is used\n as the default.  Otherwise the system default is used.\n @param model   The keyboard model.\n If `NULL` or the empty string `\"\"`, a default value is used.\n If the `XKB_DEFAULT_MODEL` environment variable is set, it is used\n as the default.  Otherwise the system default is used.\n @param flags   Optional flags for the builder, or 0.\n\n @returns A `xkb_rmlvo_builder`, or `NULL` if the compilation failed.\n\n @see `xkb_rule_names` for a detailed description of `rules` and `model`.\n @since 1.11.0\n @memberof xkb_rmlvo_builder\n\n [RMLVO]: @ref RMLVO-intro"]
    #[link_name = "\u{1}xkb_rmlvo_builder_new"]
    pub fn xkb_rmlvo_builder_new(
        context: *mut xkb_context, rules: *const ::core::ffi::c_char,
        model: *const ::core::ffi::c_char, flags: xkb_rmlvo_builder_flags,
    ) -> *mut xkb_rmlvo_builder;
    #[doc = " Append a layout to the given [RMLVO] builder.\n\n @param rmlvo         The builder to modify.\n @param layout        The name of the layout.\n @param variant       The name of the layout variant, or `NULL` to\n select the default variant.\n @param options       An array of options to apply only to this layout, or\n `NULL` if there is no such options.\n @param options_len   The length of @p options.\n\n @note The options are only effectual if the corresponding ruleset has the\n proper rules to handle them as *layout-specific* options.\n @note See `rxkb_option_is_layout_specific()` to query whether an option\n supports the layout-specific feature.\n\n @returns `true` if the call succeeded, otherwise `false`.\n\n @since 1.11.0\n @memberof xkb_rmlvo_builder\n\n [RMLVO]: @ref RMLVO-intro"]
    #[link_name = "\u{1}xkb_rmlvo_builder_append_layout"]
    pub fn xkb_rmlvo_builder_append_layout(
        rmlvo: *mut xkb_rmlvo_builder, layout: *const ::core::ffi::c_char,
        variant: *const ::core::ffi::c_char, options: *const *const ::core::ffi::c_char,
        options_len: usize,
    ) -> bool;
    #[doc = " Append an option to the given [RMLVO] builder.\n\n @param rmlvo   The builder to modify.\n @param option  The name of the option.\n\n @returns `true` if the call succeeded, otherwise `false`.\n\n @since 1.11.0\n @memberof xkb_rmlvo_builder\n\n [RMLVO]: @ref RMLVO-intro"]
    #[link_name = "\u{1}xkb_rmlvo_builder_append_option"]
    pub fn xkb_rmlvo_builder_append_option(
        rmlvo: *mut xkb_rmlvo_builder, option: *const ::core::ffi::c_char,
    ) -> bool;
    #[doc = " Take a new reference on a [RMLVO] builder.\n\n @param rmlvo The builder to reference.\n\n @returns The passed in builder.\n\n @since 1.11.0\n @memberof xkb_rmlvo_builder\n\n [RMLVO]: @ref RMLVO-intro"]
    #[link_name = "\u{1}xkb_rmlvo_builder_ref"]
    pub fn xkb_rmlvo_builder_ref(rmlvo: *mut xkb_rmlvo_builder) -> *mut xkb_rmlvo_builder;
    #[doc = " Release a reference on a [RMLVO] builder, and possibly free it.\n\n @param rmlvo The builder.  If it is `NULL`, this function does nothing.\n\n @since 1.11.0\n @memberof xkb_rmlvo_builder\n\n [RMLVO]: @ref RMLVO-intro"]
    #[link_name = "\u{1}xkb_rmlvo_builder_unref"]
    pub fn xkb_rmlvo_builder_unref(rmlvo: *mut xkb_rmlvo_builder);
    #[doc = " Resolve [RMLVO] names to [KcCGST] components.\n\n This function is used primarily for *debugging*. See\n `xkb_keymap::xkb_keymap_new_from_names2()` for creating keymaps from\n [RMLVO] names.\n\n @param[in]  context    The context in which to resolve the names.\n @param[in]  rmlvo_in   The [RMLVO] names to use.\n @param[out] rmlvo_out  The [RMLVO] names actually used after resolving\n missing values.\n @param[out] components_out The [KcCGST] components resulting of the [RMLVO]\n resolution.\n\n @c rmlvo_out and @c components can be omitted by using `NULL`, but not both.\n\n If @c components is not `NULL`, it is filled with dynamically-allocated\n strings that should be freed by the caller.\n\n @returns `true` if the [RMLVO] names could be resolved, `false` otherwise.\n\n @see [Introduction to RMLVO][RMLVO]\n @see [Introduction to KcCGST][KcCGST]\n @see xkb_rule_names\n @see xkb_component_names\n @see xkb_keymap::xkb_keymap_new_from_names2()\n\n @since 1.9.0\n @memberof xkb_component_names\n\n [RMLVO]: @ref RMLVO-intro\n [KcCGST]: @ref KcCGST-intro"]
    #[link_name = "\u{1}xkb_components_names_from_rules"]
    pub fn xkb_components_names_from_rules(
        context: *mut xkb_context, rmlvo_in: *const xkb_rule_names, rmlvo_out: *mut xkb_rule_names,
        components_out: *mut xkb_component_names,
    ) -> bool;
    #[doc = " Get the name of a keysym.\n\n For a description of how keysyms are named, see @ref xkb_keysym_t.\n\n @param[in]  keysym The keysym.\n @param[out] buffer A string buffer to write the name into.\n @param[in]  size   Size of the buffer.\n\n @warning If the buffer passed is too small, the string is truncated\n (though still `NULL`-terminated); a size of at least 64 bytes is recommended.\n\n @returns The number of bytes in the name, excluding the `NULL` byte. If\n the keysym is invalid, returns -1.\n\n You may check if truncation has occurred by comparing the return value\n with the length of buffer, similarly to the `snprintf(3)` function.\n\n @sa `xkb_keysym_t`"]
    #[link_name = "\u{1}xkb_keysym_get_name"]
    pub fn xkb_keysym_get_name(
        keysym: xkb_keysym_t, buffer: *mut ::core::ffi::c_char, size: usize,
    ) -> ::core::ffi::c_int;
    #[doc = " Get a keysym from its name.\n\n @param name The name of a keysym. See remarks in `xkb_keysym_get_name()`;\n this function will accept any name returned by that function.\n @param flags A set of flags controlling how the search is done. If\n invalid flags are passed, this will fail with `XKB_KEY_NoSymbol`.\n\n If you use the `::XKB_KEYSYM_CASE_INSENSITIVE` flag and two keysym names\n differ only by case, then the lower-case keysym name is returned.  For\n instance, for KEY_a and KEY_A, this function would return KEY_a for the\n case-insensitive search.  If this functionality is needed, it is\n recommended to first call this function without this flag; and if that\n fails, only then to try with this flag, while possibly warning the user\n he had misspelled the name, and might get wrong results.\n\n Case folding is done according to the C locale; the current locale is not\n consulted.\n\n @returns The keysym. If the name is invalid, returns `XKB_KEY_NoSymbol`.\n\n @sa xkb_keysym_t\n @since 1.9.0: Enable support for C0 and C1 control characters in the Unicode\n notation."]
    #[link_name = "\u{1}xkb_keysym_from_name"]
    pub fn xkb_keysym_from_name(
        name: *const ::core::ffi::c_char, flags: xkb_keysym_flags,
    ) -> xkb_keysym_t;
    #[doc = " Get the Unicode/UTF-8 representation of a keysym.\n\n @param[in]  keysym The keysym.\n @param[out] buffer A buffer to write the UTF-8 string into.\n @param[in]  size   The size of buffer.  Must be at least 5.\n\n @returns The number of bytes written to the buffer (including the\n terminating byte).  If the keysym does not have a Unicode\n representation, returns 0.  If the buffer is too small, returns -1.\n\n This function does not perform any @ref keysym-transformations.\n Therefore, prefer to use `xkb_state::xkb_state_key_get_utf8()` if possible.\n\n @sa `xkb_state::xkb_state_key_get_utf8()`"]
    #[link_name = "\u{1}xkb_keysym_to_utf8"]
    pub fn xkb_keysym_to_utf8(
        keysym: xkb_keysym_t, buffer: *mut ::core::ffi::c_char, size: usize,
    ) -> ::core::ffi::c_int;
    #[doc = " Get the Unicode/UTF-32 representation of a keysym.\n\n @returns The Unicode/UTF-32 representation of keysym, which is also\n compatible with UCS-4.  If the keysym does not have a Unicode\n representation, returns 0.\n\n This function does not perform any @ref keysym-transformations.\n Therefore, prefer to use xkb_state_key_get_utf32() if possible.\n\n @sa `xkb_state::xkb_state_key_get_utf32()`"]
    #[link_name = "\u{1}xkb_keysym_to_utf32"]
    pub fn xkb_keysym_to_utf32(keysym: xkb_keysym_t) -> u32;
    #[doc = " Get the keysym corresponding to a Unicode/UTF-32 codepoint.\n\n @returns The keysym corresponding to the specified Unicode\n codepoint, or XKB_KEY_NoSymbol if there is none.\n\n This function is the inverse of @ref xkb_keysym_to_utf32. In cases\n where a single codepoint corresponds to multiple keysyms, returns\n the keysym with the lowest value.\n\n Unicode codepoints which do not have a special (legacy) keysym\n encoding use a direct encoding scheme. These keysyms don’t usually\n have an associated keysym constant (`XKB_KEY_*`).\n\n @sa `xkb_keysym_to_utf32()`\n @since 1.0.0\n @since 1.9.0: Enable support for all noncharacters."]
    #[link_name = "\u{1}xkb_utf32_to_keysym"]
    pub fn xkb_utf32_to_keysym(ucs: u32) -> xkb_keysym_t;
    #[doc = " Convert a keysym to its uppercase form.\n\n If there is no such form, the keysym is returned unchanged.\n\n The conversion rules are the *simple* (i.e. one-to-one) Unicode case\n mappings (with some exceptions, see hereinafter) and do not depend\n on the locale. If you need the special case mappings (i.e. not\n one-to-one or locale-dependent), prefer to work with the Unicode\n representation instead, when possible.\n\n Exceptions to the Unicode mappings:\n\n | Lower keysym | Lower letter | Upper keysym | Upper letter | Comment |\n | ------------ | ------------ | ------------ | ------------ | ------- |\n | `ssharp`     | `U+00DF`: ß  | `U1E9E`      | `U+1E9E`: ẞ  | [Council for German Orthography] |\n\n [Council for German Orthography]: https://www.rechtschreibrat.com/regeln-und-woerterverzeichnis/\n\n @since 0.8.0: Initial implementation, based on `libX11`.\n @since 1.8.0: Use Unicode 16.0 mappings for complete Unicode coverage.\n @since 1.12.0: Update to Unicode 17.0."]
    #[link_name = "\u{1}xkb_keysym_to_upper"]
    pub fn xkb_keysym_to_upper(ks: xkb_keysym_t) -> xkb_keysym_t;
    #[doc = " Convert a keysym to its lowercase form.\n\n If there is no such form, the keysym is returned unchanged.\n\n The conversion rules are the *simple* (i.e. one-to-one) Unicode case\n mappings and do not depend on the locale. If you need the special\n case mappings (i.e. not one-to-one or locale-dependent), prefer to\n work with the Unicode representation instead, when possible.\n\n @since 0.8.0: Initial implementation, based on `libX11`.\n @since 1.8.0: Use Unicode 16.0 mappings for complete Unicode coverage.\n @since 1.12.0: Update to Unicode 17.0."]
    #[link_name = "\u{1}xkb_keysym_to_lower"]
    pub fn xkb_keysym_to_lower(ks: xkb_keysym_t) -> xkb_keysym_t;
    #[doc = " Create a new context.\n\n @param flags Optional flags for the context, or 0.\n\n @returns A new context, or `NULL` on failure.\n\n @memberof xkb_context"]
    #[link_name = "\u{1}xkb_context_new"]
    pub fn xkb_context_new(flags: xkb_context_flags) -> *mut xkb_context;
    #[doc = " Take a new reference on a context.\n\n @returns The passed in context.\n\n @memberof xkb_context"]
    #[link_name = "\u{1}xkb_context_ref"]
    pub fn xkb_context_ref(context: *mut xkb_context) -> *mut xkb_context;
    #[doc = " Release a reference on a context, and possibly free it.\n\n @param context The context.  If it is `NULL`, this function does nothing.\n\n @memberof xkb_context"]
    #[link_name = "\u{1}xkb_context_unref"]
    pub fn xkb_context_unref(context: *mut xkb_context);
    #[doc = " Store custom user data in the context.\n\n This may be useful in conjunction with `xkb_context::xkb_context_set_log_fn()`\n or other callbacks.\n\n @memberof xkb_context"]
    #[link_name = "\u{1}xkb_context_set_user_data"]
    pub fn xkb_context_set_user_data(
        context: *mut xkb_context, user_data: *mut ::core::ffi::c_void,
    );
    #[doc = " Retrieves stored user data from the context.\n\n @returns The stored user data.  If the user data wasn’t set, or the\n passed in context is `NULL`, returns `NULL`.\n\n This may be useful to access private user data from callbacks like a\n custom logging function.\n\n @memberof xkb_context"]
    #[link_name = "\u{1}xkb_context_get_user_data"]
    pub fn xkb_context_get_user_data(context: *mut xkb_context) -> *mut ::core::ffi::c_void;
    #[doc = " Append a new entry to the context’s include path.\n\n @returns 1 on success, or 0 if the include path could not be added or is\n inaccessible.\n\n @memberof xkb_context"]
    #[link_name = "\u{1}xkb_context_include_path_append"]
    pub fn xkb_context_include_path_append(
        context: *mut xkb_context, path: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    #[doc = " Append the default include paths to the context’s include path.\n\n @returns 1 on success, or 0 if no default include path could be added.\n\n @memberof xkb_context"]
    #[link_name = "\u{1}xkb_context_include_path_append_default"]
    pub fn xkb_context_include_path_append_default(context: *mut xkb_context)
    -> ::core::ffi::c_int;
    #[doc = " Reset the context’s include path to the default.\n\n Removes all entries from the context’s include path, and inserts the\n default paths.\n\n @returns 1 on success, or 0 if the primary include path could not be added.\n\n @memberof xkb_context"]
    #[link_name = "\u{1}xkb_context_include_path_reset_defaults"]
    pub fn xkb_context_include_path_reset_defaults(context: *mut xkb_context)
    -> ::core::ffi::c_int;
    #[doc = " Remove all entries from the context’s include path.\n\n @memberof xkb_context"]
    #[link_name = "\u{1}xkb_context_include_path_clear"]
    pub fn xkb_context_include_path_clear(context: *mut xkb_context);
    #[doc = " Get the number of paths in the context’s include path.\n\n @memberof xkb_context"]
    #[link_name = "\u{1}xkb_context_num_include_paths"]
    pub fn xkb_context_num_include_paths(context: *mut xkb_context) -> ::core::ffi::c_uint;
    #[doc = " Get a specific include path from the context’s include path.\n\n @returns The include path at the specified index.  If the index is\n invalid, returns NULL.\n\n @memberof xkb_context"]
    #[link_name = "\u{1}xkb_context_include_path_get"]
    pub fn xkb_context_include_path_get(
        context: *mut xkb_context, index: ::core::ffi::c_uint,
    ) -> *const ::core::ffi::c_char;
    #[doc = " Set the current logging level.\n\n @param context The context in which to set the logging level.\n @param level   The logging level to use.  Only messages from this level\n and below will be logged.\n\n The default level is `::XKB_LOG_LEVEL_ERROR`.  The environment variable\n `XKB_LOG_LEVEL`, if set in the time the context was created, overrides the\n default value.  It may be specified as a level number or name.\n\n @memberof xkb_context"]
    #[link_name = "\u{1}xkb_context_set_log_level"]
    pub fn xkb_context_set_log_level(context: *mut xkb_context, level: xkb_log_level);
    #[doc = " Get the current logging level.\n\n @memberof xkb_context"]
    #[link_name = "\u{1}xkb_context_get_log_level"]
    pub fn xkb_context_get_log_level(context: *mut xkb_context) -> xkb_log_level;
    #[doc = " Sets the current logging verbosity.\n\n The library can generate a number of warnings which are not helpful to\n ordinary users of the library.  The verbosity may be increased if more\n information is desired (e.g. when developing a new keymap).\n\n The default verbosity is 0.  The environment variable `XKB_LOG_VERBOSITY`,\n if set in the time the context was created, overrides the default value.\n\n @param context   The context in which to use the set verbosity.\n @param verbosity The verbosity to use.  Currently used values are\n 1 to 10, higher values being more verbose.  0 would result in no verbose\n messages being logged.\n\n Most verbose messages are of level `::XKB_LOG_LEVEL_WARNING` or lower.\n\n @memberof xkb_context"]
    #[link_name = "\u{1}xkb_context_set_log_verbosity"]
    pub fn xkb_context_set_log_verbosity(context: *mut xkb_context, verbosity: ::core::ffi::c_int);
    #[doc = " Get the current logging verbosity of the context.\n\n @memberof xkb_context"]
    #[link_name = "\u{1}xkb_context_get_log_verbosity"]
    pub fn xkb_context_get_log_verbosity(context: *mut xkb_context) -> ::core::ffi::c_int;
    #[doc = " Set a custom function to handle logging messages.\n\n @param context The context in which to use the set logging function.\n @param log_fn  The function that will be called for logging messages.\n Passing `NULL` restores the default function, which logs to stderr.\n\n By default, log messages from this library are printed to stderr.  This\n function allows you to replace the default behavior with a custom\n handler.  The handler is only called with messages which match the\n current logging level and verbosity settings for the context.\n level is the logging level of the message.  @a format and @a args are\n the same as in the `vprintf(3)` function.\n\n You may use `xkb_context::xkb_context_set_user_data()` on the context, and\n then call `xkb_context::xkb_context_get_user_data()` from within the logging\n function to provide it with additional private context.\n\n @memberof xkb_context"]
    #[link_name = "\u{1}xkb_context_set_log_fn"]
    pub fn xkb_context_set_log_fn(
        context: *mut xkb_context,
        log_fn: ::core::option::Option<
            unsafe extern "C" fn(
                context: *mut xkb_context,
                level: xkb_log_level,
                format: *const ::core::ffi::c_char,
                args: *mut __va_list_tag,
            ),
        >,
    );
    #[doc = " Create a keymap from a [RMLVO] builder.\n\n The primary keymap entry point: creates a new XKB keymap from a set of\n [RMLVO] \\(Rules + Model + Layouts + Variants + Options) names.\n\n @param rmlvo   The [RMLVO] builder to use.  See `xkb_rmlvo_builder`.\n @param format  The text format of the keymap file to compile.\n @param flags   Optional flags for the keymap, or 0.\n\n @returns A keymap compiled according to the [RMLVO] names, or `NULL` if\n the compilation failed.\n\n @since 1.11.0\n @sa `xkb_keymap_new_from_names2()`\n @sa `xkb_rmlvo_builder`\n @memberof xkb_keymap\n\n [RMLVO]: @ref RMLVO-intro"]
    #[link_name = "\u{1}xkb_keymap_new_from_rmlvo"]
    pub fn xkb_keymap_new_from_rmlvo(
        rmlvo: *const xkb_rmlvo_builder, format: xkb_keymap_format, flags: xkb_keymap_compile_flags,
    ) -> *mut xkb_keymap;
    #[doc = " Create a keymap from [RMLVO] names.\n\n Same as `xkb_keymap_new_from_names2()`, but with the keymap format fixed to:\n `::XKB_KEYMAP_FORMAT_TEXT_V2`.\n\n @deprecated Use `xkb_keymap_new_from_names2()` instead.\n @since 1.11.0: Deprecated\n @since 1.11.0: Use internally `::XKB_KEYMAP_FORMAT_TEXT_V2` instead of\n `::XKB_KEYMAP_FORMAT_TEXT_V1`\n @sa `xkb_keymap_new_from_names2()`\n @sa `xkb_rule_names`\n @sa `xkb_keymap_new_from_rmlvo()`\n @memberof xkb_keymap\n\n [RMLVO]: @ref RMLVO-intro"]
    #[link_name = "\u{1}xkb_keymap_new_from_names"]
    pub fn xkb_keymap_new_from_names(
        context: *mut xkb_context, names: *const xkb_rule_names, flags: xkb_keymap_compile_flags,
    ) -> *mut xkb_keymap;
    #[doc = " Create a keymap from [RMLVO] names.\n\n The primary keymap entry point: creates a new XKB keymap from a set of\n [RMLVO] \\(Rules + Model + Layouts + Variants + Options) names.\n\n @param context The context in which to create the keymap.\n @param names   The [RMLVO] names to use.  See xkb_rule_names.\n @param format  The text format of the keymap file to compile.\n @param flags   Optional flags for the keymap, or 0.\n\n @returns A keymap compiled according to the [RMLVO] names, or `NULL` if\n the compilation failed.\n\n @sa `xkb_rule_names`\n @sa `xkb_keymap_new_from_rmlvo()`\n @memberof xkb_keymap\n @since 1.11.0\n\n [RMLVO]: @ref RMLVO-intro"]
    #[link_name = "\u{1}xkb_keymap_new_from_names2"]
    pub fn xkb_keymap_new_from_names2(
        context: *mut xkb_context, names: *const xkb_rule_names, format: xkb_keymap_format,
        flags: xkb_keymap_compile_flags,
    ) -> *mut xkb_keymap;
    #[doc = " Create a keymap from a keymap file.\n\n @param context The context in which to create the keymap.\n @param file    The keymap file to compile.\n @param format  The text format of the keymap file to compile.\n @param flags   Optional flags for the keymap, or 0.\n\n @returns A keymap compiled from the given XKB keymap file, or `NULL` if\n the compilation failed.\n\n The file must contain a complete keymap.  For example, in the\n `::XKB_KEYMAP_FORMAT_TEXT_V1` format, this means the file must contain one\n top level `%xkb_keymap` section, which in turn contains other required\n sections.\n\n @memberof xkb_keymap"]
    #[link_name = "\u{1}xkb_keymap_new_from_file"]
    pub fn xkb_keymap_new_from_file(
        context: *mut xkb_context, file: *mut FILE, format: xkb_keymap_format,
        flags: xkb_keymap_compile_flags,
    ) -> *mut xkb_keymap;
    #[doc = " Create a keymap from a keymap string.\n\n This is just like `xkb_keymap_new_from_file()`, but instead of a file, gets\n the keymap as one enormous string.\n\n @see `xkb_keymap_new_from_file()`\n @memberof xkb_keymap"]
    #[link_name = "\u{1}xkb_keymap_new_from_string"]
    pub fn xkb_keymap_new_from_string(
        context: *mut xkb_context, string: *const ::core::ffi::c_char, format: xkb_keymap_format,
        flags: xkb_keymap_compile_flags,
    ) -> *mut xkb_keymap;
    #[doc = " Create a keymap from a memory buffer.\n\n This is just like `xkb_keymap_new_from_string()`, but takes a length argument\n so the input string does not have to be zero-terminated.\n\n @see `xkb_keymap_new_from_string()`\n @memberof xkb_keymap\n @since 0.3.0"]
    #[link_name = "\u{1}xkb_keymap_new_from_buffer"]
    pub fn xkb_keymap_new_from_buffer(
        context: *mut xkb_context, buffer: *const ::core::ffi::c_char, length: usize,
        format: xkb_keymap_format, flags: xkb_keymap_compile_flags,
    ) -> *mut xkb_keymap;
    #[doc = " Take a new reference on a keymap.\n\n @returns The passed in keymap.\n\n @memberof xkb_keymap"]
    #[link_name = "\u{1}xkb_keymap_ref"]
    pub fn xkb_keymap_ref(keymap: *mut xkb_keymap) -> *mut xkb_keymap;
    #[doc = " Release a reference on a keymap, and possibly free it.\n\n @param keymap The keymap.  If it is `NULL`, this function does nothing.\n\n @memberof xkb_keymap"]
    #[link_name = "\u{1}xkb_keymap_unref"]
    pub fn xkb_keymap_unref(keymap: *mut xkb_keymap);
    #[doc = " Get the compiled keymap as a string.\n\n Same as `xkb_keymap::xkb_keymap_get_as_string2()` using\n `::XKB_KEYMAP_SERIALIZE_NO_FLAGS`.\n\n @since 1.12.0: Drop unused types and compatibility entries and do not\n pretty-print.\n\n @sa `xkb_keymap::xkb_keymap_get_as_string2()`\n @memberof xkb_keymap"]
    #[link_name = "\u{1}xkb_keymap_get_as_string"]
    pub fn xkb_keymap_get_as_string(
        keymap: *mut xkb_keymap, format: xkb_keymap_format,
    ) -> *mut ::core::ffi::c_char;
    #[doc = " Get the compiled keymap as a string.\n\n @param keymap The keymap to get as a string.\n @param format The keymap format to use for the string.  You can pass\n in the special value `::XKB_KEYMAP_USE_ORIGINAL_FORMAT` to use the format\n from which the keymap was originally created. When used as an *interchange*\n format such as Wayland <code>[xkb_v1]</code>, the format should be explicit.\n @param flags  Optional flags to control the serialization, or 0.\n\n @returns The keymap as a `NULL`-terminated string, or `NULL` if unsuccessful.\n\n The returned string may be fed back into `xkb_keymap_new_from_string()`\n to get the exact same keymap (possibly in another process, etc.).\n\n The returned string is *dynamically allocated* and should be freed by the\n caller.\n\n @since 1.12.0\n\n @sa `xkb_keymap_get_as_string()`\n @sa `xkb_keymap_new_from_string()`\n @memberof xkb_keymap\n\n [xkb_v1]: https://wayland.freedesktop.org/docs/html/apa.html#protocol-spec-wl_keyboard-enum-keymap_format"]
    #[link_name = "\u{1}xkb_keymap_get_as_string2"]
    pub fn xkb_keymap_get_as_string2(
        keymap: *mut xkb_keymap, format: xkb_keymap_format, flags: xkb_keymap_serialize_flags,
    ) -> *mut ::core::ffi::c_char;
    #[doc = " Get the minimum keycode in the keymap.\n\n @sa xkb_keycode_t\n @memberof xkb_keymap\n @since 0.3.1"]
    #[link_name = "\u{1}xkb_keymap_min_keycode"]
    pub fn xkb_keymap_min_keycode(keymap: *mut xkb_keymap) -> xkb_keycode_t;
    #[doc = " Get the maximum keycode in the keymap.\n\n @sa xkb_keycode_t\n @memberof xkb_keymap\n @since 0.3.1"]
    #[link_name = "\u{1}xkb_keymap_max_keycode"]
    pub fn xkb_keymap_max_keycode(keymap: *mut xkb_keymap) -> xkb_keycode_t;
    #[doc = " Run a specified function for every valid keycode in the keymap.  If a\n keymap is sparse, this function may be called fewer than\n (max_keycode - min_keycode + 1) times.\n\n @sa xkb_keymap_min_keycode()\n @sa xkb_keymap_max_keycode()\n @sa xkb_keycode_t\n @memberof xkb_keymap\n @since 0.3.1"]
    #[link_name = "\u{1}xkb_keymap_key_for_each"]
    pub fn xkb_keymap_key_for_each(
        keymap: *mut xkb_keymap, iter: xkb_keymap_key_iter_t, data: *mut ::core::ffi::c_void,
    );
    #[doc = " Find the name of the key with the given keycode.\n\n This function always returns the canonical name of the key (see\n description in `xkb_keycode_t`).\n\n @returns The key name. If no key with this keycode exists,\n returns `NULL`.\n\n @sa xkb_keycode_t\n @memberof xkb_keymap\n @since 0.6.0"]
    #[link_name = "\u{1}xkb_keymap_key_get_name"]
    pub fn xkb_keymap_key_get_name(
        keymap: *mut xkb_keymap, key: xkb_keycode_t,
    ) -> *const ::core::ffi::c_char;
    #[doc = " Find the keycode of the key with the given name.\n\n The name can be either a canonical name or an alias.\n\n @returns The keycode. If no key with this name exists,\n returns `::XKB_KEYCODE_INVALID`.\n\n @sa xkb_keycode_t\n @memberof xkb_keymap\n @since 0.6.0"]
    #[link_name = "\u{1}xkb_keymap_key_by_name"]
    pub fn xkb_keymap_key_by_name(
        keymap: *mut xkb_keymap, name: *const ::core::ffi::c_char,
    ) -> xkb_keycode_t;
    #[doc = " Get the number of modifiers in the keymap.\n\n @sa xkb_mod_index_t\n @memberof xkb_keymap"]
    #[link_name = "\u{1}xkb_keymap_num_mods"]
    pub fn xkb_keymap_num_mods(keymap: *mut xkb_keymap) -> xkb_mod_index_t;
    #[doc = " Get the name of a modifier by index.\n\n @returns The name.  If the index is invalid, returns `NULL`.\n\n @sa xkb_mod_index_t\n @memberof xkb_keymap"]
    #[link_name = "\u{1}xkb_keymap_mod_get_name"]
    pub fn xkb_keymap_mod_get_name(
        keymap: *mut xkb_keymap, idx: xkb_mod_index_t,
    ) -> *const ::core::ffi::c_char;
    #[doc = " Get the index of a modifier by name.\n\n @returns The index.  If no modifier with this name exists, returns\n `::XKB_MOD_INVALID`.\n\n @sa xkb_mod_index_t\n @memberof xkb_keymap"]
    #[link_name = "\u{1}xkb_keymap_mod_get_index"]
    pub fn xkb_keymap_mod_get_index(
        keymap: *mut xkb_keymap, name: *const ::core::ffi::c_char,
    ) -> xkb_mod_index_t;
    #[doc = " Get the encoding of a modifier by name.\n\n In X11 terminology it corresponds to the mapping to the *[real modifiers]*.\n\n @returns The encoding of a modifier.  Note that it may be 0 if the name does\n not exist or if the modifier is not mapped.\n\n @since 1.10.0\n @sa `xkb_keymap_mod_get_mask2()`\n @memberof xkb_keymap\n\n [real modifiers]: @ref real-modifier-def"]
    #[link_name = "\u{1}xkb_keymap_mod_get_mask"]
    pub fn xkb_keymap_mod_get_mask(
        keymap: *mut xkb_keymap, name: *const ::core::ffi::c_char,
    ) -> xkb_mod_mask_t;
    #[doc = " Get the encoding of a modifier by index.\n\n In X11 terminology it corresponds to the mapping to the *[real modifiers]*.\n\n @returns The encoding of a modifier.  Note that it may be 0 if the modifier is\n not mapped.\n\n @since 1.11.0\n @sa `xkb_keymap_mod_get_mask()`\n @memberof xkb_keymap\n\n [real modifiers]: @ref real-modifier-def"]
    #[link_name = "\u{1}xkb_keymap_mod_get_mask2"]
    pub fn xkb_keymap_mod_get_mask2(
        keymap: *mut xkb_keymap, idx: xkb_mod_index_t,
    ) -> xkb_mod_mask_t;
    #[doc = " Get the number of layouts in the keymap.\n\n @sa `xkb_layout_index_t`\n @sa `xkb_rule_names`\n @sa `xkb_keymap_num_layouts_for_key()`\n @memberof xkb_keymap"]
    #[link_name = "\u{1}xkb_keymap_num_layouts"]
    pub fn xkb_keymap_num_layouts(keymap: *mut xkb_keymap) -> xkb_layout_index_t;
    #[doc = " Get the name of a layout by index.\n\n @returns The name.  If the index is invalid, or the layout does not have\n a name, returns `NULL`.\n\n @sa xkb_layout_index_t\n     For notes on layout names.\n @memberof xkb_keymap"]
    #[link_name = "\u{1}xkb_keymap_layout_get_name"]
    pub fn xkb_keymap_layout_get_name(
        keymap: *mut xkb_keymap, idx: xkb_layout_index_t,
    ) -> *const ::core::ffi::c_char;
    #[doc = " Get the index of a layout by name.\n\n @returns The index.  If no layout exists with this name, returns\n `::XKB_LAYOUT_INVALID`.  If more than one layout in the keymap has this name,\n returns the lowest index among them.\n\n @sa `xkb_layout_index_t` for notes on layout names.\n @memberof xkb_keymap"]
    #[link_name = "\u{1}xkb_keymap_layout_get_index"]
    pub fn xkb_keymap_layout_get_index(
        keymap: *mut xkb_keymap, name: *const ::core::ffi::c_char,
    ) -> xkb_layout_index_t;
    #[doc = " Get the number of LEDs in the keymap.\n\n @warning The range [ 0...`xkb_keymap_num_leds()` ) includes all of the LEDs\n in the keymap, but may also contain inactive LEDs.  When iterating over\n this range, you need the handle this case when calling functions such as\n `xkb_keymap_led_get_name()` or `xkb_state::xkb_state_led_index_is_active()`.\n\n @sa xkb_led_index_t\n @memberof xkb_keymap"]
    #[link_name = "\u{1}xkb_keymap_num_leds"]
    pub fn xkb_keymap_num_leds(keymap: *mut xkb_keymap) -> xkb_led_index_t;
    #[doc = " Get the name of a LED by index.\n\n @returns The name.  If the index is invalid, returns `NULL`.\n\n @memberof xkb_keymap"]
    #[link_name = "\u{1}xkb_keymap_led_get_name"]
    pub fn xkb_keymap_led_get_name(
        keymap: *mut xkb_keymap, idx: xkb_led_index_t,
    ) -> *const ::core::ffi::c_char;
    #[doc = " Get the index of a LED by name.\n\n @returns The index.  If no LED with this name exists, returns\n `::XKB_LED_INVALID`.\n\n @memberof xkb_keymap"]
    #[link_name = "\u{1}xkb_keymap_led_get_index"]
    pub fn xkb_keymap_led_get_index(
        keymap: *mut xkb_keymap, name: *const ::core::ffi::c_char,
    ) -> xkb_led_index_t;
    #[doc = " Get the number of layouts for a specific key.\n\n This number can be different from `xkb_keymap_num_layouts()`, but is always\n smaller.  It is the appropriate value to use when iterating over the\n layouts of a key.\n\n @sa xkb_layout_index_t\n @memberof xkb_keymap"]
    #[link_name = "\u{1}xkb_keymap_num_layouts_for_key"]
    pub fn xkb_keymap_num_layouts_for_key(
        keymap: *mut xkb_keymap, key: xkb_keycode_t,
    ) -> xkb_layout_index_t;
    #[doc = " Get the number of shift levels for a specific key and layout.\n\n If @c layout is out of range for this key (that is, larger or equal to\n the value returned by `xkb_keymap_num_layouts_for_key()`), it is brought\n back into range in a manner consistent with\n `xkb_state::xkb_state_key_get_layout()`.\n\n @sa xkb_level_index_t\n @memberof xkb_keymap"]
    #[link_name = "\u{1}xkb_keymap_num_levels_for_key"]
    pub fn xkb_keymap_num_levels_for_key(
        keymap: *mut xkb_keymap, key: xkb_keycode_t, layout: xkb_layout_index_t,
    ) -> xkb_level_index_t;
    #[doc = " Retrieves every possible modifier mask that produces the specified\n shift level for a specific key and layout.\n\n This API is useful for inverse key transformation; i.e. finding out\n which modifiers need to be active in order to be able to type the\n keysym(s) corresponding to the specific key code, layout and level.\n\n @warning It returns only up to masks_size modifier masks. If the\n buffer passed is too small, some of the possible modifier combinations\n will not be returned.\n\n @param[in] keymap      The keymap.\n @param[in] key         The keycode of the key.\n @param[in] layout      The layout for which to get modifiers.\n @param[in] level       The shift level in the layout for which to get the\n modifiers. This should be smaller than:\n @code xkb_keymap_num_levels_for_key(keymap, key) @endcode\n @param[out] masks_out  A buffer in which the requested masks should be\n stored.\n @param[out] masks_size The number of elements in the buffer pointed to by\n masks_out.\n\n If @c layout is out of range for this key (that is, larger or equal to\n the value returned by `xkb_keymap_num_layouts_for_key()`), it is brought\n back into range in a manner consistent with\n `xkb_state::xkb_state_key_get_layout()`.\n\n @returns The number of modifier masks stored in the masks_out array.\n If the key is not in the keymap or if the specified shift level cannot\n be reached it returns 0 and does not modify the masks_out buffer.\n\n @sa xkb_level_index_t\n @sa xkb_mod_mask_t\n @memberof xkb_keymap\n @since 1.0.0"]
    #[link_name = "\u{1}xkb_keymap_key_get_mods_for_level"]
    pub fn xkb_keymap_key_get_mods_for_level(
        keymap: *mut xkb_keymap, key: xkb_keycode_t, layout: xkb_layout_index_t,
        level: xkb_level_index_t, masks_out: *mut xkb_mod_mask_t, masks_size: usize,
    ) -> usize;
    #[doc = " Get the keysyms obtained from pressing a key in a given layout and\n shift level.\n\n This function is like `xkb_state::xkb_state_key_get_syms()`, only the layout\n and shift level are not derived from the keyboard state but are instead\n specified explicitly.\n\n @param[in] keymap    The keymap.\n @param[in] key       The keycode of the key.\n @param[in] layout    The layout for which to get the keysyms.\n @param[in] level     The shift level in the layout for which to get the\n keysyms. This should be smaller than:\n @code xkb_keymap_num_levels_for_key(keymap, key) @endcode\n @param[out] syms_out An immutable array of keysyms corresponding to the\n key in the given layout and shift level.\n\n If @c layout is out of range for this key (that is, larger or equal to\n the value returned by `xkb_keymap_num_layouts_for_key()`), it is brought\n back into range in a manner consistent with\n `xkb_state::xkb_state_key_get_layout()`.\n\n @returns The number of keysyms in the syms_out array.  If no keysyms\n are produced by the key in the given layout and shift level, returns 0\n and sets @p syms_out to `NULL`.\n\n @sa `xkb_state::xkb_state_key_get_syms()`\n @memberof xkb_keymap"]
    #[link_name = "\u{1}xkb_keymap_key_get_syms_by_level"]
    pub fn xkb_keymap_key_get_syms_by_level(
        keymap: *mut xkb_keymap, key: xkb_keycode_t, layout: xkb_layout_index_t,
        level: xkb_level_index_t, syms_out: *mut *const xkb_keysym_t,
    ) -> ::core::ffi::c_int;
    #[doc = " Determine whether a key should repeat or not.\n\n A keymap may specify different repeat behaviors for different keys.\n Most keys should generally exhibit repeat behavior; for example, holding\n the `a` key down in a text editor should normally insert a single ‘a’\n character every few milliseconds, until the key is released.  However,\n there are keys which should not or do not need to be repeated.  For\n example, repeating modifier keys such as Left/Right Shift or Caps Lock\n is not generally useful or desired.\n\n @returns 1 if the key should repeat, 0 otherwise.\n\n @memberof xkb_keymap"]
    #[link_name = "\u{1}xkb_keymap_key_repeats"]
    pub fn xkb_keymap_key_repeats(
        keymap: *mut xkb_keymap, key: xkb_keycode_t,
    ) -> ::core::ffi::c_int;
    #[doc = " Create a new keyboard state object.\n\n @param keymap The keymap which the state will use.\n\n @returns A new keyboard state object, or `NULL` on failure.\n\n @memberof xkb_state"]
    #[link_name = "\u{1}xkb_state_new"]
    pub fn xkb_state_new(keymap: *mut xkb_keymap) -> *mut xkb_state;
    #[doc = " Take a new reference on a keyboard state object.\n\n @returns The passed in object.\n\n @memberof xkb_state"]
    #[link_name = "\u{1}xkb_state_ref"]
    pub fn xkb_state_ref(state: *mut xkb_state) -> *mut xkb_state;
    #[doc = " Release a reference on a keyboard state object, and possibly free it.\n\n @param state The state.  If it is `NULL`, this function does nothing.\n\n @memberof xkb_state"]
    #[link_name = "\u{1}xkb_state_unref"]
    pub fn xkb_state_unref(state: *mut xkb_state);
    #[doc = " Get the keymap which a keyboard state object is using.\n\n @returns The keymap which was passed to `xkb_state_new()` when creating\n this state object.\n\n This function does not take a new reference on the keymap; you must\n explicitly reference it yourself if you plan to use it beyond the\n lifetime of the state.\n\n @memberof xkb_state"]
    #[link_name = "\u{1}xkb_state_get_keymap"]
    pub fn xkb_state_get_keymap(state: *mut xkb_state) -> *mut xkb_keymap;
    #[doc = " Update the keyboard state to reflect a given key being pressed or\n released.\n\n This entry point is intended for *server* applications and should not be used\n by *client* applications; see @ref server-client-state for details.\n\n A series of calls to this function should be consistent; that is, a call\n with `::XKB_KEY_DOWN` for a key should be matched by an `::XKB_KEY_UP`; if a\n key is pressed twice, it should be released twice; etc. Otherwise (e.g. due\n to missed input events), situations like “stuck modifiers” may occur.\n\n This function is often used in conjunction with the function\n `xkb_state_key_get_syms()` (or `xkb_state_key_get_one_sym()`), for example,\n when handling a key event.  In this case, you should prefer to get the\n keysyms *before* updating the key, such that the keysyms reported for\n the key event are not affected by the event itself.  This is the\n conventional behavior.\n\n @returns A mask of state components that have changed as a result of\n the update.  If nothing in the state has changed, returns 0.\n\n @memberof xkb_state\n\n @sa `xkb_state_update_mask()`"]
    #[link_name = "\u{1}xkb_state_update_key"]
    pub fn xkb_state_update_key(
        state: *mut xkb_state, key: xkb_keycode_t, direction: xkb_key_direction,
    ) -> xkb_state_component;
    #[doc = " Update the keyboard state to change the latched and locked state of\n the modifiers and layout.\n\n This entry point is intended for *server* applications and should not be used\n by *client* applications; see @ref server-client-state for details.\n\n Use this function to update the latched and locked state according to\n “out of band” (non-device) inputs, such as UI layout switchers.\n\n @par Layout out of range\n @parblock\n\n If the effective layout, after taking into account the depressed, latched and\n locked layout, is out of range (negative or greater than the maximum layout),\n it is brought into range. Currently, the layout is wrapped using integer\n modulus (with negative values wrapping from the end). The wrapping behavior\n may be made configurable in the future.\n\n @endparblock\n\n @param state The keyboard state object.\n @param affect_latched_mods\n @param latched_mods\n     Modifiers to set as latched or unlatched. Only modifiers in\n     @p affect_latched_mods are considered.\n @param affect_latched_layout\n @param latched_layout\n     Layout to latch. Only considered if @p affect_latched_layout is true.\n     Maybe be out of range (including negative) -- see note above.\n @param affect_locked_mods\n @param locked_mods\n     Modifiers to set as locked or unlocked. Only modifiers in\n     @p affect_locked_mods are considered.\n @param affect_locked_layout\n @param locked_layout\n     Layout to lock. Only considered if @p affect_locked_layout is true.\n     Maybe be out of range (including negative) -- see note above.\n\n @returns A mask of state components that have changed as a result of\n the update.  If nothing in the state has changed, returns 0.\n\n @memberof xkb_state\n\n @sa xkb_state_update_mask()"]
    #[link_name = "\u{1}xkb_state_update_latched_locked"]
    pub fn xkb_state_update_latched_locked(
        state: *mut xkb_state, affect_latched_mods: xkb_mod_mask_t, latched_mods: xkb_mod_mask_t,
        affect_latched_layout: bool, latched_layout: i32, affect_locked_mods: xkb_mod_mask_t,
        locked_mods: xkb_mod_mask_t, affect_locked_layout: bool, locked_layout: i32,
    ) -> xkb_state_component;
    #[doc = " Update a keyboard state from a set of explicit masks.\n\n This entry point is intended for *client* applications; see @ref\n server-client-state for details. *Server* applications should use\n `xkb_state_update_key()` instead.\n\n All parameters must always be passed, or the resulting state may be\n incoherent.\n\n @warning The serialization is lossy and will not survive round trips; it must\n only be used to feed client state objects, and must not be used to update the\n server state.\n\n @returns A mask of state components that have changed as a result of\n the update.  If nothing in the state has changed, returns 0.\n\n @memberof xkb_state\n\n @sa `xkb_state_component`\n @sa `xkb_state_update_key()`"]
    #[link_name = "\u{1}xkb_state_update_mask"]
    pub fn xkb_state_update_mask(
        state: *mut xkb_state, depressed_mods: xkb_mod_mask_t, latched_mods: xkb_mod_mask_t,
        locked_mods: xkb_mod_mask_t, depressed_layout: xkb_layout_index_t,
        latched_layout: xkb_layout_index_t, locked_layout: xkb_layout_index_t,
    ) -> xkb_state_component;
    #[doc = " Get the keysyms obtained from pressing a particular key in a given\n keyboard state.\n\n Get the keysyms for a key according to the current active layout,\n modifiers and shift level for the key, as determined by a keyboard\n state.\n\n @param[in]  state    The keyboard state object.\n @param[in]  key      The keycode of the key.\n @param[out] syms_out An immutable array of keysyms corresponding the\n key in the given keyboard state.\n\n As an extension to XKB, this function can return more than one keysym.\n If you do not want to handle this case, you can use\n `xkb_state_key_get_one_sym()` for a simpler interface.\n\n @returns The number of keysyms in the syms_out array.  If no keysyms\n are produced by the key in the given keyboard state, returns 0 and sets\n syms_out to `NULL`.\n\n This function performs Capitalization @ref keysym-transformations.\n\n @memberof xkb_state\n\n @since 1.9.0 This function now performs @ref keysym-transformations."]
    #[link_name = "\u{1}xkb_state_key_get_syms"]
    pub fn xkb_state_key_get_syms(
        state: *mut xkb_state, key: xkb_keycode_t, syms_out: *mut *const xkb_keysym_t,
    ) -> ::core::ffi::c_int;
    #[doc = " Get the Unicode/UTF-8 string obtained from pressing a particular key\n in a given keyboard state.\n\n @param[in]  state  The keyboard state object.\n @param[in]  key    The keycode of the key.\n @param[out] buffer A buffer to write the string into.\n @param[in]  size   Size of the buffer.\n\n @warning If the buffer passed is too small, the string is truncated\n (though still `NULL`-terminated).\n\n @returns The number of bytes required for the string, excluding the\n `NULL` byte.  If there is nothing to write, returns 0.\n\n You may check if truncation has occurred by comparing the return value\n with the size of @p buffer, similarly to the `snprintf(3)` function.\n You may safely pass `NULL` and 0 to @p buffer and @p size to find the\n required size (without the `NULL`-byte).\n\n This function performs Capitalization and Control @ref\n keysym-transformations.\n\n @memberof xkb_state\n @since 0.4.1"]
    #[link_name = "\u{1}xkb_state_key_get_utf8"]
    pub fn xkb_state_key_get_utf8(
        state: *mut xkb_state, key: xkb_keycode_t, buffer: *mut ::core::ffi::c_char, size: usize,
    ) -> ::core::ffi::c_int;
    #[doc = " Get the Unicode/UTF-32 codepoint obtained from pressing a particular\n key in a a given keyboard state.\n\n @returns The UTF-32 representation for the key, if it consists of only\n a single codepoint.  Otherwise, returns 0.\n\n This function performs Capitalization and Control @ref\n keysym-transformations.\n\n @memberof xkb_state\n @since 0.4.1"]
    #[link_name = "\u{1}xkb_state_key_get_utf32"]
    pub fn xkb_state_key_get_utf32(state: *mut xkb_state, key: xkb_keycode_t) -> u32;
    #[doc = " Get the single keysym obtained from pressing a particular key in a\n given keyboard state.\n\n This function is similar to `xkb_state_key_get_syms()`, but intended\n for users which cannot or do not want to handle the case where\n multiple keysyms are returned (in which case this function is\n preferred).\n\n @returns The keysym.  If the key does not have exactly one keysym,\n returns `XKB_KEY_NoSymbol`.\n\n This function performs Capitalization @ref keysym-transformations.\n\n @sa xkb_state_key_get_syms()\n @memberof xkb_state"]
    #[link_name = "\u{1}xkb_state_key_get_one_sym"]
    pub fn xkb_state_key_get_one_sym(state: *mut xkb_state, key: xkb_keycode_t) -> xkb_keysym_t;
    #[doc = " Get the effective layout index for a key in a given keyboard state.\n\n @returns The layout index for the key in the given keyboard state.  If\n the given keycode is invalid, or if the key is not included in any\n layout at all, returns `::XKB_LAYOUT_INVALID`.\n\n @invariant If the returned layout is valid, the following always holds:\n @code\n xkb_state_key_get_layout(state, key) < xkb_keymap_num_layouts_for_key(keymap, key)\n @endcode\n\n @memberof xkb_state"]
    #[link_name = "\u{1}xkb_state_key_get_layout"]
    pub fn xkb_state_key_get_layout(
        state: *mut xkb_state, key: xkb_keycode_t,
    ) -> xkb_layout_index_t;
    #[doc = " Get the effective shift level for a key in a given keyboard state and\n layout.\n\n @param state The keyboard state.\n @param key The keycode of the key.\n @param layout The layout for which to get the shift level.  This must be\n smaller than:\n @code xkb_keymap_num_layouts_for_key(keymap, key) @endcode\n usually it would be:\n @code xkb_state_key_get_layout(state, key) @endcode\n\n @return The shift level index.  If the key or layout are invalid,\n returns `::XKB_LEVEL_INVALID`.\n\n @invariant If the returned level is valid, the following always holds:\n @code\n xkb_state_key_get_level(state, key, layout) < xkb_keymap_num_levels_for_key(keymap, key, layout)\n @endcode\n\n @memberof xkb_state"]
    #[link_name = "\u{1}xkb_state_key_get_level"]
    pub fn xkb_state_key_get_level(
        state: *mut xkb_state, key: xkb_keycode_t, layout: xkb_layout_index_t,
    ) -> xkb_level_index_t;
    #[doc = " The counterpart to `xkb_state::xkb_state_update_mask()` for modifiers, to be\n used on the server side of serialization.\n\n This entry point is intended for *server* applications; see @ref\n server-client-state for details. *Client* applications should use the\n `xkb_state_mod_*_is_active` API.\n\n @param state      The keyboard state.\n @param components A mask of the modifier state components to serialize.\n State components other than `XKB_STATE_MODS_*` are ignored.\n If `::XKB_STATE_MODS_EFFECTIVE` is included, all other state components are\n ignored.\n\n @returns A `xkb_mod_mask_t` representing the given components of the\n modifier state.\n\n @memberof xkb_state"]
    #[link_name = "\u{1}xkb_state_serialize_mods"]
    pub fn xkb_state_serialize_mods(
        state: *mut xkb_state, components: xkb_state_component,
    ) -> xkb_mod_mask_t;
    #[doc = " The counterpart to `xkb_state::xkb_state_update_mask()` for layouts, to be\n used on the server side of serialization.\n\n This entry point is intended for *server* applications; see @ref\n server-client-state for details. *Client* applications should use the\n xkb_state_layout_*_is_active API.\n\n @param state      The keyboard state.\n @param components A mask of the layout state components to serialize.\n State components other than `XKB_STATE_LAYOUT_*` are ignored.\n If `::XKB_STATE_LAYOUT_EFFECTIVE` is included, all other state components are\n ignored.\n\n @returns A layout index representing the given components of the\n layout state.\n\n @memberof xkb_state"]
    #[link_name = "\u{1}xkb_state_serialize_layout"]
    pub fn xkb_state_serialize_layout(
        state: *mut xkb_state, components: xkb_state_component,
    ) -> xkb_layout_index_t;
    #[doc = " Test whether a modifier is active in a given keyboard state by name.\n\n @warning For [virtual modifiers], this function may *overmatch* in case\n there are virtual modifiers with overlapping mappings to [real modifiers].\n\n @returns 1 if the modifier is active, 0 if it is not.  If the modifier\n name does not exist in the keymap, returns -1.\n\n @memberof xkb_state\n\n @since 0.1.0: Works only with *real* modifiers\n @since 1.8.0: Works also with *virtual* modifiers\n\n [virtual modifiers]: @ref virtual-modifier-def\n [real modifiers]: @ref real-modifier-def"]
    #[link_name = "\u{1}xkb_state_mod_name_is_active"]
    pub fn xkb_state_mod_name_is_active(
        state: *mut xkb_state, name: *const ::core::ffi::c_char, type_: xkb_state_component,
    ) -> ::core::ffi::c_int;
    #[doc = " Test whether a set of modifiers are active in a given keyboard state by\n name.\n\n @warning For [virtual modifiers], this function may *overmatch* in case\n there are virtual modifiers with overlapping mappings to [real modifiers].\n\n @param state The keyboard state.\n @param type  The component of the state against which to match the\n given modifiers.\n @param match The manner by which to match the state against the\n given modifiers.\n @param ...   The set of of modifier names to test, terminated by a NULL\n argument (sentinel).\n\n @returns 1 if the modifiers are active, 0 if they are not.  If any of\n the modifier names do not exist in the keymap, returns -1.\n\n @memberof xkb_state\n\n @since 0.1.0: Works only with *real* modifiers\n @since 1.8.0: Works also with *virtual* modifiers\n\n [virtual modifiers]: @ref virtual-modifier-def\n [real modifiers]: @ref real-modifier-def"]
    #[link_name = "\u{1}xkb_state_mod_names_are_active"]
    pub fn xkb_state_mod_names_are_active(
        state: *mut xkb_state, type_: xkb_state_component, match_: xkb_state_match, ...
    ) -> ::core::ffi::c_int;
    #[doc = " Test whether a modifier is active in a given keyboard state by index.\n\n @warning For [virtual modifiers], this function may *overmatch* in case\n there are virtual modifiers with overlapping mappings to [real modifiers].\n\n @returns 1 if the modifier is active, 0 if it is not.  If the modifier\n index is invalid in the keymap, returns -1.\n\n @memberof xkb_state\n\n @since 0.1.0: Works only with *real* modifiers\n @since 1.8.0: Works also with *virtual* modifiers\n\n [virtual modifiers]: @ref virtual-modifier-def\n [real modifiers]: @ref real-modifier-def"]
    #[link_name = "\u{1}xkb_state_mod_index_is_active"]
    pub fn xkb_state_mod_index_is_active(
        state: *mut xkb_state, idx: xkb_mod_index_t, type_: xkb_state_component,
    ) -> ::core::ffi::c_int;
    #[doc = " Test whether a set of modifiers are active in a given keyboard state by\n index.\n\n @warning For [virtual modifiers], this function may *overmatch* in case\n there are virtual modifiers with overlapping mappings to [real modifiers].\n\n @param state The keyboard state.\n @param type  The component of the state against which to match the\n given modifiers.\n @param match The manner by which to match the state against the\n given modifiers.\n @param ...   The set of of modifier indices to test, terminated by a\n `::XKB_MOD_INVALID` argument (sentinel).\n\n @returns 1 if the modifiers are active, 0 if they are not.  If any of\n the modifier indices are invalid in the keymap, returns -1.\n\n @memberof xkb_state\n\n @since 0.1.0: Works only with *real* modifiers\n @since 1.8.0: Works also with *virtual* modifiers\n\n [virtual modifiers]: @ref virtual-modifier-def\n [real modifiers]: @ref real-modifier-def"]
    #[link_name = "\u{1}xkb_state_mod_indices_are_active"]
    pub fn xkb_state_mod_indices_are_active(
        state: *mut xkb_state, type_: xkb_state_component, match_: xkb_state_match, ...
    ) -> ::core::ffi::c_int;
    #[doc = " Get the mask of modifiers consumed by translating a given key.\n\n @param state The keyboard state.\n @param key   The keycode of the key.\n @param mode  The consumed modifiers mode to use; see enum description.\n\n @returns a mask of the consumed [real modifiers] modifiers.\n\n @memberof xkb_state\n @since 0.7.0\n\n [real modifiers]: @ref real-modifier-def"]
    #[link_name = "\u{1}xkb_state_key_get_consumed_mods2"]
    pub fn xkb_state_key_get_consumed_mods2(
        state: *mut xkb_state, key: xkb_keycode_t, mode: xkb_consumed_mode,
    ) -> xkb_mod_mask_t;
    #[doc = " Same as `xkb_state_key_get_consumed_mods2()` with mode `::XKB_CONSUMED_MODE_XKB`.\n\n @memberof xkb_state\n @since 0.4.1"]
    #[link_name = "\u{1}xkb_state_key_get_consumed_mods"]
    pub fn xkb_state_key_get_consumed_mods(
        state: *mut xkb_state, key: xkb_keycode_t,
    ) -> xkb_mod_mask_t;
    #[doc = " Test whether a modifier is consumed by keyboard state translation for\n a key.\n\n @warning For [virtual modifiers], this function may *overmatch* in case\n there are virtual modifiers with overlapping mappings to [real modifiers].\n\n @param state The keyboard state.\n @param key   The keycode of the key.\n @param idx   The index of the modifier to check.\n @param mode  The consumed modifiers mode to use; see enum description.\n\n @returns 1 if the modifier is consumed, 0 if it is not.  If the modifier\n index is not valid in the keymap, returns -1.\n\n @sa xkb_state_mod_mask_remove_consumed()\n @sa xkb_state_key_get_consumed_mods()\n @memberof xkb_state\n @since 0.7.0: Works only with *real* modifiers\n @since 1.8.0: Works also with *virtual* modifiers\n\n [virtual modifiers]: @ref virtual-modifier-def\n [real modifiers]: @ref real-modifier-def"]
    #[link_name = "\u{1}xkb_state_mod_index_is_consumed2"]
    pub fn xkb_state_mod_index_is_consumed2(
        state: *mut xkb_state, key: xkb_keycode_t, idx: xkb_mod_index_t, mode: xkb_consumed_mode,
    ) -> ::core::ffi::c_int;
    #[doc = " Same as `xkb_state_mod_index_is_consumed2()` with mode `::XKB_CONSUMED_MOD_XKB`.\n\n @warning For [virtual modifiers], this function may *overmatch* in case\n there are virtual modifiers with overlapping mappings to [real modifiers].\n\n @memberof xkb_state\n @since 0.4.1: Works only with *real* modifiers\n @since 1.8.0: Works also with *virtual* modifiers\n\n [virtual modifiers]: @ref virtual-modifier-def\n [real modifiers]: @ref real-modifier-def"]
    #[link_name = "\u{1}xkb_state_mod_index_is_consumed"]
    pub fn xkb_state_mod_index_is_consumed(
        state: *mut xkb_state, key: xkb_keycode_t, idx: xkb_mod_index_t,
    ) -> ::core::ffi::c_int;
    #[doc = " Remove consumed modifiers from a modifier mask for a key.\n\n @deprecated Use `xkb_state_key_get_consumed_mods2()` instead.\n\n Takes the given modifier mask, and removes all modifiers which are\n consumed for that particular key (as in `xkb_state_mod_index_is_consumed()`).\n\n @returns a mask of [real modifiers] modifiers.\n\n @sa xkb_state_mod_index_is_consumed()\n @memberof xkb_state\n @since 0.5.0: Works only with *real* modifiers\n @since 1.8.0: Works also with *virtual* modifiers\n\n [real modifiers]: @ref real-modifier-def"]
    #[link_name = "\u{1}xkb_state_mod_mask_remove_consumed"]
    pub fn xkb_state_mod_mask_remove_consumed(
        state: *mut xkb_state, key: xkb_keycode_t, mask: xkb_mod_mask_t,
    ) -> xkb_mod_mask_t;
    #[doc = " Test whether a layout is active in a given keyboard state by name.\n\n @returns 1 if the layout is active, 0 if it is not.  If no layout with\n this name exists in the keymap, return -1.\n\n If multiple layouts in the keymap have this name, the one with the lowest\n index is tested.\n\n @sa xkb_layout_index_t\n @memberof xkb_state"]
    #[link_name = "\u{1}xkb_state_layout_name_is_active"]
    pub fn xkb_state_layout_name_is_active(
        state: *mut xkb_state, name: *const ::core::ffi::c_char, type_: xkb_state_component,
    ) -> ::core::ffi::c_int;
    #[doc = " Test whether a layout is active in a given keyboard state by index.\n\n @returns 1 if the layout is active, 0 if it is not.  If the layout index\n is not valid in the keymap, returns -1.\n\n @sa xkb_layout_index_t\n @memberof xkb_state"]
    #[link_name = "\u{1}xkb_state_layout_index_is_active"]
    pub fn xkb_state_layout_index_is_active(
        state: *mut xkb_state, idx: xkb_layout_index_t, type_: xkb_state_component,
    ) -> ::core::ffi::c_int;
    #[doc = " Test whether a LED is active in a given keyboard state by name.\n\n @returns 1 if the LED is active, 0 if it not.  If no LED with this name\n exists in the keymap, returns -1.\n\n @sa xkb_led_index_t\n @memberof xkb_state"]
    #[link_name = "\u{1}xkb_state_led_name_is_active"]
    pub fn xkb_state_led_name_is_active(
        state: *mut xkb_state, name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    #[doc = " Test whether a LED is active in a given keyboard state by index.\n\n @returns 1 if the LED is active, 0 if it not.  If the LED index is not\n valid in the keymap, returns -1.\n\n @sa xkb_led_index_t\n @memberof xkb_state"]
    #[link_name = "\u{1}xkb_state_led_index_is_active"]
    pub fn xkb_state_led_index_is_active(
        state: *mut xkb_state, idx: xkb_led_index_t,
    ) -> ::core::ffi::c_int;
}
