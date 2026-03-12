use std::fs;

pub struct App {
    pub apps: Vec<(String, String)>,
    pub selected: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            apps: load_apps(),
            selected: 0,
        }
    }

    pub fn move_down(&mut self) {
        if self.selected + 1 < self.apps.len() {
            self.selected += 1;
        }
    }

    pub fn move_up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn selected_url(&self) -> Option<&str> {
        self.apps.get(self.selected).map(|(_, url)| url.as_str())
    }

    pub fn reload(&mut self) {
        self.apps = load_apps();
    }
}

fn load_apps() -> Vec<(String, String)> {
    let path = format!(
        "{}/.config/webappman/apps.txt",
        std::env::var("HOME").unwrap()
    );

    let content = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => return vec![],
    };

    content
        .lines()
        .filter(|l| l.contains('|'))
        .map(|line| {
            let parts: Vec<&str> = line.splitn(2, '|').collect();
            (parts[0].to_string(), parts[1].to_string())
        })
        .collect()
}
