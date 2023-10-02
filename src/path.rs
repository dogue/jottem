use crate::{config, note::Note};

#[derive(Debug)]
pub struct NotePath {
    title: String,
    parent: Option<String>,
}

impl NotePath {
    pub fn parse(input: &str) -> anyhow::Result<Self> {
        if input.is_empty() {
            return Err(anyhow::anyhow!("Error: No path provided"));
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

    pub fn from_note(note: &Note) -> anyhow::Result<Self> {
        Self::parse(&note.relative_path)
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
        self.relative_parent()
            .map(|path| format!("{}/{}", config::get_root(), path))
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_path_from_str() {
        let input = "";
        let path = NotePath::parse(input);

        assert!(path.is_err());

        let input = "test";
        let path = NotePath::parse(input);

        assert!(path.is_ok());
        let path = path.unwrap();

        assert_eq!(path.title, "test");
        assert!(path.parent.is_none());

        let input = "parent/test";
        let path = NotePath::parse(input);

        assert!(path.is_ok());
        let path = path.unwrap();

        assert_eq!(path.title, "test");
        assert!(path.parent.is_some());
        assert_eq!(path.parent, Some("parent".into()));
    }
}
