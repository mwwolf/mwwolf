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
