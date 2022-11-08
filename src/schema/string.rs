use serde_json::Value;
use crate::{parser::Parser, converter_error::ConverterError, object_type::ObjectType, value_type::ValueType};
use super::{consts, key_value::KeyValue, required::RequiredProperty, format, key_name::KeyName};

pub struct JsonString
{
    name : String,
    id : String,
    title : String,
    description : Option<String>,
    default : Option<String>,
    examples : Vec<String>,
    object_type : ObjectType,
    ///"^\\d*$"
    pattern : Option<String>,
    ///"contentEncoding": "base64"
    content_encoding : Option<String>,
    ///"contentMediaType": "image/png"
    content_media_type : Option<String>,
    ///Больше не используется\устарело
    deprecated : Option<bool>,
    required : Option<RequiredProperty>
}
impl JsonString
{
    pub fn get_pattern(data : &Value) -> Result<String, ConverterError>
    {
        let v = data.get_value_as_string("pattern")?;
        Ok(v)
    }
    pub fn get_content_encoding(data : &Value) -> Result<String, ConverterError>
    {
        let v = data.get_value_as_string("contentEncoding")?;
        Ok(v)
    }
    pub fn get_content_media_type(data : &Value) -> Result<String, ConverterError>
    {
        let v = data.get_value_as_string("contentMediaType")?;
        Ok(v)
    }
}

impl Parser for JsonString
{
    fn parse(object : KeyValue, required : Option<RequiredProperty>) -> Result<Box<Self>, ConverterError>
    {
        let o_type = object.value.get_schema_type()?;
        if o_type != ObjectType::String
        {
            return Err(ConverterError::ObjectTypeNotComparable(ObjectType::String, o_type));
        }
        if object.name.is_none()
        {
            return Err(ConverterError::PropertyNameNotFound("отсутсвует имя свойства".to_owned(), object.value.to_string()));
        }
        //если тип объекта - строка, значит внутри него только описание строки - один штука
        //let string_object = value.as_object().unwrap();
        //let string_value = string_object.values().nth(0).unwrap();
        //let string_key = string_object.keys().nth(0).unwrap();
        
        
        let id = object.value.get_id()?;
        let title = object.value.get_title()?;
        let description = object.value.get_description();
        let default = object.value.get_default();
        let examples = object.value.get_value_as_array( "examples")?;
        let pattern =  Self::get_pattern(&object.value);
        let content_encoding = Self::get_content_encoding(&object.value);
        let content_media_type = Self::get_content_media_type(&object.value);
        let deprecated = object.value.get_deprecated();
        Ok(Box::new(JsonString
        {
            name : object.name.unwrap().to_owned(),
            id,
            title,
            default: default.ok(),
            description : description.ok(),
            examples,
            object_type: o_type,
            pattern: pattern.ok(),
            content_encoding: content_encoding.ok(),
            content_media_type: content_media_type.ok(),
            deprecated: deprecated.ok(),
            required

        }))
    }

   
    // fn get_markdown(&self, level : i16) -> Vec<String>
    // {
    //     let mut vec : Vec<String> = vec![];
    //    // format::converter::write_avoid_page_break_start(vec.as_mut(), level);
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
    //     // format::table::write_custom_val(self.content_encoding.as_ref(), "Кодировка содержимого", vec.as_mut(), level, false);
    //     // format::table::write_custom_val(self.content_media_type.as_ref(), "Тип содержимого", vec.as_mut(), level, false);
    //     // format::table::write_custom_val(self.content_media_type.as_ref(), "Тип содержимого", vec.as_mut(), level, false);
    //     // format::table::write_pattern(self.pattern.as_ref(), vec.as_mut(), level);
    //     // format::table::write_examples(Some(&self.examples), vec.as_mut(), level);
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
        if let Some(item) = self.content_encoding.as_ref()
        {
            data.push(KeyName::ContentEncoding(item.clone()));
        }
        if let Some(item) = self.content_media_type.as_ref()
        {
            data.push(KeyName::ContentMediaType(item.clone()));
        }
        if let Some(item) = self.pattern.as_ref()
        {
            data.push(KeyName::Pattern(item.clone()));
        }
        data.push(KeyName::Examples(self.examples.clone()));
        data
    }
}