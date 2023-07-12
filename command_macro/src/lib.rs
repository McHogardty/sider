


use convert_case::{Casing, Case};
use devise::{FromMeta, MetaItem, Result};
use devise::ext::{SpanDiagnosticExt};
#[allow(non_camel_case_types)]

use proc_macro::TokenStream;
use proc_macro2::{self, TokenTree};
use quote::{quote, ToTokens, format_ident};


use sider_command::{AclCategory, Flag, CommandTip, RequestPolicyTipOption, ResponsePolicyTipOption};

#[derive(Debug)]
struct FlagToken(Flag);


impl ToTokens for FlagToken {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let f = match self.0 {
            Flag::Fast => "Fast",
            Flag::Sentinel => "Sentinel",
        };

        let result = format_ident!("{}", f);

        tokens.extend(quote!{ Flag::#result });
    }
}


#[derive(Debug)]
struct AclCategoryToken(AclCategory);

impl ToTokens for AclCategoryToken {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let c = match self.0 {
            AclCategory::Connection => "Connection",
        };

        let result = format_ident!("{}", c);

        tokens.extend(quote!{ AclCategory::#result });
    }
}



#[derive(Debug)]
struct Flags {
    inner: Vec<FlagToken>
}


impl ToTokens for Flags {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let f = &self.inner;

        let result = quote! {
            &[ #( #f ),* ]
        };

        tokens.extend(result);
    }
}



impl FromMeta for Flags {
    fn from_meta(meta: &MetaItem) -> Result<Self> {      
        let result: proc_macro2::Group = meta.parse_value("expected a comma-delimited list of strings")?;

        let mut flags = vec![];
        
        for group in result.stream() {
            if let TokenTree::Literal(s) = group {
                let syn::Lit::Str(v) = syn::parse2(s.to_token_stream())? else {
                    return Err(s.span().error("expected a string"));
                };

                let Ok(f) = Flag::try_from(v.value()) else {
                    return Err(s.span().error("expected a valid Flag value"))
                };

                flags.push(FlagToken(f));
            } else if let TokenTree::Punct(s) = group {
                if s.as_char() != ',' {
                    return Err(s.span().error("expected a comma-delimited list of strings"));
                }

                continue;
            } else {
                return Err(group.span().error("expected a comma-delimited list of strings"));
            }
        }

        Ok(Flags{inner: flags})
    }
}


#[derive(Debug)]
struct AclCategories {
    inner: Vec<AclCategoryToken>
}

impl ToTokens for AclCategories {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let f = &self.inner;

        let result = quote! {
            &[ #( #f ),* ]
        };

        tokens.extend(result);
    }
}

impl FromMeta for AclCategories {
    fn from_meta(meta: &MetaItem) -> Result<Self> {      
        let result: proc_macro2::Group = meta.parse_value("expected a comma-delimited list of strings")?;

        let mut categories = vec![];
        
        for group in result.stream() {
            if let TokenTree::Literal(s) = group {
                let syn::Lit::Str(v) = syn::parse2(s.to_token_stream())? else {
                    return Err(s.span().error("expected a string"));
                };

                let Ok(c) = AclCategory::try_from(v.value()) else {
                    return Err(s.span().error("expected a valid AclCategory value"))
                };

                categories.push(AclCategoryToken(c));
            } else if let TokenTree::Punct(s) = group {
                if s.as_char() != ',' {
                    return Err(s.span().error("expected a comma-delimited list of strings"));
                }

                continue;
            } else {
                return Err(group.span().error("expected a comma-delimited list of strings"));
            }
        }

        Ok(AclCategories {inner: categories})
    }
}


#[derive(Debug)]
struct CommandTipToken(CommandTip);

impl ToTokens for CommandTipToken {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let result = match &self.0 {
            CommandTip::NonDeterministicOutput => {
                let i = format_ident!("NonDeterministicOutput");
                quote!{ CommandTip::#i }
            }
            CommandTip::NonDeterministicOutputOrder => {
                let i = format_ident!("NonDeterministicOutputOrder");
                quote!{ CommandTip::#i }
            }
            CommandTip::RequestPolicy(o) => {
                let i = format_ident!("RequestPolicy");

                let option_name = match o {
                    RequestPolicyTipOption::AllNodes => "AllNodes",
                    RequestPolicyTipOption::AllShards => "AllShards",
                    RequestPolicyTipOption::MultiShard => "MultiShard",
                    RequestPolicyTipOption::Special => "Special",
                    
                };

                let option_token = format_ident!("{}", option_name);

                quote!{ CommandTip::#i(RequestPolicyTipOption::#option_token) }
            },
            CommandTip::ResponsePolicy(o) => {
                let i = format_ident!("ResponsePolicy");

                let option_name = match o {
                    ResponsePolicyTipOption::OneSucceeded => "OneSucceeded",
                    ResponsePolicyTipOption::AllSucceeded => "AllSucceeded",
                    ResponsePolicyTipOption::AggLogicalAnd => "AggLogicalAnd",
                    ResponsePolicyTipOption::AggLogicalOr => "AggLogicalOr",
                    ResponsePolicyTipOption::AggMin => "AggMin",
                    ResponsePolicyTipOption::AggMax => "AggMax",
                    ResponsePolicyTipOption::AggSum => "AggSum",
                    ResponsePolicyTipOption::Special => "Special",
                };

                let option_token = format_ident!("{}", option_name);

                quote!{ CommandTip::#i(ResponsePolicyTipOption::#option_token) }
            },
        };

        tokens.extend(result);
    }
}


#[derive(Debug)]
struct CommandTips {
    inner: Vec<CommandTipToken>,
}

impl ToTokens for CommandTips {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let f = &self.inner;

        let result = quote! {
            &[ #( #f ),* ]
        };

        tokens.extend(result);
    }
}



impl FromMeta for CommandTips {
    fn from_meta(meta: &MetaItem) -> Result<Self> {      
        let result: proc_macro2::Group = meta.parse_value("expected a comma-delimited list of strings")?;

        let mut tips = vec![];
        
        for group in result.stream() {
            if let TokenTree::Literal(s) = group {
                let syn::Lit::Str(v) = syn::parse2(s.to_token_stream())? else {
                    return Err(s.span().error("expected a string"));
                };

                let Ok(c) = CommandTip::try_from(v.value()) else {
                    return Err(s.span().error("expected a valid CommandTip value"))
                };

                tips.push(CommandTipToken(c));
            } else if let TokenTree::Punct(s) = group {
                if s.as_char() != ',' {
                    return Err(s.span().error("expected a comma-delimited list of strings"));
                }

                continue;
            } else {
                return Err(group.span().error("expected a comma-delimited list of strings"));
            }
        }

        Ok(CommandTips {inner: tips})
    }
}



#[derive(Debug)]
struct Integer(i64);

impl FromMeta for Integer {
    fn from_meta(meta: &MetaItem) -> Result<Self> {
        if let syn::Expr::Unary(u) = meta.expr()? {
            let syn::UnOp::Neg(_) = u.op else {
                return Err(meta.value_span().error("expected positive or negative integer"));
            };

            let syn::Expr::Lit(v) = *u.expr else {
                return Err(meta.value_span().error("expected positive or negative integer"));
            };

            let syn::Lit::Int(v) = v.lit else {
                return Err(meta.value_span().error("expected positive or negative integer"));
            };

            return Ok(Integer(v.base10_parse()?))
        } else if let syn::Lit::Int(v) = meta.lit()? {
            return Ok(Integer(v.base10_parse()?))
        }

        Err(meta.value_span().error("expected positive or negative integer"))
    }
}


#[derive(Debug, FromMeta)]
struct CommandAttribute {
    name: String,
    arity: Integer,
    flags: Flags,
    first_key: usize,
    last_key: Integer,
    step: usize,
    acl_categories: AclCategories,
    command_tips: CommandTips,
}


fn command_attribute(attr: proc_macro2::TokenStream, item: proc_macro2::TokenStream) -> Result<proc_macro2::TokenStream> {
    
    let full_attr = quote!(command(#attr));
    let attribute = CommandAttribute::from_meta(&syn::parse2(full_attr).unwrap())?;

    let command_handler: syn::ItemFn = syn::parse2(item).unwrap();

    let function_name = &command_handler.sig.ident;
    let struct_name: proc_macro2::TokenStream = function_name.to_string().to_case(Case::UpperCamel).parse().unwrap(); 

    let command_name = attribute.name;
    let command_arity = attribute.arity.0;
    let command_flags = attribute.flags;
    let first_key = attribute.first_key as u64;
    let last_key = attribute.last_key.0;
    let step = attribute.step as u64;
    let categories = attribute.acl_categories;
    let tips = attribute.command_tips;

    Ok(quote! {
        use sider_command::*;
        use super::base::Command;

        #command_handler

        pub(crate) struct #struct_name {}

        impl<'a, 'b> #struct_name {
            pub(crate) const fn into_command() -> Command<'a> {
                Command {
                    name: #command_name,
                    handler: #function_name,
                    arity: #command_arity,
                    flags: #command_flags,
                    first_key: #first_key,
                    last_key: #last_key,
                    step: #step,
                    acl_categories: #categories,
                    tips: #tips,
                }
            }
        }
    })
}


#[proc_macro_attribute]
pub fn command(attr: TokenStream, item: TokenStream) -> TokenStream {
    let result = command_attribute(attr.into(), item.into());

    result.unwrap_or_else(|diag| diag.emit_as_item_tokens().into()).into()
}