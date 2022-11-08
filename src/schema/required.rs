use super::consts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RequiredProperty
{
    pub required : Option<Vec<String>>,
    pub oneof : Option<Vec<String>>,
    pub allof :  Option<Vec<Vec<String>>>,
}
impl RequiredProperty
{
    pub fn get_required(&self, property_name : &String) -> Option<(String, String)>
    {
        if let Some(r) = self.required.as_ref()
        {
            if r.contains(property_name)
            {
                return Some((consts::REQUIRED.to_owned(), "`Да`".to_owned()));
            }
            else if let Some(n) = self.get_oneof(property_name)
            {
                return Some(n);
            }
            else
            {
                return Some((consts::REQUIRED.to_owned(), "`Нет`".to_owned()));
            }
        }
        return self.get_oneof(property_name);
    }

    fn get_oneof(&self, property_name : &String) -> Option<(String, String)>
    {
        if self.allof.is_some()
        {
            let mut all_req : String = String::from("");
            let mut contains : bool = false;
            for (i, a) in self.allof.as_ref().unwrap().into_iter().enumerate()
            {
                if a.contains(property_name)
                {
                    contains = true;
                }
                if i < self.allof.as_ref().unwrap().len() - 1
                {
                    all_req.push_str(format!("`{}` и ", a.join("` или `")).as_str());
                }
                else 
                {
                    all_req.push_str(format!("`{}`", a.join("` или `")).as_str())
                }
            }
            if contains
            {
                return Some(("Используется в комбинации".to_owned(), all_req));
            }
        }
        if self.oneof.is_some()
        {
            if self.oneof.as_ref().unwrap().contains(property_name)
            {
                return Some(("Должно быть только одно".to_owned(), format!("`{}`", self.oneof.as_ref().unwrap().join("` или `"))));
            }
        }
        return None
    }
}