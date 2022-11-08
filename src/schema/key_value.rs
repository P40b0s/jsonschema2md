use serde_json::Value;

#[derive(Clone, Debug)]
pub struct KeyValue<'a>
{
    pub name : Option<&'a str>,
    pub value : &'a Value
}
impl<'a> KeyValue<'a>
{
    pub fn new(name : Option<&'a String>, val :  &'a Value) -> Self
    {
        KeyValue { name: name.map_or(None, |f| Some(f.as_str())), value: val } 
    }
}