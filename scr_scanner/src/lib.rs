#![allow(unused)]
extern crate proc_macro;

use proc_macro2::{Ident, Span, TokenStream};
use quote::{ToTokens, format_ident, quote};

mod parser;

#[proc_macro]
pub fn generate(path: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut path = path.into_iter();

    let Some(proc_macro::TokenTree::Literal(path)) = path.next() else {
        return quote!(compile_error!("Invalid arguements")).into();
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
        panic!("Provided path doesn't exists, {}", path.display());
    }

    if !path.is_file() {
        panic!("Provided path isn't a file, {}", path.display());
    }

    let name = Ident::new("App", Span::call_site());
    let file = std::fs::read_to_string(path).unwrap();
    let mut reader = quick_xml::Reader::from_str(&file);
    reader.config_mut().trim_text(true);

    let parser = parser::Parser::new(&file);
    let Ok(protocol) = parser.get_grotocol() else {
        return quote!(compile_error("Failed to parse protocol")).into();
    };

    let interfaces = protocol.interfaces.iter().map(|o| {
        let iface_name = &o.name;
        if iface_name == "wl_display" || iface_name == "wl_registry" {
            return quote! {};
        }
        let iface_mod = Ident::new(&o.name, Span::call_site());
        let object_name = Ident::new(&o.name.snake_to_pascal(), Span::call_site());
        let events_enum = o.events.iter().map(|e| {
            let ev_idnt = Ident::new(&e.name.snake_to_pascal(), Span::call_site());
            quote!{
                #ev_idnt
            }
        });

        let reqs = o.requests.iter().enumerate().map(|(i, r)| {
            let req_idnt = if r.name != "move" { Ident::new(&r.name, Span::call_site()) } else {
                format_ident!("_{}", r.name)
            };
            let mut fn_body = Vec::new();
            let mut params = Vec::new();
            let mut size = 8usize;
            let mut args = Vec::new();
            let mut log_msg = format!("{{}}.{}(", r.name);
            let opcode = i as u16;
            let (mut return_ty, mut return_stmnt)  = (quote! { () }, quote! {});
            for arg in &r.args {
                let arg_idnt = Ident::new(&arg.name, Span::call_site());
                match &arg.arg_type {
                    parser::ArgType::Int => {
                        size += 4;
                        params.push(quote!{
                            #arg_idnt: i32
                        });
                        fn_body.push(quote!{
                            msg.write_i32(#arg_idnt);
                        });
                        args.push(quote! {
                            #arg_idnt
                        });
                        log_msg.push_str("{}, ");
                    },
                    parser::ArgType::Uint => {
                        size += 4;
                        params.push(quote!{
                            #arg_idnt: u32
                        });
                        fn_body.push(quote!{
                            msg.write_u32(#arg_idnt);
                        });
                        args.push(quote! {
                            #arg_idnt
                        });
                        log_msg.push_str("{}, ");
                    },
                    parser::ArgType::Enum(_) => {
                        size += 4;
                        params.push(quote!{
                            #arg_idnt: u32
                        });
                        fn_body.push(quote!{
                            msg.write_u32(#arg_idnt);
                        });
                        args.push(quote! {
                            #arg_idnt
                        });
                        log_msg.push_str("{}, ");
                    },
                    parser::ArgType::Fixed => {
                        size += 4;
                        params.push(quote!{
                            #arg_idnt: f32
                        });
                        fn_body.push(quote!{
                            msg.write_fixed(#arg_idnt);
                        });
                        args.push(quote! {
                            #arg_idnt
                        });
                        log_msg.push_str("{:.2}, ");
                    },
                    parser::ArgType::String { allow_null } => {
                        size += 54;
                        params.push(quote!{
                            #arg_idnt: &str
                        });
                        fn_body.push(quote!{
                            msg.write_string(#arg_idnt);
                        });
                        args.push(quote! {
                            #arg_idnt
                        });
                        log_msg.push_str("\"{}\", ");
                    },
                    parser::ArgType::Object { allow_null, iface } => {
                        size += 4;
                        let iface_idnt = Ident::new(&iface.as_ref().unwrap().snake_to_pascal(), Span::call_site());
                        let mod_idnt = Ident::new(iface.as_ref().unwrap(), Span::call_site());
                        let arg_id = format_ident!("{}_id", arg.name);
                        if *allow_null {
                            params.push(quote! {
                                #arg_idnt: Option<&#mod_idnt::#iface_idnt>
                            });
                            fn_body.push(quote!{
                                let #arg_id = #arg_idnt.map_or(0, |o| o.id());
                            })
                        } else {
                            params.push(quote! {
                                #arg_idnt: &#mod_idnt::#iface_idnt
                            });
                            fn_body.push(quote!{
                                let #arg_id = #arg_idnt.id();
                            })
                        }
                        fn_body.push(quote!{
                            msg.write_u32(#arg_id);
                        });
                        args.push(quote! {
                            #arg_idnt
                        });
                        log_msg.push_str("{:?}, ");
                    },
                    parser::ArgType::NewId { iface } => {
                        size += 4;
                        let new_idnt = format_ident!("new_{}", iface.as_ref().unwrap());
                        let new_type_ob = format_ident!("{}", iface.as_ref().unwrap().snake_to_pascal());
                        let iface_mod = format_ident!("{}", iface.as_ref().unwrap());
                        return_stmnt = quote! {
                            #new_idnt
                        };
                        return_ty = quote! {
                            #iface_mod::#new_type_ob
                        };
                        fn_body.push(quote!{
                            let new_id = writer.new_id();
                            let #new_idnt: #return_ty = Object::from_id(new_id);
                            msg.write_u32(new_id);
                        });
                        args.push(quote! {
                            #new_idnt
                        });
                        log_msg.push_str("new {}, ");
                    },
                    parser::ArgType::Array => {
                        size += 28;
                        params.push(quote!{
                            #arg_idnt: &[u32]
                        });
                        fn_body.push(quote!{
                            msg.write_array(#arg_idnt);
                        });
                        args.push(quote! {
                            #arg_idnt
                        });
                        log_msg.push_str("{:?}, ");
                    },
                    parser::ArgType::Fd => {
                        params.push(quote!{
                            #arg_idnt: i32
                        });
                        fn_body.push(quote!{
                            writer.add_fd(#arg_idnt);
                        });
                        args.push(quote! {
                            #arg_idnt
                        });
                        log_msg.push_str("{}, ");
                    },
                }
            }

            if !r.args.is_empty() {
                fn_body.push(quote! {
                    msg.build();
                });
            }

            let log_msg = {
                let msg = log_msg.trim_end();
                let mut end = msg.len();
                if let Some(',') = msg.chars().last() {
                    end -= 1;
                }
                format!("{})", &msg[..end])
            };
            quote!{
                pub fn #req_idnt(&self, writer: &WaylandBuffer<Writer>, #(#params,)*) -> #return_ty {
                    let mut msg = Message::<#size>::new(self.id, #opcode);
                    #(#fn_body)*
                    writer.write_request(msg.data());
                    {
                        log!(WAYLAND, #log_msg, self, #(#args,)*);
                    }
                    #return_stmnt
                }
            }
        });

        let mut ev_variants = Vec::<TokenStream>::new();
        let mut ev_parse = Vec::<TokenStream>::new();
        let mut ev_lifetime = false;

        for (i, ev) in o.events.iter().enumerate() {
            let i = i as u16;
            let ev_idnt = Ident::new(&ev.name.snake_to_pascal(), Span::call_site());
            let mut variant_parse = Vec::new();
            let mut ev_fields: Vec<TokenStream> = Vec::new();
            let mut log_msg = format!("==> {{}}.{}(", ev.name);
            let mut args = Vec::new();
            let ty = quote!{ u32 };
            if ev.args.is_empty() {
                ev_variants.push(quote!{
                    #ev_idnt
                });
                log_msg.push_str(")");
                variant_parse.push(quote!{
                    {
                        log!(WAYLAND, #log_msg, self, #(#args,)*);
                    }
                    Self::Event::#ev_idnt
                });
            } else {
                let mut fields = Vec::new();
                for arg in &ev.args {
                    let field_idnt = Ident::new(&arg.name, Span::call_site());
                    let field_type = match &arg.arg_type {
                        parser::ArgType::Int => {
                            variant_parse.push(quote!{
                                let #field_idnt = parser.get_i32();
                            });
                            args.push(quote! {
                                #field_idnt
                            });
                            log_msg.push_str("{}, ");
                            quote! { i32 }
                        },
                        parser::ArgType::Uint => {
                            variant_parse.push(quote!{
                                let #field_idnt = parser.get_u32();
                            });
                            args.push(quote! {
                                #field_idnt
                            });
                            log_msg.push_str("{}, ");
                            quote! { u32 }
                        },
                        parser::ArgType::Enum(_) => {
                            variant_parse.push(quote!{
                                let #field_idnt = parser.get_u32();
                            });
                            args.push(quote! {
                                #field_idnt
                            });
                            log_msg.push_str("{}, ");
                            quote! { u32 }
                        },
                        parser::ArgType::Fixed => {
                            variant_parse.push(quote!{
                                let #field_idnt = parser.get_fixed();
                            });
                            args.push(quote! {
                                #field_idnt
                            });
                            log_msg.push_str("{:.2}, ");
                            quote! { f32 }
                        },
                        parser::ArgType::String { allow_null } => {
                            variant_parse.push(quote!{
                                let #field_idnt = parser.get_string();
                            });
                            args.push(quote! {
                                #field_idnt
                            });
                            log_msg.push_str("\"{}\", ");
                            ev_lifetime = true;
                            quote! { &'a str }
                        },
                        parser::ArgType::Object { allow_null, iface } => {
                            let iface_mod = format_ident!("{}", iface.as_ref().unwrap());
                            let iface_obj = format_ident!("{}", iface.as_ref().unwrap().snake_to_pascal());
                            args.push(quote! {
                                #field_idnt
                            });
                            log_msg.push_str("{:?}, ");
                            if *allow_null {
                                variant_parse.push(quote!{
                                    let #field_idnt = {
                                        let id = parser.get_u32();
                                        if id == 0 {
                                            None
                                        } else {
                                            Some(Object::from_id(id))
                                        }
                                    };
                                });
                                quote! { Option<#iface_mod::#iface_obj> }
                            } else {
                                variant_parse.push(quote!{
                                    let #field_idnt = Object::from_id(parser.get_u32());
                                });
                                quote! { #iface_mod::#iface_obj }
                            }
                        },
                        parser::ArgType::NewId { iface } => {
                            variant_parse.push(quote!{
                                let #field_idnt = parser.get_u32();
                            });
                            args.push(quote! {
                                #field_idnt
                            });
                            log_msg.push_str("{}, ");
                            quote! { u32 }
                        },
                        parser::ArgType::Array => {
                            variant_parse.push(quote!{
                                let #field_idnt = parser.get_array();
                            });
                            args.push(quote! {
                                #field_idnt
                            });
                            log_msg.push_str("{:?}, ");
                            ev_lifetime = true;
                            quote! { &'a [u32] }
                        },
                        parser::ArgType::Fd => {
                            variant_parse.push(quote!{
                                let #field_idnt = reader.get_fd().unwrap();
                            });
                            args.push(quote! {
                                #field_idnt
                            });
                            log_msg.push_str("{:?}, ");
                            quote! { std::os::fd::OwnedFd }
                        },
                    };
                    fields.push(quote! {
                        #field_idnt
                    });
                    ev_fields.push(quote!{
                        #field_idnt: #field_type
                    });
                }
                let log_msg = {
                    let msg = log_msg.trim_end();
                    let mut end = msg.len();
                    if let Some(',') = msg.chars().last() {
                        end -= 1;
                    }
                    format!("{})", &msg[..end])
                };
                variant_parse.push(quote! {
                    {
                        log!(WAYLAND, #log_msg, self, #(#args,)*);
                    }
                    Self::Event::#ev_idnt { #(#fields,)* }
                });

                ev_variants.push(quote!{
                    #ev_idnt {
                        #(#ev_fields,)*
                    }
                });
            }
            ev_parse.push(quote!{
                #i => {
                    #(#variant_parse)*
                }
            });
        }

        let ev_lifetime = if ev_lifetime { quote! {<'a>} } else { quote! {} };

        let mut event_enum = quote!{
            #[derive(Debug)]
            pub enum Event #ev_lifetime {
                 #(#ev_variants,)*
            }
        };

        let mut enums = Vec::<TokenStream>::new();
        for en in &o.enums {
            let en_idnt = Ident::new(&en.name.snake_to_pascal(), Span::call_site());
            if en.is_bitfield {
                for e in &en.items {
                    let e_idnt = format_ident!("{}_{}", en.name.to_uppercase(), e.name.to_uppercase());
                    let val = e.value;
                    enums.push(quote! {
                        pub const #e_idnt: u32 = #val;
                    })
                }
            } else {
                let mut variants = Vec::new();
                for e in &en.items {
                    let e_idnt = {
                        if let Some('A'..'z') = e.name.chars().next() {
                            format_ident!("{}", e.name.snake_to_pascal())
                        } else {
                            format_ident!("_{}", e.name.snake_to_pascal())
                        }
                    };
                    let val = e.value;
                    variants.push(quote! {
                        #e_idnt = #val
                    })
                }
                enums.push(quote! {
                    #[repr(u32)]
                    #[derive(Debug, Copy, Clone, PartialEq)]
                    pub enum #en_idnt {
                        #(#variants,)*
                    }
                    impl PartialEq<#en_idnt> for u32 {
                        fn eq(&self, other: &#en_idnt) -> bool {
                            *self == *other as u32
                        }
                    }
                    impl PartialEq<u32> for #en_idnt {
                        fn eq(&self, other: &u32) -> bool {
                            *self as u32 == *other
                        }
                    }
                    impl Eq for #en_idnt {}
                })
            }
        }
        // fucking hell
        let parse_body = if ev_parse.is_empty() {
            quote! {
                unreachable!();
            }
        } else {
            quote! {
                let parser = event.parser();
                match event.header.opcode {
                    #(#ev_parse)*,
                    _ => {
                        unreachable!();
                    }
                }
            }
        };
        quote! {

            pub mod #iface_mod {
                use super::*;
                pub struct #object_name {
                    id: u32,
                    interface: &'static str
                }
                impl ::std::fmt::Display for #object_name {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_fmt(format_args!("{}#{}", Self::INTERFACE, self.id))
                    }
                }
                impl ::std::fmt::Debug for #object_name {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_fmt(format_args!("{}#{}", Self::INTERFACE, self.id))
                    }
                }
                #event_enum
                #(#enums)*
                impl #object_name {
                    pub const INTERFACE: &'static str = #iface_name;
                    #(#reqs)*
                }
                impl Object for #object_name {
                    type Event<'a> = Event #ev_lifetime;
                    fn from_id(id: u32) -> Self {
                        Self {
                            id,
                            interface: Self::INTERFACE
                        }
                    }
                    fn id(&self) -> u32 {
                        self.id
                    }
                    fn interface(&self) -> &'static str {
                        Self::INTERFACE
                    }
                    fn parse_event<'a>(&self, reader: &WaylandBuffer<Reader>, event: WlEvent<'a>) -> Self::Event<'a> {
                        #parse_body
                    }
                }
            }
        }

    });

    interfaces.collect::<TokenStream>().into()
}

trait SnakeToPascal {
    fn snake_to_pascal(&self) -> String;
}

impl<T: AsRef<str>> SnakeToPascal for T {
    fn snake_to_pascal(&self) -> String {
        let str = self.as_ref();
        let mut output = String::with_capacity(str.len());
        let mut capatilize_next = false;
        for (i, c) in str.chars().enumerate() {
            if c == '_' {
                capatilize_next = true;
                continue;
            }
            if capatilize_next || i == 0 {
                output.push(c.to_ascii_uppercase());
                capatilize_next = false;
            } else {
                output.push(c)
            }
        }
        output
    }
}
