use iced::{
  Alignment, Color, Element, Length, Theme,
  widget::{Button, column, container, row, svg, vertical_rule},
};

use crate::{
  State,
  ui::{
    pages::{Icon, Pages},
    styles::{self, PADDING, RULE_WIDTH, SPACING},
    update::Message,
  },
};

impl State {
  /// Возвращает заголовок окна приложения
  pub fn title(&self) -> String {
    "Claws".to_string()
  }

  /**
  Возвращает текущее представление приложения
  Строит UI на основе текущего состояния приложения
  */
  pub fn view(&self) -> Element<'_, Message> {
    let page = if cfg!(debug_assertions) {
      Pages::get_content(self, &self.profile).explain(Color::from_rgb(255., 0., 0.))
    } else {
      Pages::get_content(self, &self.profile)
    };

    let sidebar = container(
      column![
        create_button_with_svg_and_text(&Icon::Profiles, Message::ChangePage(Pages::Profiles)),
        create_button_with_svg_and_text(&Icon::Settings, Message::ChangePage(Pages::Settings)),
        create_button_with_svg_and_text(&Icon::Update, Message::ChangePage(Pages::Updater)),
      ]
      .spacing(SPACING),
    )
    .align_y(Alignment::Center)
    .padding(PADDING)
    .height(Length::Fill);

    let content = match self.keypad.is_open {
      true => match self.stick_callibrate {
        true => row![page],
        false => row![sidebar, vertical_rule(RULE_WIDTH), page],
      },
      false => {
        if cfg!(debug_assertions) {
          return row![sidebar, vertical_rule(RULE_WIDTH), page].into();
        }
        row![page]
      }
    };

    container(content).into()
  }

  /// Возвращает текущую тему приложения
  pub fn theme(&self) -> Theme {
    match dark_light::detect() {
      Ok(theme) => match theme {
        dark_light::Mode::Dark => Theme::Dark,
        dark_light::Mode::Light | dark_light::Mode::Unspecified => Theme::Light,
      },
      Err(_) => Theme::Light,
    }
  }
}

/**
Создает кнопку с SVG-иконкой
# Аргументы
* `icon` - Иконка из перечисления `Icon`
* `on_press` - Сообщение при нажатии
# Возвращает
Кнопка с иконкой
*/
fn create_button_with_svg_and_text<'a>(icon: &Icon, on_press: Message) -> Button<'a, Message> {
  Button::new(container(
    svg(svg::Handle::from_memory(icon.icon()))
      .height(Length::Fill)
      .width(Length::Fill),
  ))
  .width(Length::Fixed(50.))
  .height(Length::Fixed(50.))
  .on_press(on_press)
  .style(styles::button::rounding)
}
