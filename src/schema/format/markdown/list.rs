use crate::{schema::{consts, required::RequiredProperty}, object_type::ObjectType};

use super::converter;


/// Формат вывода имени свойства
pub fn write_property_name(name : Option<&String>, vec : &mut Vec<String>, prop_ref : Option<&String>, level:i16)
{
    if let Some(name) = name
    {
        let identation = super::converter::get_identation(level);
        let mut output = format!("{}- #### Свойство: <span style=\"color:{}\">`{}`</span>",
        identation,
        consts::PROPERTY_NAME_COLOR,
        name);
        if let Some(prop_ref) = prop_ref
        {
            let id_url = super::converter::id_conversion(prop_ref);
            let ref_f = format!(", схема - [{}](#{})",
                    prop_ref,
                    id_url);
            output.push_str(ref_f.as_str());
        }
        output.push_str("  \n");
        vec.push(output);
    }
}
    
/// Формат вывода поля title
pub fn write_title(title : Option<&String>, vec : &mut Vec<String>, level:i16)
{
    if let Some(title) = title
    {
        let identation = super::converter::get_identation(level);
        vec.push(format!("{}  <span style=\"color:{}\">{}</span>  \n",
            identation,
            consts::TITLE_COLOR,
            title));
    }
}
/// Формат вывода поля description
pub fn write_description(desc : Option<&String>, vec : &mut Vec<String>, level:i16)
{
    if let Some(desc) = desc
    {
        let identation = super::converter::get_identation(level);
        vec.push(format!("{}  {}  \n",
                identation,
                desc));
    }
}
/// Формат вывода поля type
pub fn write_type(object_type : &ObjectType, vec : &mut Vec<String>, level:i16)
{
    let identation = super::converter::get_identation(level);
    vec.push(format!("{}  {}: `{}`  \n",
            identation,
            consts::TYPE,
            object_type.to_string()));
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
