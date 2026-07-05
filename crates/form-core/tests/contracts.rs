use form_core::*;
use serde_json::{Value, json};

fn round_trip<T>(value: &T) -> T
where
    T: serde::Serialize + serde::de::DeserializeOwned,
{
    serde_json::from_value(serde_json::to_value(value).unwrap()).unwrap()
}

#[test]
fn session_entry_round_trips_docs_shape() {
    let json = json!({
        "schema_version": 1,
        "session_id": "session-1",
        "branch_id": "branch-1",
        "entry_id": "entry-1",
        "parent_entry_id": null,
        "ts": "2026-07-02T00:00:00Z",
        "kind": "tool_result",
        "content": [{ "type": "text", "text": "done" }],
        "tool_call": { "id": "toolu_1", "name": "read", "input": { "path": "docs/mvp.md" } },
        "tool_result": {
            "tool_call_id": "toolu_1",
            "status": "ok",
            "content": [{ "type": "text", "text": "file text" }],
            "details": { "bytes": 9 },
            "patches": [],
            "metrics": { "duration_ms": 3, "bytes_read": 9, "output_bytes": 9 },
            "error": null
        },
        "approval": null,
        "audit": null,
        "usage": { "input_tokens": 10, "output_tokens": 5 },
        "meta": {}
    });

    let entry: SessionEntry = serde_json::from_value(json.clone()).unwrap();

    assert_eq!(entry.parent_entry_id, None);
    assert_eq!(entry.kind, SessionEntryKind::ToolResult);
    assert_eq!(serde_json::to_value(&entry).unwrap(), json);
}

#[test]
fn structured_error_round_trips() {
    let error = ErrorEnvelope {
        kind: "WorkspaceEscape".to_owned(),
        source: ErrorSource::Workspace,
        retryable: false,
        user_message: "Path is outside workspace".to_owned(),
        audit_level: AuditLevel::None,
        exit_code: 1,
        details: json!({ "path": "../secret" }),
        cause: Some("canonicalize failed".to_owned()),
        redact_details: false,
    };

    let json = serde_json::to_value(&error).unwrap();

    assert_eq!(json["source"], "workspace");
    assert_eq!(round_trip(&error), error);
}

#[test]
fn session_entry_allows_omitted_irrelevant_fields() {
    let json = json!({
        "schema_version": 1,
        "session_id": "session-1",
        "branch_id": "branch-1",
        "entry_id": "entry-2",
        "parent_entry_id": "entry-1",
        "ts": "2026-07-02T00:00:00Z",
        "kind": "approval",
        "tool_call": null,
        "tool_result": null,
        "approval": { "request_id": "approval-1", "decision": "deny", "reason": "no" },
        "audit": null,
        "usage": null
    });

    let entry: SessionEntry = serde_json::from_value(json).unwrap();

    assert!(entry.content.is_empty());
    assert_eq!(entry.meta, Value::Null);
}

#[test]
fn tool_contracts_round_trip() {
    let call = ToolCall {
        id: "toolu_1".to_owned(),
        name: "read".to_owned(),
        input: json!({ "path": "Cargo.toml" }),
    };
    let result = ToolResult {
        tool_call_id: call.id.clone(),
        status: ToolResultStatus::Denied,
        content: vec![ContentBlock::text("denied")],
        details: json!({ "reason": "approval denied" }),
        patches: vec!["patch-1".to_owned()],
        metrics: None,
        error: None,
    };

    assert_eq!(round_trip(&call), call);
    assert_eq!(serde_json::to_value(&ToolResultStatus::Ok).unwrap(), "ok");
    assert_eq!(
        serde_json::to_value(&ToolResultStatus::Error).unwrap(),
        "error"
    );
    assert_eq!(serde_json::to_value(&result.status).unwrap(), "denied");
    assert_eq!(round_trip(&result), result);
}

#[test]
fn provider_contracts_round_trip() {
    let request = ProviderRequest {
        model: "gpt5.5".to_owned(),
        system: "You are Form.".to_owned(),
        messages: vec![ProviderMessage {
            role: MessageRole::User,
            content: vec![ContentBlock::text("hello")],
        }],
        tools: vec![ToolSpec {
            name: "read".to_owned(),
            description: "Read a file".to_owned(),
            input_schema: json!({ "type": "object" }),
        }],
        max_tokens: 1024,
        temperature: 0.0,
    };
    let result = ProviderResult {
        assistant_content: vec![ContentBlock::text("hi")],
        tool_calls: vec![],
        stop_reason: ProviderStopReason::EndTurn,
        usage: Usage {
            input_tokens: 1,
            output_tokens: 2,
        },
        raw_provider_id: Some("msg_1".to_owned()),
        error: None,
    };

    assert_eq!(
        serde_json::to_value(&result.stop_reason).unwrap(),
        "end_turn"
    );
    assert_eq!(round_trip(&request), request);
    assert_eq!(round_trip(&result), result);
}

#[test]
fn approval_patch_audit_and_limits_round_trip() {
    let approval = ApprovalRequest {
        id: "approval-1".to_owned(),
        kind: ApprovalKind::PluginPatch,
        summary: "Apply patch".to_owned(),
        path: Some("src/main.rs".to_owned()),
        command: None,
        diff: Some("@@ -1 +1 @@".to_owned()),
        risk: "medium".to_owned(),
        tool_call_id: Some("toolu_1".to_owned()),
        created_at: "2026-07-02T00:00:00Z".to_owned(),
    };
    let result = ApprovalResult {
        request_id: approval.id.clone(),
        decision: ApprovalDecision::Allow,
        scope: ApprovalScope::Once,
        reason: Some("looks safe".to_owned()),
        decided_at: "2026-07-02T00:00:01Z".to_owned(),
    };
    let patch = patch_proposal("src/main.rs");
    let audit = AuditEntry {
        schema_version: 1,
        id: "audit-1".to_owned(),
        ts: "2026-07-02T00:00:00Z".to_owned(),
        event_kind: "approval_allowed".to_owned(),
        level: AuditLevel::Audit,
        session_id: Some("session-1".to_owned()),
        branch_id: Some("branch-1".to_owned()),
        entry_id: Some("entry-1".to_owned()),
        tool_call_id: Some("toolu_1".to_owned()),
        details: json!({ "path": "src/main.rs" }),
    };
    let limits = Limits::default();

    assert_eq!(serde_json::to_value(&ApprovalKind::Shell).unwrap(), "shell");
    assert_eq!(round_trip(&approval), approval);
    assert_eq!(round_trip(&result), result);
    assert_eq!(round_trip(&patch), patch);
    assert_eq!(round_trip(&audit), audit);
    assert_eq!(
        serde_json::to_value(&limits).unwrap()["shell_timeout_seconds"],
        30
    );
    assert_eq!(round_trip(&limits), limits);
}

#[test]
fn patch_paths_preserve_platform_strings() {
    let posix = patch_proposal("src/main.rs");
    let windows = patch_proposal(r"src\main.rs");

    assert_eq!(round_trip(&posix).target_path, "src/main.rs");
    assert_eq!(round_trip(&windows).target_path, r"src\main.rs");
}

#[test]
fn plugin_manifest_round_trips_with_platform_paths() {
    let posix = plugin_manifest("plugins/example/plugin.wasm");
    let windows = plugin_manifest(r"plugins\example\plugin.wasm");
    let json = serde_json::to_value(&posix).unwrap();

    assert_eq!(
        json["permissions"],
        json!(["workspace.read", "patch.propose"])
    );
    assert_eq!(round_trip(&posix), posix);
    assert_eq!(
        round_trip(&windows).component,
        r"plugins\example\plugin.wasm"
    );
}

#[test]
fn public_reexports_are_importable() {
    let content: ContentBlock = ContentBlock::text("hello");
    let details: Value = json!({ "ok": true });

    assert_eq!(serde_json::to_value(&content).unwrap()["type"], "text");
    assert_eq!(details["ok"], true);
}

fn patch_proposal(target_path: &str) -> PatchProposal {
    PatchProposal {
        id: "patch-1".to_owned(),
        source: PatchSource::Tool,
        target_path: target_path.to_owned(),
        unified_diff: "@@ -1 +1 @@\n-old\n+new\n".to_owned(),
        summary: "Change one line".to_owned(),
        risk: "low".to_owned(),
        created_at: "2026-07-02T00:00:00Z".to_owned(),
    }
}

fn plugin_manifest(component: &str) -> PluginManifest {
    PluginManifest {
        id: "workspace-summary".to_owned(),
        name: "Workspace Summary".to_owned(),
        version: "0.1.0".to_owned(),
        api_version: "form-plugin-0.1".to_owned(),
        component: component.to_owned(),
        permissions: vec![
            PluginPermission::WorkspaceRead,
            PluginPermission::PatchPropose,
        ],
        commands: vec![PluginCommand {
            id: "summarize".to_owned(),
            description: "Summarize the workspace".to_owned(),
            input: "json".to_owned(),
            output: "json".to_owned(),
        }],
    }
}
