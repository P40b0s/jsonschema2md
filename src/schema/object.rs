use serde_json::{Value, Map};

use crate::{object_type::ObjectType, parser::Parser, converter_error::ConverterError, value_type::ValueType};

use super::{consts, key_value::KeyValue, required::RequiredProperty, format, key_name::KeyName};

pub struct JsonObject
{
    schema : Option<String>,
    id : String,
    title : String,
    description : Option<String>,
    object_type : ObjectType,
    required : Option<RequiredProperty>,
    name: Option<String>,
    pub items : Option<Value>,
    pub properties : Map<String, Value>,
    ///Флаг дополнительный свойств может быть либо булевым значением
    additional_properties : bool,
    ///Либо он может представлять из себя объект с возможными дополнительными типами
    /// <br>
    /// ```
    /// { "type": "string" }
    /// ```
    additional_properties_types : Option<Vec<ObjectType>>,
    min_properties: Option<String>,
    max_properties: Option<String>,
    ///Больше не используется\устарело
    deprecated : Option<bool>
}

impl JsonObject
{
    pub fn get_min_properties(data : &Value) -> Result<String, ConverterError>
    {
        let v = data.get_value_as_number( "minProperties")?;
        Ok(v)
    }
    pub fn get_max_properties(data : &Value) -> Result<String, ConverterError>
    {
        let v = data.get_value_as_number( "maxProperties")?;
        Ok(v)
    }
    pub fn get_properties(data : &Value) -> Result<&Map<String, Value>, ConverterError>
    {
        let v = data["properties"].as_object();
        if v.is_none()
        {
            return Err(ConverterError::ValueNotFound("Не найдено обязательное свойство properties!".to_owned(), data.to_string()));
        }
        Ok(v.unwrap())
    }
}


impl Parser for JsonObject
{
    fn parse(object : KeyValue, required : Option<RequiredProperty>) -> Result<Box<Self>, ConverterError>
    {
        let o_type = object.value.get_schema_type()?;
        if o_type != ObjectType::Object
        {
            return Err(ConverterError::ObjectTypeNotComparable(ObjectType::Object, o_type));
        }
        
        let schema =  object.value.get_schema();
        if schema.is_err()
        {
            if object.name.is_none()
            {
                return Err(schema.err().unwrap());
            }
        }
        let id = object.value.get_id()?;
        let title = object.value.get_title()?;
        let description = object.value.get_description();
        let default = object.value.get_default();
        let min_properties = Self::get_min_properties(&object.value);
        let max_properties = Self::get_max_properties(&object.value);
        let deprecated = object.value.get_deprecated();
        let properties = Self::get_properties(&object.value)?;
        
        let name:Option<String> = object.name.map_or(None, |m|Some(m.to_owned()));
        Ok(Box::new(JsonObject
        {
            schema : schema.ok(),
            id,
            title,
            name,
            description : description.ok(),
            object_type: o_type,
            deprecated: deprecated.ok(),
            required,
            items: None,
            properties: properties.clone(),
            additional_properties: false,
            additional_properties_types: None,
            min_properties: min_properties.ok(),
            max_properties: max_properties.ok(),
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
    //     // format::list::write_property_name(self.name.as_ref(), vec.as_mut(), None, level);
    //     // format::list::write_title(Some(&self.title), vec.as_mut(), level);
    //     // format::list::write_description(self.description.as_ref(), vec.as_mut(), level);
    //     // format::list::write_schema_name(self.schema.as_ref(), vec.as_mut(), level);
    //     // format::list::write_id(Some(&self.id), vec.as_mut(), level);
    //     // format::list::write_type(&self.object_type, vec.as_mut(), level);            
    //     // format::list::write_required(self.required.as_ref(), self.name.as_ref(), vec.as_mut(), level);
    //     // format::list::write_custom_val(self.min_properties.as_ref(), "Минимальное количество свойств", vec.as_mut(), level);
    //     // format::list::write_custom_val(self.max_properties.as_ref(), "Максимальное количество свойств", vec.as_mut(), level);
    //     vec.push("\n".to_owned());
    //     //format::converter::write_avoid_page_break_end(vec.as_mut(), level+1);
    //     vec
    // }

    fn as_keys(&self) -> Vec<KeyName> 
    {
        let mut data : Vec<KeyName> = Vec::new();
        if let Some(n) = self.name.as_ref()
        {
            data.push(KeyName::Name(n.clone()));
            if let Some(r) = self.required.as_ref()
            {
                data.push(KeyName::Required(r, n.clone()));
            }
        }
        data.push(KeyName::Title(self.title.clone()));
        if self.description.is_some()
        {
            data.push(KeyName::Description(self.description.as_ref().unwrap().clone()));
        }
        data.push(KeyName::Id(self.id.clone()));
        data.push(KeyName::Type(self.object_type.clone()));
       
        if let Some(item) = self.min_properties.as_ref()
        {
            data.push(KeyName::MinProperties(item.clone()));
        }
        if let Some(item) = self.max_properties.as_ref()
        {
            data.push(KeyName::MaxProperties(item.clone()));
        }
        data
    }
}