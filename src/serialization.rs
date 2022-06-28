/*
 * Deserialize isn't working
 */

//use std::marker::PhantomData;
//use serde::{Serialize, Deserialize, ser::SerializeStruct, Deserializer, de::{Visitor, SeqAccess}};
use serde::{Serialize, ser::SerializeStruct};

use crate::mat::Matrix;

impl<T, const M: usize, const N: usize, const LEN: usize> Serialize for Matrix<T, M, N, LEN>
where
    T: Serialize,
    [T; LEN]: serde::Serialize
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            let mut s = serializer.serialize_struct("Matrix", 3)?;

            s.serialize_field("rows", &M)?;
            s.serialize_field("cols", &N)?;
            s.serialize_field("data", self.as_flat_array() as &[T])?;

            s.end()
    }
}

/*impl<'de, T, const M: usize, const N: usize, const LEN: usize> Deserialize<'de> for Matrix<T, M, N, LEN>
where
    T: Deserialize<'de>+Default+Copy,
    [T; LEN]: serde::Deserialize<'de>
{
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let name = "Matrix";
        let fields= &["rows", "cols", "data"];
        let visitor: MatrixVisitor<T, M, N, LEN> = MatrixVisitor{ __boo: PhantomData };
        deserializer.deserialize_struct(name, fields, visitor)
    }
}

struct MatrixVisitor<T, const M: usize, const N: usize, const LEN: usize> {
    __boo: PhantomData<T>,
}

impl<'de, T: Deserialize<'de>+Default+Copy, const M: usize, const N: usize, const LEN: usize> Visitor<'de> for MatrixVisitor<T, M, N, LEN> 
where
    [T; LEN]: serde::Deserialize<'de>
{
    type Value = Matrix<T, M, N, LEN>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("struct Matrix")
    }

    fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
    where
        V: SeqAccess<'de>,
    {
        let rows: usize = seq.next_element()?
            .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
        let cols: usize = seq.next_element()?
            .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
        let data: [T; LEN] = seq.next_element()?
            .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
        
        assert!(rows==M);
        assert!(cols==N);
        assert!(data.len()==LEN);

        Ok(Self::Value::new(data).unwrap())
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::MapAccess<'de>, {
        let mut data = None;

        while let Some(key) = map.next_key()? {
            if let Field::Data = key {
                if data.is_some() {
                    return Err(serde::de::Error::duplicate_field("data"));
                }
                data = map.next_value()?;
            }
        }

        let data = data.ok_or_else(|| serde::de::Error::missing_field("data"))?;
        Ok(Self::Value::new(data).unwrap())
    }
}

#[derive(Deserialize)]
#[serde(field_identifier, rename_all="lowercase")]
enum Field {
    Rows, 
    Cols,
    Data,
}*/