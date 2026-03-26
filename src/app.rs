use ratatui::widgets::ListState;

pub enum CurrentScreen {
    Loading,
    Input,
    FileList,
}

pub struct App {
    pub screen: CurrentScreen,
    pub owner: String,
    pub repo: String,
    pub input_buffer: String,
    pub cursor_position: usize,
    pub current_path: String,
    pub items: Vec<RepoItem>,
    pub selected: usize,
    pub marked: Vec<RepoItem>,
    pub loading: bool,
    pub list_state: ListState,
}

#[derive(Debug)]
pub struct RepoItem {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub download_url: Option<String>,
}

impl App {
    pub fn next(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn new(owner: String, repo: String) -> Self {
        Self {
            screen: CurrentScreen::Input,
            owner,
            repo,
            input_buffer: String::new(),
            cursor_position: 0,
            current_path: String::new(),
            items: Vec::new(),
            selected: 0,
            marked: Vec::new(),
            loading: false,
            list_state: ListState::default(),
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_position < self.input_buffer.len() {
            self.cursor_position += 1;
        }
    }

    pub fn enter_char(&mut self, new_char: char) {
        self.input_buffer.insert(self.cursor_position, new_char);
        self.move_cursor_right()
    }

    pub fn delete_char(&mut self) {
        if self.cursor_position != 0 {
            let from_left_to_current_index = self.cursor_position - 1;
            self.input_buffer.remove(from_left_to_current_index);
            self.move_cursor_left();
        }
    }
}
