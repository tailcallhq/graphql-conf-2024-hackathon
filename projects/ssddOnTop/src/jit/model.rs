use std::borrow::Cow;
use std::fmt::{Debug, Formatter};
use std::future::Future;
use std::pin::Pin;
use crate::blueprint::model::{FieldName, TypeName};
use crate::blueprint::{Blueprint, FieldHash};
use crate::ir::IR;
use crate::value::Value;
use async_graphql::parser::types::{DocumentOperations, ExecutableDocument, OperationType, Selection, SelectionSet};
use async_graphql::Positioned;
use serde_json::Map;
use crate::ir::eval_ctx::EvalContext;

pub struct PathFinder<'a> {
    doc: ExecutableDocument,
    blueprint: &'a Blueprint,
}
#[derive(Debug)]
pub struct Fields {
    fields: Vec<Field>,
}

// #[derive(Debug)]
// TODO: give it a lifetime
// it won't make much difference..
// but anyways
pub struct Field {
    ir: Option<IR>,
    pub name: String,
    pub type_of: crate::blueprint::wrapping_type::Type,
    nested: Vec<Field>,
    pub args: Option<Value>,
    pub resolved: Option<Value>,
}

pub struct Field1<'a> {
    value: serde_json_borrow::Value<'a>,
    pub name: &'a str,
    pub type_of: crate::blueprint::wrapping_type::Type,
    nexted: Vec<Field1<'a>>,
    pub args: Option<Value>,
}

impl Debug for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut debug_struct = f.debug_struct("Field");
        debug_struct.field("name", &self.name);
        if self.ir.is_some() {
            debug_struct.field("ir", &"Some(..)");
        }
        debug_struct.field("type_of", &self.type_of);
        if self.args.is_some() {
            debug_struct.field("args", &self.args);
        }
        if self.resolved.is_some() {
            debug_struct.field("resolved", &self.resolved.as_ref().map(|v| v.serde()));
        }
        debug_struct.field("nested", &self.nested);
        debug_struct.finish()
    }
}

impl Fields {
    #[inline(always)]
    pub fn finalize(self) -> serde_json::Value {
        let mut map = Map::new();
        for field in self.fields {
            if field.nested.is_empty() {
                map.insert(field.name.clone(), field.resolved.unwrap_or(Value::new(serde_json::Value::Null)).into_serde());
            } else {
                let nested_value = Fields { fields: field.nested }.finalize();
                map.insert(field.name.clone(), nested_value);
            }
        }
        let mut data = Map::new();
        data.insert("data".to_string(), serde_json::Value::Object(map));
        serde_json::Value::Object(data)
    }
    #[inline(always)]
    pub async fn resolve<'a>(mut self, eval_context: EvalContext<'a>) -> anyhow::Result<Fields> {
        let mut ans = vec![];
        ans = Self::resolve_inner(self.fields, eval_context).await?;
        Ok(Fields {
            fields: ans,
        })
    }

    #[inline(always)]
    fn resolve_inner<'a>(fields: Vec<Field>, mut eval_context: EvalContext<'a>) -> Pin<Box<dyn Future<Output = anyhow::Result<Vec<Field>>> + Send + 'a>> {
        Box::pin(async move {
            let mut ans = vec![];
            for mut field in fields {
                if let Some(ir) = field.ir.clone() {
                    if let Some(val) = field.args.clone() {
                        eval_context = eval_context.with_args(val);
                    }
                    let val = ir.eval(&mut eval_context.clone()).await?;
                    eval_context = eval_context.with_value(val.clone());
                    field.resolved = Some(val);
                } else {
                    // println!("{:?}", eval_context.graphql_ctx_value);
                    let val = eval_context.path_value(&[field.name.as_str()]);
                    // println!("{}", val.is_some());
                    let val = val.unwrap_or(Cow::Owned(Value::new(serde_json::Value::Null))).into_owned();

                    // println!("field: {} val: {}",field.name, val);
                    // eval_context = eval_context.with_value(val.clone());
                    field.resolved = Some(val);
                }
                field.nested = Self::resolve_inner(field.nested, eval_context.clone()).await?;
                ans.push(field);
            }
            Ok(ans)
        })
    }
}

pub struct Holder<'a> {
    field_name: &'a str,
    field_type: &'a str,
    args: Vec<(&'a str, Value)>,
}

impl<'a> PathFinder<'a> {
    pub fn new(doc: ExecutableDocument, blueprint: &'a Blueprint) -> Self {
        Self { doc, blueprint }
    }
    pub async fn exec(&'a self) -> Fields {
        match &self.doc.operations {
            DocumentOperations::Single(single) => {
                let operation = &single.node;
                let selection_set = &operation.selection_set.node;
                let ty = match &operation.ty {
                    OperationType::Query => {
                        let query = self.blueprint.schema.query.as_ref().map(|v| v.as_str());
                        query
                    }
                    OperationType::Mutation => None,
                    OperationType::Subscription => None,
                };
                if let Some(ty) = ty {
                    Fields {
                        fields: self.iter(selection_set, ty),
                    }
                } else {
                    Fields {
                        fields: vec![],
                    }
                }
            }
            DocumentOperations::Multiple(_) => todo!()
        }
    }
    #[inline(always)]
    fn iter(
        &self,
        selection: &SelectionSet,
        type_condition: &str,
    ) -> Vec<Field> {
        let mut fields = vec![];
        for selection in &selection.items {
            match &selection.node {
                Selection::Field(Positioned { node: gql_field, .. }) => {
                    // let conditions = self.include(&gql_field.directives);

                    /*for directive in &gql_field.directives {
                        let directive = &directive.node;
                        if directive.name.node == "skip" || directive.name.node == "include" {
                            continue;
                        }
                        let arguments = directive
                            .arguments
                            .iter()
                            .map(|(k, v)| (k.node.to_string(), v.node.clone()))
                            .collect::<Vec<_>>();
                        // println!("directive args: {:?}", arguments);
                    }*/

                    // let (include, skip) = conditions.into_variable_tuple();

                    let field_name = gql_field.name.node.as_str();
                    let request_args = gql_field
                        .arguments
                        .iter()
                        .map(|(k, v)| (k.node.as_str().to_string(), v.node.to_owned().into_const().map(|v| v.into_json().ok()).flatten().unwrap()))
                        .collect::<Map<_, _>>();

                    // println!("req args: {:?}", request_args);
                    // println!("{}: {}",field_name, type_condition);
                    // println!("{:#?}", self.blueprint.fields);

                    if let Some(field_def) = self.blueprint.fields.get(&FieldHash::new(FieldName(field_name.to_string()), TypeName(type_condition.to_string()))) {
                        // let mut args = Vec::with_capacity(request_args.len());
                        /*                 if let QueryField::Field((_, schema_args)) = field_def {
                                         for (arg_name, arg_value) in schema_args {
                                             let type_of = arg_value.of_type.clone();
                                             let id = ArgId::new(self.arg_id.next());
                                             let name = arg_name.clone();
                                             let default_value = arg_value
                                                 .default_value
                                                 .as_ref()
                                                 .and_then(|v| v.to_owned().try_into().ok());
                                             args.push(Arg {
                                                 id,
                                                 name,
                                                 type_of,
                                                 // TODO: handle errors for non existing request_args without the
                                                 // default
                                                 value: request_args.get(arg_name).cloned(),
                                                 default_value,
                                             });
                                         }
                                     }*/

                        /*            let type_of = match field_def {
                                        QueryField::Field((field_def, _)) => field_def.of_type.clone(),
                                        QueryField::InputField(field_def) => field_def.of_type.clone(),
                                    };

                                    let id = FieldId::new(self.field_id.next());*/
                        let type_of = field_def.type_of.clone();
                        let child_fields = self.iter(
                            &gql_field.selection_set.node,
                            type_of.name(),
                        );
                        let field = Field {
                            ir: field_def.ir.clone(),
                            name: field_def.name.as_ref().to_string(),
                            type_of,
                            nested: child_fields,
                            args: match request_args.is_empty() {
                                false => Some(Value::new(serde_json::Value::Object(request_args))),
                                true => None,
                            },
                            resolved: None,
                        };

                        /*                 let ir = match field_def {
                                             QueryField::Field((field_def, _)) => field_def.resolver.clone(),
                                             _ => None,
                                         };
                                         let flat_field = Field {
                                             id,
                                             name: field_name.to_string(),
                                             output_name: gql_field
                                                 .alias
                                                 .as_ref()
                                                 .map(|a| a.node.to_string())
                                                 .unwrap_or(field_name.to_owned()),
                                             ir,
                                             type_of,
                                             type_condition: Some(type_condition.to_string()),
                                             skip,
                                             include,
                                             args,
                                             pos: selection.pos.into(),
                                             extensions: exts.clone(),
                                             directives,
                                         };*/

                        fields.push(field);
                        // fields = fields.merge_right(child_fields);
                    } /*else if field_name == "__typename" {
                        let flat_field = Field {
                            id: FieldId::new(self.field_id.next()),
                            name: field_name.to_string(),
                            output_name: field_name.to_string(),
                            ir: None,
                            type_of: Type::Named { name: "String".to_owned(), non_null: true },
                            // __typename has a special meaning and could be applied
                            // to any type
                            type_condition: None,
                            skip,
                            include,
                            args: Vec::new(),
                            pos: selection.pos.into(),
                            extensions: exts.clone(),
                            directives,
                        };

                        fields.push(flat_field);
                    }*/
                }
                _ => (),
            }
        }

        fields
    }
}