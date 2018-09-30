//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

//! Module containing `Task` type as well as trait implementations

use std::result::Result as RResult;
use std::fmt;

use serde::{Serialize, Serializer};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer};
use serde::de::{Visitor, Error, MapAccess};
use uuid::Uuid;
use chrono::Utc;

use priority::TaskPriority;
use status::TaskStatus;
use project::Project;
use tag::Tag;
use date::Date;
use annotation::Annotation;
use uda::{UDA, UDAName, UDAValue};

/// Task type
///
/// A task must have four things:
///
/// - A Status
/// - An UUID
/// - An Entry-Date
/// - A Description
///
/// all other Data is optional by taskwarrior. This type is a simple rust representation of the
/// JSON exported by taskwarrior.
///
/// For further explanations of the fields please consult the documentation on https://taskwarrior.org/
///
/// It is deserializeable and serializeable via serde_json, so importing and exporting taskwarrior
/// tasks is simply serializing and deserializing objects of this type.
#[derive(Debug, Clone, PartialEq, Builder)]
#[builder(setter(into))]
pub struct Task {
    /// The temporary assigned task id
    #[builder(default)]
    id: Option<u64>,

    /// The status of the task
    #[builder(default = "TaskStatus::Pending")]
    status: TaskStatus,
    /// The uuid which identifies the task and is important for syncing
    #[builder(default = "Uuid::new_v4()")]
    uuid: Uuid,
    /// The entry date, when this task was created
    #[builder(default = "Date::from(Utc::now().naive_utc())")]
    entry: Date,
    /// The description of the task (i.e. its main content)
    /// This field is the only mandatory field, when using the TaskBuilder.
    description: String,
    /// A list of annotations with timestamps
    #[builder(default)]
    annotations: Option<Vec<Annotation>>,
    /// The uuids of other tasks which have to be completed before this one becomes unblocked.
    #[builder(default)]
    depends: Option<Vec<Uuid>>,
    /// The due date of the task
    #[builder(default)]
    due: Option<Date>,
    /// When the task was last deleted or completed
    #[builder(default)]
    end: Option<Date>,
    /// The imask is used internally for recurrence
    #[builder(default)]
    imask: Option<i64>,
    /// The mask is used internally for recurrence
    #[builder(default)]
    mask: Option<String>,
    /// When the task was last modified
    #[builder(default)]
    modified: Option<Date>,
    /// A task can have a parent task
    #[builder(default)]
    parent: Option<Uuid>,
    /// The priority of the task
    #[builder(default)]
    priority: Option<TaskPriority>,
    /// A task can be part of a project. Typically of the form "project.subproject.subsubproject"
    #[builder(default)]
    project: Option<Project>,
    /// The timespan after which this task should recur
    #[builder(default)]
    recur: Option<String>,
    /// When the task becomes ready
    #[builder(default)]
    scheduled: Option<Date>,
    /// When the task becomes active
    #[builder(default)]
    start: Option<Date>,
    /// The tags associated with the task
    #[builder(default)]
    tags: Option<Vec<Tag>>,
    /// When the recurrence stops
    #[builder(default)]
    until: Option<Date>,
    /// This hides the task until the wait date
    #[builder(default)]
    wait: Option<Date>,

    /// A map of user defined attributes
    #[builder(default)]
    uda: UDA,
}

/*
 * TODO: We do not fail if the JSON parsing fails. This panics. We rely on taskwarrior to be nice
 * to us. I guess this should be fixed.
 */
impl Task {
    /// Create a new Task instance
    pub fn new(
        id: Option<u64>,

        status: TaskStatus,
        uuid: Uuid,
        entry: Date,
        description: String,

        annotations: Option<Vec<Annotation>>,
        depends: Option<Vec<Uuid>>,
        due: Option<Date>,
        end: Option<Date>,
        imask: Option<i64>,
        mask: Option<String>,
        modified: Option<Date>,
        parent: Option<Uuid>,
        priority: Option<TaskPriority>,
        project: Option<Project>,
        recur: Option<String>,
        scheduled: Option<Date>,
        start: Option<Date>,
        tags: Option<Vec<Tag>>,
        until: Option<Date>,
        wait: Option<Date>,
        uda: UDA,
    ) -> Task {
        Task {
            id: id,
            status: status,
            uuid: uuid,
            entry: entry,
            description: description,

            annotations: annotations,
            depends: depends,
            due: due,
            end: end,
            imask: imask,
            mask: mask,
            modified: modified,
            parent: parent,
            priority: priority,
            project: project,
            recur: recur,
            scheduled: scheduled,
            start: start,
            tags: tags,
            until: until,
            wait: wait,
            uda: uda,
        }
    }

    /// Get the id of the task
    pub fn id(&self) -> Option<u64> {
        self.id
    }

    /// Get the status of the task
    pub fn status(&self) -> &TaskStatus {
        &self.status
    }

    /// Get the status of the task mutable
    pub fn status_mut(&mut self) -> &mut TaskStatus {
        &mut self.status
    }

    /// Get the uuid of the task
    pub fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    /// Get the uuid of the task mutable
    pub fn uuid_mut(&mut self) -> &mut Uuid {
        &mut self.uuid
    }

    /// Get the entry date of the task
    pub fn entry(&self) -> &Date {
        &self.entry
    }

    /// Get the entry date of the task mutable
    pub fn entry_mut(&mut self) -> &mut Date {
        &mut self.entry
    }

    /// Get the description of the task
    pub fn description(&self) -> &String {
        &self.description
    }

    /// Get the description of the task mutable
    pub fn description_mut(&mut self) -> &mut String {
        &mut self.description
    }

    /// Get the annotations of the task
    pub fn annotations(&self) -> Option<&Vec<Annotation>> {
        self.annotations.as_ref()
    }

    /// Get the annotations of the task mutable
    pub fn annotations_mut(&mut self) -> Option<&mut Vec<Annotation>> {
        self.annotations.as_mut()
    }

    /// Set annotations
    pub fn set_annotations<T, A>(&mut self, new: Option<T>)
    where
        T: IntoIterator,
        T::Item: Into<Annotation>,
    {
        self.annotations = new.map(|x| x.into_iter().map(Into::into).collect());
    }

    /// Get the dependencies of the task
    pub fn depends(&self) -> Option<&Vec<Uuid>> {
        self.depends.as_ref()
    }

    /// Get the dependencies of the task mutable
    pub fn depends_mut(&mut self) -> Option<&mut Vec<Uuid>> {
        self.depends.as_mut()
    }

    /// Set depends
    pub fn set_depends<T, U>(&mut self, new: Option<T>)
    where
        T: IntoIterator,
        T::Item: Into<Uuid>,
    {
        self.depends = new.map(|x| x.into_iter().map(Into::into).collect());
    }

    /// Get the due date of the task
    pub fn due(&self) -> Option<&Date> {
        self.due.as_ref()
    }

    /// Get the due date of the task mutable
    pub fn due_mut(&mut self) -> Option<&mut Date> {
        self.due.as_mut()
    }

    /// Set due
    pub fn set_due<T>(&mut self, new: Option<T>)
    where
        T: Into<Date>,
    {
        self.due = new.map(Into::into)
    }

    /// Get the end date of the task
    pub fn end(&self) -> Option<&Date> {
        self.end.as_ref()
    }

    /// Get the end date of the task mutable
    pub fn end_mut(&mut self) -> Option<&mut Date> {
        self.end.as_mut()
    }

    /// Set end
    pub fn set_end<T>(&mut self, new: Option<T>)
    where
        T: Into<Date>,
    {
        self.end = new.map(Into::into)
    }

    /// Get the imask of the task
    pub fn imask(&self) -> Option<&i64> {
        self.imask.as_ref()
    }

    /// Get the imask of the task mutable
    pub fn imask_mut(&mut self) -> Option<&mut i64> {
        self.imask.as_mut()
    }

    /// Set imask
    pub fn set_imask<T>(&mut self, new: Option<T>)
    where
        T: Into<i64>,
    {
        self.imask = new.map(Into::into)
    }

    /// Get the mask of the task
    pub fn mask(&self) -> Option<&String> {
        self.mask.as_ref()
    }

    /// Get the mask of the task mutable
    pub fn mask_mut(&mut self) -> Option<&mut String> {
        self.mask.as_mut()
    }

    /// Set mask
    pub fn set_mask<T>(&mut self, new: Option<T>)
    where
        T: Into<String>,
    {
        self.mask = new.map(Into::into)
    }

    /// Get the modified date of the task
    pub fn modified(&self) -> Option<&Date> {
        self.modified.as_ref()
    }

    /// Get the modified date of the task mutable
    pub fn modified_mut(&mut self) -> Option<&mut Date> {
        self.modified.as_mut()
    }

    /// Set modified
    pub fn set_modified<T>(&mut self, new: Option<T>)
    where
        T: Into<Date>,
    {
        self.modified = new.map(Into::into)
    }

    /// Get the parent of the task
    pub fn parent(&self) -> Option<&Uuid> {
        self.parent.as_ref()
    }

    /// Get the parent of the task mutable
    pub fn parent_mut(&mut self) -> Option<&mut Uuid> {
        self.parent.as_mut()
    }

    /// Set parent
    pub fn set_parent<T>(&mut self, new: Option<T>)
    where
        T: Into<Uuid>,
    {
        self.parent = new.map(Into::into)
    }

    /// Get the priority of the task
    pub fn priority(&self) -> Option<&TaskPriority> {
        self.priority.as_ref()
    }

    /// Get the priority of the task mutable
    pub fn priority_mut(&mut self) -> Option<&mut TaskPriority> {
        self.priority.as_mut()
    }

    /// Set priority
    pub fn set_priority<T>(&mut self, new: Option<T>)
    where
        T: Into<TaskPriority>,
    {
        self.priority = new.map(Into::into)
    }

    /// Get the project of the task
    pub fn project(&self) -> Option<&Project> {
        self.project.as_ref()
    }

    /// Get the project of the task mutable
    pub fn project_mut(&mut self) -> Option<&mut Project> {
        self.project.as_mut()
    }

    /// Set project
    pub fn set_project<T>(&mut self, new: Option<T>)
    where
        T: Into<Project>,
    {
        self.project = new.map(Into::into)
    }

    /// Get the recur of the task
    ///
    /// This is exported as String by now. This might change in future versions of this crate.
    pub fn recur(&self) -> Option<&String> {
        self.recur.as_ref()
    }

    /// This is exported as String by now. This might change in future versions of this crate.
    /// mutable
    pub fn recur_mut(&mut self) -> Option<&mut String> {
        self.recur.as_mut()
    }

    /// Set recur
    pub fn set_recur<T>(&mut self, new: Option<T>)
    where
        T: Into<String>,
    {
        self.recur = new.map(Into::into)
    }

    /// Get the scheduled date of the task
    pub fn scheduled(&self) -> Option<&Date> {
        self.scheduled.as_ref()
    }

    /// Get the scheduled date of the task mutable
    pub fn scheduled_mut(&mut self) -> Option<&mut Date> {
        self.scheduled.as_mut()
    }

    /// Set scheduled
    pub fn set_scheduled<T>(&mut self, new: Option<T>)
    where
        T: Into<Date>,
    {
        self.scheduled = new.map(Into::into)
    }

    /// Get the start date of the task
    pub fn start(&self) -> Option<&Date> {
        self.start.as_ref()
    }

    /// Get the start date of the task mutable
    pub fn start_mut(&mut self) -> Option<&mut Date> {
        self.start.as_mut()
    }

    /// Set start
    pub fn set_start<T>(&mut self, new: Option<T>)
    where
        T: Into<Date>,
    {
        self.start = new.map(Into::into)
    }

    /// Get the tags of the task
    pub fn tags(&self) -> Option<&Vec<Tag>> {
        self.tags.as_ref()
    }

    /// Get the tags of the task mutable
    pub fn tags_mut(&mut self) -> Option<&mut Vec<Tag>> {
        self.tags.as_mut()
    }

    /// Set tags
    pub fn set_tags<T, K>(&mut self, new: Option<T>)
    where
        T: IntoIterator,
        T::Item: Into<Tag>,
    {
        self.tags = new.map(|x| x.into_iter().map(Into::into).collect());
    }

    /// Get the until date of the task
    pub fn until(&self) -> Option<&Date> {
        self.until.as_ref()
    }

    /// Get the until date of the task mutable
    pub fn until_mut(&mut self) -> Option<&mut Date> {
        self.until.as_mut()
    }

    /// Set until
    pub fn set_until<T>(&mut self, new: Option<T>)
    where
        T: Into<Date>,
    {
        self.until = new.map(Into::into);
    }

    /// Get the wait date of the task
    pub fn wait(&self) -> Option<&Date> {
        self.wait.as_ref()
    }

    /// Get the wait date of the task mutable
    pub fn wait_mut(&mut self) -> Option<&mut Date> {
        self.wait.as_mut()
    }

    /// Set wait
    pub fn set_wait<T>(&mut self, new: Option<T>)
    where
        T: Into<Date>,
    {
        self.wait = new.map(Into::into);
    }

    /// Get the BTreeMap that contains the UDA
    pub fn uda(&self) -> &UDA {
        &self.uda
    }
    /// Get the BTreeMap that contains the UDA mutable
    pub fn uda_mut(&mut self) -> &mut UDA {
        &mut self.uda
    }
}

impl Serialize for Task {
    fn serialize<S>(&self, serializer: S) -> RResult<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("status", &self.status)?;
        state.serialize_entry("uuid", &self.uuid)?;
        state.serialize_entry("entry", &self.entry)?;
        state.serialize_entry("description", &self.description)?;

        self.annotations.as_ref().map(|v| {
            state.serialize_entry("annotations", v)
        });
        self.tags.as_ref().map(|v| state.serialize_entry("tags", v));
        self.id.as_ref().map(|v| state.serialize_entry("id", v));
        self.recur.as_ref().map(
            |ref v| state.serialize_entry("recur", v),
        );
        self.depends.as_ref().map(|ref v| {
            let v: Vec<String> = v.iter().map(Uuid::to_string).collect();
            state.serialize_entry("depends", &v.join(","))
        });
        self.due.as_ref().map(
            |ref v| state.serialize_entry("due", v),
        );
        self.end.as_ref().map(
            |ref v| state.serialize_entry("end", v),
        );
        self.imask.as_ref().map(
            |ref v| state.serialize_entry("imask", v),
        );
        self.mask.as_ref().map(
            |ref v| state.serialize_entry("mask", v),
        );
        self.modified.as_ref().map(|ref v| {
            state.serialize_entry("modified", v)
        });
        self.parent.as_ref().map(|ref v| {
            state.serialize_entry("parent", v)
        });
        self.priority.as_ref().map(|ref v| {
            state.serialize_entry("priority", v)
        });
        self.project.as_ref().map(|ref v| {
            state.serialize_entry("project", v)
        });
        self.scheduled.as_ref().map(|ref v| {
            state.serialize_entry("scheduled", v)
        });
        self.start.as_ref().map(
            |ref v| state.serialize_entry("start", v),
        );
        self.until.as_ref().map(
            |ref v| state.serialize_entry("until", v),
        );
        self.wait.as_ref().map(
            |ref v| state.serialize_entry("wait", v),
        );

        for (key, value) in self.uda().iter() {
            state.serialize_entry(key, value)?;
        }

        state.end()
    }
}

impl<'de> Deserialize<'de> for Task {
    fn deserialize<D>(deserializer: D) -> RResult<Task, D::Error>
    where
        D: Deserializer<'de>,
    {
        static FIELDS: &'static [&'static str] = &[
            "id",
            "status",
            "uuid",
            "entry",
            "description",

            "annotations",
            "depends",
            "due",
            "end",
            "imask",
            "mask",
            "modified",
            "parent",
            "priority",
            "project",
            "recur",
            "scheduled",
            "start",
            "tags",
            "until",
            "wait",
            "uda",
        ];
        deserializer.deserialize_struct("Task", FIELDS, TaskDeserializeVisitor)
    }
}

/// Helper type for task deserialization
struct TaskDeserializeVisitor;

impl<'de> Visitor<'de> for TaskDeserializeVisitor {
    type Value = Task;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "A dictionary containing task properties")
    }

    fn visit_map<V>(self, mut visitor: V) -> RResult<Task, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut id = None;

        let mut status = None;
        let mut uuid = None;
        let mut entry = None;
        let mut description = None;

        let mut annotations = None;
        let mut depends = None;
        let mut due = None;
        let mut end = None;
        let mut imask = None;
        let mut mask = None;
        let mut modified = None;
        let mut parent = None;
        let mut priority = None;
        let mut project = None;
        let mut recur = None;
        let mut scheduled = None;
        let mut start = None;
        let mut tags = None;
        let mut until = None;
        let mut wait = None;
        let mut uda = UDA::default();

        loop {
            let key: Option<String> = visitor.next_key()?;
            if key.is_none() {
                break;
            }
            let key = key.unwrap();

            match &key[..] {
                "id" => {
                    id = Some(visitor.next_value()?);
                }

                "status" => {
                    status = Some(visitor.next_value()?);
                }
                "uuid" => {
                    uuid = Some(visitor.next_value()?);
                }
                "entry" => {
                    entry = Some(visitor.next_value()?);
                }
                "description" => {
                    description = Some(visitor.next_value()?);
                }

                "annotations" => {
                    annotations = Some(visitor.next_value()?);
                }
                "depends" => {
                    let raw: String = visitor.next_value()?;
                    let mut uuids = vec![];
                    for uuid in raw.split(",") {
                        uuids.push(Uuid::parse_str(uuid).map_err(V::Error::custom)?);
                    }
                    depends = Some(uuids);
                }
                "due" => {
                    due = Some(visitor.next_value()?);
                }
                "end" => {
                    end = Some(visitor.next_value()?);
                }
                "imask" => {
                    imask = Some(visitor.next_value()?);
                }
                "mask" => {
                    mask = Some(visitor.next_value()?);
                }
                "modified" => {
                    modified = Some(visitor.next_value()?);
                }
                "parent" => {
                    parent = Some(visitor.next_value()?);
                }
                "priority" => {
                    priority = Some(visitor.next_value()?);
                }
                "project" => {
                    project = Some(visitor.next_value()?);
                }
                "recur" => {
                    recur = Some(visitor.next_value()?);
                }
                "scheduled" => {
                    scheduled = Some(visitor.next_value()?);
                }
                "start" => {
                    start = Some(visitor.next_value()?);
                }
                "tags" => {
                    tags = Some(visitor.next_value()?);
                }
                "until" => {
                    until = Some(visitor.next_value()?);
                }
                "wait" => {
                    wait = Some(visitor.next_value()?);
                }

                field => {
                    debug!("Inserting '{}' as UDA", field);
                    let uda_value: UDAValue = visitor.next_value()?;
                    uda.insert(UDAName::from(field), uda_value);
                }
            }
        }

        let status = match status {
            Some(status) => status,
            None => Err(V::Error::missing_field("status"))?,
        };

        let uuid = match uuid {
            Some(uuid) => uuid,
            None => Err(V::Error::missing_field("uuid"))?,
        };

        let entry = match entry {
            Some(entry) => entry,
            None => Err(V::Error::missing_field("entry"))?,
        };

        let description = match description {
            Some(description) => description,
            None => Err(V::Error::missing_field("description"))?,
        };

        let task = Task::new(
            id,
            status,
            uuid,
            entry,
            description,

            annotations,
            depends,
            due,
            end,
            imask,
            mask,
            modified,
            parent,
            priority,
            project,
            recur,
            scheduled,
            start,
            tags,
            until,
            wait,
            uda,
        );

        Ok(task)
    }
}

#[cfg(test)]
mod test {
    use date::Date;
    use date::TASKWARRIOR_DATETIME_TEMPLATE;
    use status::TaskStatus;
    use task::Task;
    use annotation::Annotation;
    use uda::UDAValue;

    use uuid::Uuid;
    use chrono::NaiveDateTime;
    use serde_json;

    fn mklogger() {
        use env_logger;
        let _ = env_logger::init();
        debug!("Env-logger enabled");
    }

    fn mkdate(s: &str) -> Date {
        let n = NaiveDateTime::parse_from_str(s, TASKWARRIOR_DATETIME_TEMPLATE);
        Date::from(n.unwrap())
    }

    #[test]
    fn test_deser() {
        let s = r#"{
"id": 1,
"description": "test",
"entry": "20150619T165438Z",
"status": "waiting",
"uuid": "8ca953d5-18b4-4eb9-bd56-18f2e5b752f0"
}"#;

        println!("{}", s);

        let task = serde_json::from_str(s);
        println!("{:?}", task);
        assert!(task.is_ok());
        let task: Task = task.unwrap();

        assert!(task.status().clone() == TaskStatus::Waiting);
        assert!(task.description() == "test");
        assert!(task.entry().clone() == mkdate("20150619T165438Z"));
        assert!(
            task.uuid().clone() == Uuid::parse_str("8ca953d5-18b4-4eb9-bd56-18f2e5b752f0").unwrap()
        );

        let back = serde_json::to_string(&task).unwrap();

        assert!(back.contains("description"));
        assert!(back.contains("test"));
        assert!(back.contains("entry"));
        assert!(back.contains("20150619T165438Z"));
        assert!(back.contains("status"));
        assert!(back.contains("waiting"));
        assert!(back.contains("uuid"));
        assert!(back.contains("8ca953d5-18b4-4eb9-bd56-18f2e5b752f0"));
    }

    #[test]
    fn test_deser_more() {
        mklogger();
        let s = r#"{
"id": 1,
"description": "some description",
"entry": "20150619T165438Z",
"modified": "20160327T164007Z",
"project": "someproject",
"status": "waiting",
"tags": ["some", "tags", "are", "here"],
"uuid": "8ca953d5-18b4-4eb9-bd56-18f2e5b752f0",
"depends": "8ca953d5-18b4-4eb9-bd56-18f2e5b752f0,5a04bb1e-3f4b-49fb-b9ba-44407ca223b5",
"wait": "20160508T164007Z",
"urgency": 0.583562
}"#;

        println!("{}", s);

        let task = serde_json::from_str(s);
        println!("{:?}", task);
        assert!(task.is_ok());
        let task: Task = task.unwrap();

        assert!(task.status().clone() == TaskStatus::Waiting);
        assert!(task.description() == "some description");
        assert!(task.entry().clone() == mkdate("20150619T165438Z"));
        assert!(
            task.uuid().clone() == Uuid::parse_str("8ca953d5-18b4-4eb9-bd56-18f2e5b752f0").unwrap()
        );

        assert!(task.modified() == Some(&mkdate("20160327T164007Z")));
        assert!(task.project() == Some(&String::from("someproject")));

        if let Some(tags) = task.tags() {
            for tag in tags {
                let any_tag = ["some", "tags", "are", "here"].into_iter().any(
                    |t| tag == *t,
                );
                assert!(any_tag, "Tag {} missing", tag);
            }
        } else {
            assert!(false, "Tags completely missing");
        }

        assert!(task.wait() == Some(&mkdate("20160508T164007Z")));

        let back = serde_json::to_string(&task).unwrap();

        assert!(back.contains("description"));
        assert!(back.contains("some description"));
        assert!(back.contains("entry"));
        assert!(back.contains("20150619T165438Z"));
        assert!(back.contains("project"));
        assert!(back.contains("someproject"));
        assert!(back.contains("status"));
        assert!(back.contains("waiting"));
        assert!(back.contains("tags"));
        assert!(back.contains("some"));
        assert!(back.contains("tags"));
        assert!(back.contains("are"));
        assert!(back.contains("here"));
        assert!(back.contains("uuid"));
        assert!(back.contains("8ca953d5-18b4-4eb9-bd56-18f2e5b752f0"));
        assert!(back.contains(
            "8ca953d5-18b4-4eb9-bd56-18f2e5b752f0,5a04bb1e-3f4b-49fb-b9ba-44407ca223b5",
        ));
    }

    #[test]
    fn test_deser_annotation() {
        let s = r#"{
"id":192,
"description":"Some long description for a task",
"entry":"20160423T125820Z",
"modified":"20160423T125942Z",
"project":"project",
"status":"pending",
"tags":["search","things"],
"uuid":"5a04bb1e-3f4b-49fb-b9ba-44407ca223b5",
"annotations":[{"entry":"20160423T125911Z","description":"An Annotation"},
               {"entry":"20160423T125926Z","description":"Another Annotation"},
               {"entry":"20160422T125942Z","description":"A Third Anno"}
               ],
"urgency": 0.583562
}"#;

        println!("{}", s);

        let task = serde_json::from_str(s);
        println!("{:?}", task);
        assert!(task.is_ok());
        let task: Task = task.unwrap();

        let all_annotations =
            vec![
                Annotation::new(mkdate("20160423T125911Z"), String::from("An Annotation")),
                Annotation::new(
                    mkdate("20160423T125926Z"),
                    String::from("Another Annotation")
                ),
                Annotation::new(mkdate("20160422T125942Z"), String::from("A Third Anno")),
            ];

        if let Some(annotations) = task.annotations() {
            for annotation in annotations {
                let r = all_annotations.iter().any(|ann| {
                    let descr = ann.description() == annotation.description();
                    let entry = ann.entry() == annotation.entry();

                    descr && entry
                });
                assert!(r, "Annotation {:?} missing or buggy", annotation);
            }
        } else {
            assert!(false, "Annotations missing");
        }
    }
    #[test]
    fn test_uda() {
        let s = r#"{
"description":"Some long description for a task",
"entry":"20160423T125820Z",
"modified":"20160423T125942Z",
"project":"project",
"status":"pending",
"uuid":"5a04bb1e-3f4b-49fb-b9ba-44407ca223b5",
"test_str_uda":"test_str_uda_value",
"test_float_uda":-17.1234,
"test_int_uda":1234
}"#;

        println!("{}", s);

        let task = serde_json::from_str(s);
        println!("{:?}", task);
        assert!(task.is_ok());
        let task: Task = task.unwrap();

        let str_uda = task.uda().get(&"test_str_uda".to_owned());
        assert!(str_uda.is_some());
        let str_uda = str_uda.unwrap();
        assert_eq!(str_uda, &UDAValue::Str("test_str_uda_value".to_owned()));

        let float_uda = task.uda().get(&"test_float_uda".to_owned());
        assert!(float_uda.is_some());
        let float_uda = float_uda.unwrap();
        assert_eq!(float_uda, &UDAValue::F64(-17.1234));

        let int_uda = task.uda().get(&"test_int_uda".to_owned());
        assert!(int_uda.is_some());
        let int_uda = int_uda.unwrap();
        assert_eq!(int_uda, &UDAValue::U64(1234));

        let back = serde_json::to_string_pretty(&task);
        assert!(back.is_ok());
        let back = back.unwrap();
        println!("{}", back);
        assert!(back.contains("description"));
        assert!(back.contains("Some long description for a task"));
        assert!(back.contains("entry"));
        assert!(back.contains("20160423T125820Z"));
        assert!(back.contains("project"));
        assert!(back.contains("status"));
        assert!(back.contains("pending"));
        assert!(back.contains("uuid"));
        assert!(back.contains("5a04bb1e-3f4b-49fb-b9ba-44407ca223b5"));
        assert!(back.contains("test_str_uda"));
        assert!(back.contains("test_str_uda_value"));
        assert!(back.contains("test_float_uda"));
        assert!(back.contains("-17.1234"));
        assert!(back.contains("test_int_uda"));
        assert!(back.contains("1234"));
    }

    #[test]
    fn test_builder_simple() {
        use task::TaskBuilder;

        let t = TaskBuilder::default()
            .description("test")
            .entry(mkdate("20150619T165438Z"))
            .build();
        println!("{:?}", t);
        assert!(t.is_ok());
        let t = t.unwrap();

        assert_eq!(t.status(), &TaskStatus::Pending);
        assert_eq!(t.description(), "test");
        assert_eq!(t.entry(), &mkdate("20150619T165438Z"));
    }
    #[test]
    fn test_builder_extensive() {
        use task::TaskBuilder;
        use uda::{UDA, UDAValue};
        let mut uda = UDA::new();
        uda.insert(
            "test_str_uda".into(),
            UDAValue::Str("test_str_uda_value".into()),
        );
        uda.insert("test_int_uda".into(), UDAValue::U64(1234));
        uda.insert("test_float_uda".into(), UDAValue::F64(-17.1234));
        let t = TaskBuilder::default()
            .description("test")
            .entry(mkdate("20150619T165438Z"))
            .id(192)
            .modified(mkdate("20160423T125942Z"))
            .project("project".to_owned())
            .tags(vec!["search".to_owned(), "things".to_owned()])
            .uda(uda)
            .build();
        println!("{:?}", t);
        assert!(t.is_ok());
        let t = t.unwrap();

        assert!(t.id().is_some());
        assert_eq!(t.id().unwrap(), 192);
        assert_eq!(t.status(), &TaskStatus::Pending);
        assert_eq!(t.description(), "test");
        assert_eq!(t.entry(), &mkdate("20150619T165438Z"));
        assert!(t.modified().is_some());
        assert_eq!(t.modified().unwrap(), &mkdate("20160423T125942Z"));
        assert!(t.project().is_some());
        assert_eq!(t.project().unwrap(), "project");
        assert!(t.tags().is_some());
        assert_eq!(
            t.tags().unwrap(),
            &vec!["search".to_owned(), "things".to_owned()]
        );
    }
    #[test]
    fn test_builder_defaults() {
        use task::TaskBuilder;
        assert!(
            TaskBuilder::default()
                .description("Nice Task")
                .build()
                .is_ok()
        );
    }

    #[test]
    fn test_builder_fail() {
        use task::TaskBuilder;
        assert!(TaskBuilder::default().build().is_err());
    }

}
