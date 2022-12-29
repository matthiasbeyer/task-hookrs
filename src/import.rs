//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

//! Module containing the `import()` function

use std::io::BufRead;
use std::io::Read;

use serde_json;

use crate::error::Error;
use crate::task::{Task, TaskWarriorVersion};

/// Import taskwarrior-exported JSON. This expects an JSON Array of objects, as exported by
/// taskwarrior.
pub fn import<T: TaskWarriorVersion, R: Read>(r: R) -> Result<Vec<Task<T>>, Error> {
    serde_json::from_reader(r).map_err(Error::from)
}

/// Import a single JSON-formatted Task
pub fn import_task<T: TaskWarriorVersion>(s: &str) -> Result<Task<T>, Error> {
    serde_json::from_str(s).map_err(Error::from)
}

/// Reads line by line and tries to parse a task-object per line.
pub fn import_tasks<T: TaskWarriorVersion, BR: BufRead>(r: BR) -> Vec<Result<Task<T>, Error>> {
    let mut vt = Vec::new();
    for line in r.lines() {
        if let Err(err) = line {
            vt.push(Err(Error::from(err)));
            continue;
        }
        // Unwrap is safe because of continue above
        if line.as_ref().unwrap().is_empty() {
            // Empty strings are not usable, and shall be silently ignored
            continue;
        }
        vt.push(import_task(line.unwrap().as_str()));
    }
    vt
}

#[cfg(test)]
mod test {
    use crate::import::{import, import_task, import_tasks};
    use crate::task::{Task, TW25, TW26};

    #[test]
    fn test_one_tw25() {
        let s = r#"
[
    {
        "id": 1,
        "description": "some description",
        "entry": "20150619T165438Z",
        "modified": "20160327T164007Z",
        "project": "someproject",
        "status": "waiting",
        "tags": ["some", "tags", "are", "here"],
        "uuid": "8ca953d5-18b4-4eb9-bd56-18f2e5b752f0",
        "depends": "8ca953d5-18b5-4eb9-bd56-18f2e5b752f0",
        "wait": "20160508T164007Z",
        "urgency": 0.583562
    }
]
"#;

        let imported = import::<TW25, _>(s.as_bytes());
        assert!(imported.is_ok());
        let imported = imported.unwrap();
        assert!(imported.len() == 1);
    }

    #[test]
    fn test_one_tw26() {
        let s = r#"
[
    {
        "id": 1,
        "description": "some description",
        "entry": "20150619T165438Z",
        "modified": "20160327T164007Z",
        "project": "someproject",
        "status": "waiting",
        "tags": ["some", "tags", "are", "here"],
        "uuid": "8ca953d5-18b4-4eb9-bd56-18f2e5b752f0",
        "depends": ["8ca953d5-18b5-4eb9-bd56-18f2e5b752f0"],
        "wait": "20160508T164007Z",
        "urgency": 0.583562
    }
]
"#;

        let imported = import::<TW26, _>(s.as_bytes());
        assert!(imported.is_ok());
        let imported = imported.unwrap();
        assert!(imported.len() == 1);
    }

    #[test]
    fn test_two_tw25() {
        let s = r#"
[
    {
        "id"          : 1,
        "description" : "test",
        "entry"       : "20150619T165438Z",
        "modified"    : "20160327T164007Z",
        "project"     : "self.software",
        "status"      : "waiting",
        "tags"        : ["check", "this", "crate", "out"],
        "uuid"        : "8ca953d5-18b4-4eb9-bd56-18f2e5b752f0",
        "wait"        : "20160508T164007Z",
        "urgency"     : 0.583562
    },
    {
        "id"          : 2,
        "description" : "another test",
        "entry"       : "20150623T181011Z",
        "modified"    : "20160327T163718Z",
        "priority"    : "L",
        "project"     : "studying",
        "status"      : "waiting",
        "tags"        : ["I", "have", "awesome", "kittens"],
        "uuid"        : "54d49ffc-a06b-4dd8-b7d1-db5f50594312",
        "wait"        : "20160508T163718Z",
        "annotations" : [
            {
                "entry"       : "20150623T181018Z",
                "description" : "fooooooobar"
            }
        ],
        "urgency"     : 3.16164
    },
    {
        "id"          : 3,
        "description" : "I love kittens, really!",
        "entry"       : "20150919T222323Z",
        "modified"    : "20160327T163718Z",
        "project"     : "getkittens",
        "status"      : "waiting",
        "tags"        : ["kittens", "are", "so", "damn", "awesome"],
        "uuid"        : "08ee8dce-cb97-4c8c-9940-c9a440e90119",
        "wait"        : "20160508T163718Z",
        "urgency"     : 1.07397
    }
]

"#;

        assert!(import::<TW25, _>(s.as_bytes()).unwrap().len() == 3);
    }

    #[test]
    fn test_one_single_tw25() {
        use crate::date::Date;
        use crate::date::TASKWARRIOR_DATETIME_TEMPLATE;
        use crate::status::TaskStatus;
        use chrono::NaiveDateTime;
        use uuid::Uuid;
        fn mkdate(s: &str) -> Date {
            let n = NaiveDateTime::parse_from_str(s, TASKWARRIOR_DATETIME_TEMPLATE);
            Date::from(n.unwrap())
        }
        let s = r#"
{
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
}
"#;
        let imported = import_task(s);
        assert!(imported.is_ok());

        // Check for every information
        let task: Task<TW25> = imported.unwrap();
        assert_eq!(*task.status(), TaskStatus::Waiting);
        assert_eq!(task.description(), "some description");
        assert_eq!(*task.entry(), mkdate("20150619T165438Z"));
        assert_eq!(
            *task.uuid(),
            Uuid::parse_str("8ca953d5-18b4-4eb9-bd56-18f2e5b752f0").unwrap()
        );
        assert_eq!(task.modified(), Some(&mkdate("20160327T164007Z")));
        assert_eq!(task.project(), Some(&String::from("someproject")));
        if let Some(tags) = task.tags() {
            for tag in tags {
                let any_tag = ["some", "tags", "are", "here"].iter().any(|t| tag == *t);
                assert!(any_tag, "Tag {} missing", tag);
            }
        } else {
            panic!("Tags completely missing");
        }

        assert_eq!(task.wait(), Some(&mkdate("20160508T164007Z")));
    }

    #[test]
    fn test_two_single_tw25() {
        use crate::status::TaskStatus;
        use std::io::BufReader;
        let s = r#"
{"id":1,"description":"some description","entry":"20150619T165438Z","modified":"20160327T164007Z","project":"someproject","status":"waiting","tags":["some","tags","are","here"],"uuid":"8ca953d5-18b4-4eb9-bd56-18f2e5b752f0","wait":"20160508T164007Z","urgency":0.583562}
{"id":1,"description":"some description","entry":"20150619T165438Z","modified":"20160327T164007Z","project":"someproject","status":"waiting","tags":["some","tags","are","here"],"uuid":"8ca953d5-18b4-4eb9-bd56-18f2e5b752f0","wait":"20160508T164007Z","urgency":0.583562}"#;
        let imported = import_tasks(BufReader::new(s.as_bytes()));
        assert_eq!(imported.len(), 2);
        assert!(imported[0].is_ok());
        assert!(imported[1].is_ok());
        let import0: &Task<TW25> = imported[0].as_ref().unwrap();
        let import1 = imported[1].as_ref().unwrap();
        assert_eq!(*import0.status(), TaskStatus::Waiting);
        assert_eq!(*import1.status(), TaskStatus::Waiting);
    }
}
