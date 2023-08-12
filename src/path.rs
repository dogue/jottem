use crate::config;

#[derive(Debug)]
pub struct NotePath {
    title: String,
    parent: Option<String>,
}

impl NotePath {
    pub fn parse(input: &str) -> anyhow::Result<Self> {
        if input.is_empty() {
            return Err(anyhow::anyhow!("No path provided"));
        }

        let input = input.trim_start_matches('/');
        let input = input.trim_end_matches('/');

        let mut path = input.split('/').collect::<Vec<&str>>();
        let title = path.pop().unwrap().to_string();
        let parent = {
            if path.join("/") == "" {
                None
            } else {
                Some(path.join("/"))
            }
        };

        Ok(Self { title, parent })
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn has_parent(&self) -> bool {
        self.parent.is_some()
    }

    pub fn relative_parent(&self) -> Option<String> {
        self.parent.clone()
    }

    pub fn absolute_parent(&self) -> Option<String> {
        if let Some(path) = self.relative_parent() {
            Some(format!("{}/{}", config::get_root(), path))
        } else {
            None
        }
    }

    pub fn relative_path(&self) -> String {
        if let Some(path) = self.relative_parent() {
            format!("{}/{}", path, self.title)
        } else {
            self.title.clone()
        }
    }

    pub fn relative_path_with_ext(&self) -> String {
        format!("{}.md", self.relative_path())
    }

    pub fn absolute_path(&self) -> String {
        format!("{}/{}", config::get_root(), self.relative_path())
    }

    pub fn absolute_path_with_ext(&self) -> String {
        format!("{}.md", self.absolute_path())
    }
}
