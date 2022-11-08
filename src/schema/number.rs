use serde_json::Value;
use crate::{object_type::ObjectType, parser::Parser, converter_error::ConverterError, value_type::ValueType};
use super::{consts, key_value::KeyValue, required::RequiredProperty, format, key_name::KeyName};

///в json один тип номера, а в расте 4 типа... поэтому приводим все к строке, всеравно компилить документацию нужна строка
pub struct JsonNumber
{
    
    name : String,
    id : String,
    title : String,
    description : Option<String>,
    examples : Vec<String>,
    default : Option<String>,
    object_type : ObjectType,
    ///Валидно если указанный номер больше проверяемого (должен быть больше нуля в том числе и дробным например 0.5)
    /// ```
    ///{     
    ///     "type": "number",
    ///     "multipleOf": 0.5
    ///}
    /// ```
    multiple_of : Option<String>,
    /**
     * Валидно если значение номера меньше 10.5.
    ```
    {
        "type": "number",
        "exclusiveMaximum": 10.5
    }
    ```
    */
    exclusive_maximum : Option<String>,
    /**
     * Валидно если значение номера больше 10.5.
    ```
    {
        "type": "number",
        "exclusiveMaximum": 10.5
    }
    ```
    */
    exclusive_minimum : Option<String>,
    /**
     * Валидно если значение номера не больше 10.5.
    ```
    {
        "type": "number",
        "maximum": 10.5
    }
    ```
    */
    maximum : Option<String>,
    /**
     * Валидно если значение номера не меньше 10.5.
    ```
    {
        "type": "number",
        "minimum": 10.5
    }
    ```
    */
    minimum : Option<String>,
    ///Больше не используется\устарело
    deprecated : Option<bool>,
    ///Свйоство обязательное
    required : Option<RequiredProperty>
}

impl JsonNumber
{
    pub fn get_multiple_of(data : &Value) -> Result<String, ConverterError>
    {
        let v = data.get_value_as_number("multipleOf")?;
        Ok(v)
    }
    pub fn get_maximum(data : &Value) -> Result<String, ConverterError>
    {
        let v = data.get_value_as_number("maximum")?;
        Ok(v)
    }
    pub fn get_minimum(data : &Value) -> Result<String, ConverterError>
    {
        let v = data.get_value_as_number("minimum")?;
        Ok(v)
    }
    pub fn get_exclusive_minimum(data : &Value) -> Result<String, ConverterError>
    {
        let v = data.get_value_as_number("exclusiveMinimum")?;
        Ok(v)
    }
    pub fn get_exclusive_maximum(data : &Value) -> Result<String, ConverterError>
    {
        let v = data.get_value_as_number("exclusiveMaximum")?;
        Ok(v)
    }
}

impl Parser for JsonNumber
{
    fn parse(object : KeyValue, required : Option<RequiredProperty>) -> Result<Box<Self>, ConverterError>
    {
        let o_type = object.value.get_schema_type()?;
        if o_type != ObjectType::Integer
        {
            return Err(ConverterError::ObjectTypeNotComparable(ObjectType::Boolean, o_type));
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
        //примеры сделал обязательными, но в другой схеме не факт что их не будет
        let examples = object.value.get_value_as_array("examples")?;
        let multiple_of =  Self::get_multiple_of(&object.value);
        let minimum = Self::get_minimum(&object.value);
        let maximum = Self::get_maximum(&object.value);
        let exclusive_minimum = Self::get_exclusive_minimum(&object.value);
        let exclusive_maximum = Self::get_exclusive_maximum(&object.value);
        let deprecated = object.value.get_deprecated();
       
        Ok(Box::new(JsonNumber
        {
            name : object.name.unwrap().to_owned(),
            id,
            title,
            description : description.ok(),
            default: default.ok(),
            examples : examples,
            object_type: o_type,
            multiple_of: multiple_of.ok(),
            exclusive_maximum: exclusive_maximum.ok(),
            exclusive_minimum: exclusive_minimum.ok(),
            maximum: maximum.ok(),
            minimum: minimum.ok(),
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
    //     //format::table::write_property_name(Some(&self.title),
    //     //                                    self.description.as_ref(),
    //     //                                    Some(&self.name),
    //     //                                    vec.as_mut(),
    //     //                                    None,
    //     //                                    level);

    //     //format::table::write_custom_val(Some(&self.object_type.to_string()), consts::TYPE, vec.as_mut(), level, true);
    //     //format::table::write_id(Some(&self.id), vec.as_mut(), level);
    //     //format::table::write_required(self.required.as_ref(), Some(&self.name), vec.as_mut(), level);
    //     //format::table::write_custom_val(self.default.as_ref(), consts::DEFAULT, vec.as_mut(), level, false);
    //     //format::table::write_custom_val(self.multiple_of.as_ref(), "<", vec.as_mut(), level, false);
    //     //format::table::write_custom_val(self.exclusive_minimum.as_ref(), ">", vec.as_mut(), level, false);
    //     //format::table::write_custom_val(self.exclusive_maximum.as_ref(), "<", vec.as_mut(), level, false);
    //     //format::table::write_custom_val(self.minimum.as_ref(), "≤", vec.as_mut(), level, false);
    //     //format::table::write_custom_val(self.maximum.as_ref(), "≥", vec.as_mut(), level, false);
    //     //format::table::write_examples(Some(&self.examples), vec.as_mut(), level);
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
            data.push(KeyName::Required(&r, self.name.clone()));
        }
        if let Some(def) = self.default.as_ref()
        {
            data.push(KeyName::Default(def.clone()));
        }
        if let Some(item) = self.multiple_of.as_ref()
        {
            data.push(KeyName::MultipleOf(item.clone()));
        }
        if let Some(item) = self.exclusive_minimum.as_ref()
        {
            data.push(KeyName::ExclusiveMinimum(item.clone()));
        }
        if let Some(item) = self.exclusive_maximum.as_ref()
        {
            data.push(KeyName::ExclusiveMaximum(item.clone()));
        }
        if let Some(item) = self.minimum.as_ref()
        {
            data.push(KeyName::Minimum(item.clone()));
        }
        if let Some(item) = self.maximum.as_ref()
        {
            data.push(KeyName::Maximum(item.clone()));
        }
        data.push(KeyName::Examples(self.examples.clone()));
        data
    }
}