use crate::errors::ParserError;
use roxmltree::{ExpandedName, Node, NS_XML_URI};
use std::borrow::Cow;
use std::fs;
use std::io;

macro_rules! const_strs {
    ( $( $id:ident : $val:expr ),* $(,)? ) => {
        $(const $id: &str = $val;)*
    };
}

pub fn find_child_tag<'a>(n: Node<'a, 'a>, tag_name: &str) -> Option<Node<'a, 'a>> {
    n.children().find(|c| c.tag_name().name() == tag_name)
}

pub fn get_node_text<'a>(n: Node<'a, 'a>) -> Result<Cow<'a, str>, ParserError> {
    n.text().ok_or(ParserError::MissingText).map(|t| t.into())
}

pub fn ns_xml_attr(attr: &str) -> ExpandedName {
    (NS_XML_URI, attr).into()
}

pub fn read_file(filepath: &str) -> Result<String, io::Error> {
    fs::read_to_string(filepath)
}