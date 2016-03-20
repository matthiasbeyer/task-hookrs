use std::collections::BTreeMap;

use serde_json::value::Value;

use priority::TaskPriority;

pub type DateTime= String; // FIXME
pub type Project = String;
pub type Status  = String;
pub type Tag     = String;
pub type UUID    = String; // FIXME
pub type Urgency = f64;

pub struct Task {
    id: u64,
    desc: String,
    entry: DateTime,
    modified: DateTime,
    priority: TaskPriority,
    project: Project,
    status: Status,
    tags: Vec<Tag>,
    uuid: UUID,
    urgency: Urgency,
}

/*
 * TODO: We do not fail if the JSON parsing fails. This panics. We rely on taskwarrior to be nice
 * to us. I guess this should be fixed.
 */
impl Task {

    pub fn from_value(v: Value) -> Option<Task> {
        if !v.is_object() {
            return None;
        }

        let map = v.as_object().unwrap();

        let keys = [ "id", "description", "modified", "priority", "project",
            "status", "tags", "uuid", "urgency"];

        if keys.iter().any(|x| map.contains_key(*x)) {
            return None
        }

        Some(Task {
            id       : get_id(&map),
            desc     : get_desc(&map),
            entry    : get_entry(&map),
            modified : get_modified(&map),
            priority : TaskPriority::from(get_priority(&map)),
            project  : get_project(&map),
            status   : get_status(&map),
            tags     : get_tags(&map),
            uuid     : get_uuid(&map),
            urgency  : get_urgency(&map),
        })
    }


}

fn get_id(map: &BTreeMap<String, Value>) -> u64 {
    map.get("id").unwrap().as_u64().unwrap()
}

fn get_desc(map: &BTreeMap<String, Value>) -> String {
    map.get("description").unwrap().as_string().map(String::from).unwrap()
}

fn get_entry(map: &BTreeMap<String, Value>) -> String {
    map.get("entry").unwrap().as_string().map(String::from).unwrap()
}

fn get_modified(map: &BTreeMap<String, Value>) -> String {
    map.get("modified").unwrap().as_string().map(String::from).unwrap()
}

fn get_priority(map: &BTreeMap<String, Value>) -> &str {
    map.get("priority").unwrap().as_string().unwrap()
}

fn get_project(map: &BTreeMap<String, Value>) -> String {
    map.get("project").unwrap().as_string().map(String::from).unwrap()
}

fn get_status(map: &BTreeMap<String, Value>) -> String {
    map.get("status").unwrap().as_string().map(String::from).unwrap()
}

fn get_tags(map: &BTreeMap<String, Value>) -> Vec<Tag> {
    map.get("tags")
        .unwrap()
        .as_array()
        .unwrap()
        .clone()
        .into_iter()
        .map(|item| item.as_string().map(String::from).unwrap())
        .collect()
}

fn get_uuid(map: &BTreeMap<String, Value>) -> String {
    map.get("uuid").unwrap().as_string().map(String::from).unwrap()
}

fn get_urgency(map: &BTreeMap<String, Value>) -> f64 {
    map.get("urgency").unwrap().as_f64().unwrap()
}

