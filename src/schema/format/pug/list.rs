use chrono::format::format;

use crate::{schema::{consts, required::RequiredProperty}, object_type::ObjectType};

/// Формат вывода имени свойства
pub fn write_property_name_title_description(name : Option<&String>, title : Option<&String>, description : Option<&String>, vec : &mut Vec<String>, prop_ref : Option<&String>, level:i16)
{
    if let Some(name) = name
    {
        let identation = super::converter::get_identation(level);
        vec.push(format!("{}{}",identation,"details(open)"));
        vec.push(format!("{}   {}",identation,"summary"));
        vec.push(format!("{}        {} {}",identation,"span.property-name", name));
        if let Some(title) = title
        {
            vec.push(format!("{}        {} {}",identation,".title", title));
        }
        if let Some(description) = description
        {
            vec.push(format!("{}        {} {}",identation,".description", description));
        }
    }
}

/// Формат вывода поля type
pub fn write_type(object_type : &ObjectType, vec : &mut Vec<String>, level:i16)
{
    let identation = super::converter::get_identation(level);
    vec.push(format!("{}        {} {}: {}",identation,"p", consts::TYPE, object_type.to_string()));
}

/// Формат вывода поля $id
pub fn write_id(id : Option<&String>, vec : &mut Vec<String>, level:i16)
{
    if let Some(id) = id
    {
        let identation = super::converter::get_identation(level);
        let id_url = super::converter::id_conversion(id.as_str());
        vec.push(format!("{}  {}: <b id=\"{}\">`{}`</b>  \n",
                identation,
                consts::ID,
                id_url,
                id));
    }
}
/// Формат вывода поля `$schema`
pub fn write_schema_name(schema : Option<&String>, vec : &mut Vec<String>, level:i16)
{
    if let Some(schema) = schema
    {
        let identation = super::converter::get_identation(level);
        vec.push(format!("{}  Схема: <span style=\"color:{}\">`{}`</b>  \n",
                    identation,
                    consts::PROPERTY_NAME_COLOR,
                    schema));
    }
}

/// Формат вывода поля `required`
pub fn write_required(required : Option<&RequiredProperty>, property_name: Option<&String>, vec : &mut Vec<String>, level:i16)
{
    if let Some(required) = required
    {
        let identation = super::converter::get_identation(level);
        if let Some(property_name) = property_name
        {
            if let Some(r) = required.get_required(property_name)
            {
                vec.push(format!("{}  {}: {}  \n", identation, r.0, r.1));
            }
        }
    }
}
 /// Формат вывода для различных дополнительных полей, `maxItems`, `maxProperties`, `maximum`, `minimum` итд.
 pub fn write_custom_val(value : Option<&String>, description : &str, vec : &mut Vec<String>, level : i16)
 {
     if let Some(value) = value
     {
         let identation = super::converter::get_identation(level);
         vec.push(format!("{}  {}: `{}`  \n",
                 identation,
                 description,
                 value));
     }
 }

 
/// Модуль свойств характерных только для массива
pub mod array 
{
    use crate::schema::format::converter;
    /// Формат вывода поля `items` для массива
    pub fn write_items_name(items_ref : Option<&String>, vec : &mut Vec<String>, level : i16)
    {
        let identation = converter::get_identation(level);
        if let Some(items_ref) = items_ref
        {
            let id_url = converter::id_conversion(&items_ref);
            vec.push(format!("{}  {} [{}](#{})  \n\n",
            identation,
            "Объект массива:",
            items_ref,
            id_url));
        }
        else
        {
            vec.push(format!("{}  {}  \n\n",
            identation,
            "Объект массива:"));
        }
    }
}
