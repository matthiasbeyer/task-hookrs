use std::collections::BTreeMap;

use chrono::naive::datetime::NaiveDateTime;
use serde_json::value::Value;
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

    pub fn uuid(&self) -> Uuid {
        &self.uuid
    }

    pub fn entry(&self) -> NaiveDateTime {
        &self.entry
    }

    pub fn description(&self) -> String {
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

impl Into<Value> for Task {

    fn into(self) -> Value {
        unimplemented!()
    }

}

pub trait FromJson {

    fn from_json(Value) -> Result<Self>;

}

impl FromJson for Task {

    fn from_json(v: Value) -> Result<Self> {
        unimplemented!()
    }

}

