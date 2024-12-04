use iced::{
    widget::{button, column, container, row, text, scrollable},
    Application, Command, Element, Length, Theme, alignment,
    Background, Color,
    theme,
};
use crate::models::{Project, Route, QuickAccessItem};

pub struct SoftwareManager {
    projects: Vec<Project>,
    selected_project: Option<usize>,
    current_route: Route,
    quick_access_items: Vec<QuickAccessItem>,
}

#[derive(Debug, Clone)]
pub enum Message {
    CreateProject,
    SelectProject(usize),
    UpdateProject(Project),
    DeleteProject(usize),
    NavigateTo(Route),
}

impl Application for SoftwareManager {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                projects: Vec::new(),
                selected_project: None,
                current_route: Route::Dashboard,
                quick_access_items: Vec::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Software Manager")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::CreateProject => {
                // TODO: Implement project creation
                Command::none()
            }
            Message::SelectProject(index) => {
                self.selected_project = Some(index);
                Command::none()
            }
            Message::UpdateProject(project) => {
                if let Some(index) = self.selected_project {
                    self.projects[index] = project;
                }
                Command::none()
            }
            Message::DeleteProject(index) => {
                self.projects.remove(index);
                if Some(index) == self.selected_project {
                    self.selected_project = None;
                }
                Command::none()
            }
            Message::NavigateTo(route) => {
                self.current_route = route;
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let sidebar = self.view_sidebar();
        let main_content = self.view_main_content();

        row![
            // Sidebar
            sidebar,
            // Main content
            main_content,
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}

impl SoftwareManager {
    fn view_sidebar(&self) -> Element<Message> {
        let logo = text("🦀").size(32);
        
        let nav_items = column![
            self.nav_button("Dashboard", Route::Dashboard),
            self.nav_button("Collections", Route::Collections),
            self.nav_button("Articles", Route::Articles),
            self.nav_button("Learners", Route::Learners),
            self.nav_button("Reports", Route::Reports),
        ]
        .spacing(10)
        .padding(20);

        let quick_access_section = column![
            text("Quick Access").size(14),
            // Add quick access items here
        ]
        .spacing(10)
        .padding(20);

        container(
            column![
                logo,
                nav_items,
                quick_access_section,
            ]
            .spacing(30)
        )
        .width(Length::Fixed(250.0))
        .height(Length::Fill)
        .style(theme::Container::Custom(Box::new(CustomTheme)))
        .into()
    }

    fn view_main_content(&self) -> Element<Message> {
        let header = row![
            text(self.page_title()).size(24),
            button("New Collection")
                .style(theme::Button::Custom(Box::new(CustomTheme)))
                .on_press(Message::CreateProject)
        ]
        .spacing(20)
        .padding(20)
        .align_items(alignment::Alignment::Center);

        let content = match self.current_route {
            Route::Collections => self.view_collections(),
            Route::Dashboard => text("Dashboard Content").into(),
            Route::Articles => text("Articles Content").into(),
            Route::Learners => text("Learners Content").into(),
            Route::Reports => text("Reports Content").into(),
        };

        container(
            column![
                header,
                content,
            ]
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .style(theme::Container::Custom(Box::new(CustomTheme)))
        .into()
    }

    fn view_collections(&self) -> Element<Message> {
        let projects_list = self.projects.iter().enumerate().map(|(i, project)| {
            row![
                // Project icon/thumbnail
                container(text("📁")).width(Length::Fixed(40.0)),
                // Project details
                column![
                    text(&project.name),
                    text(&project.path).size(12),
                ],
                // Version info
                text("v1.0"),
                // Status
                text("TODO"),
                // Actions
                row![
                    button("GitHub"),
                    button("Edit"),
                ]
            ]
            .spacing(20)
            .padding(10)
            .into()
        });

        scrollable(
            column(projects_list.collect())
                .spacing(10)
                .padding(20)
        )
        .into()
    }

    fn nav_button(&self, label: &str, route: Route) -> Element<Message> {
        let is_active = self.current_route == route;
        
        button(
            row![
                text(label).size(16)
            ]
            .padding(10)
        )
        .width(Length::Fill)
        .style(theme::Button::Custom(Box::new(NavButtonTheme { is_active })))
        .on_press(Message::NavigateTo(route))
        .into()
    }

    fn page_title(&self) -> String {
        match self.current_route {
            Route::Dashboard => "Dashboard",
            Route::Collections => "Collections",
            Route::Articles => "Articles",
            Route::Learners => "Learners",
            Route::Reports => "Reports",
        }
        .to_string()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CustomTheme;

impl container::StyleSheet for CustomTheme {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(Color::from_rgb(0.1, 0.1, 0.15))),
            text_color: None,
            border_radius: 0.0.into(),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        }
    }
}

impl button::StyleSheet for CustomTheme {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(Color::from_rgb(1.0, 0.5, 0.0))),
            border_radius: 4.0.into(),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            text_color: Color::WHITE,
            shadow_offset: iced::Vector::default(),
            ..button::Appearance::default()
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NavButtonTheme {
    is_active: bool,
}

impl button::StyleSheet for NavButtonTheme {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        let background_color = if self.is_active {
            Color::from_rgb(0.2, 0.2, 0.25)
        } else {
            Color::from_rgb(0.15, 0.15, 0.2)
        };

        button::Appearance {
            background: Some(Background::Color(background_color)),
            border_radius: 4.0.into(),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            text_color: Color::WHITE,
            shadow_offset: iced::Vector::default(),
            ..button::Appearance::default()
        }
    }
} 