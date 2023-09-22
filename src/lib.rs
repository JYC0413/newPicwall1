use flowsnet_platform_sdk::logger;
use github_flows::{
    get_octo, listen_to_event,
    octocrab::models::{events::payload::EventPayload, reactions::ReactionContent},
    GithubLogin,
};

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> anyhow::Result<()> {
    logger::init();
    listen_to_event(
        &GithubLogin::Default,
        "DarumaDocker",
        "github-func-test",
        vec!["issue_comment"],
        |payload| handler(payload),
    )
    .await;
    Ok(())
}

async fn handler(payload: EventPayload) {
    log::debug!("running github issue comment handler");
    if let EventPayload::IssueCommentEvent(e) = payload {
        let issue_number = e.comment.id.0;

        // installed app login
        let octo = get_octo(&GithubLogin::Default);

        octo.issues("DarumaDocker", "github-func-test")
            .create_comment_reaction(issue_number, ReactionContent::Rocket)
            .await
            .unwrap();
    };
}
