//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

//! Module containing `Task` type as well as trait implementations

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
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Task {
    status      : TaskStatus,
    uuid        : Uuid,
    entry       : Date,
    description : String,

    annotations : Vec<Annotation>,
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
    tags        : Vec<Tag>,
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

            annotations : annotations.unwrap_or(vec![]),
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
            tags        : tags.unwrap_or(vec![]),
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
        Some(&self.annotations)
    }

    /// Add an annotation to this task
    pub fn add_annotation(&mut self, an: Annotation) {
        self.annotations.push(an)
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
        Some(&self.tags)
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

