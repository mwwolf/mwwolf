use super::*;
// pub async fn allocate_ids<T>(
//     connection: &Connection,
//     keys: &[Key],
// ) -> domain::DomainResult<Vec<domain::Id<T>>> {
//     connection
//         .allocate_ids(keys)
//         .await
//         .map_err(|e| {
//             domain::DomainError::new_with_source(
//                 domain::DomainErrorKind::Fail,
//                 "failed allocated ids",
//                 e.into(),
//             )
//         })
//         .map(|keys| {
//             keys.into_iter()
//                 .map(|key| key_to_id(Key::from(key)))
//                 .collect::<Vec<_>>()
//         })
// }
//

impl<T> From<Key> for domain::Id<T> {
    fn from(key: Key) -> Self {
        match key.get_id() {
            proto_api::KeyID::IntID(id) => domain::Id::new(id.to_string()),
            id => panic!("unexpected id:{:?}", id),
        }
    }
}

impl<T> From<domain::Id<T>> for Key {
    fn from(id: domain::Id<T>) -> Self {
        Key::new(entity::kind::<T>()).id(proto_api::KeyID::IntID(id.raw_id().parse().unwrap()))
    }
}
