//! Module containing `Task` type as well as trait implementations

use std::result::Result as RResult;

use serde::Serialize;
use serde::Serializer;
use serde::Deserialize;
use serde::Deserializer;
use serde::de::Visitor;
use serde::de::Error as SerdeError;
use serde::ser::MapVisitor;
use serde::de::MapVisitor as DeserializeMapVisitor;
use uuid::Uuid;

use priority::TaskPriority;
use status::TaskStatus;
use project::Project;
use tag::Tag;
use date::Date;
use annotation::Annotation;

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
/// It is deserializeable and serializeable via serde_json, so importing and exporting taskwarrior
/// tasks is simply serializing and deserializing objects of this type.
#[derive(Debug, Clone)]
pub struct Task {
    status      : TaskStatus,
    uuid        : Uuid,
    entry       : Date,
    description : String,

    annotations : Option<Vec<Annotation>>,
    depends     : Option<String>,
    due         : Option<Date>,
    end         : Option<Date>,
    imask       : Option<i64>,
    mask        : Option<String>,
    modified    : Option<Date>,
    parent      : Option<Uuid>,
    priority    : Option<TaskPriority>,
    project     : Option<Project>,
    recur       : Option<String>,
    scheduled   : Option<Date>,
    start       : Option<Date>,
    tags        : Option<Vec<Tag>>,
    until       : Option<Date>,
    wait        : Option<Date>,
}

/*
 * TODO: We do not fail if the JSON parsing fails. This panics. We rely on taskwarrior to be nice
 * to us. I guess this should be fixed.
 */
impl Task {

    /// Create a new Task instance
    pub fn new(
        status      : TaskStatus,
        uuid        : Uuid,
        entry       : Date,
        description : String,

        annotations : Option<Vec<Annotation>>,
        depends     : Option<String>,
        due         : Option<Date>,
        end         : Option<Date>,
        imask       : Option<i64>,
        mask        : Option<String>,
        modified    : Option<Date>,
        parent      : Option<Uuid>,
        priority    : Option<TaskPriority>,
        project     : Option<Project>,
        recur       : Option<String>,
        scheduled   : Option<Date>,
        start       : Option<Date>,
        tags        : Option<Vec<Tag>>,
        until       : Option<Date>,
        wait        : Option<Date>,
    ) -> Task
    {
        Task {
            status      : status,
            uuid        : uuid,
            entry       : entry,
            description : description,

            annotations : annotations,
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

    /// Get the status of the task
    pub fn status(&self) -> &TaskStatus {
        &self.status
    }

    /// Get the uuid of the task
    pub fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    /// Get the entry date of the task
    pub fn entry(&self) -> &Date {
        &self.entry
    }

    /// Get the description of the task
    pub fn description(&self) -> &String {
        &self.description
    }

    /// Get the annotations of the task
    pub fn annotations(&self) -> Option<&Vec<Annotation>> {
        self.annotations.as_ref()
    }

    /// Add an annotation to this task
    pub fn add_annotation(&mut self, an: Annotation) {
        if self.annotations.is_none() {
            self.annotations = Some(vec![an]);
        } else {
            match self.annotations.as_mut() {
                Some(ref mut anno) => anno.push(an),
                _ => unreachable!(),
            }
        }
    }

    /// Add annotations to this task
    pub fn add_annotations<I: Iterator<Item = Annotation>>(&mut self, i: I) {
        for item in i {
            self.add_annotation(item)
        }
    }

    /// Get the depends of the task
    ///
    /// This is exported as String by now, which might change in future
    pub fn depends(&self) -> Option<&String> {
        self.depends.as_ref()
    }

    /// Get the due date of the task
    pub fn due(&self) -> Option<&Date> {
        self.due.as_ref()
    }

    /// Get the end date of the task
    pub fn end(&self) -> Option<&Date> {
        self.end.as_ref()
    }

    /// Get the imask of the task
    pub fn imask(&self) -> Option<&i64> {
        self.imask.as_ref()
    }

    /// Get the mask of the task
    pub fn mask(&self) -> Option<&String> {
        self.mask.as_ref()
    }

    /// Get the modified date of the task
    pub fn modified(&self) -> Option<&Date> {
        self.modified.as_ref()
    }

    /// Get the parent of the task
    pub fn parent(&self) -> Option<&Uuid> {
        self.parent.as_ref()
    }

    /// Get the priority of the task
    pub fn priority(&self) -> Option<&TaskPriority> {
        self.priority.as_ref()
    }

    /// Get the project of the task
    pub fn project(&self) -> Option<&Project> {
        self.project.as_ref()
    }

    /// Get the recur of the task
    ///
    /// This is exported as String by now. This might change in future versions of this crate.
    pub fn recur(&self) -> Option<&String> {
        self.recur.as_ref()
    }

    /// Get the scheduled date of the task
    pub fn scheduled(&self) -> Option<&Date> {
        self.scheduled.as_ref()
    }

    /// Get the start date of the task
    pub fn start(&self) -> Option<&Date> {
        self.start.as_ref()
    }

    /// Get the tags of the task
    pub fn tags(&self) -> Option<&Vec<Tag>> {
        self.tags.as_ref()
    }

    /// Get the until date of the task
    pub fn until(&self) -> Option<&Date> {
        self.until.as_ref()
    }

    /// Get the wait date of the task
    pub fn wait(&self) -> Option<&Date> {
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

/// Helper type for task serialization
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
                Ok(Some(try!(serializer.serialize_struct_elt("annotations", &self.value.annotations))))
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

impl Deserialize for Task {

    fn deserialize<D>(deserializer: &mut D) -> RResult<Task, D::Error>
        where D: Deserializer
    {
        static FIELDS: &'static [&'static str] = &[
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
            "wait"
        ];
        deserializer.deserialize_struct("Task", FIELDS, TaskDeserializeVisitor)
    }

}

/// Helper type for task deserialization
struct TaskDeserializeVisitor;

impl Visitor for TaskDeserializeVisitor {
    type Value = Task;

    fn visit_map<V>(&mut self, mut visitor: V) -> RResult<Task, V::Error>
        where V: DeserializeMapVisitor
    {
        let mut status      = None;
        let mut uuid        = None;
        let mut entry       = None;
        let mut description = None;

        let mut annotations = None;
        let mut depends     = None;
        let mut due         = None;
        let mut end         = None;
        let mut imask       = None;
        let mut mask        = None;
        let mut modified    = None;
        let mut parent      = None;
        let mut priority    = None;
        let mut project     = None;
        let mut recur       = None;
        let mut scheduled   = None;
        let mut start       = None;
        let mut tags        = None;
        let mut until       = None;
        let mut wait        = None;

        loop {
            let key : Option<String> = try!(visitor.visit_key());
            if key.is_none() {
                break;
            }
            let key = key.unwrap();

            match &key[..] {
                "status" => {
                    status = Some(try!(visitor.visit_value()));
                },
                "uuid" => {
                    uuid = Some(try!(visitor.visit_value()));
                },
                "entry" => {
                    entry = Some(try!(visitor.visit_value()));
                },
                "description" => {
                    description = Some(try!(visitor.visit_value()));
                },

                "annotations" => {
                    annotations = Some(try!(visitor.visit_value()));
                },
                "depends" => {
                    depends = Some(try!(visitor.visit_value()));
                },
                "due" => {
                    due = Some(try!(visitor.visit_value()));
                },
                "end" => {
                    end = Some(try!(visitor.visit_value()));
                },
                "imask" => {
                    imask = Some(try!(visitor.visit_value()));
                },
                "mask" => {
                    mask = Some(try!(visitor.visit_value()));
                },
                "modified" => {
                    modified = Some(try!(visitor.visit_value()));
                },
                "parent" => {
                    parent = Some(try!(visitor.visit_value()));
                },
                "priority" => {
                    priority = Some(try!(visitor.visit_value()));
                },
                "project" => {
                    project = Some(try!(visitor.visit_value()));
                },
                "recur" => {
                    recur = Some(try!(visitor.visit_value()));
                },
                "scheduled" => {
                    scheduled = Some(try!(visitor.visit_value()));
                },
                "start" => {
                    start = Some(try!(visitor.visit_value()));
                },
                "tags" => {
                    tags = Some(try!(visitor.visit_value()));
                },
                "until" => {
                    until = Some(try!(visitor.visit_value()));
                },
                "wait" => {
                    wait = Some(try!(visitor.visit_value()));
                },

                field => {
                    use serde::de::impls::IgnoredAny;

                    debug!("field '{}' ignored", field);
                    let _: IgnoredAny = try!(visitor.visit_value());
                }
            }
        }

        let status = match status {
            Some(status) => status,
            None => try!(visitor.missing_field("status")),
        };

        let uuid = match uuid {
            Some(uuid) => uuid,
            None => try!(visitor.missing_field("uuid")),
        };

        let entry = match entry {
            Some(entry) => entry,
            None => try!(visitor.missing_field("entry")),
        };

        let description = match description {
            Some(description) => description,
            None => try!(visitor.missing_field("description")),
        };

        try!(visitor.end());

        let task = Task::new(
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
            wait
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

    use uuid::Uuid;
    use chrono::naive::datetime::NaiveDateTime;
    use serde_json;

    fn mkdate(s: &str) -> Date {
        let n = NaiveDateTime::parse_from_str(s, TASKWARRIOR_DATETIME_TEMPLATE);
        Date::from(n.unwrap())
    }

    #[test]
    fn test_deser() {
        let s =
r#"{
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
        let task : Task = task.unwrap();

        assert!(task.status().clone() == TaskStatus::Waiting);
        assert!(task.description() == "test");
        assert!(task.entry().clone() == mkdate("20150619T165438Z"));
        assert!(task.uuid().clone() == Uuid::parse_str("8ca953d5-18b4-4eb9-bd56-18f2e5b752f0").unwrap());

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
        let s =
r#"{
"id": 1,
"description": "some description",
"entry": "20150619T165438Z",
"modified": "20160327T164007Z",
"project": "someproject",
"status": "waiting",
"tags": ["some", "tags", "are", "here"],
"uuid": "8ca953d5-18b4-4eb9-bd56-18f2e5b752f0",
"wait": "20160508T164007Z",
"urgency": 0.583562
}"#;

        println!("{}", s);

        let task = serde_json::from_str(s);
        println!("{:?}", task);
        assert!(task.is_ok());
        let task : Task = task.unwrap();

        assert!(task.status().clone() == TaskStatus::Waiting);
        assert!(task.description() == "some description");
        assert!(task.entry().clone() == mkdate("20150619T165438Z"));
        assert!(task.uuid().clone() == Uuid::parse_str("8ca953d5-18b4-4eb9-bd56-18f2e5b752f0").unwrap());

        assert!(task.modified() == Some(&mkdate("20160327T164007Z")));
        assert!(task.project() == Some(&String::from("someproject")));

        if let Some(tags) = task.tags() {
            for tag in tags {
                let any_tag = [ "some", "tags", "are", "here", ]
                    .into_iter().any(|t| tag == *t);
                assert!(any_tag, "Tag {} missing", tag);
            }
        } else {
                assert!(false, "Tags completely missing");
        }

        assert!(task.wait() == Some(&mkdate("20160508T164007Z")));
        // assert!(task.urgency().clone() == 0.583562);

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
    }

    #[test]
    fn test_deser_annotation() {
        let s =
r#"{
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
"urgency":10.911
}"#;

        println!("{}", s);

        let task = serde_json::from_str(s);
        println!("{:?}", task);
        assert!(task.is_ok());
        let task : Task = task.unwrap();

        let all_annotations = vec![
            Annotation::new(mkdate("20160423T125911Z"), String::from("An Annotation")),
            Annotation::new(mkdate("20160423T125926Z"), String::from("Another Annotation")),
            Annotation::new(mkdate("20160422T125942Z"), String::from("A Third Anno"))
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

}

