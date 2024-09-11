use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Schema {
    data: SchemaData,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct SchemaData {
    #[serde(rename = "__schema")]
    schema: SchemaDetails,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct SchemaDetails {
    query_type: QueryType,
    mutation_type: Option<QueryType>,
    subscription_type: Option<QueryType>,
    #[serde(serialize_with = "serialize_sorted_vec")]
    types: Option<Vec<TypeDetails>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct QueryType {
    name: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TypeDetails {
    kind: String,
    name: Option<String>,
    #[serde(serialize_with = "serialize_sorted_vec")]
    fields: Option<Vec<Field>>,
    #[serde(serialize_with = "serialize_sorted_vec")]
    input_fields: Option<Vec<InputValue>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Field {
    name: String,
    #[serde(serialize_with = "serialize_sorted_vec")]
    args: Option<Vec<InputValue>>,
    #[serde(rename = "type")]
    type_: TypeRef,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InputValue {
    name: String,
    #[serde(rename = "type")]
    type_: TypeRef,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct TypeRef {
    kind: String,
    name: Option<String>,
    of_type: Option<Box<TypeRef>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct EnumValue {
    name: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Directive {
    name: String,
    #[serde(serialize_with = "serialize_sorted_vec")]
    locations: Option<Vec<String>>,
    #[serde(serialize_with = "serialize_sorted_vec")]
    args: Option<Vec<InputValue>>,
}

// Add this function at the end of the file
fn serialize_sorted_vec<S, T>(vec: &Option<Vec<T>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
    T: Serialize + Ord + Clone,
{
    match vec {
        Some(v) => {
            let mut sorted = v.clone();
            sorted.sort();
            sorted.serialize(serializer)
        }
        None => serializer.serialize_none(),
    }
}