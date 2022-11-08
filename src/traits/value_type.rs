use serde_json::{Value};
use crate::{converter_error::ConverterError, object_type::ObjectType, schema::required::RequiredProperty};

pub trait ValueType
{
     ///#### Определяет тип объекта  
    ///например есть
    /// ```
    /// "name": 
    /// {
    ///     "title" : "Наименование"
    ///     "$id" : "#root",
    ///     "type" : "string"
    /// }
    /// ```
    /// В таком объекте тип объекта будет `string` а имя объекта `name`<br>
    /// Соглассно схеме draft-07 есть:  `string bool integer array object null`
    fn get_schema_type(&self) -> Result<ObjectType, ConverterError>;
    /// #### Получение данных как строки
    fn get_value_as_string(&self, val : &str) -> Result<String, ConverterError>;
    /// #### Получение данных как булевого значения
    fn get_value_as_bool(&self, val : &str) -> Result<bool, ConverterError>;
    /// #### Получение данных как номера, с переводом в строку
    fn get_value_as_number(&self, val : &str) -> Result<String, ConverterError>;
    /// #### Получение данных как массива с переводом номеров в строку
    fn get_value_as_array(&self, val : &str) -> Result<Vec<String>, ConverterError>;
    /// #### Получение поля `$id`
    fn get_id(&self) -> Result<String, ConverterError>;
    /// #### Получение поля `title`
    fn get_title(&self) -> Result<String, ConverterError>;
    /// #### Получение поля `description`
    fn get_description(&self) -> Result<String, ConverterError>;
    /// #### Получение поля `$schema`
    fn get_schema(&self) -> Result<String, ConverterError>;
    /// #### Получение поля `default`
    fn get_default(&self) -> Result<String, ConverterError>;
    /// #### Получение поля `deprecated`
    fn get_deprecated(&self) -> Result<bool, ConverterError>;
    /// #### Ищет свойства
    /// * Ищет поле `requred`
    /// * Ищет поле `allOf` и в нем `oneOf` и извлекает `required` <br>
    ///     в других случаях `allOf` не заполняется так как это поле аналогно <br>
    ///     просто `required` в корне
    /// * Ищет поле `oneOf` и извлекает `required`
    /// 
    fn get_required(&self) -> Result<RequiredProperty, ConverterError>;


}

impl ValueType for Value 
{
   
    fn get_schema_type(&self) -> Result<ObjectType, ConverterError>
    {
        if let Some(obj) = self.as_object()
        {
            let o_type = obj.contains_key("type");
            if !o_type
            {
                if obj.contains_key("$ref")
                {
                    return Ok(ObjectType::Ref);
                }
                return Err(ConverterError::ParseUnsuitableType(format!("Невозможно определить тип, в объекте {} не найдено свойство type или $ref!", self.to_string())));  
            }
            let o_type = match obj["type"].as_str().unwrap()
            {
                "string"  => ObjectType::String,
                "bool" => ObjectType::Boolean,
                "integer" => ObjectType::Integer,
                "array" => ObjectType::Array,
                "object" => ObjectType::Object,
                "null" => ObjectType::Null,
                "$ref" => ObjectType::Ref,
                _ => ObjectType::Undefined,
            };
            return Ok(o_type);
        }
        //БЫвает что сюда может попасть только строка, потому что изначальное значение типа "$ref":"#annex" сюда попадает только "#annex"
        //и оно не является объектом
        else
        {
            
            return Err(ConverterError::ParseUnsuitableType(format!("{} не является объектом, невозможно получить свойства", self.to_string())));
        }
        
    }

    fn get_value_as_string(&self, val : &str) -> Result<String, ConverterError>
    {
        if self[val].is_null()
        {
            if val == "$id"
            {
                return Err(ConverterError::PropertyNameNotFound(self["$id"].to_string(), self.to_string()));
            }
            else
            {
                return Err(ConverterError::PropertyNameNotFound(self["$id"].to_string(), val.to_owned()));
            }
           
        }
        else if let Some(v) = self[val].as_str()
        {
            let value : Option<String> = Some(v.to_owned());
            if value.is_some()
            {
                Ok(value.unwrap())
            }
            else
            {
                Err(ConverterError::ValueNotFound(val.to_owned(), self[val].to_string()))
            }
        }
        else
        {
            return Err(ConverterError::ParseUnsuitableType(format!("{} не является строкой",self[val].to_string())));
        }
    }

    ///Конвертирование строки в другой формат 
    fn get_value_as_bool(&self, val : &str) -> Result<bool, ConverterError>
    {
        if self[val].is_null()
        {
            if val == "$id"
            {
                return Err(ConverterError::PropertyNameNotFound(self["$id"].to_string(), self.to_string()));
            }
            else
            {
                return Err(ConverterError::PropertyNameNotFound(self["$id"].to_string(), val.to_owned()));
            }
        }
        else if let Some(b) = self[val].as_bool()
        {
            Ok(b)
        }
        else
        {
            return Err(ConverterError::ParseUnsuitableType(format!("{} не является булевым значением",self[val].to_string())));
        }
    }
    fn get_value_as_number(&self, val : &str) -> Result<String, ConverterError>
    {
        if self[val].is_null()
        {
            if val == "$id"
            {
                return Err(ConverterError::PropertyNameNotFound(self["$id"].to_string(), self.to_string()));
            }
            else
            {
                return Err(ConverterError::PropertyNameNotFound(self["$id"].to_string(), val.to_owned()));
            }
        }
        else if self[val].is_f64() || self[val].is_i64() || self[val].is_u64() || self[val].is_i64()
        {
            Ok(self[val].to_string())
        }
        else
        {
            return Err(ConverterError::ParseUnsuitableType(format!("{} не является номером",self[val].to_string())));
        }
    }
    fn get_value_as_array(&self, val : &str) -> Result<Vec<String>, ConverterError>
    {
        if self[val].is_null()
        {
            return Err(ConverterError::PropertyNameNotFound(self["$id"].to_string(), val.to_owned()));
        }
        if self[val].is_null()
        {
            return Err(ConverterError::ParseUnsuitableType(format!("Отсутсвует свойство {}", val)));
        }
        if let Some(v) = self[val].as_array()
        {
            if v.into_iter().nth(0).unwrap().is_string()
            {
                let values : Vec<String>  = v.into_iter()
                                        .map(|s|s.as_str()
                                            .unwrap()
                                            .to_owned())
                                        .collect();
            
                Ok(values)
            }
            else if v.into_iter().nth(0).unwrap().is_number()
            {
                let values : Vec<String>  = v.into_iter()
                                        .map(|s|s.as_u64()
                                            .unwrap()
                                            .to_string())
                                        .collect();
            
                Ok(values)
            }
            else
            {
                Err(ConverterError::ArrayInternalTypeNotSupported(self[val].to_string()))
            }
        }
        else
        {
            return Err(ConverterError::ArrayInternalTypeNotSupported(self[val].to_string()));
        }
    }

    fn get_id(&self) -> Result<String, ConverterError>
    {
        let v = <Self as ValueType>::get_value_as_string(self, "$id")?;
        Ok(v)
    }
    fn get_title(&self) -> Result<String, ConverterError>
    {
        let v = <Self as ValueType>::get_value_as_string(self,"title")?;
        Ok(v)
    }
    fn get_description(&self) -> Result<String, ConverterError>
    {
        let v = <Self as ValueType>::get_value_as_string(self,"description")?;
        Ok(v)
    }
    fn get_schema(&self) -> Result<String, ConverterError>
    {
        let v = <Self as ValueType>::get_value_as_string(self,"$schema")?;
        Ok(v)
    }
    fn get_default(&self) -> Result<String, ConverterError>
    {
        let v = <Self as ValueType>::get_value_as_string(self,"default")?;
        Ok(v)
    }
    fn get_deprecated(&self) -> Result<bool, ConverterError>
    {
        let v = <Self as ValueType>::get_value_as_bool(self,"deprecated")?;
        Ok(v)
    }
   
    fn get_required(&self) -> Result<RequiredProperty, ConverterError>
    {
        let mut req : Option<RequiredProperty> = None;
        if let Some(req_object) = self.as_object()
        {
            if req_object.contains_key("required") || req_object.contains_key("allOf") || req_object.contains_key("oneOf")
            {
                //ищем requireq
                if req_object.contains_key("required")
                {
                    let r = <Self as ValueType>::get_value_as_array(self,"required")?;
                    req = Some(RequiredProperty { required: Some(r), oneof: None, allof: None })
                }
                //ищем allOf и вложение в виде oneOf
                if req_object.contains_key("allOf")
                {
                    if req.is_none()
                    {
                        req = Some(RequiredProperty { required: None, oneof: None, allof: Some(vec![]) })
                    }
                    else
                    {
                        req.as_mut().unwrap().allof = Some(vec![]);
                    }
                    if let Some(array_all_of) = req_object["allOf"].as_array()
                    {
                        for a_o in array_all_of
                        {
                            if let Some(all_obj) = a_o.as_object()
                            {
                                if all_obj.contains_key("oneOf")
                                {
                                    let mut one_vec : Option<Vec<String>> = Some(vec![]);
                                    if let Some(array_one_of) = all_obj["oneOf"].as_array()
                                    {
                                        for o_o in array_one_of
                                        {               
                                            if let Some(one_obj) = o_o.as_object()
                                            {
                                                if one_obj.contains_key("required")
                                                {
                                                    if let Some(one_of_required_array) = one_obj["required"].as_array()
                                                    {
                                                        for one in one_of_required_array
                                                        {
                                                            one_vec.as_mut().unwrap().push(one.as_str().unwrap().to_owned());
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    if one_vec.is_some()
                                    {
                                        req.as_mut().unwrap().allof.as_mut().unwrap().push(one_vec.unwrap());
                                    }
                                    
                                }
                            }
                        }
                    } 
                }
                if req_object.contains_key("oneOf")
                {
                    if req.is_none()
                    {
                        req = Some(RequiredProperty { required: None, oneof: Some(vec![]), allof: None })
                    }
                    else
                    {
                        req.as_mut().unwrap().oneof = Some(vec![]);
                    }

                    if let Some(array_one_of) = req_object["oneOf"].as_array()
                    {
                        for o_o in array_one_of
                        {               
                            if let Some(one_obj) = o_o.as_object()
                            {
                                if one_obj.contains_key("required")
                                {
                                    if let Some(one_of_required_array) = one_obj["required"].as_array()
                                    {
                                        for one in one_of_required_array
                                        {
                                            req.as_mut().unwrap().oneof.as_mut().unwrap().push(one.as_str().unwrap().to_owned());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            return req.ok_or(ConverterError::PropertyNameNotFound(req_object["$id"].to_string(), "required".to_owned()));
        }
        else
        {
            return Err(ConverterError::ParseUnsuitableType(format!("{} не является объектом, не могу обнаружить required", self["$id"].to_string())));
        }
    }
    
}