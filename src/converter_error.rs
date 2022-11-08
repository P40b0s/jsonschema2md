use std::{fmt, io::ErrorKind};

use crate::object_type::ObjectType;
#[derive(Debug)]
pub enum ConverterError
{
    Errors(String),
    ///Неподходящий тип, если структура не может распарсить данный тип
    ParseUnsuitableType(String),
    ValueNotFound(String, String),
    ///У объекта {} не найдено имя свойства
    PropertyNameNotFound(String, String),
    ArrayInternalTypeNotSupported(String),
    //Для парсинга данного значения необходим другой тип парсера
    ObjectTypeNotComparable(ObjectType, ObjectType)
}


impl fmt::Display for ConverterError
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
    {
        match self 
        {
            ConverterError::Errors(path) => write!(f, "Ошибки конвертирования: {}", path),
            ConverterError::ValueNotFound(path, json) => write!(f, "Значение {} не найдено или неподходящий тип {}", path, json),
            ConverterError::ArrayInternalTypeNotSupported(path) => write!(f, "Тип определенный в массиве не поддерживается: {}", path),
            ConverterError::ParseUnsuitableType(path) => write!(f, "Неподходящий тип: {}", path),
            ConverterError::PropertyNameNotFound(id, name) => write!(f, "У объекта id {} не найдено имя свойства {}", id, name),
            ConverterError::ObjectTypeNotComparable(waiting, fact) => write!(f, "Ожидался тип объекта {waiting} а найден {fact}"),
        }
    }
}
impl From<std::io::Error> for ConverterError 
{
    fn from(error: std::io::Error) -> Self 
    {
        let not_found = String::from("Ошибка, файл схемы json не найден");
        if error.kind() == ErrorKind::NotFound 
        {
            ConverterError::Errors(not_found)
        }
        else 
        {
            ConverterError::Errors(error.to_string())
        }
        
    }
}
impl From<serde_json::Error> for ConverterError 
{
    fn from(error: serde_json::Error) -> Self 
    {
        ConverterError::Errors(error.to_string())
    }
}
