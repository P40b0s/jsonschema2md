use serde_json::Value;
use crate::{converter_error::ConverterError, object_type::ObjectType, parser::Parser, value_type::ValueType};
use super::{key_value::KeyValue, required::RequiredProperty, consts, format, key_name::KeyName};

pub struct JsonRef
{
    name : String,
    reference : String,
}


impl Parser for JsonRef
{
    fn parse(object : KeyValue, _required : Option<RequiredProperty>) -> Result<Box<Self>, ConverterError>
    {
        let o_type = object.value.get_schema_type()?;
        if o_type != ObjectType::Ref
        {
            return Err(ConverterError::ObjectTypeNotComparable(ObjectType::Ref, o_type));
        }
        if object.name.is_none()
        {
            return Err(ConverterError::PropertyNameNotFound("отсутсвует имя свойства".to_owned(), object.value.to_string()));
        }
        let reference = object.value.as_object();
        if reference.is_none()
        {
            return Err(ConverterError::PropertyNameNotFound(format!("В свойстве {}", object.name.unwrap()), "(ошибка чтения значения $ref)".to_owned()));
        }
        let reference = reference.unwrap()["$ref"].as_str();
        if reference.is_none()
        {
            return Err(ConverterError::PropertyNameNotFound(format!("В свойстве {}", object.name.unwrap()), "(ошибка чтения значения $ref)".to_owned()));
        }
        Ok(Box::new(JsonRef
        {
            name: object.name.unwrap().to_owned(),
            reference: reference.unwrap().to_owned(),
        }))
    }
    ///Для нашего отображения в таблице, стыкуем все по парам
    ///<br>потом останется только сделать таблицу по количеству итемов в массиве<br>
    /// * заголовок : определение
    /// * свойство : имя свойства
    // fn get_markdown(&self, level : i16) -> Vec<String>
    // {
    //     let mut vec : Vec<String> = vec![];
    //     //format::list::write_property_name(Some(&self.name), vec.as_mut(), Some(&self.reference), level);
    //     vec
    // }

    fn as_keys(&self) -> Vec<KeyName> 
    {
        let mut data : Vec<KeyName> = Vec::new();
        data.push(KeyName::Name(self.name.clone()));
        data.push(KeyName::RefId(self.reference.clone()));
        data
    }
}