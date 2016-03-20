use std::collections::BTreeMap;

use serde_json::value::Value;

use priority::TaskPriority;

pub type DateTime= String; // FIXME
pub type Project = String;
pub type Status  = String;
pub type Tag     = String;
pub type UUID    = String; // FIXME
pub type Urgency = f64;

#[derive(Debug, Clone)]
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

    pub fn new(id: u64, desc: String, entry: DateTime, modified: DateTime,
        priority: TaskPriority, project: Project, status: Status,
        tags: Vec<Tag>, uuid: UUID, urgency: Urgency)
        -> Task
    {
        Task {
            id       : id,
            desc     : desc,
            entry    : entry,
            modified : modified,
            priority : priority,
            project  : project,
            status   : status,
            tags     : tags,
            uuid     : uuid,
            urgency  : urgency,
        }
    }

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

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn desc(&self) -> &String {
        &self.desc
    }

    pub fn entry(&self) -> &DateTime {
        &self.entry
    }

    pub fn modified(&self) -> &DateTime {
        &self.modified
    }

    pub fn priority(&self) -> TaskPriority {
        self.priority.clone()
    }

    pub fn project(&self) -> &Project {
        &self.project
    }

    pub fn status(&self) -> &Status {
        &self.status
    }

    pub fn tags(&self) -> &Vec<Tag> {
        &self.tags
    }

    pub fn uuid(&self) -> &UUID {
        &self.uuid
    }

    pub fn urgency(&self) -> Urgency {
        self.urgency
    }

}

impl Into<Value> for Task {

    fn into(self) -> Value {
        let id       = Value::U64(self.id);
        let desc     = Value::String(self.desc);
        let entry    = Value::String(self.entry);
        let modified = Value::String(self.modified);
        let priority = Value::String(self.priority.into());
        let project  = Value::String(self.project);
        let status   = Value::String(self.status);
        let tags     = Value::Array(self.tags
                                        .into_iter()
                                        .map(|s| Value::String(String::from(s)))
                                        .collect());
        let uuid     = Value::String(self.uuid);
        let urgency  = Value::F64(self.urgency);

        let mut map = BTreeMap::new();
        map.insert(String::from("id")       , id);
        map.insert(String::from("desc")     , desc);
        map.insert(String::from("entry")    , entry);
        map.insert(String::from("modified") , modified);
        map.insert(String::from("priority") , priority);
        map.insert(String::from("project")  , project);
        map.insert(String::from("status")   , status);
        map.insert(String::from("tags")     , tags);
        map.insert(String::from("uuid")     , uuid);
        map.insert(String::from("urgency")  , urgency);

        Value::Object(map)
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

