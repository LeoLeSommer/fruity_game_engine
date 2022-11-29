/// Delete the childs of a parent when it's deleted
pub mod delete_cascade;

/// An internal system to update the nested level of a hierarchy component
/// It's mainly used to update the position in cascade cause the position of
/// a child must be updated after the parent
pub mod update_nested_level;
