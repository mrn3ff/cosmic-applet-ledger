mod about;
mod accounts;
mod general;

use super::{
    Alignment, AppState, Background, Color, Config, DetectionSnapshot, Element, Length, Message,
    PopupRoute, ProviderId, ProviderLoginStates, UpdateStatus, component_container_style,
    component_divider_color, container, fl, provider_icon_handle, provider_icon_variant, row,
    widget,
};

pub(super) fn settings_view(
    state: &AppState,
    update_status: &UpdateStatus,
) -> Element<'static, Message> {
    let providers_label = container(widget::text("PROVIDERS").size(11))
        .style(|_| widget::container::Style {
            text_color: Some(Color::from_rgba(1.0, 1.0, 1.0, 0.45)),
            ..Default::default()
        });

    let providers = ProviderId::ALL;
    let mut provider_rows = cosmic::iced::widget::column![].spacing(6).width(Length::Fill);

    for provider_id in providers {
        let enabled = state
            .provider(provider_id)
            .is_some_and(|p| p.enabled);

        let icon = widget::icon::icon(provider_icon_handle(provider_id, provider_icon_variant()))
            .size(18);

        let name = widget::text(provider_id.label()).size(14);

        let toggle = cosmic::widget::toggler(enabled)
            .on_toggle(move |enabled| Message::SetProviderEnabled(provider_id, enabled));

        let configure_btn = widget::button::icon(widget::icon::from_name("pan-end-symbolic"))
            .extra_small()
            .padding(4)
            .class(cosmic::theme::Button::Custom {
                active: Box::new(|_focused, _theme| widget::button::Style {
                    text_color: Some(Color::from_rgba(1.0, 1.0, 1.0, 0.45)),
                    icon_color: Some(Color::from_rgba(1.0, 1.0, 1.0, 0.45)),
                    ..Default::default()
                }),
                disabled: Box::new(|_| widget::button::Style::new()),
                hovered: Box::new(|_focused, _theme| widget::button::Style {
                    text_color: Some(Color::WHITE),
                    icon_color: Some(Color::WHITE),
                    ..Default::default()
                }),
                pressed: Box::new(|_focused, _theme| widget::button::Style {
                    text_color: Some(Color::WHITE),
                    icon_color: Some(Color::WHITE),
                    ..Default::default()
                }),
            })
            .on_press(Message::NavigateTo(PopupRoute::ManageAccounts(provider_id)));

        let row = container(
            row![
                icon,
                name,
                cosmic::iced::widget::Space::new().width(Length::Fill),
                toggle,
                configure_btn,
            ]
            .spacing(10)
            .align_y(Alignment::Center)
            .width(Length::Fill)
        )
        .padding([8, 10])
        .width(Length::Fill)
        .style(|_| widget::container::Style {
            background: Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.02))),
            border: cosmic::iced::Border {
                radius: 2.0.into(),
                width: 1.0,
                color: Color::from_rgba(1.0, 1.0, 1.0, 0.08),
            },
            ..Default::default()
        });

        provider_rows = provider_rows.push(row);
    }

    let divider = container(cosmic::iced::widget::Space::new().height(Length::Fixed(1.0)))
        .width(Length::Fill)
        .style(|_| widget::container::Style {
            background: Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.08))),
            ..Default::default()
        });

    let about_label = container(widget::text("ABOUT").size(11))
        .style(|_| widget::container::Style {
            text_color: Some(Color::from_rgba(1.0, 1.0, 1.0, 0.45)),
            ..Default::default()
        });

    let about_content = about::about_view(update_status);

    cosmic::iced::widget::column![
        providers_label,
        provider_rows,
        divider,
        about_label,
        about_content,
    ]
    .spacing(14)
    .width(Length::Fill)
    .into()
}

#[allow(dead_code)]
pub(super) fn general_settings_view<'a>(config: &'a Config) -> Element<'a, Message> {
    general::general_settings_view(config)
}

pub(super) fn manage_providers_view(state: &AppState) -> Element<'static, Message> {
    let providers = ProviderId::ALL;
    let mut rows = cosmic::iced::widget::column![].width(Length::Fill);
    for (index, provider_id) in providers.into_iter().enumerate() {
        let enabled = state
            .provider(provider_id)
            .is_some_and(|provider| provider.enabled);
        rows = rows.push(manage_provider_row(provider_id, enabled));
        if index + 1 < providers.len() {
            rows = rows.push(manage_provider_divider());
        }
    }
    Element::from(
        cosmic::iced::widget::column![
            cosmic::widget::text(fl!("manage-providers")).size(20),
            container(rows)
                .width(Length::Fill)
                .style(manage_provider_list_style),
        ]
        .spacing(16)
        .width(Length::Fill),
    )
}

fn manage_provider_row(provider: ProviderId, enabled: bool) -> Element<'static, Message> {
    container(
        cosmic::iced::widget::row![
            widget::icon::icon(provider_icon_handle(provider, provider_icon_variant())).size(20),
            cosmic::widget::text(provider.label()).size(16),
            cosmic::iced::widget::Space::new().width(Length::Fill),
            cosmic::widget::toggler(enabled)
                .on_toggle(move |enabled| Message::SetProviderEnabled(provider, enabled)),
        ]
        .spacing(10)
        .align_y(Alignment::Center)
        .width(Length::Fill),
    )
    .padding([12, 12])
    .width(Length::Fill)
    .into()
}

fn manage_provider_divider() -> Element<'static, Message> {
    container(cosmic::iced::widget::Space::new().height(Length::Fixed(1.0)))
        .width(Length::Fill)
        .style(|theme: &cosmic::Theme| widget::container::Style {
            text_color: None,
            background: Some(Background::Color(component_divider_color(theme))),
            border: cosmic::iced::Border::default(),
            shadow: cosmic::iced::Shadow::default(),
            icon_color: None,
            snap: true,
        })
        .into()
}

fn manage_provider_list_style(theme: &cosmic::Theme) -> widget::container::Style {
    component_container_style(theme)
}

pub(super) fn about_view(update_status: &UpdateStatus) -> Element<'static, Message> {
    about::about_view(update_status)
}

pub(super) fn provider_settings_view<'a>(
    state: &'a AppState,
    config: &'a Config,
    detection: &'a DetectionSnapshot,
    logins: ProviderLoginStates<'a>,
    provider_id: ProviderId,
) -> Element<'a, Message> {
    accounts::provider_settings_view(state, config, detection, logins, provider_id)
}
