use crate::blueprint::model::{Arg, ArgId, Field, FieldId, FieldName, TypeName};
use crate::config::Config;
use std::collections::HashMap;


#[derive(Debug, Eq, Hash, PartialEq)]
pub struct FieldHash {
    pub name: FieldName,
    pub id: TypeName,
}

#[derive(Debug)]
pub struct Blueprint {
    pub fields: HashMap<FieldHash, Field>,
    pub server: Server,
    pub upstream: Upstream,
}

#[derive(Debug)]
pub struct Server {
    pub port: u16,
}
#[derive(Debug)]
pub struct Upstream {
    pub base_url: Option<String>,
}

impl TryFrom<&Config> for Blueprint {
    type Error = anyhow::Error;

    fn try_from(config: &Config) -> Result<Self, Self::Error> {
        let qry = config.schema.query.as_ref().ok_or(anyhow::anyhow!("Query not found"))?;
        let fields = fields_to_map(qry, config);
        let server = Server {
            port: config.server.port,
        };
        let upstream = Upstream {
            base_url: config.upstream.base_url.clone(),
        };
        Ok(Blueprint {
            fields,
            server,
            upstream,
        })
    }
}

fn fields_to_map(qry: &str, config: &Config) -> HashMap<FieldHash, Field> {
    let mut fields = HashMap::new();
    populate_nested_field(config, qry, &mut fields);
    fields
}

fn populate_nested_field(
    config: &Config,
    ty_name: &str,
    field_map: &mut HashMap<FieldHash, Field>,
) {
    // I don't have additional check for scalars as of now..
    // This should work fine
    if let Some(ty) = config.types.get(ty_name) {
        for (field_name, field) in ty.fields.iter() {
            let field_name = FieldName(field_name.clone());
            populate_nested_field(config, field.ty_of.name(), field_map);
            let mut arg_id = 0;
            let field = Field {
                id: FieldId::new(0),
                name: field_name.clone(),
                type_of: field.ty_of.clone(),
                args: field.args.iter().map(|(arg_name, arg)| {
                    let arg = Arg {
                        id: ArgId::new(arg_id),
                        name: arg_name.clone(),
                        type_of: arg.type_of.clone(),
                    };
                    arg_id += 1;

                    arg
                }).collect(),
            };

            field_map.insert(FieldHash {
                name: field_name,
                id: TypeName(ty_name.to_string()),
            }, field);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::config::ConfigReader;

    #[test]
    fn test() {
        let reader = ConfigReader::init();
        let root = env!("CARGO_MANIFEST_DIR");
        let config = reader.read(format!("{}/schema/schema.graphql",root)).unwrap();
        let blueprint = crate::blueprint::Blueprint::try_from(&config).unwrap();
        println!("{:#?}", blueprint);
    }
}