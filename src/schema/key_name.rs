use std::fmt;

use serde_json::Value;

use crate::object_type::ObjectType;

use super::required::RequiredProperty;
#[derive(Eq, PartialEq, Clone)]
pub enum KeyName<'a>
{
    Id(String),
    Schema(String),
    Name(String),
    Title(String),
    Description(String),
    Type(ObjectType),
    ///Свойство типа `array`
    MinItems(String),
    ///Свойство типа `array`
    MaxItems(String),
    //Object(Value),
    Default(String),
    ///Список обязательных параметров и имя текущего параметра
    Required(&'a RequiredProperty, String),
    ///Если описание объекта ссылается на друкую схему то здесь будет id этой схемы
    RefId(String),
    ///Если объект массива является ссылкой на схему
    ItemsRef(String),
    ///Регекс согласно которому происходит валидация значчения
    Pattern(String),
    ///Примеры значениий `examples`
    Examples(Vec<String>),
    ///Свойство типа `number` или `integer`
    MultipleOf(String),
    ///Свойство типа `number` или `integer`
    ExclusiveMinimum(String),
    ///Свойство типа `number` или `integer`
    ExclusiveMaximum(String),
    ///Свойство типа `number` или `integer`
    Minimum(String),
    ///Свойство типа `number` или `integer`
    Maximum(String),
    ///Минимальное количество свойтсв в типе `object`
    MinProperties(String),
    ///Максимальное количество свойтсв в типе `object`
    MaxProperties(String),
    ///Кодировка содержимого - свойство типа `string`
    ContentEncoding(String),
    ///Тип содержимого - свойство типа `string`
    ContentMediaType(String)
}
impl<'a> fmt::Display for KeyName<'a>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result 
    {
        let o_type = match self
        {
            KeyName::Id(_) => "Идентификатор",
            KeyName::Schema(_) => "Схема",
            KeyName::Name(_) => "Свойство",
            KeyName::Title(_) => "",
            KeyName::Type(_) => "Тип",
            KeyName::Description(_) => "",
            KeyName::Required(_, _) => "Обязательное",
            KeyName::Default(_) => "По умолчанию",
            KeyName::RefId(_) => "Ссылка на схему",
            KeyName::ItemsRef(_) => "Объект",
            //KeyName::Object(_) => "",
            //Свойства массива
            KeyName::MinItems(_) => "Минимальное количество",
            KeyName::MaxItems(_) => "Максимальное количество",
            //Свойства числа
            KeyName::MultipleOf(_) => "<",
            KeyName::ExclusiveMinimum(_) => ">",
            KeyName::ExclusiveMaximum(_) => "<",
            KeyName::Minimum(_) => "≤",
            KeyName::Maximum(_) => "≥",
            //Свойства объекта
            KeyName::MinProperties(_) => "Минимальное количество свойств",
            KeyName::MaxProperties(_) => "Максимальное количество свойств",
            //Свойства строки
            KeyName::ContentEncoding(_) => "Кодировка содержимого",
            KeyName::ContentMediaType(_) => "Тип содержимого",
            KeyName::Pattern(_) => "Регекс",
            KeyName::Examples(_) => "Примеры",
        };
        write!(f, "{}", o_type)
        // или как альтернатива:
        // fmt::Debug::fmt(self, f)
    }
}