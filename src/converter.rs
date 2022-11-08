use std::{fs::{self, File}, io::Write, path::Path};
use chrono::{Local, DateTime, NaiveDateTime, TimeZone};
use colored::Colorize;
use serde_json::{Value, Map};
use crate::{converter_error::ConverterError,
            schema::{object::JsonObject,
                key_value::KeyValue,
                bool::JsonBool,
                string::JsonString,
                number::JsonNumber,
                array::JsonArray,
                referience::JsonRef,
                version::Version,
                required::RequiredProperty, key_name::KeyName},
                parser::Parser,
                value_type::ValueType, markdown::Markdown, converter_type::ConverterType};

#[derive(Debug, Clone)]
pub struct JsonshchemaConverter
{
    pub schema : Value,
    pub data : Vec<String>,
    ///Это на случай если надо будет создать разные документации (для печати)
    pub defs : Vec<Vec<String>>,
    pub errors : Vec<String>,
    pub json_schema_path : String,
    pub json_schema_version_path : String,
    pub output_file_path : String,
    pub converter_type : ConverterType
}

impl JsonshchemaConverter
{
    //pub fn new(file_path : &str) -> Result<Self, Box<dyn Error>>
    //В этом случае ошибка транслируется напрямую, без обертки
    //т.е. если была io::Error то будет транслироваться сразу она
    //если я поставлю ConverterError, то io::Error будет транслироваться через обертку ConverterError
    pub fn new(json_schema_path : &str, json_schema_version_path : &str, output_file_path : &str,  converter : ConverterType) -> Result<Self, ConverterError>
    {
        //так работает но выводим сразу напрямую
        let json = fs::read_to_string(json_schema_path)?;
        let schm: Value = serde_json::from_str(json.as_str())?;
        //println!("{} {}", "Прочитана схема".green(), &schm["$id"].as_str().unwrap().green());
        Ok(JsonshchemaConverter 
        {
            schema: schm,
            data: vec![],
            errors : vec![],
            defs : vec![vec![]],
            json_schema_path : json_schema_path.to_owned(),
            json_schema_version_path : json_schema_version_path.to_owned(),
            output_file_path : output_file_path.to_owned(),
            converter_type : converter
        })
    }
    pub fn generate(&mut self) -> Result<Self, ConverterError>
    {
        let root = self.schema.clone();
        let root_object = JsonObject::parse(KeyValue::new(None, &root), None)?;
        self.write_header();
        match self.converter_type
        {
            ConverterType::Markdown => self.data.append(root_object.as_keys().get_markdown(0, false, None).as_mut()),
            ConverterType::Pug => todo!(),
            ConverterType::Tex => todo!()

        }
        let required = root.get_required();
        self.recursive_props(&root_object.properties,
                            required.ok(),
                            1,
                            None)?;

        self.write_defs(&root);
        self.write_footer();
        self.write_table_style();
        
        
        Ok(self.clone())
    }

    pub fn write_data(&self)
    {
        let _wr =  self.write_each(self.output_file_path.as_str(), &self.data);
    }
    fn write_each(&self, path: impl AsRef<Path>, items: impl IntoIterator<Item=impl AsRef<[u8]>>) -> std::io::Result<()> 
    {
        let mut file = File::create(path)?;
        for i in items 
        {
            file.write_all(i.as_ref())?;
        }
        // Surface any I/O errors that could otherwise be swallowed when
        // the file is closed implicitly by being dropped.
        file.sync_all()
    }
    // fn write_each2(&self, path: impl AsRef<Path>, items: impl IntoIterator<Item=String>) -> std::io::Result<()> 
    // {
    //     let mut file = File::create(path)?;
    //     for i in items 
    //     {
    //         file.write_all(i.as_ref())?;
    //     }
    //     // Surface any I/O errors that could otherwise be swallowed when
    //     // the file is closed implicitly by being dropped.
    //     file.sync_all()
    // }
    
    fn write_defs(&mut self, root : &Value)
    {
        let split_schemas = false;
        if !self.schema["$defs"].is_null()
        {
            let defs = root["$defs"].as_object();
            let mut defs_vec : Vec<JsonshchemaConverter> = vec![];
            if split_schemas
            {
                for d in defs
                {
                    let mut d1 = JsonshchemaConverter 
                    {
                        schema: self.schema.clone(),
                        data: vec![],
                        errors : vec![],
                        defs : vec![vec![]],
                        json_schema_path : "json_schema_path".to_owned(),
                        json_schema_version_path : "json_schema_version_path".to_owned(),
                        output_file_path : "output_file_path".to_owned(),
                        converter_type : self.converter_type
                    };
                    let _w = d1.recursive_props(d, None, 0, None);
                    defs_vec.push(d1);
                    if _w.is_err()
                    {
                        println!("{}", _w.err().unwrap())
                    }
                }
            }
            else
            {
                for d in defs
                {
                    let _w = self.recursive_props(d,
                        None,
                        0,
                        None);
                    if _w.is_err()
                    {
                        println!("{}", _w.err().unwrap())
                    }
                }
            }
        }
    }
    fn recursive_props(&mut self, props : &Map<String, Value>, required: Option<RequiredProperty>, level : i16, parent : Option<&Vec<KeyName>>) -> Result<(), ConverterError>
    {
        for p in props
        {
            if let Ok(tp) = p.1.get_schema_type()
            {
                let key = KeyValue::new(Some(p.0), p.1);
                match tp
                {
                    crate::object_type::ObjectType::Boolean => 
                    {
                        let obj = JsonBool::parse(key, required.clone())?;
                        match self.converter_type
                        {
                            ConverterType::Markdown => self.data.append(obj.as_keys().get_markdown(level, false, parent).as_mut()),
                            ConverterType::Pug => todo!(),
                            ConverterType::Tex => todo!()

                        }
                    }
                    crate::object_type::ObjectType::String => 
                    {
                        let obj = JsonString::parse(key, required.clone())?;
                        match self.converter_type
                        {
                            ConverterType::Markdown => self.data.append(obj.as_keys().get_markdown(level, false, parent).as_mut()),
                            ConverterType::Pug => todo!(),
                            ConverterType::Tex => todo!()

                        }
                    }
                    crate::object_type::ObjectType::Integer => 
                    {
                        let obj = JsonNumber::parse(key, required.clone())?;
                        match self.converter_type
                        {
                            ConverterType::Markdown => self.data.append(obj.as_keys().get_markdown(level, false, parent).as_mut()),
                            ConverterType::Pug => todo!(),
                            ConverterType::Tex => todo!()

                        }
                    }
                    crate::object_type::ObjectType::Object => 
                    {
                        let obj = JsonObject::parse(key.clone(), required.clone())?;
                        match self.converter_type
                        {
                            ConverterType::Markdown => self.data.append(obj.as_keys().get_markdown(level, true, parent).as_mut()),
                            ConverterType::Pug => todo!(),
                            ConverterType::Tex => todo!()

                        }
                        let req = key.value.get_required().ok();
                        self.recursive_props(&obj.properties, req, level + 1, Some(&obj.as_keys()))?;
                    },
                    crate::object_type::ObjectType::Array => 
                    {
                        let obj = JsonArray::parse(key.clone(), required.clone())?;
                        match self.converter_type
                        {
                            ConverterType::Markdown => self.data.append(obj.as_keys().get_markdown(level, true, parent).as_mut()),
                            ConverterType::Pug => todo!(),
                            ConverterType::Tex => todo!()

                        }
                        //если итемы не ссылка, то надо обработать объект внутри массива
                        if obj.items_ref.is_none()
                        {
                            let mut m = Map::new();
                            m.insert("items".to_owned(), obj.items.clone());
                            let req = key.value.get_required().ok();
                            self.recursive_props(&m, req, level + 1, Some(&obj.as_keys()))?;
                        }
                    },
                    crate::object_type::ObjectType::Ref => 
                    {
                        let obj = JsonRef::parse(key.clone(), required.clone())?;
                        match self.converter_type
                        {
                            ConverterType::Markdown => self.data.append(obj.as_keys().get_markdown(level, true, parent).as_mut()),
                            ConverterType::Pug => todo!(),
                            ConverterType::Tex => todo!()
                        }
                    },
                    crate::object_type::ObjectType::Null => 
                    {
                        let error = "Тип объекта не может быть NULL!".to_owned();
                        return Err(ConverterError::ParseUnsuitableType(error));
                    },
                    crate::object_type::ObjectType::Undefined => ()
                }
            }
            
        }
        Ok(())
    }

    fn write_header(&mut self)
    {
        match self.converter_type
        {
            ConverterType::Markdown => 
            {
                self.data.push("## Схема формата разметки".to_owned());
                self.data.push("\n".to_owned());
                self.data.push("\n".to_owned());
            },
            ConverterType::Pug => todo!(),
            ConverterType::Tex => todo!()
        }
    }

    fn write_footer(&mut self)
    {
        let version_json = fs::read_to_string(self.json_schema_version_path.as_str());
        if version_json.is_err()
        {
            //self.errors.push(version_json.err().unwrap().to_string());
            println!("{}", version_json.err().unwrap().to_string().red());
            return;
        }
        let version = serde_json::from_str::<Version>(version_json.unwrap().as_str());
        if version.is_err()
        {
            println!("{}", version.err().unwrap().to_string().red());
            //self.errors.push(version.err().unwrap().to_string());
            return;
        }
        let mut  version = version.unwrap();
        let schema_metadata = fs::metadata(self.json_schema_path.as_str());
        if schema_metadata.is_err()
        {
            self.errors.push(schema_metadata.err().unwrap().to_string());
            return;
        }
        let schema_metadata = schema_metadata.unwrap();
        let schema_modifed : DateTime<Local> = schema_metadata.modified().unwrap().into();
        let rtest = NaiveDateTime::parse_from_str(version.modifed.as_str(), "%Y-%m-%dT%H:%M:%S");
        let version_modifed: DateTime<Local> = Local.from_local_datetime(&rtest.unwrap()).unwrap();
        let date_now : DateTime<Local> = Local::now();
        if version_modifed < schema_modifed
        {
            version.ver.path = version.ver.path +1;
            version.modifed = date_now.format("%Y-%m-%dT%H:%M:%S").to_string();
            let _r = std::fs::write(
                "../format/version.json",
                serde_json::to_string_pretty(&version).unwrap(),
            );
        }
        match self.converter_type
        {
            ConverterType::Markdown => 
            {
                self.data.push(format!("\n{}\n\n", "----"));
                self.data.push(format!("\n| Документация сгенерирована |**{}**|", date_now.format("%Y-%m-%d в %H:%M:%S")));
                self.data.push("\n|---------------------|:--------------------|".to_owned());
                self.data.push(format!("\n| **Дата изменения схемы** |**{}**|", version_modifed.format("%Y-%m-%d в %H:%M:%S")));
                self.data.push(format!("\n| **Версия схемы** |**{}.{}.{}**|", version.ver.major, version.ver.minor, version.ver.path));
            },
            ConverterType::Pug => todo!(),
            ConverterType::Tex => todo!()
        }
       
        println!("{} {}, версия схемы {}.{}.{}", "Успешно сгенерирован файл".green(),
                        &self.output_file_path.green(),
                        version.ver.major.to_string().red(),
                        version.ver.minor.to_string().red(),
                        version.ver.path.to_string().red());
    }

    fn write_table_style(&mut self)
    {
        match self.converter_type
        {
            ConverterType::Markdown => 
            {
                self.data.push("\n".to_owned());
                self.data.push("\n".to_owned());
                self.data.push(r#"<style>
                table th:first-of-type 
                {
                    width: 200px;
                }
                table th:nth-of-type(2) 
                {
                    width: 400px;
                }
                .avoid-break
                {
                  break-inside: avoid;
                }
                </style>"#.to_owned());
            },
            ConverterType::Pug => todo!(),
            ConverterType::Tex => todo!()
        }
        
    }

}