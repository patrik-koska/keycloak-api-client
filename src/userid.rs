use uuid::Uuid;

pub fn gen_uuid(uuid: &Uuid) -> String {
   let id = format!("{}", uuid.to_hyphenated());
   return id;
}