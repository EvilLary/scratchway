#![allow(unused)]

extern crate proc_macro;

use crate::parser::{ArgType, Enum, Message};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};
use std::path::PathBuf;

static SKIP_IFACES: [&str; 2] = ["wl_display", "wl_registry"];

mod parser;

#[proc_macro]
pub fn generate(path: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let path = match parse_path(path) {
        Ok(p) => p,
        Err(e) => return e.into(),
    };

    let file = std::fs::read_to_string(path).unwrap();
    let mut reader = quick_xml::Reader::from_str(&file);
    reader.config_mut().trim_text(true);

    let parser = parser::Parser::new(&file);
    let Ok(protocol) = parser.get_grotocol() else {
        return quote!(compile_error("Failed to parse protocol")).into();
    };

    protocol
        .interfaces
        .iter()
        .map(|iface| {
            if SKIP_IFACES.contains(&iface.name.as_str()) {
                return quote! {};
            }

            let iface_name = &iface.name;
            let iface_version = iface.version;
            let iface_mod = Ident::new(&iface.name, Span::call_site());
            let iface_struct = Ident::new(&iface.name.snake_to_pascal(), Span::call_site());

            let requests = iface.requests.iter().enumerate().map(generate_requests);

            let (ev_enom, ev_parse) = generate_events(&iface.events);

            let ev_array = iface.events.iter().map(|e| {
                let ev_name = e.name.as_str();
                quote!{ #ev_name }
            });
            let req_array = iface.requests.iter().map(|r| {
                let req_name = r.name.as_str();
                quote!{ #req_name }
            });

            let enoms = iface.enums.iter().map(generate_enoms);

            quote! {
                pub use #iface_mod::#iface_struct;
                pub mod #iface_mod {
                    #![allow(unused_mut, clippy::too_many_arguments, clippy::unused_unit)]
                    use super::*;

                    #[derive(Copy, Clone)]
                    pub struct #iface_struct {
                        id: u32,
                        version: u32,
                    }

                    impl ::core::fmt::Debug for #iface_struct
                    {
                        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::std::fmt::Result {
                            f.write_fmt(format_args!(
                                    "{}#{}",
                                    <#iface_struct as WlInterface>::interface().name,
                                    self.id
                            ))
                        }
                    }

                    impl ::core::fmt::Display for #iface_struct
                    {
                        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::std::fmt::Result {
                            f.write_fmt(format_args!(
                                    "{}#{}",
                                    <#iface_struct as WlInterface>::interface().name,
                                    self.id
                            ))
                        }
                    }

                    impl ::core::cmp::PartialEq for #iface_struct {
                        fn eq(&self, other: &Self) -> bool {
                            self.id == other.id
                        }
                    }

                    impl ::core::cmp::Eq for #iface_struct {}


                    impl #iface_struct {
                        pub const INTERFACE: &'static Interface = &Interface {
                            name: #iface_name,
                            version: #iface_version,
                            events: &[#(#ev_array,)*],
                            requests: &[#(#req_array,)*],
                        };

                        #(#requests)*
                    }

                    #(#enoms)*

                    #ev_enom

                    impl WlInterface for #iface_struct {
                        fn version(&self) -> u32 {
                            self.version
                        }

                        fn id(&self) -> u32 {
                            self.id
                        }

                        fn interface() -> &'static Interface {
                            Self::INTERFACE
                        }

                        fn from_proxy(wl_proxy: WlProxy) -> Self {
                            Self {
                                id: wl_proxy.id,
                                version: wl_proxy.version,
                            }
                        }

                        #ev_parse
                    }
                }
            }
        })
        .collect::<TokenStream>()
        .into()
}

trait StrUtils: AsRef<str> {
    fn is_reserved(&self) -> bool {
        let str = self.as_ref();
        const RESERVED: [&str; 2] = ["move", "try"];
        RESERVED.contains(&str)
    }

    fn snake_to_pascal(&self) -> String {
        let str = self.as_ref();
        let mut output = String::with_capacity(str.len());
        let mut capatilize_next = false;
        let mut chars = str.chars();

        if let Some(c) = chars.next() {
            if !c.is_ascii_alphabetic() {
                output.push('_');
            }
            output.push(c.to_ascii_uppercase());
        }

        for c in chars {
            if c == '_' {
                capatilize_next = true;
                continue;
            }
            if capatilize_next {
                output.push(c.to_ascii_uppercase());
                capatilize_next = false;
            } else {
                output.push(c)
            }
        }
        output
    }
}

impl<T: AsRef<str>> StrUtils for T {}

fn iface_to_enom(name: &str) -> TokenStream {
    let ff = name.split_once('.');
    if ff.is_none() {
        let enom = Ident::new(&name.snake_to_pascal(), Span::call_site());
        return quote! { #enom };
    }
    let (mod_name, enom) = ff.unwrap();
    let mod_name = Ident::new(mod_name, Span::call_site());
    let enom = Ident::new(&enom.snake_to_pascal(), Span::call_site());
    quote! { #mod_name::#enom }
}

fn iface_to_object(iface: &str) -> TokenStream {
    let iface_mod = iface;
    let iface_struct = Ident::new(&iface_mod.snake_to_pascal(), Span::call_site());
    let iface_mod = Ident::new(iface_mod, Span::call_site());
    quote! { #iface_mod::#iface_struct }
}

// I'm iterating over the same values multiple times, can't care enough
fn generate_events(events: &[Message]) -> (TokenStream, TokenStream) {
    let ev_lifetime = events
        .iter()
        .any(|e| {
            e.args.iter().any(|arg| {
                matches!(&arg.arg_type, ArgType::Array | ArgType::String { .. })
            })
        })
        .then(|| quote! {<'a>})
        .unwrap_or(quote! {});

    let enom_varients = events.iter().map(generate_event_enom);
    let mut event_parse = Vec::<TokenStream>::new();

    for (op, ev) in events.iter().enumerate() {
        let ev_name = format_ident!("{}", ev.name.snake_to_pascal());
        let mut ev_parse_body = Vec::<TokenStream>::new();
        let mut trait_bounds = Vec::<TokenStream>::new();

        let mut ev_fields = ev
            .args
            .iter()
            .map(|a| {
                let name = format_ident!("{}", a.name);
                match &a.arg_type {
                    ArgType::Int => {
                        ev_parse_body.push(quote! {
                            let #name = __parser.get_i32();
                        });
                    },
                    ArgType::Uint => {
                        ev_parse_body.push(quote! {
                            let #name = __parser.get_u32();
                        });
                    },
                    ArgType::Enum(enom) => {
                        let enom_enom = iface_to_enom(enom);
                        ev_parse_body.push(quote! {
                            let #name: #enom_enom = __parser.get_u32().try_into().unwrap();
                        });
                    },
                    ArgType::Fixed => {
                        ev_parse_body.push(quote! {
                            let #name = __parser.get_fixed();
                        });
                    },
                    ArgType::String { allow_null } => {
                        ev_parse_body.push(quote! {
                            let #name = __parser.get_string();
                        });
                    },
                    ArgType::Object { allow_null, iface } => {
                        let object = iface_to_object(iface.as_ref().unwrap());
                        if *allow_null {
                            ev_parse_body.push(quote! {
                                let #name = __parser.get_u32();
                                let #name: Option<#object> = if #name == 0 {
                                    None
                                } else {
                                    Some(conn.get_object(#name))
                                };
                            });
                        } else {
                            ev_parse_body.push(quote! {
                                let #name: #object = conn.get_object(__parser.get_u32());
                            });
                        }
                    },
                    ArgType::NewId { iface } => {
                        let object = iface_to_object(iface.as_ref().unwrap());
                        ev_parse_body.push(quote! {
                            let #name: #object = conn.deaf_wlinterface::<#object>(Some(__parser.get_u32()), self.version);
                        });
                    },
                    ArgType::Array => {
                        ev_parse_body.push(quote! {
                            let #name = __parser.get_array();
                        });
                    },
                    ArgType::Fd => {
                        ev_parse_body.push(quote! {
                            let #name = conn.get_fd().unwrap();
                        });
                    },
                }
                quote! { #name, }
            })
            .collect::<TokenStream>();

        if !ev_fields.is_empty() {
            ev_fields = quote! {
                {
                    #ev_fields
                }
            };
        }

        let opcode = op as u16;

        event_parse.push(quote! {
            #opcode => {
                #(#ev_parse_body)*
                Self::Event::#ev_name #ev_fields
            }
        });
    }

    (
        quote! {
            #[derive(Debug)]
            pub enum Event #ev_lifetime {
                #(#enom_varients)*
            }
        },
        quote! {
            type Event<'a> = Event #ev_lifetime;

            fn parse_event<'a, S>(&self, conn: &mut Connection<S>, event: &'a WlEvent) -> Event #ev_lifetime
            {
                let __parser = event.parser();
                match event.header.opcode {
                    #(#event_parse,)*
                    _ => unsafe {
                        std::hint::unreachable_unchecked()
                    }
                }
            }
        },
    )
}

fn generate_event_enom(ev: &Message) -> TokenStream {
    let ev_name = format_ident!("{}", ev.name.snake_to_pascal());
    let mut ev_fields = ev
        .args
        .iter()
        .map(|r| {
            let req_name = format_ident!("{}", r.name);
            let req_type = r.arg_type.as_rust_type(true).unwrap_or_else(|| {
                let ArgType::NewId { iface } = r.arg_type.clone() else {
                    panic!("duh")
                };

                let iface_mod = iface.as_ref().unwrap();
                let iface_struct = Ident::new(&iface_mod.snake_to_pascal(), Span::call_site());
                let iface_mod = Ident::new(iface_mod, Span::call_site());

                quote!( #iface_mod::#iface_struct )
            });
            quote! {
                #req_name: #req_type,
            }
        })
        .collect::<TokenStream>();

    if !ev_fields.is_empty() {
        ev_fields = quote! {
            {
                #ev_fields
            }
        };
    }

    quote! {
        #ev_name #ev_fields,
    }
}

fn generate_requests((i, req): (usize, &Message)) -> TokenStream {
    let opcode = i as u16;
    let mut req_name_str = req.name.clone();
    if req_name_str.is_reserved() {
        req_name_str.push('_');
    }
    let req_name = Ident::new(&req_name_str, Span::call_site());

    let mut version_check = quote! {};
    let min_version = req.since;
    if req.since > 1 {
        version_check = quote! {
            if self.version() < #min_version {
                panic!("below version requirement for {}.{}. {}, min: {}", Self::interface().name, #req_name_str, self.version(), #min_version);
            }
        }
    }

    let (mut return_type, mut return_stmt, mut trait_bounds) = (quote! { () }, quote! { () }, quote! {});

    let mut fn_args = Vec::<TokenStream>::new();

    let mut estimated_size = 8;
    let mut args_writer = Vec::new();
    for arg in &req.args {
        estimated_size += arg.arg_type.estimate_size();
        let arg_type = arg.arg_type.as_rust_type(false);
        let arg_name = Ident::new(&arg.name, Span::call_site());

        args_writer.push(arg.arg_type.write_them(&arg_name));
        if let Some(typ) = arg_type.as_ref() {
            fn_args.push(quote! {#arg_name: #typ});
        } else {
            let ArgType::NewId { iface } = arg.arg_type.clone() else {
                panic!("duh")
            };

            let iface_mod = iface.as_ref().unwrap();
            let iface_struct = Ident::new(&iface_mod.snake_to_pascal(), Span::call_site());
            let iface_mod = Ident::new(iface_mod, Span::call_site());

            return_type = quote!( #iface_mod::#iface_struct );
            trait_bounds = quote!( Listener<#return_type> );
            return_stmt = quote! { #arg_name };
        }
    }

    if !trait_bounds.is_empty() {
        trait_bounds = quote! {
            where
                S: #trait_bounds,
        }
    }

    quote! {
        pub fn #req_name<S>(&self, conn: &mut Connection<S>, #(#fn_args,)*) -> #return_type
            #trait_bounds
        {
            let mut __msg = Message::<#estimated_size>::new(self.id, #opcode);
            #version_check
            #(#args_writer)*
            __msg.build();
            if conn.debug {
                log!(WAYLAND, "{}.{}()", self, Self::INTERFACE.requests[#opcode as usize]);
            }
            conn.write_request(__msg.data());
            #return_stmt
        }
    }
}

fn generate_enoms(enom: &Enum) -> TokenStream {
    if enom.is_bitfield {
        let enom_name = Ident::new(&enom.name.snake_to_pascal(), Span::call_site());
        let fields = enom.items.iter().map(|varient| {
            let varient_name = varient.name.snake_to_pascal();
            let varient_name = format_ident!("{}", varient_name.to_ascii_uppercase());
            let varient_val = varient.value;

            quote! {
                const #varient_name = #varient_val;
            }
        });
        return quote! {
            bitflags::bitflags! {
                #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
                pub struct #enom_name: u32 {
                    #(#fields)*
                }
            }
            impl ::core::convert::From<#enom_name> for u32 {
                fn from(value: #enom_name) -> u32 {
                    value.bits()
                }
            }
            impl ::core::convert::TryFrom<u32> for #enom_name {
                type Error = ();
                fn try_from(value: u32) -> Result<Self, ()> {
                    #enom_name::from_bits(value).ok_or(())
                }
            }
        };
    }
    let enom_name = Ident::new(&enom.name.snake_to_pascal(), Span::call_site());
    let mut from_match_cases = Vec::new();
    let fields = enom.items.iter().map(|varient| {
        let varient_name = varient.name.snake_to_pascal();
        let varient_name = Ident::new(&varient_name, Span::call_site());
        let varient_val = varient.value;

        from_match_cases.push(quote! {
             #varient_val => Ok(Self::#varient_name),
        });
        quote! {
            #varient_name = #varient_val,
        }
    });

    quote! {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
        pub enum #enom_name {
            #(#fields)*
        }

        impl ::core::convert::From<#enom_name> for u32 {
            fn from(value: #enom_name) -> u32 {
                value as u32
            }
        }
        impl ::core::convert::TryFrom<u32> for #enom_name {
            type Error = ();
            fn try_from(value: u32) -> Result<Self, ()> {
                match value {
                    #(#from_match_cases)*
                    _ => Err(())
                }
            }
        }
    }
}

fn parse_path(path: proc_macro::TokenStream) -> Result<PathBuf, TokenStream> {
    let mut path = path.into_iter();

    let Some(proc_macro::TokenTree::Literal(path)) = path.next() else {
        return Err(quote!(compile_error!("Invalid arguements")));
    };

    let path = {
        let p = path.to_string();
        let mut begin = 0;
        let mut end = p.len();
        if p.starts_with('"') {
            begin += 1;
        }
        if p.starts_with('"') {
            end -= 1;
        }
        let path = p[begin..end].to_string();
        if let Some(mani_dir) = std::env::var_os("CARGO_MANIFEST_DIR") {
            std::path::PathBuf::from(mani_dir).join(path)
        } else {
            std::path::PathBuf::from(path)
        }
    };

    if !path.exists() {
        return Err(quote!(compile_error!("Provided path doesn't exists")));
    }

    if !path.is_file() {
        return Err(quote!(compile_error!("Provided path isn't not a file")));
    }

    Ok(path)
}
