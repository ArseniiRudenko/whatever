use super::scheme::RedirectScheme;

#[derive(Clone, Default)]
pub struct RedirectSchemeBuilder {
    // Temporary redirect (true: 307 Temporary Redirect, false: 301 Moved Permanently)
    temporary: bool,
    // List of string replacements
    replacements: Vec<(String, String)>,
    // List of paths that are not redirected
    ignore_paths: Vec<String>,
}



impl RedirectSchemeBuilder {
    /// Create new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set answer code for permanent redirection
    pub fn set_permanent(&mut self) -> &mut Self {
        let mut new = self;
        new.temporary =false;
        new
    }

    /// Set answer code for temporary redirection
    pub fn set_temporary(&mut self) -> &mut Self {
        let mut new = self;
        new.temporary = true;
        new
    }

    /// Set list of replacements
    pub fn replacements<S: ToString>(&mut self, value: &[(S, S)]) -> &mut Self {
        self.replacements = value
            .iter()
            .map(|(a, b)| ((*a).to_string(), (*b).to_string()))
            .collect();
        self
    }

    /// Add a path to not include in the redirect
    pub fn ignore_path<S: ToString>(&mut self, path: S) -> &mut Self {
        self.ignore_paths.push(path.to_string());
        self
    }

    /// Build RedirectScheme
    pub fn build(&self) -> RedirectScheme {
        RedirectScheme {
            temporary: self.temporary,
            replacements: self.replacements.clone(),
            ignore_paths: self.ignore_paths.clone(),
        }
    }
}
