use std::collections::BTreeMap;
use std::result::Result as RResult;

use chrono::naive::datetime::NaiveDateTime;
use serde_json::value::Value;
use serde::Serialize;
use serde::Serializer;
use serde::Deserialize;
use serde::Deserializer;
use serde::de::Error as SerdeError;
use serde::ser::MapVisitor;
use uuid::Uuid;

use error::TaskError;
use error::TaskErrorKind;
use priority::TaskPriority;
use result::Result;
use status::TaskStatus;
use project::Project;
use tag::Tag;

pub static TASKWARRIOR_DATETIME_TEMPLATE : &'static str = "%Y%m%dT%H%M%SZ";

#[derive(Debug, Clone)]
pub struct Task {
    status      : TaskStatus,
    uuid        : Uuid,
    entry       : NaiveDateTime,
    description : String,

    annotation  : Option<Vec<String>>,
    depends     : Option<String>,
    due         : Option<NaiveDateTime>,
    end         : Option<NaiveDateTime>,
    imask       : Option<i64>,
    mask        : Option<String>,
    modified    : Option<NaiveDateTime>,
    parent      : Option<Uuid>,
    priority    : Option<TaskPriority>,
    project     : Option<Project>,
    recur       : Option<String>,
    scheduled   : Option<NaiveDateTime>,
    start       : Option<NaiveDateTime>,
    tags        : Option<Vec<Tag>>,
    until       : Option<NaiveDateTime>,
    wait        : Option<NaiveDateTime>,
}

/*
 * TODO: We do not fail if the JSON parsing fails. This panics. We rely on taskwarrior to be nice
 * to us. I guess this should be fixed.
 */
impl Task {

    pub fn new(
        status      : TaskStatus,
        uuid        : Uuid,
        entry       : NaiveDateTime,
        description : String,

        annotation  : Option<Vec<String>>,
        depends     : Option<String>,
        due         : Option<NaiveDateTime>,
        end         : Option<NaiveDateTime>,
        imask       : Option<i64>,
        mask        : Option<String>,
        modified    : Option<NaiveDateTime>,
        parent      : Option<Uuid>,
        priority    : Option<TaskPriority>,
        project     : Option<Project>,
        recur       : Option<String>,
        scheduled   : Option<NaiveDateTime>,
        start       : Option<NaiveDateTime>,
        tags        : Option<Vec<Tag>>,
        until       : Option<NaiveDateTime>,
        wait        : Option<NaiveDateTime>,
    ) -> Task
    {
        Task {
            status      : status,
            uuid        : uuid,
            entry       : entry,
            description : description,

            annotation  : annotation,
            depends     : depends,
            due         : due,
            end         : end,
            imask       : imask,
            mask        : mask,
            modified    : modified,
            parent      : parent,
            priority    : priority,
            project     : project,
            recur       : recur,
            scheduled   : scheduled,
            start       : start,
            tags        : tags,
            until       : until,
            wait        : wait,
        }
    }

    pub fn status(&self) -> &TaskStatus {
        &self.status
    }

    pub fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    pub fn entry(&self) -> &NaiveDateTime {
        &self.entry
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn annotation(&self) -> Option<&Vec<String>> {
        self.annotation.as_ref()
    }

    pub fn depends(&self) -> Option<&String> {
        self.depends.as_ref()
    }

    pub fn due(&self) -> Option<&NaiveDateTime> {
        self.due.as_ref()
    }

    pub fn end(&self) -> Option<&NaiveDateTime> {
        self.end.as_ref()
    }

    pub fn imask(&self) -> Option<&i64> {
        self.imask.as_ref()
    }

    pub fn mask(&self) -> Option<&String> {
        self.mask.as_ref()
    }

    pub fn modified(&self) -> Option<&NaiveDateTime> {
        self.modified.as_ref()
    }

    pub fn parent(&self) -> Option<&Uuid> {
        self.parent.as_ref()
    }

    pub fn priority(&self) -> Option<&TaskPriority> {
        self.priority.as_ref()
    }

    pub fn project(&self) -> Option<&Project> {
        self.project.as_ref()
    }

    pub fn recur(&self) -> Option<&String> {
        self.recur.as_ref()
    }

    pub fn scheduled(&self) -> Option<&NaiveDateTime> {
        self.scheduled.as_ref()
    }

    pub fn start(&self) -> Option<&NaiveDateTime> {
        self.start.as_ref()
    }

    pub fn tags(&self) -> Option<&Vec<Tag>> {
        self.tags.as_ref()
    }

    pub fn until(&self) -> Option<&NaiveDateTime> {
        self.until.as_ref()
    }

    pub fn wait(&self) -> Option<&NaiveDateTime> {
        self.wait.as_ref()
    }

}

impl Serialize for Task {

    fn serialize<S>(&self, serializer: &mut S) -> RResult<(), S::Error>
        where S: Serializer
    {
        serializer.serialize_struct("Task", TaskVisitor {
            value: self,
            state: 0,
        })
    }

}

struct TaskVisitor<'a> {
    value: &'a Task,
    state: u8,
}

impl<'a> MapVisitor for TaskVisitor<'a> {

    fn visit<S>(&mut self, serializer: &mut S) -> RResult<Option<()>, S::Error>
        where S: Serializer
    {
        match self.state {
            0 => {
                self.state += 1;
                Ok(Some(try!(serializer.serialize_struct_elt("status", &self.value.status))))
            },
            1 => {
                self.state += 1;
                Ok(Some(try!(serializer.serialize_struct_elt("uuid", &self.value.uuid))))
            },
            2 => {
                self.state += 1;
                Ok(Some(try!(serializer.serialize_struct_elt("entry", &self.value.entry))))
            },
            3 => {
                self.state += 1;
                Ok(Some(try!(serializer.serialize_struct_elt("description", &self.value.description))))
            },
            4 => {
                self.state += 1;
                Ok(Some(try!(serializer.serialize_struct_elt("annotation", &self.value.annotation))))
            },
            5 => {
                self.state += 1;
                Ok(Some(try!(serializer.serialize_struct_elt("depends", &self.value.depends))))
            },
            6 => {
                self.state += 1;
                Ok(Some(try!(serializer.serialize_struct_elt("due", &self.value.due))))
            },
            7 => {
                self.state += 1;
                Ok(Some(try!(serializer.serialize_struct_elt("end", &self.value.end))))
            },
            8 => {
                self.state += 1;
                Ok(Some(try!(serializer.serialize_struct_elt("imask", &self.value.imask))))
            },
            9 => {
                self.state += 1;
                Ok(Some(try!(serializer.serialize_struct_elt("mask", &self.value.mask))))
            },
            10 => {
                self.state += 1;
                Ok(Some(try!(serializer.serialize_struct_elt("modified", &self.value.modified))))
            },
            11 => {
                self.state += 1;
                Ok(Some(try!(serializer.serialize_struct_elt("parent", &self.value.parent))))
            },
            12 => {
                self.state += 1;
                Ok(Some(try!(serializer.serialize_struct_elt("priority", &self.value.priority))))
            },
            13 => {
                self.state += 1;
                Ok(Some(try!(serializer.serialize_struct_elt("project", &self.value.project))))
            },
            14 => {
                self.state += 1;
                Ok(Some(try!(serializer.serialize_struct_elt("recur", &self.value.recur))))
            },
            15 => {
                self.state += 1;
                Ok(Some(try!(serializer.serialize_struct_elt("scheduled", &self.value.scheduled))))
            },
            16 => {
                self.state += 1;
                Ok(Some(try!(serializer.serialize_struct_elt("start", &self.value.start))))
            },
            17 => {
                self.state += 1;
                Ok(Some(try!(serializer.serialize_struct_elt("tags", &self.value.tags))))
            },
            18 => {
                self.state += 1;
                Ok(Some(try!(serializer.serialize_struct_elt("until", &self.value.until))))
            },
            19 => {
                self.state += 1;
                Ok(Some(try!(serializer.serialize_struct_elt("wait", &self.value.wait))))
            },
            _ => {
                Ok(None)
            }
        }
    }

}
