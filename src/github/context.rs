const CI: &'static str = "CI";
const GITHUB_ACTION: &'static str = "GITHUB_ACTION";
const GITHUB_ACTION_PATH: &'static str = "GITHUB_ACTION_PATH";
const GITHUB_ACTION_REPOSITORY: &'static str = "GITHUB_ACTION_REPOSITORY";
const GITHUB_ACTIONS: &'static str = "GITHUB_ACTIONS";
const GITHUB_ACTION_STATUS: &'static str = "GITHUB_ACTION_STATUS";
const GITHUB_ACTION_REF: &'static str = "GITHUB_ACTION_REF";
const GITHUB_ACTOR: &'static str = "GITHUB_ACTOR";
const GITHUB_ACTOR_ID: &'static str = "GITHUB_ACTOR_ID";
const GITHUB_API_URL: &'static str = "GITHUB_API_URL";
const GITHUB_BASE_REF: &'static str = "GITHUB_BASE_REF";
const GITHUB_ENV: &'static str = "GITHUB_ENV";
const GITHUB_EVENT_NAME: &'static str = "GITHUB_EVENT_NAME";
const GITHUB_EVENT_PATH: &'static str = "GITHUB_EVENT_PATH";
const GITHUB_GRAPHQL_URL: &'static str = "GITHUB_GRAPHQL_URL";
const GITHUB_HEAD_REF: &'static str = "GITHUB_HEAD_REF";
const GITHUB_JOB: &'static str = "GITHUB_JOB";
const GITHUB_OUTPUT: &'static str = "GITHUB_OUTPUT";
const GITHUB_PATH: &'static str = "GITHUB_PATH";
const GITHUB_REF: &'static str = "GITHUB_REF";
const GITHUB_REF_NAME: &'static str = "GITHUB_REF_NAME";
const GITHUB_REF_PROTECTED: &'static str = "GITHUB_REF_PROTECTED";
const GITHUB_REF_TYPE: &'static str = "GITHUB_REF_TYPE";
const GITHUB_REPOSITORY: &'static str = "GITHUB_REPOSITORY";
const GITHUB_REPOSITORY_ID: &'static str = "GITHUB_REPOSITORY_ID";
const GITHUB_REPOSITORY_OWNER: &'static str = "GITHUB_REPOSITORY_OWNER";
const GITHUB_REPOSITORY_OWNER_ID: &'static str = "GITHUB_REPOSITORY_OWNER_ID";
const GITHUB_RETENTION_DAYS: &'static str = "GITHUB_RETENTION_DAYS";
const GITHUB_RUN_ATTEMPT: &'static str = "GITHUB_RUN_ATTEMPT";
const GITHUB_RUN_ID: &'static str = "GITHUB_RUN_ID";
const GITHUB_RUN_NUMBER: &'static str = "GITHUB_RUN_NUMBER";
const GITHUB_SERVER_URL: &'static str = "GITHUB_SERVER_URL";
const GITHUB_SHA: &'static str = "GITHUB_SHA";
const GITHUB_STEP_SUMMARY: &'static str = "GITHUB_STEP_SUMMARY";
const GITHUB_TRIGGERING_ACTOR: &'static str = "GITHUB_TRIGGERING_ACTOR";
const GITHUB_WORKFLOW: &'static str = "GITHUB_WORKFLOW";
const GITHUB_WORKFLOW_REF: &'static str = "GITHUB_WORKFLOW_REF";
const GITHUB_WORKFLOW_SHA: &'static str = "GITHUB_WORKFLOW_SHA";
const GITHUB_WORKSPACE: &'static str = "GITHUB_WORKSPACE";
const RUNNER_ARCH: &'static str = "RUNNER_ARCH";
const RUNNER_DEBUG: &'static str = "RUNNER_DEBUG";
const RUNNER_NAME: &'static str = "RUNNER_NAME";
const RUNNER_OS: &'static str = "RUNNER_OS";
const RUNNER_TEMP: &'static str = "RUNNER_TEMP";
const RUNNER_TOOL_CACHE: &'static str = "RUNNER_TOOL_CACHE";

/// https://docs.github.com/en/actions/writing-workflows/choosing-what-your-workflow-does/accessing-contextual-information-about-workflow-runs#github-context
#[derive(Debug)]
pub struct GithubContext {
    is_ci: bool,
    /// The name of the action currently running, or the id of a step.
    action: String,
    /// The path where an action is located. This property is only supported in composite actions.
    action_path: String,
    /// For a step executing an action, this is the ref of the action being executed. For example, v2.
    action_ref: String,
    /// For a step executing an action, this is the owner and repository name of the action. For example, actions/checkout.
    action_repository: String,
    /// The username of the user that triggered the initial workflow run.
    actor: String,
    /// The account ID of the person or app that triggered the initial workflow run.
    actor_id: String,
    /// The URL of the GitHub REST API.
    api_url: String,
    /// The base_ref or target branch of the pull request in a workflow run.
    base_ref: String,
    /// The [job_id](https://docs.github.com/en/actions/writing-workflows/workflow-syntax-for-github-actions#jobsjob_id) of the current job.
    job: String,
}

impl GithubContext {
    /// Pulls all context from environment.
    pub fn new() -> Self {
        Self {
            is_ci: std::env::var(CI)
                .unwrap_or("true".to_string())
                .parse::<bool>()
                .unwrap_or(true),
            action: ensure(GITHUB_ACTION),
            action_path: ensure(GITHUB_PATH),
            action_ref: ensure(GITHUB_ACTION_REF),
            action_repository: ensure(GITHUB_REPOSITORY),
            actor: ensure(GITHUB_ACTOR),
            actor_id: ensure(GITHUB_ACTOR_ID),
            api_url: ensure(GITHUB_API_URL),
            base_ref: ensure(GITHUB_BASE_REF),
            job: ensure(GITHUB_JOB),
        }
    }
}

fn ensure(key: &str) -> String {
    std::env::var(key).expect(&format!("[{key}]: should be set by GitHub."))
}
