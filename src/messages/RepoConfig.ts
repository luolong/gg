// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { DisplayPath } from "./DisplayPath";
import type { RepoStatus } from "./RepoStatus";

export type RepoConfig = { "type": "Workspace", absolute_path: DisplayPath, default_revset: string, status: RepoStatus, } | { "type": "NoWorkspace", absolute_path: DisplayPath, error: string, } | { "type": "NoOperation", absolute_path: DisplayPath, error: string, } | { "type": "DeadWorker", error: string, };