use crate::process::traversal::traverser::{ShadeSync, Traverser};
use dyn_type::Object;

// TODO: more result type downcast

/// downcast result of group, i.e., a pair(Traverser, Traverser), which is the generated by
/// 1. groupCount, where the pair is (Traverser, Traverser::Unknown(Object::DynOwned(ShadeSync<u64>)))
/// 2. group().by(), where the pair is (Traverser, Traverser::Unknown(Object::DynOwned(ShadeSync<Vec<Traverser>>)))
/// 3. group.by().by(sub_traversal) (via GroupBySubJoin)
pub fn try_downcast_pair(obj: &Object) -> Option<&(Traverser, Traverser)> {
    if let Object::DynOwned(object) = obj {
        let shade_sync_pair = object.try_downcast_ref::<ShadeSync<(Traverser, Traverser)>>();
        if let Some(shade_sync_pair) = shade_sync_pair {
            Some(shade_sync_pair.get())
        } else {
            None
        }
    } else {
        None
    }
}

/// downcast list value, i.e., Traverser::Unknown(Object::DynOwned(ShadeSync<Vec<Traverser>>))
pub fn try_downcast_list(obj: &Object) -> Option<Vec<Traverser>> {
    if let Object::DynOwned(object) = obj {
        if let Some(list) = object.try_downcast_ref::<ShadeSync<Vec<Traverser>>>() {
            Some(list.clone().inner)
        } else {
            None
        }
    } else {
        None
    }
}

/// downcast result of group().by() and get key where key is a Traverser
pub fn try_downcast_group_key(obj: &Object) -> Option<&Traverser> {
    if let Some(pair) = try_downcast_pair(obj) {
        let first = &pair.0;
        Some(first)
    } else {
        None
    }
}

/// downcast result of group().by() and get value where value is a Traverser
pub fn try_downcast_group_value(obj: &Object) -> Option<&Traverser> {
    if let Some(pair) = try_downcast_pair(obj) {
        let second = &pair.1;
        Some(second)
    } else {
        None
    }
}

/// downcast result of groupCount() and get value where value is u64 (i.e., AccumKind is CNT)
pub fn try_downcast_group_count_value(obj: &Object) -> Option<u64> {
    if let Some(pair) = try_downcast_pair(obj) {
        if let Some(object) = pair.1.get_object() {
            object.as_u64().ok()
        } else {
            None
        }
    } else {
        None
    }
}
