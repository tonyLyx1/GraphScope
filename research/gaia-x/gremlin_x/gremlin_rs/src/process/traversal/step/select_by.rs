use crate::generated::gremlin::select_step::Pop;
use crate::process::traversal::traverser::{Element, Entry, TravelObject};
use crate::process::traversal::Traverser;
use crate::structure::{AsTag, Details};
use crate::{str_err, Element as GraphElement};
use pegasus::api::function::DynError;
use std::borrow::Cow;

pub enum SelectKey {
    Current,
    // TODO(bingqing):check if this should be Vec<AsTag> or AsTag
    Tagged(AsTag),
}

impl SelectKey {
    pub fn select(&self, input: &Traverser) -> Option<Entry> {
        self.select_pop(input, Pop::Last)
    }

    pub fn select_pop(&self, input: &Traverser, pop: Pop) -> Option<Entry> {
        match self {
            SelectKey::Current => Some(input.get_entry().clone()),
            SelectKey::Tagged(tag) => match pop {
                // TODO(bingqing): select_first/select_last etc.
                Pop::First => input.select(*tag).map(|e| e.clone()),
                Pop::Last => input.select(*tag).map(|e| e.clone()),
                Pop::All => unimplemented!(),
                Pop::Mixed => unimplemented!(),
            },
        }
    }

    pub fn get_tag(&self) -> Option<AsTag> {
        match self {
            SelectKey::Current => None,
            SelectKey::Tagged(t) => Some(*t),
        }
    }
}

pub enum ByModulating {
    Itself,
    Keys,
    Values,
    Id,
    Label,
    // by("name")/by(values(name))
    Property(String),
    // by(properties("name"))
    Properties(String),
    // by(valueMap("name"))
    ValueMap(Vec<String>),
    Prepared,
}

impl ByModulating {
    pub fn modulate_by(&self, selected_entry: Entry) -> Result<Entry, DynError> {
        match self {
            ByModulating::Itself => Ok(selected_entry),
            ByModulating::Keys => match selected_entry {
                Entry::Pair(k, _) => Ok(k.into()),
                Entry::Group(k, _) => Ok(k.into()),
                _ => Err(str_err(&format!(
                    "The provided object does not have accessible keys: {:?}",
                    selected_entry
                ))),
            },
            ByModulating::Values => match selected_entry {
                Entry::Pair(_, v) => Ok(v.into()),
                Entry::Group(_, v) => Ok(Entry::Collection(v)),
                _ => Err(str_err(&format!(
                    "The provided object does not have accessible values: {:?}",
                    selected_entry,
                ))),
            },
            ByModulating::Id => match selected_entry {
                Entry::Element(e) => match e {
                    Element::InGraph(e) => Ok(TravelObject::Value(e.id().into()).into()),
                    _ => Err(str_err("The provided object is not graph_element")),
                },
                _ => Err(str_err("The provided object is not element")),
            },
            ByModulating::Label => match selected_entry {
                Entry::Element(e) => match e {
                    Element::InGraph(e) => Ok(TravelObject::Value(e.label().into()).into()),
                    _ => Err(str_err("The provided object is not graph_element")),
                },
                _ => Err(str_err("The provided object is not element")),
            },
            ByModulating::Property(prop) => match selected_entry {
                Entry::Element(e) => match e {
                    Element::InGraph(e) => {
                        let prop_value = e.details().get_property(prop).unwrap().try_to_owned().unwrap();
                        Ok(TravelObject::Value(prop_value).into())
                    }
                    _ => Err(str_err("The provided object is not graph_element")),
                },
                _ => Err(str_err("The provided object is not element")),
            },
            ByModulating::Properties(prop) => match selected_entry {
                Entry::Element(e) => match e {
                    Element::InGraph(e) => {
                        let prop_value = e.details().get_property(prop).unwrap().try_to_owned().unwrap();
                        Ok(TravelObject::Properties((prop.clone(), prop_value)).into())
                    }
                    _ => Err(str_err("The provided object is not graph_element")),
                },
                _ => Err(str_err("The provided object is not element")),
            },
            ByModulating::ValueMap(props) => match selected_entry {
                Entry::Element(e) => match e {
                    Element::InGraph(e) => {
                        let mut prop_values = vec![];
                        for prop in props {
                            let prop_value =
                                e.details().get_property(&prop).unwrap().try_to_owned().unwrap();
                            prop_values.push((prop.clone(), prop_value));
                        }
                        Ok(TravelObject::ValueMap(prop_values).into())
                    }
                    _ => Err(str_err("The provided object is not graph_element")),
                },
                _ => Err(str_err("The provided object is not element")),
            },
            ByModulating::Prepared => unimplemented!(),
        }
    }
}
