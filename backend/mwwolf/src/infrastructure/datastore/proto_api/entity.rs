use super::api;
use super::error::ConvertError;
use super::{IntoValue, Key, Value};

#[derive(Debug, Clone, PartialEq)]
pub struct Entity {
    pub(crate) key: Key,
    pub(crate) properties: Value,
}

impl Entity {
    pub fn new(key: Key, value: impl IntoValue) -> Result<Entity, ConvertError> {
        let properties = value.into_value();
        match properties {
            Value::Entity(_) => Ok(Entity { key, properties }),
            _ => Err(ConvertError::UnexpectedPropertyType {
                expected: String::from("entity"),
                got: String::from(properties.type_name()),
            }),
        }
    }

    pub fn into_key(self) -> Key {
        self.key
    }

    pub fn key(&self) -> &Key {
        &self.key
    }

    pub fn into_properties(self) -> Value {
        self.properties
    }

    pub fn properties(&self) -> &Value {
        &self.properties
    }

    pub fn properties_mut(&mut self) -> &mut Value {
        &mut self.properties
    }
}

pub trait IntoEntity {
    fn into_entity(self) -> Result<Entity, ConvertError>;
}
pub trait FromEntity: Sized {
    fn from_entity(e: Entity) -> Result<Self, ConvertError>;
}

impl IntoEntity for Entity {
    fn into_entity(self) -> Result<Entity, ConvertError> {
        Ok(self)
    }
}

impl<V> IntoEntity for (Key, V)
where
    V: IntoValue,
{
    fn into_entity(self) -> Result<Entity, ConvertError> {
        let (k, v) = self;
        Entity::new(k, v)
    }
}

impl From<api::Entity> for Entity {
    fn from(entity: api::Entity) -> Entity {
        let key = Key::from(entity.key.unwrap());
        let properties = entity.properties;

        let properties = properties
            .into_iter()
            .map(|(k, v)| (k, Value::from(v.value_type.unwrap())))
            .collect();
        let properties = Value::Entity(properties);

        Entity { key, properties }
    }
}
