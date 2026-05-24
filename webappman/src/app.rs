use std::fs;

pub struct App {
    pub apps: Vec<(String, String)>,
    pub selected: usize,
    pub show_all: bool,
    pub adding: bool,
    pub input_name: String,
    pub input_url: String,
    // 0 = name, 1 = url
    pub input_stage: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            apps: load_apps(),
            selected: 0,
            show_all: false,
            adding: false,
            input_name: String::new(),
            input_url: String::new(),
            input_stage: 0,
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

    pub fn toggle_show_all(&mut self) {
        self.show_all = !self.show_all;
        if self.selected >= self.apps.len() && !self.apps.is_empty() {
            self.selected = 0;
        }
    }

    pub fn start_add(&mut self) {
        self.adding = true;
        self.input_name.clear();
        self.input_url.clear();
        self.input_stage = 0;
    }

    pub fn cancel_add(&mut self) {
        self.adding = false;
        self.input_name.clear();
        self.input_url.clear();
        self.input_stage = 0;
    }

    pub fn input_char(&mut self, ch: char) {
        if self.input_stage == 0 {
            self.input_name.push(ch);
        } else {
            self.input_url.push(ch);
        }
    }

    pub fn backspace(&mut self) {
        if self.input_stage == 0 {
            self.input_name.pop();
        } else {
            self.input_url.pop();
        }
    }

    pub fn next_field(&mut self) {
        if self.input_stage == 0 {
            self.input_stage = 1;
        } else {
            self.commit_add();
        }
    }

    pub fn commit_add(&mut self) {
        let name = self.input_name.trim().to_string();
        let url = self.input_url.trim().to_string();
        if !name.is_empty() && !url.is_empty() {
            crate::bash::add_app(&name, &url);
            self.reload();
        }
        self.adding = false;
        self.input_name.clear();
        self.input_url.clear();
        self.input_stage = 0;
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
