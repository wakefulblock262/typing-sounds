use zed_extension_api as zed;

struct TypingSoundsExtension;

impl zed::Extension for TypingSoundsExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        let path = worktree.which("typing-sounds-server").ok_or_else(|| "typing-sounds-server not found in PATH. Please compile the server and add it to your PATH.".to_string())?;

        Ok(zed::Command {
            command: path,
            args: vec![],
            env: Default::default(),
        })
    }
}

zed::register_extension!(TypingSoundsExtension);
