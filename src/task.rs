use std::collections::BTreeMap;

use chrono::DateTime;
use chrono::UTC;
use serde_json::value::Value;
use uuid::Uuid;

use error::TaskError;
use error::TaskErrorKind;
use priority::TaskPriority;
use result::Result;

pub type Project = String;
pub type Status  = String;
pub type Tag     = String;
pub type Urgency = f64;

#[derive(Debug, Clone)]
pub struct Task {
    id: u64,
    desc: String,
    entry: DateTime<UTC>,
    modified: Option<DateTime<UTC>>,
    priority: Option<TaskPriority>,
    project: Option<Project>,
    status: Status,
    tags: Vec<Tag>,
    uuid: Uuid,
    urgency: Urgency,
}

/*
 * TODO: We do not fail if the JSON parsing fails. This panics. We rely on taskwarrior to be nice
 * to us. I guess this should be fixed.
 */
impl Task {

    pub fn new(id: u64,
                desc: String,
                entry: DateTime<UTC>,
                modified: Option<DateTime<UTC>>,
                priority: Option<TaskPriority>,
                project: Option<Project>,
                status: Status,
                tags: Vec<Tag>,
                uuid: Uuid,
                urgency: Urgency)
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

    pub fn from_value(v: Value) -> Result<Task> {
        if !v.is_object() {
            trace!("JSON Value is not an object, cannot parse into Task");
            return Err(TaskError::new(TaskErrorKind::ParserError, None));
        }
        trace!("Found object");

        let map = v.as_object().unwrap();
        trace!("map = {:?}", map);

        let entry_dt = get_entry(&map);
        trace!("entry_dt = {:?}", entry_dt);
        if entry_dt.is_err() {
            return Err(entry_dt.err().unwrap());
        }
        let entry_dt = entry_dt.unwrap();

        Ok(Task {
            id       : get_id(&map),
            desc     : get_desc(&map),
            entry    : entry_dt,
            modified : get_modified(&map),
            priority : get_priority(&map),
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

    pub fn entry(&self) -> &DateTime<UTC> {
        &self.entry
    }

    pub fn modified(&self) -> Option<&DateTime<UTC>> {
        self.modified.as_ref()
    }

    pub fn priority(&self) -> Option<&TaskPriority> {
        self.priority.as_ref()
    }

    pub fn project(&self) -> Option<&Project> {
        self.project.as_ref()
    }

    pub fn status(&self) -> &Status {
        &self.status
    }

    pub fn tags(&self) -> &Vec<Tag> {
        &self.tags
    }

    pub fn uuid(&self) -> &Uuid {
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
        let status   = Value::String(self.status);
        let tags     = Value::Array(self.tags
                                        .into_iter()
                                        .map(|s| Value::String(String::from(s)))
                                        .collect());
        let uuid     = Value::String(format!("{}", self.uuid));
        let urgency  = Value::F64(self.urgency);

        let mut map = BTreeMap::new();
        map.insert(String::from("id")       , id);
        map.insert(String::from("desc")     , desc);
        {
            let v = Value::String(format!("{:?}", self.entry));
            map.insert(String::from("entry"), v);
        }
        if self.modified.is_some() {
            let v = Value::String(format!("{:?}", self.modified.unwrap()));
            map.insert(String::from("modified"), v);
        }
        if self.priority.is_some() {
            map.insert(String::from("priority") , Value::String(self.priority.unwrap().into()));
        }
        if self.project.is_some() {
            map.insert(String::from("project"), Value::String(self.project.unwrap()));
        }
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

fn get_entry(map: &BTreeMap<String, Value>) -> Result<DateTime<UTC>> {
    if let Some(s) = map.get("entry").unwrap().as_string() {
        trace!("Found 'entry': {:?}", s);
        String::from(s)
            .parse::<DateTime<UTC>>()
            .map_err(|e| TaskError::new(TaskErrorKind::ParserError, Some(Box::new(e))))
    } else {
        trace!("No 'entry'");
        Err(TaskError::new(TaskErrorKind::ParserError, None))
    }
}

fn get_modified(map: &BTreeMap<String, Value>) -> Option<DateTime<UTC>> {
    map.get("modified")
        .and_then(|m| m.as_string())
        .and_then(|s| String::from(s).parse::<DateTime<UTC>>().ok())
}

fn get_priority(map: &BTreeMap<String, Value>) -> Option<TaskPriority> {
    map.get("priority")
        .map(|m| m.as_string().map(String::from).unwrap())
        .map(|s| TaskPriority::from(&s[..]))
}

fn get_project(map: &BTreeMap<String, Value>) -> Option<String> {
    map.get("project")
        .map(|p| p.as_string().map(String::from).unwrap())
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

fn get_uuid(map: &BTreeMap<String, Value>) -> Uuid {
    map.get("uuid")
        .unwrap()
        .as_string()
        .and_then(|s| Uuid::parse_str(s).ok())
        .unwrap()
}

fn get_urgency(map: &BTreeMap<String, Value>) -> f64 {
    map.get("urgency").unwrap().as_f64().unwrap()
}

#[cfg(test)]
mod test {
    extern crate env_logger;
    extern crate uuid;

    use chrono::DateTime;
    use chrono::UTC;
    use uuid::Uuid;

    use core::reader::Reader;
    use core::reader::JsonObjectReader;
    use super::Task;
    use priority::TaskPriority;
    use std::borrow::Borrow;

    #[test]
    fn test_from_json() {
        env_logger::init().unwrap();

        let json = String::from("{\"id\":1,\"description\":\"desc\",\"entry\":\"20150612T164806Z\",\"modified\":\"20160315T215656Z\",\"priority\":\"L\",\"project\":\"someproj\",\"status\":\"pending\",\"tags\":[\"test\",\"task\"],\"uuid\":\"93cfc5fa-2f0c-44e6-bede-c2b1ca7ceff3\",\"urgency\":1.0}");
        let bytes = json.into_bytes();
        let mut reader = JsonObjectReader::new(Reader::new(bytes.borrow()));

        let v = reader.next();
        assert!(v.is_some());
        let v = v.unwrap();

        debug!("{:?}", v);
        let t = Task::from_value(v);
        debug!("{:?}", t);
        assert!(t.is_ok());
        let t = t.unwrap();

        debug!("Task created successfully!");

        assert_eq!(t.id(), 1);
        assert_eq!(t.desc().clone(), String::from("desc"));
        assert_eq!(t.entry().clone(), String::from("20150612T164806Z").parse::<DateTime<UTC>>().unwrap());
        assert_eq!(t.modified().clone(), String::from("20160315T215656Z").parse::<DateTime<UTC>>().ok().as_ref());
        assert_eq!(t.priority(), Some(&TaskPriority::Low));
        assert_eq!(t.project().clone(), Some(&String::from("someproj")));
        assert_eq!(t.status().clone(), String::from("pending"));
        for n in ["test", "task"].iter() {
            assert!(t.tags().contains(&String::from(*n)));
        }
        assert_eq!(t.uuid().clone(), Uuid::parse_str("93cfc5fa-2f0c-44e6-bede-c2b1ca7ceff3").unwrap());
        assert_eq!(t.urgency() as u64, 1.0 as u64);
    }

}
