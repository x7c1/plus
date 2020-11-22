use clap::Arg;

pub fn user_name<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("git-user-name")
        .long("git-user-name")
        .required(true)
        .takes_value(true)
        .help("For 'git config user.name'")
}

pub fn user_email<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("git-user-email")
        .long("git-user-email")
        .required(true)
        .takes_value(true)
        .help("For 'git config user.email'")
}

#[derive(Debug)]
pub struct GitConfig {
    pub user_name: String,
    pub user_email: String,
}

impl GitConfig {
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
