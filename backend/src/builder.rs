use std::{iter::Map, slice::Iter};

use darling::FromDeriveInput;
use heck::ToUpperCamelCase;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{
    Data, DataStruct, DeriveInput, Fields, FieldsNamed, GenericArgument, Path, PathArguments, Type,
    TypePath,
};

type TokenStreamIter<'a> = Map<Iter<'a, Fd>, fn(&'a Fd) -> TokenStream>;

#[derive(Debug, Default, FromDeriveInput)]
#[darling(default, attributes(builder))]
struct Table {
    name: String,
}

pub struct BuilderContext {
    name: Ident,
    table_name: String,
    fields: Vec<Fd>,
}

struct Fd {
    tname: String,
    name: Ident,
    ty: Type,
}

impl BuilderContext {
    pub fn new(input: DeriveInput) -> Self {
        let table_name = Table::from_derive_input(&input).unwrap().name;
        let name = input.ident;
        let fields = if let Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { named, .. }),
            ..
        }) = input.data
        {
            named
        } else {
            panic!("Unsupported data type");
        };

        let fields = fields
            .into_iter()
            .map(|f| Fd {
                tname: table_name.clone(),
                name: f.ident.unwrap(),
                ty: f.ty,
            })
            .collect();

        Self {
            name,
            table_name,
            fields,
        }
    }

    pub fn generate(&self) -> TokenStream {
        let table_name = &self.table_name;
        let fn_name = String::from("init_") + table_name;
        let cols = self.gen_cols();

        let ast = quote! {
            use sea_orm::sea_query::{self, ColumnDef, TableCreateStatement};
            use sea_orm::{ConnectionTrait, DbConn, DbErr, ExecResult};
            pub async fn #fn_name(db: DbConn) -> Result<ExecResult, DbErr> {

                let stmt = sea_query::Table::create()
                    .table(crate::entity::#table_name::Entity)
                    .if_not_exists()
                    #(#cols)*
                    .to_owned();

                let builder = db.get_database_backend();
                db.execute(builder.build(stmt)).await
            }
        };
        ast.into()
    }

    fn gen_cols(&self) -> TokenStreamIter {
        self.fields.iter().map(|f| {
            let tname = Ident::new(&f.tname, f.name.span());
            let cname = &f.name.to_string().to_upper_camel_case();
            let (_is_options, ty) = get_option_inner(&f.ty);
            let tyname = get_type_name(ty);
            if tyname == "i32" || tyname == "i64" {
                quote! {
                    .col(ColumnDef::new(crate::entity::#tname::Column::#cname)
                         .integer()
                         .not_null()
                         .auto_increment()
                         .primary_key(),
                         )
                }
            } else if tyname == "String" {
                quote! {
                    .col(ColumnDef::new(crate::entity::#tname::Column::#cname)
                         .string()
                         .not_null()
                         )
                }
            } else if tyname == "DateTime" {
                quote! {
                    .col(ColumnDef::new(crate::entity::#tname::Column::#cname)
                         .date_time()
                         .not_null()
                         )
                }
            } else if tyname == "bool" {
                quote! {
                    .col(ColumnDef::new(crate::entity::#tname::Column::#cname)
                         .boolean()
                         .not_null()
                         )
                }
            } else {
                panic!("now Unsupported type {:#?}", tyname)
            }
        })
    }
}

fn get_option_inner(ty: &Type) -> (bool, &Type) {
    get_type_inner(ty, "Option")
}

fn get_type_inner<'a>(ty: &'a Type, name: &str) -> (bool, &'a Type) {
    if let Type::Path(TypePath {
        path: Path { segments, .. },
        ..
    }) = ty
    {
        if let Some(v) = segments.iter().next() {
            if v.ident == name {
                let t = match &v.arguments {
                    PathArguments::AngleBracketed(a) => match a.args.iter().next() {
                        Some(GenericArgument::Type(t)) => t,
                        _ => panic!("Not sure what to do with other GenericArgument"),
                    },
                    _ => panic!("Not sure what to do with other PathArguments"),
                };
                return (true, t);
            }
        }
    }

    return (false, ty);
}

fn get_type_name(ty: &Type) -> &Ident {
    if let Type::Path(TypePath {
        path: Path { segments, .. },
        ..
    }) = ty
    {
        if let Some(v) = segments.iter().next() {
            return &v.ident;
        }
    }

    panic!("Unsupported")
}
