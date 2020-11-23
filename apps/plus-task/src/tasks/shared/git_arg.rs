#[derive(Debug)]
pub struct GitConfig {
    pub user_name: String,
    pub user_email: String,
}

impl GitConfig {
    pub fn gh_actions_bot() -> GitConfig {
        GitConfig {
            user_name: "github-actions[bot]".to_string(),
            user_email: "41898282+github-actions[bot]@users.noreply.github.com".to_string(),
        }
    }
    pub fn as_cli_format(&self) -> Vec<String> {
        vec![
            "-c".to_string(),
            format!("user.name={}", self.user_name),
            "-c".to_string(),
            format!("user.email={}", self.user_email),
        ]
    }
}

pub trait HasGitConfig {
    fn get_git_config(&self) -> &GitConfig;
}
