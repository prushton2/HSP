pub mod student_repository;

pub use student_repository::StudentRepository;

// FieldValue is an enum that holds multiple possible values. This is useful when updating a field where we dont know what the required type will be

// #[derive(Debug)]
// pub enum FieldValue<'a> {
//     Str(&'a str),
//     I32(i32),
//     Role(Role)
// }

// impl ToSql for FieldValue<'_> {
//     fn to_sql(&self, _ty: &tokio_postgres::types::Type, out: &mut tokio_postgres::types::private::BytesMut) -> Result<tokio_postgres::types::IsNull, Box<dyn std::error::Error + Sync + Send>>
//         where
//             Self: Sized {
//         match self {
//             FieldValue::Str(t) => t.to_sql(&tokio_postgres::types::Type::TEXT, out),
//             FieldValue::I32(t) => t.to_sql(&tokio_postgres::types::Type::INT4, out),
//             FieldValue::Role(t) =>  String::from(t).to_sql(&tokio_postgres::types::Type::TEXT, out),
//         }
//     }

//     fn accepts(ty: &tokio_postgres::types::Type) -> bool
//         where
//             Self: Sized {
//         return ty == &tokio_postgres::types::Type::INT4 || ty == &tokio_postgres::types::Type::TEXT;
//     }

//     to_sql_checked!();
// }