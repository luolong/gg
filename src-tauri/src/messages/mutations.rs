use super::*;

/// Common result type for mutating commands
#[derive(Serialize, Clone)]
#[serde(tag = "type")]
#[cfg_attr(
    feature = "ts-rs",
    derive(TS),
    ts(export, export_to = "../src/messages/")
)]
pub enum MutationResult {
    Unchanged,
    Updated {
        new_status: RepoStatus,
    },
    UpdatedSelection {
        new_status: RepoStatus,
        new_selection: RevHeader,
    },
    PreconditionError {
        message: String,
    },
    InternalError {
        message: String,
    },
}

impl From<String> for MutationResult {
    fn from(value: String) -> Self {
        MutationResult::PreconditionError { message: value }
    }
}

impl From<&str> for MutationResult {
    fn from(value: &str) -> Self {
        MutationResult::PreconditionError {
            message: value.to_owned(),
        }
    }
}

/// Makes a revision the working copy
#[derive(Deserialize, Debug)]
#[cfg_attr(
    feature = "ts-rs",
    derive(TS),
    ts(export, export_to = "../src/messages/")
)]
pub struct CheckoutRevision {
    pub change_id: RevId,
}

/// Creates a new revision and makes it the working copy
#[derive(Deserialize, Debug)]
#[cfg_attr(
    feature = "ts-rs",
    derive(TS),
    ts(export, export_to = "../src/messages/")
)]
pub struct CreateRevision {
    pub parent_change_ids: Vec<RevId>,
}

/// Updates a revision's description
#[derive(Deserialize, Debug)]
#[cfg_attr(
    feature = "ts-rs",
    derive(TS),
    ts(export, export_to = "../src/messages/")
)]
pub struct DescribeRevision {
    pub change_id: RevId,
    pub new_description: String,
    pub reset_author: bool,
}

/// Creates a copy of the revision with the same parents and content
#[derive(Deserialize, Debug)]
#[cfg_attr(
    feature = "ts-rs",
    derive(TS),
    ts(export, export_to = "../src/messages/")
)]
pub struct DuplicateRevisions {
    pub change_ids: Vec<RevId>,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(
    feature = "ts-rs",
    derive(TS),
    ts(export, export_to = "../src/messages/")
)]
pub struct AbandonRevisions {
    pub change_ids: Vec<RevId>,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(
    feature = "ts-rs",
    derive(TS),
    ts(export, export_to = "../src/messages/")
)]
pub struct MoveChanges {
    pub from_change_id: RevId,
    pub to_id: RevId,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(
    feature = "ts-rs",
    derive(TS),
    ts(export, export_to = "../src/messages/")
)]
pub struct CopyChanges {
    pub from_change_id: RevId,
    pub to_id: RevId,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(
    feature = "ts-rs",
    derive(TS),
    ts(export, export_to = "../src/messages/")
)]
pub struct UndoOperation;
