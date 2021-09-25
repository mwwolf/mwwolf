use super::*;
use std::collections::HashMap;
pub fn remove_value<T: proto_api::FromValue>(
    mut vmap: HashMap<String, proto_api::Value>,
    key: &str,
) -> Result<(HashMap<String, proto_api::Value>, T), proto_api::ConvertError> {
    let v = vmap
        .remove(key)
        .ok_or_else(|| proto_api::ConvertError::MissingProperty(key.into()))?;
    Ok((vmap, T::from_value(v)?))
}

#[allow(dead_code)]
pub fn into_entity(
    e: impl proto_api::IntoEntity,
    namespace: &str,
) -> Result<proto_api::Entity, proto_api::ConvertError> {
    let mut entity = e.into_entity()?;
    entity.key = entity.key.namespace(namespace);
    Ok(entity)
}

pub fn kind<T>() -> String {
    std::any::type_name::<T>()
        .split("::")
        .last()
        .unwrap()
        .into()
}
