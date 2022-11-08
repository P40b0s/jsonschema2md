use chrono::format::format;

use crate::schema::consts;

pub const PAGE_BRAKE_AFTER_AUTO : &str = "<div style=\"page-break-after: auto;\">";
pub const PAGE_BRAKE_AFTER_AVOID : &str = "<div style=\"page-break-after: avoid;\">";
///Форматирование списка примеров
pub fn get_examples(examples : &Vec<String>) -> String
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
pub fn get_identation(level : i16) -> String
{
    let mut indent :String = String::from("");
    if level > 0
    {
        indent = consts::INDENTATION.repeat(level.try_into().unwrap())
    }
    indent
}
///Конвертация id для возможности простановки ссылок
pub fn id_conversion(id : &str) -> String
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
pub fn pattern_conversion(pattern : &str) -> String
{
    let p:String = pattern
    .replace("|", "&#124;")
    .replace("[", "&#91;")
    .replace("]", "&#93;")
    .replace("^", "&#94;");
    p
}
///Начало блока в котором нельзя делеть перенос на другую страницу, <br> необходимо для распечатки документа, иначе вся инфа получается в разрыве строки
pub fn page_breke_after_auto(vec : &mut Vec<String>, level:i16)
{
    let identation = get_identation(level);
    vec.push(format!("{}{} \n\n", identation, PAGE_BRAKE_AFTER_AUTO ));
}
///Конец блока в котором нельзя делеть перенос на другую страницу, <br> необходимо для распечатки документа, иначе вся инфа получается в разрыве строки
pub fn page_brake_after_avoid(vec : &mut Vec<String>, level:i16)
{
    let identation = get_identation(level);
    vec.push(format!("\n\n{}{}\n\n", identation, PAGE_BRAKE_AFTER_AVOID ));
}
///Встраиваем тэг в конец строки последнего элемента массива
///Клонируем добавляем тэг и заменяем целиком, 
///потому что чтобы получить мутабельную ссылку на сьтроку, она должна быть изначально мутабельна, а у нас она иммутабельна
pub fn inject_brake_off(vec : &mut Vec<String>)
{
    if let Some(last) = vec.last()
    {
        let l = last.clone().replace('\n', "");
        let new_str = format!("{}{}\n\n", l, "</div>");
        let index = vec.len() - 1;
        let _got = std::mem::replace(&mut vec[index], new_str);
    }
}

pub fn write_avoid_page_break_start(vec : &mut Vec<String>, level:i16)
{
    let identation = get_identation(level);
    vec.push(format!("{}{} \n\n", identation, PAGE_BRAKE_AFTER_AVOID ));
}
pub fn write_avoid_page_break_end(vec : &mut Vec<String>, level:i16)
{
    let identation = get_identation(level);
    vec.push(format!("{}{} \n\n", identation, "</div>" ));
}