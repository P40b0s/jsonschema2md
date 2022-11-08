use std::collections::HashMap;

use serde_json::{Value};
use super::key_name::KeyName;

use crate::{object_type::ObjectType, parser::Parser, converter_error::ConverterError, value_type::ValueType};

use super::{key_value::KeyValue, required::RequiredProperty, format::{self},};

pub struct JsonArray
{
    name : String,
    id : String,
    title : String,
    description : Option<String>,
    default : String,
    min_items : Option<String>,
    max_items : Option<String>,
    object_type : ObjectType,
    pub items : Value,
    pub items_ref : Option<String>,
    ///Больше не используется\устарело
    deprecated : Option<bool>,
    required : Option<RequiredProperty>
}
impl JsonArray
{
    pub fn get_min_items(data : &Value) -> Result<String, ConverterError>
    {
        let v = data.get_value_as_number( "minItems")?;
        Ok(v)
    }
    pub fn get_max_items(data : &Value) -> Result<String, ConverterError>
    {
        let v = data.get_value_as_number( "maxItems")?;
        Ok(v)
    }
    pub fn get_items(data : &Value) -> Result<Value, ConverterError>
    {
        if data["items"].as_object().is_some()
        {
            Ok(data["items"].clone())
        }
        else 
        {
            return Err(ConverterError::PropertyNameNotFound(data["$id"].to_string(),"items".to_owned()));
        }
    }
}


impl Parser for JsonArray
{
    fn parse(object : KeyValue, required : Option<RequiredProperty>) -> Result<Box<Self>, ConverterError>
    {
        let o_type = object.value.get_schema_type()?;
        if o_type != ObjectType::Array
        {
            return Err(ConverterError::ObjectTypeNotComparable(ObjectType::Array, o_type));
        }
        
        let id = object.value.get_id()?;
        let title = object.value.get_title()?;
        let description = object.value.get_description();
        let min_items = Self::get_min_items(&object.value);
        let max_items = Self::get_max_items(&object.value);
        let deprecated = object.value.get_deprecated();
        let default = object.value.get_default();
        let items = Self::get_items(&object.value)?;
        let items_ref = object.value["items"].get_value_as_string("$ref");
        let name:Option<String> = object.name.map_or(None, |m|Some(m.to_owned()));
        //нужен массив oneof и в поле обязательное выводить: или свойство такое или свойство такое или такое
        Ok(Box::new(JsonArray
        {
            name: name.unwrap_or("ОШИБКА_ИМЕНИ_МАССИВА".to_owned()),
            id,
            title,
            description : description.ok(),
            object_type: o_type,
            deprecated: deprecated.ok(),
            items,
            items_ref : items_ref.ok(),
            min_items: min_items.ok(),
            max_items: max_items.ok(),
            required,
            default : default.unwrap_or("[]".to_owned())
            
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
    //     //format::list::write_property_name(Some(&self.name), vec.as_mut(), None, level);
    //     //format::list::write_title(Some(&self.title), vec.as_mut(), level);
    //     //format::list::write_description(self.description.as_ref(), vec.as_mut(), level);
    //     //format::list::write_id(Some(&self.id), vec.as_mut(), level);
    //     //format::list::write_type(&self.object_type, vec.as_mut(), level);            
    //     //format::list::write_required(self.required.as_ref(), Some(&self.name), vec.as_mut(), level);
    //     //format::list::write_custom_val(self.min_items.as_ref(), "Минимальное количество", vec.as_mut(), level);
    //     //format::list::write_custom_val(self.max_items.as_ref(), "Максимальное количество", vec.as_mut(), level);
    //     //format::list::array::write_items_name(self.items_ref.as_ref(), vec.as_mut(), level);
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
        if let Some(min) = self.min_items.as_ref()
        {
            data.push(KeyName::MinItems(min.clone()));
        }
        if let Some(max) = self.max_items.as_ref()
        {
            data.push(KeyName::MaxItems(max.clone()));
        }
        if let Some(rf) = self.items_ref.as_ref()
        {
            data.push(KeyName::ItemsRef(rf.clone()));
        }
        data
    }
}