use std::collections::HashMap;
use std::iter::FromIterator;

use chrono::NaiveDateTime;

use bytes::Bytes;

use crate::datastore::proto_api::api::value::ValueType;
use crate::datastore::proto_api::error::ConvertError;
use crate::datastore::proto_api::Key;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Boolean(bool),

    Integer(i64),

    Double(f64),

    Timestamp(chrono::NaiveDateTime),

    Key(Key),

    Strings(String),

    Blob(Vec<u8>),

    GeoPoint(f64, f64),

    Entity(HashMap<String, Value>),

    Array(Vec<Value>),
}

impl Value {
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Boolean(_) => "bool",
            Value::Integer(_) => "integer",
            Value::Double(_) => "double",
            Value::Timestamp(_) => "timestamp",
            Value::Key(_) => "key",
            Value::Strings(_) => "string",
            Value::Blob(_) => "blob",
            Value::GeoPoint(_, _) => "geopoint",
            Value::Entity(_) => "entity",
            Value::Array(_) => "array",
        }
    }
}

pub trait IntoValue {
    fn into_value(self) -> Value;
}

pub trait FromValue: Sized {
    fn from_value(value: Value) -> Result<Self, ConvertError>;
}

impl IntoValue for Value {
    fn into_value(self) -> Value {
        self
    }
}

impl IntoValue for String {
    fn into_value(self) -> Value {
        Value::Strings(self)
    }
}

impl IntoValue for &str {
    fn into_value(self) -> Value {
        String::from(self).into_value()
    }
}

impl IntoValue for i8 {
    fn into_value(self) -> Value {
        Value::Integer(self as i64)
    }
}

impl IntoValue for i16 {
    fn into_value(self) -> Value {
        Value::Integer(self as i64)
    }
}

impl IntoValue for i32 {
    fn into_value(self) -> Value {
        Value::Integer(self as i64)
    }
}

impl IntoValue for i64 {
    fn into_value(self) -> Value {
        Value::Integer(self)
    }
}

impl IntoValue for f32 {
    fn into_value(self) -> Value {
        Value::Double(self as f64)
    }
}

impl IntoValue for f64 {
    fn into_value(self) -> Value {
        Value::Double(self)
    }
}

impl IntoValue for bool {
    fn into_value(self) -> Value {
        Value::Boolean(self)
    }
}

impl IntoValue for Key {
    fn into_value(self) -> Value {
        Value::Key(self)
    }
}

impl IntoValue for NaiveDateTime {
    fn into_value(self) -> Value {
        Value::Timestamp(self)
    }
}

impl IntoValue for Bytes {
    fn into_value(self) -> Value {
        Value::Blob(self.to_vec())
    }
}

impl<T> IntoValue for Vec<T>
where
    T: IntoValue,
{
    fn into_value(self) -> Value {
        Value::Array(self.into_iter().map(IntoValue::into_value).collect())
    }
}

impl<T> IntoValue for HashMap<String, T>
where
    T: IntoValue,
{
    fn into_value(self) -> Value {
        Value::Entity(self.into_iter().map(|(k, v)| (k, v.into_value())).collect())
    }
}

impl<T> FromIterator<T> for Value
where
    T: IntoValue,
{
    fn from_iter<I>(iter: I) -> Value
    where
        I: IntoIterator<Item = T>,
    {
        Value::Array(iter.into_iter().map(IntoValue::into_value).collect())
    }
}

impl FromValue for Value {
    fn from_value(value: Value) -> Result<Value, ConvertError> {
        Ok(value)
    }
}

impl FromValue for String {
    fn from_value(value: Value) -> Result<String, ConvertError> {
        match value {
            Value::Strings(value) => Ok(value),
            _ => Err(ConvertError::UnexpectedPropertyType {
                expected: String::from("string"),
                got: String::from(value.type_name()),
            }),
        }
    }
}

impl FromValue for i64 {
    fn from_value(value: Value) -> Result<i64, ConvertError> {
        match value {
            Value::Integer(value) => Ok(value),
            _ => Err(ConvertError::UnexpectedPropertyType {
                expected: String::from("integer"),
                got: String::from(value.type_name()),
            }),
        }
    }
}

impl FromValue for f64 {
    fn from_value(value: Value) -> Result<f64, ConvertError> {
        match value {
            Value::Double(value) => Ok(value),
            _ => Err(ConvertError::UnexpectedPropertyType {
                expected: String::from("double"),
                got: String::from(value.type_name()),
            }),
        }
    }
}

impl FromValue for bool {
    fn from_value(value: Value) -> Result<bool, ConvertError> {
        match value {
            Value::Boolean(value) => Ok(value),
            _ => Err(ConvertError::UnexpectedPropertyType {
                expected: String::from("bool"),
                got: String::from(value.type_name()),
            }),
        }
    }
}

impl FromValue for Key {
    fn from_value(value: Value) -> Result<Key, ConvertError> {
        match value {
            Value::Key(value) => Ok(value),
            _ => Err(ConvertError::UnexpectedPropertyType {
                expected: String::from("key"),
                got: String::from(value.type_name()),
            }),
        }
    }
}

impl FromValue for NaiveDateTime {
    fn from_value(value: Value) -> Result<NaiveDateTime, ConvertError> {
        match value {
            Value::Timestamp(value) => Ok(value),
            _ => Err(ConvertError::UnexpectedPropertyType {
                expected: String::from("timestamp"),
                got: String::from(value.type_name()),
            }),
        }
    }
}

impl FromValue for Bytes {
    fn from_value(value: Value) -> Result<Bytes, ConvertError> {
        match value {
            Value::Blob(value) => Ok(Bytes::from(value)),
            _ => Err(ConvertError::UnexpectedPropertyType {
                expected: String::from("blob"),
                got: String::from(value.type_name()),
            }),
        }
    }
}

impl<T> FromValue for Vec<T>
where
    T: FromValue,
{
    fn from_value(value: Value) -> Result<Vec<T>, ConvertError> {
        match value {
            Value::Array(values) => {
                let values = values
                    .into_iter()
                    .map(FromValue::from_value)
                    .collect::<Result<Vec<T>, ConvertError>>()?;
                Ok(values)
            }
            _ => Err(ConvertError::UnexpectedPropertyType {
                expected: String::from("array"),
                got: String::from(value.type_name()),
            }),
        }
    }
}

impl<T> FromValue for HashMap<String, T>
where
    T: FromValue,
{
    fn from_value(value: Value) -> Result<HashMap<String, T>, ConvertError> {
        match value {
            Value::Entity(values) => {
                let values = values
                    .into_iter()
                    .map(|(k, v)| {
                        let v = FromValue::from_value(v)?;
                        Ok((k, v))
                    })
                    .collect::<Result<HashMap<String, T>, ConvertError>>()?;
                Ok(values)
            }
            _ => Err(ConvertError::UnexpectedPropertyType {
                expected: String::from("entity"),
                got: String::from(value.type_name()),
            }),
        }
    }
}

impl From<ValueType> for Value {
    fn from(value: ValueType) -> Value {
        match value {
            ValueType::NullValue(_) => unreachable!(),
            ValueType::BooleanValue(val) => Value::Boolean(val),
            ValueType::IntegerValue(val) => Value::Integer(val),
            ValueType::DoubleValue(val) => Value::Double(val),
            ValueType::TimestampValue(val) => {
                Value::Timestamp(NaiveDateTime::from_timestamp(val.seconds, val.nanos as u32))
            }
            ValueType::KeyValue(key) => Value::Key(Key::from(key)),
            ValueType::StringValue(val) => Value::Strings(val),
            ValueType::BlobValue(val) => Value::Blob(val),
            ValueType::GeoPointValue(val) => Value::GeoPoint(val.latitude, val.longitude),
            ValueType::EntityValue(entity) => Value::Entity({
                entity
                    .properties
                    .into_iter()
                    .map(|(k, v)| (k, Value::from(v.value_type.unwrap())))
                    .collect()
            }),
            ValueType::ArrayValue(seq) => Value::Array(
                seq.values
                    .into_iter()
                    .map(|val| Value::from(val.value_type.unwrap()))
                    .collect(),
            ),
        }
    }
}
