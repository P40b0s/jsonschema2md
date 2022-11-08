use crate::{converter_error::ConverterError, object_type::ObjectType, parser::Parser, value_type::ValueType};
use super::{consts, key_value::KeyValue, required::RequiredProperty, format, key_name::KeyName};

pub struct JsonBool
{
    name : String,
    id : String,
    title : String,
    description : Option<String>,
    default : Option<String>,
    object_type : ObjectType,
    ///Больше не используется\устарело
    deprecated : Option<bool>,
    required : Option<RequiredProperty>
}

impl Parser for JsonBool
{
    fn parse(object : KeyValue, required : Option<RequiredProperty>) -> Result<Box<Self>, ConverterError>
    {
        //тут ожно из двух, либо value содержит ключ - значение
        let o_type = object.value.get_schema_type()?;
        if o_type != ObjectType::Boolean
        {
            return Err(ConverterError::ObjectTypeNotComparable(ObjectType::Boolean, o_type));
        }
        if object.name.is_none()
        {
            return Err(ConverterError::PropertyNameNotFound("отсутсвует имя свойства".to_owned(), object.value.to_string()));
        }
        
        
        let id = object.value.get_id()?;
        let title = object.value.get_title()?;
        let description = object.value.get_description();
        let default = object.value.get_default();
        let deprecated = object.value.get_deprecated();

        Ok(Box::new(JsonBool
        {
            name : object.name.unwrap().to_owned(),
            id,
            title,
            description : description.ok(),
            default: default.ok(),
            object_type: o_type,
            deprecated: deprecated.ok(),
            required
        }))
    }

    ///Для нашего отображения в таблице, стыкуем все по парам
    ///<br>потом останется только сделать таблицу по количеству итемов в массиве<br>
    /// * заголовок : определение
    /// * свойство : имя свойства
    // fn get_markdown(&self, level : i16) -> Vec<String>
    // {
    //     let mut vec : Vec<String> = vec![];
    //     //format::converter::write_avoid_page_break_start(vec.as_mut(), level);
    //     // format::table::write_property_name(Some(&self.title),
    //     //                                     self.description.as_ref(),
    //     //                                     Some(&self.name),
    //     //                                     vec.as_mut(),
    //     //                                     None,
    //     //                                     level);

    //     // format::table::write_custom_val(Some(&self.object_type.to_string()), consts::TYPE, vec.as_mut(), level, true);
    //     // format::table::write_id(Some(&self.id), vec.as_mut(), level);
    //     // format::table::write_required(self.required.as_ref(), Some(&self.name), vec.as_mut(), level);
    //     // format::table::write_custom_val(self.default.as_ref(), consts::DEFAULT, vec.as_mut(), level, false);
    //     vec.push("\n".to_owned());
    //     //format::converter::write_avoid_page_break_end(vec.as_mut(), level+1);
    //     vec
    // }

    fn as_keys(&self) -> Vec<KeyName> 
    {
        let mut data : Vec<KeyName> = Vec::new();
        data.push(KeyName::Name(self.name.clone()));
        data.push(KeyName::Title(self.title.clone()));
        if self.description.is_some()
        {
            data.push(KeyName::Description(self.description.as_ref().unwrap().clone()));
        }
        data.push(KeyName::Id(self.id.clone()));
        data.push(KeyName::Type(self.object_type.clone()));
        if let Some(r) = self.required.as_ref()
        {
            data.push(KeyName::Required(r, self.name.clone()));
        }
        if let Some(def) = self.default.as_ref()
        {
            data.push(KeyName::Default(def.clone()));
        }
        data
    }
}