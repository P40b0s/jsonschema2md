use crate::schema::{consts, required::RequiredProperty};

use super::converter;

/// Формат вывода имени свойства
/// * `title` - поле `title`
/// * `description` - поле `description`
/// * `property_name` - текущее имя свойства
/// * `prof_ref` - если текущий объект является `object` и у него есть ссылка на какцю-то схему `$ref`
/// 
pub fn write_property_name(title : Option<&String>, description : Option<&String>, property_name : Option<&String>, vec : &mut Vec<String>, prop_ref : Option<&String>, level:i16)
{
    if let Some(property_name) = property_name
    {
        let identation = super::converter::get_identation(level);
        let mut output = format!("{}|{}<br><span style=\"color:{}\">{}</span>|<b><span style=\"color:{};\">`{}`</b>|",
        &identation,
        title.unwrap_or(&"Свойство".to_owned()),
        consts::TITLE_COLOR,
        description.unwrap_or(&"".to_owned()),
        consts::PROPERTY_NAME_COLOR,
        property_name);
        if let Some(prop_ref) = prop_ref
        {
            let id_url = super::converter::id_conversion(prop_ref);
            let ref_f = format!(", id: [{}](#{})",
                    prop_ref,
                    id_url);
            vec.push(ref_f);
        }
        output.push_str("  \n");
        vec.push(output);
        vec.push(format!("{}|:-|:-:|\n", &identation));
    }
}

/// Формат вывода для различных дополнительных полей, `maxItems`, `maxProperties`, `maximum`, `minimum` итд.
pub fn write_custom_val(value : Option<&String>, description : &str, vec : &mut Vec<String>, level : i16, is_bool : bool)
{
    if let Some(value) = value
    {
        if !value.is_empty()
        {
            let identation = super::converter::get_identation(level);
            let mut first = format!("{}|{}|",identation, description);
            if is_bool
            {
                first.push_str(format!("<b>`{}`</b>|  \n", value).as_str());
            }
            else
            {
                first.push_str(format!("`{}`|  \n", value).as_str());
            }
            vec.push(first);
        }
    }
}

/// Формат вывода поля $id
pub fn write_id(id : Option<&String>, vec : &mut Vec<String>, level:i16)
{
    if let Some(id) = id
    {
        let identation = super::converter::get_identation(level);
        let id_url = super::converter::id_conversion(id.as_str());
        vec.push(format!("{}|{}|<b id=\"{}\">`{}`</b>|  \n",
                identation,
                consts::ID,
                id_url,
                id));
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
                vec.push(format!("{}|{}|{}|  \n", identation, r.0, r.1));
            }
        }
    }
}
/// Формат вывода поля `required`
pub fn write_examples(examples : Option<&Vec<String>>, vec : &mut Vec<String>, level:i16)
{
    if let Some(examples) = examples
    {
        let examples = super::converter::get_examples(examples);
        let identation = super::converter::get_identation(level);
        vec.push(format!("{}|{}|{}|  \n", identation, consts::EXAMPLES, examples));
    }
}
/// Формат вывода поля `required`
pub fn write_pattern(pattern : Option<&String>, vec : &mut Vec<String>, level:i16)
{
    if let Some(pattern) = pattern
    {
        let pattern = super::converter::pattern_conversion(pattern);
        let identation = super::converter::get_identation(level);
        vec.push(format!("{}|{}|{}|  \n", identation, "Паттерн", pattern));
    }
}