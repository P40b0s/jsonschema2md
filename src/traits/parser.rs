use std::collections::HashMap;

use serde_json::Value;

use crate::{converter_error::ConverterError, object_type::ObjectType, schema::{key_value::KeyValue, required::RequiredProperty, key_name::KeyName}};



pub trait Parser
{
    /**
    * Парсим объект, сюда передаем ключ-значение оно либо будет в виде самого объекта, если у него нет имени
    ```
        {
            "type": "string"  
        }
        
    ```
    * либо в виде имени-объекта
    ```
       "is_bool":{
            "type": "string"  
       }
    ```
    */
    fn parse(object : KeyValue, required : Option<RequiredProperty>) -> Result<Box<Self>, ConverterError>;
    /// Получение разметки в виде массива строк
    //fn get_markdown(&self, level : i16) -> Vec<String>;
    //fn get_pug(&self, level : i16) -> Vec<String>;
    fn as_keys(&self) -> Vec<KeyName>;

}