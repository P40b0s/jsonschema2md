use crate::schema::key_name::KeyName;
pub const TITLE_COLOR : &str ="#698c2b";
pub const PROPERTY_NAME_COLOR : &str ="#3180b4";
pub const INDENTATION : &str = "  ";

pub trait Markdown
{
    fn get_markdown(&self, level : i16, as_list : bool, parent : Option<&Vec<KeyName>>) -> Vec<String>;
}

impl<'a> Markdown for Vec<KeyName<'a>>
{
    fn get_markdown(&self, level : i16, as_list : bool, parent : Option<&Vec<KeyName>>) -> Vec<String>
    {
        let mut vec : Vec<String>= vec![];
        let identation = get_identation(level);
        for k in self
        {
            match k 
            {
                KeyName::Name(name) => 
                {
                    if parent.is_some()
                    {
                        for k1 in parent.unwrap()
                        {
                            if let KeyName::Id(val) = k1
                            {
                                println!("{}", val);
                            }
                            
                        }
                    }
                    //Включает name и ref
                    if as_list
                    {
                        let complex_name : String = format!("{}- #### Свойство: <span style=\"color:{}\">`{}`</span>",
                        &identation,
                        PROPERTY_NAME_COLOR,
                        name);
                        vec.push(check_ref_as_list(complex_name, self))
                    }
                    else 
                    {
                        let mut title : String = "Свойство".to_owned();
                        let mut description : String = "".to_owned();
                        self.iter().any(|k| match k
                            {
                                KeyName::Title(ttl) =>
                                {
                                    title = ttl.clone();
                                    true
                                },
                                KeyName::Description(ds) =>
                                {
                                    description = ds.clone();
                                    true
                                },
                                _ => false
                            });
                        let output = format!("{}|{}<br><span style=\"color:{}\">{}</span>|<b><span style=\"color:{};\">`{}`</b>|",
                        &identation,
                        title,
                        TITLE_COLOR,
                        description,
                        PROPERTY_NAME_COLOR,
                        name);
                        vec.push(check_ref_as_table(output, self));
                        vec.push("\n".to_owned());
                        vec.push(format!("{}|:-|:-:|\n", &identation));
                    }
                }
                //$ref мы уже проверили выше, дальше ненужно
                KeyName::RefId(_rf) => (),
                KeyName::Title(title) =>
                {
                    if as_list
                    {
                        vec.push(format!("{}  <span style=\"color:{}\">{}</span>  \n",
                            &identation,
                            TITLE_COLOR,
                            title));
                        
                    }
                }
                KeyName::Description(description) =>
                {
                    if as_list
                    {
                        vec.push(format!("{}  {}  \n",
                            &identation,
                            description));
                        
                    }
                }
                KeyName::Schema(schema) =>
                {
                    let property_name = KeyName::Schema("".to_owned()).to_string();
                    if as_list
                    {
                        vec.push(format!("{}  {}: <span style=\"color:{}\">`{}`</b>  \n",
                        identation,
                        &property_name,
                        PROPERTY_NAME_COLOR,
                        schema));
                    }
                    else
                    {
                        push_value_table_format(vec.as_mut(),
                                            &identation,
                                            &property_name,
                                            schema,
                                            ValueStyle::Code)
                    }
                  
                }
                KeyName::Id(id) =>
                {
                    let property_name = KeyName::Id("".to_owned()).to_string();
                    let id_url = id_conversion(id.as_str());
                    if as_list
                    {
                        vec.push(format!("{}  {}: <b id=\"{}\">`{}`</b>  \n",
                            &identation,
                            &property_name,
                            id_url,
                            id));
                    }
                    else
                    {
                        vec.push(format!("{}|{}|<b id=\"{}\">`{}`</b>|  \n",
                                &identation,
                                &property_name,
                                id_url,
                                id));
                    }
                    
                }
                KeyName::Type(object_type) =>
                {
                    let property_name = KeyName::Type(crate::object_type::ObjectType::Undefined).to_string();
                    if as_list
                    {
                        push_value_list_format(vec.as_mut(), &identation,  &property_name, object_type.to_string().as_str(), ValueStyle::Code)
                    }
                    else 
                    {
                        push_value_table_format(vec.as_mut(), &identation, &property_name, object_type.to_string().as_str(), ValueStyle::Code)
                    }
                }
                KeyName::Required(required, prop) =>
                {
                    if let Some(r) = required.get_required(prop)
                    {
                        if as_list
                        {
                            vec.push(format!("{}  {}: {}  \n", &identation, r.0, r.1));
                        }
                        else
                        {
                            vec.push(format!("{}|{}|{}|  \n", &identation, r.0, r.1));
                        }
                    }
                }
                KeyName::Default(def)=>
                {
                    if !def.is_empty()
                    {
                        let property_name = KeyName::Default("".to_owned()).to_string();
                        if as_list
                        {
                            push_value_list_format(vec.as_mut(), &identation, &property_name, def, ValueStyle::Code)
                        }
                        else 
                        {
                            push_value_table_format(vec.as_mut(), &identation, &property_name, def,ValueStyle::Code)
                        }
                    }
                }
                KeyName::Pattern(pat)=>
                {
                    let property_name = KeyName::Pattern("".to_owned()).to_string();
                    let pattern = pattern_conversion(pat);
                    if as_list
                    {
                        push_value_list_format(vec.as_mut(), &identation, &property_name, &pattern, ValueStyle::Bold)
                    }
                    else 
                    {
                        push_value_table_format(vec.as_mut(), &identation, &property_name, &pattern, ValueStyle::Bold)
                    }
                }
                KeyName::Examples(ex)=>
                {
                    let property_name = KeyName::Examples(vec![]).to_string();
                    let examples = get_examples(ex);
                    if as_list
                    {
                        
                        push_value_list_format(vec.as_mut(), &identation, &property_name, &examples, ValueStyle::Bold)
                    }
                    else 
                    {
                        push_value_table_format(vec.as_mut(), &identation, &property_name, &examples, ValueStyle::Bold)
                    }
                }
                KeyName::MaxItems(item)=>
                {
                    let property_name = KeyName::MaxItems("".to_owned()).to_string();
                    if as_list
                    {
                        push_value_list_format(vec.as_mut(), &identation, &property_name, item,ValueStyle::Code)
                    }
                    else 
                    {
                        push_value_table_format(vec.as_mut(), &identation, &property_name, item,ValueStyle::Code)
                    }
                }
                KeyName::MinItems(item)=>
                {
                    let property_name = KeyName::MinItems("".to_owned()).to_string();
                    if as_list
                    {
                        push_value_list_format(vec.as_mut(), &identation, &property_name, item,ValueStyle::Code)
                    }
                    else 
                    {
                        push_value_table_format(vec.as_mut(), &identation, &property_name, item,ValueStyle::Code)
                    }
                }
                KeyName::ItemsRef(items_ref)=>
                {
                    let property_name = KeyName::ItemsRef("".to_owned()).to_string();
                    if as_list
                    {
                        let id_url = id_conversion(items_ref);
                        vec.push(format!("{}  {}: [{}](#{})  \n",
                        identation,
                        property_name,
                        items_ref,
                        id_url));
                    }
                },
                KeyName::Minimum(item) =>
                {
                    let property_name = KeyName::Minimum("".to_owned()).to_string();
                    if as_list
                    {
                        push_value_list_format(vec.as_mut(), &identation, &property_name, item,ValueStyle::Code)
                    }
                    else 
                    {
                        push_value_table_format(vec.as_mut(), &identation, &property_name, item,ValueStyle::Code)
                    }
                },
                KeyName::Maximum(item) =>
                {
                    let property_name = KeyName::Maximum("".to_owned()).to_string();
                    if as_list
                    {
                        push_value_list_format(vec.as_mut(), &identation, &property_name, item,ValueStyle::Code)
                    }
                    else 
                    {
                        push_value_table_format(vec.as_mut(), &identation, &property_name, item,ValueStyle::Code)
                    }
                },
                KeyName::MultipleOf(item) =>
                {
                    let property_name = KeyName::MultipleOf("".to_owned()).to_string();
                    if as_list
                    {
                        push_value_list_format(vec.as_mut(), &identation, &property_name, item,ValueStyle::Code)
                    }
                    else 
                    {
                        push_value_table_format(vec.as_mut(), &identation, &property_name, item,ValueStyle::Code)
                    }
                }
                KeyName::ExclusiveMinimum(item) =>
                {
                    let property_name = KeyName::ExclusiveMinimum("".to_owned()).to_string();
                    if as_list
                    {
                        push_value_list_format(vec.as_mut(), &identation, &property_name, item,ValueStyle::Code)
                    }
                    else 
                    {
                        push_value_table_format(vec.as_mut(), &identation, &property_name, item,ValueStyle::Code)
                    }
                },
                KeyName::ExclusiveMaximum(item) =>
                {
                    let property_name = KeyName::ExclusiveMaximum("".to_owned()).to_string();
                    if as_list
                    {
                        push_value_list_format(vec.as_mut(), &identation, &property_name, item,ValueStyle::Code)
                    }
                    else 
                    {
                        push_value_table_format(vec.as_mut(), &identation, &property_name, item,ValueStyle::Code)
                    }
                },
                KeyName::MinProperties(item) =>
                {
                    let property_name = KeyName::MinProperties("".to_owned()).to_string();
                    if as_list
                    {
                        push_value_list_format(vec.as_mut(), &identation, &property_name, item,ValueStyle::Code)
                    }
                    else 
                    {
                        push_value_table_format(vec.as_mut(), &identation, &property_name, item,ValueStyle::Code)
                    }
                },
                KeyName::MaxProperties(item) =>
                {
                    let property_name = KeyName::MaxProperties("".to_owned()).to_string();
                    if as_list
                    {
                        push_value_list_format(vec.as_mut(), &identation, &property_name, item,ValueStyle::Code)
                    }
                    else 
                    {
                        push_value_table_format(vec.as_mut(), &identation, &property_name, item,ValueStyle::Code)
                    }
                },
                KeyName::ContentEncoding(item) =>
                {
                    let property_name = KeyName::ContentEncoding("".to_owned()).to_string();
                    if as_list
                    {
                        push_value_list_format(vec.as_mut(), &identation, &property_name, item,ValueStyle::Code)
                    }
                    else 
                    {
                        push_value_table_format(vec.as_mut(), &identation, &property_name, item,ValueStyle::Code)
                    }
                },
                KeyName::ContentMediaType(item) =>
                {
                    let property_name = KeyName::ContentMediaType("".to_owned()).to_string();
                    if as_list
                    {
                        push_value_list_format(vec.as_mut(), &identation, &property_name, item,ValueStyle::Code)
                    }
                    else 
                    {
                        push_value_table_format(vec.as_mut(), &identation, &property_name, item,ValueStyle::Code)
                    }
                }

            }
        }
        vec
    }
    
}

fn check_ref_as_list(parent_string : String, vec : &Vec<KeyName>) -> String
{
    let mut name = parent_string;
    vec.iter().any(|k| match k
        {
            KeyName::RefId(rf) =>
            {
                let id_url = id_conversion(rf);
                let ref_f = format!(", схема - [{}](#{})",
                        rf,
                        id_url);
                name.push_str(ref_f.as_str());
                true
            },
            _ => false
        });
        name.push_str("  \n");
        name
}
fn check_ref_as_table(parent_string : String, vec : &Vec<KeyName>) -> String
{
    let mut name = parent_string;
    vec.iter().any(|k| match k
        {
            KeyName::RefId(rf) =>
            {
                let id_url = id_conversion(rf);
                let ref_f = format!(", id: [{}](#{})",
                        rf,
                        id_url);
                name.push_str(ref_f.as_str());
                true
            },
            _ => false
        });
        name.push_str("  \n");
        name
}

fn push_value_list_format(vec : &mut Vec<String>, identation : &str, property_name : &str, property_value : &str, style : ValueStyle)
{
    match style 
    {
        ValueStyle::Code =>
        {
            vec.push(format!("{}  {}: `{}`  \n",
                identation,
                property_name,
                property_value));
        },
        ValueStyle::Bold =>
        {
            vec.push(format!("{}  {}: **{}**  \n",
                identation,
                property_name,
                property_value));
        },
        ValueStyle::Normal =>
        {
            vec.push(format!("{}  {}: {}  \n",
                identation,
                property_name,
                property_value));
        }
    }
}

fn push_value_table_format(vec : &mut Vec<String>, identation : &str, property_name : &str, property_value : &str, style : ValueStyle)
{
    match style 
    {
        ValueStyle::Bold =>
        {
            vec.push(format!("{}|{}|<b>{}</b>|  \n",
                identation,
                property_name,
                property_value));
        },
        ValueStyle::Code =>
        {
            vec.push(format!("{}|{}|`{}`|  \n",
                identation,
                property_name,
                property_value));
        },
        ValueStyle::Normal =>
        {
            vec.push(format!("{}|{}|{}|  \n",
                identation,
                property_name,
                property_value));
        }
    }
    
}

///Форматирование списка примеров
fn get_examples(examples : &Vec<String>) -> String
{
    let mut examples_string = String::from("");
    for (i, e) in examples.into_iter().enumerate()
    {
        if i == 0
        {
            examples_string.push_str(format!("`{}`", e).as_str())
        }
        else 
        {
            examples_string.push_str(format!("<br>`{}`", e).as_str())
        }
    }
    examples_string
}
///Получения нужного количества отступов для формирования уровня
fn get_identation(level : i16) -> String
{
    let mut indent :String = String::from("");
    if level > 0
    {
        indent = INDENTATION.repeat(level.try_into().unwrap())
    }
    indent
}
///Конвертация id для возможности простановки ссылок
fn id_conversion(id : &str) -> String
{
    let id_for_url:String = id
    .to_lowercase()
    .replace("#", "")
    .chars()
    .map(|x| match x 
        { 
            '/' => '-',
            '_' => '-', 
            _ => x
        }).collect();
        // { 
        // '/' => '-', 
        // '0'..='9' => 'x', 
        // 'a'..='z' => 'x',
        // _ => x
        // }).collect();
    id_for_url
}
///Конвертация символов | для корректного отображения регексов
fn pattern_conversion(pattern : &str) -> String
{
    let p:String = pattern
    .replace("|", "&#124;")
    .replace("[", "&#91;")
    .replace("]", "&#93;")
    .replace("^", "&#94;");
    p
}

enum ValueStyle
{
    Bold,
    Code,
    Normal
}