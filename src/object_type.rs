use std::fmt;

#[derive(PartialEq, Debug, Eq, Clone)]
pub enum ObjectType
{
    Object,
    Array,
    String,
    Boolean,
    Integer,
    Null,
    Ref,
    Undefined
}
impl fmt::Display for ObjectType 
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result 
    {
        let o_type = match self
        {
            ObjectType::String => "string",
            ObjectType::Boolean => "bool",
            ObjectType::Integer => "integer",
            ObjectType::Array => "array",
            ObjectType::Object => "object",
            ObjectType::Null => "null",
            ObjectType::Ref => "$ref",
            ObjectType::Undefined => "тип неизвестен!"
        };
        write!(f, "{}", o_type)
        // или как альтернатива:
        // fmt::Debug::fmt(self, f)
    }
}