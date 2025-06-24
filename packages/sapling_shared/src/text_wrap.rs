#[derive(Debug, Clone)]
pub struct TextNode<'a> {
    pub text: Option<&'a str>,
    pub id: Option<&'a str>,
}

pub fn wrapped_by_text<'a>(list: &[Option<TextNode<'a>>], start_index: usize) -> bool {
    let mut index = start_index;
    let mut wrapped = false;

    while index > 0 {
        index -= 1;
        if let Some(Some(node)) = list.get(index) {
            if node.text.is_some() {
                wrapped = true;
                break;
            }
            if node.id.is_some() {
                return false;
            }
        }
    }

    if !wrapped {
        return false;
    }

    index = start_index;
    while index < list.len() {
        if let Some(Some(node)) = list.get(index) {
            if node.text.is_some() {
                return true;
            }
            if node.id.is_some() {
                return false;
            }
        }
        index += 1;
    }
    false
}
