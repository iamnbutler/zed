use gpui2::rgba;

use crate::{PlayerTheme, SyntaxTheme, Theme, ThemeMetadata};

pub fn atelier_lakeside_light() -> Theme {
    Theme {
        metadata: ThemeMetadata {
            name: "Atelier Lakeside Light".into(),
            is_light: true,
        },
        transparent: rgba(0x00000000).into(),
        mac_os_traffic_light_red: rgba(0xec695eff).into(),
        mac_os_traffic_light_yellow: rgba(0xf4bf4eff).into(),
        mac_os_traffic_light_green: rgba(0x61c553ff).into(),
        border: rgba(0x80a4b6ff).into(),
        border_variant: rgba(0x80a4b6ff).into(),
        border_focused: rgba(0xb9cee0ff).into(),
        border_transparent: rgba(0x00000000).into(),
        elevated_surface: rgba(0xa6cadcff).into(),
        surface: rgba(0xcdeaf9ff).into(),
        background: rgba(0xa6cadcff).into(),
        filled_element: rgba(0xa6cadcff).into(),
        filled_element_hover: rgba(0xffffff1e).into(),
        filled_element_active: rgba(0xffffff28).into(),
        filled_element_selected: rgba(0xd8e4eeff).into(),
        filled_element_disabled: rgba(0x00000000).into(),
        ghost_element: rgba(0x00000000).into(),
        ghost_element_hover: rgba(0xffffff14).into(),
        ghost_element_active: rgba(0xffffff1e).into(),
        ghost_element_selected: rgba(0xd8e4eeff).into(),
        ghost_element_disabled: rgba(0x00000000).into(),
        text: rgba(0x161b1dff).into(),
        text_muted: rgba(0x526f7dff).into(),
        text_placeholder: rgba(0xd22e71ff).into(),
        text_disabled: rgba(0x628496ff).into(),
        text_accent: rgba(0x267eadff).into(),
        icon_muted: rgba(0x526f7dff).into(),
        syntax: SyntaxTheme {
            highlights: vec![
                ("emphasis".into(), rgba(0x267eadff).into()),
                ("number".into(), rgba(0x935c24ff).into()),
                ("embedded".into(), rgba(0x161b1dff).into()),
                ("link_text".into(), rgba(0x935c25ff).into()),
                ("string".into(), rgba(0x558c3aff).into()),
                ("constructor".into(), rgba(0x267eadff).into()),
                ("punctuation.list_marker".into(), rgba(0x1f292eff).into()),
                ("string.special".into(), rgba(0xb72cd2ff).into()),
                ("title".into(), rgba(0x161b1dff).into()),
                ("variant".into(), rgba(0x8a8a0eff).into()),
                ("tag".into(), rgba(0x267eadff).into()),
                ("attribute".into(), rgba(0x267eadff).into()),
                ("keyword".into(), rgba(0x6a6ab7ff).into()),
                ("enum".into(), rgba(0x935c25ff).into()),
                ("function".into(), rgba(0x247eadff).into()),
                ("string.escape".into(), rgba(0x516d7bff).into()),
                ("operator".into(), rgba(0x516d7bff).into()),
                ("function.method".into(), rgba(0x247eadff).into()),
                (
                    "function.special.definition".into(),
                    rgba(0x8a8a0eff).into(),
                ),
                ("punctuation.delimiter".into(), rgba(0x516d7bff).into()),
                ("comment".into(), rgba(0x7094a7ff).into()),
                ("primary".into(), rgba(0x1f292eff).into()),
                ("punctuation.bracket".into(), rgba(0x516d7bff).into()),
                ("variable".into(), rgba(0x1f292eff).into()),
                ("emphasis.strong".into(), rgba(0x267eadff).into()),
                ("predictive".into(), rgba(0x6a97b2ff).into()),
                ("punctuation.special".into(), rgba(0xb72cd2ff).into()),
                ("hint".into(), rgba(0x5a87a0ff).into()),
                ("text.literal".into(), rgba(0x935c25ff).into()),
                ("string.special.symbol".into(), rgba(0x558c3aff).into()),
                ("comment.doc".into(), rgba(0x516d7bff).into()),
                ("constant".into(), rgba(0x568c3bff).into()),
                ("boolean".into(), rgba(0x568c3bff).into()),
                ("preproc".into(), rgba(0x161b1dff).into()),
                ("variable.special".into(), rgba(0x6a6ab7ff).into()),
                ("link_uri".into(), rgba(0x568c3bff).into()),
                ("string.regex".into(), rgba(0x2c8f6eff).into()),
                ("punctuation".into(), rgba(0x1f292eff).into()),
                ("property".into(), rgba(0xd22c72ff).into()),
                ("label".into(), rgba(0x267eadff).into()),
                ("type".into(), rgba(0x8a8a0eff).into()),
            ],
        },
        status_bar: rgba(0xa6cadcff).into(),
        title_bar: rgba(0xa6cadcff).into(),
        toolbar: rgba(0xebf8ffff).into(),
        tab_bar: rgba(0xcdeaf9ff).into(),
        editor: rgba(0xebf8ffff).into(),
        editor_subheader: rgba(0xcdeaf9ff).into(),
        editor_active_line: rgba(0xcdeaf9ff).into(),
        terminal: rgba(0xebf8ffff).into(),
        image_fallback_background: rgba(0xa6cadcff).into(),
        git_created: rgba(0x568c3bff).into(),
        git_modified: rgba(0x267eadff).into(),
        git_deleted: rgba(0xd22e71ff).into(),
        git_conflict: rgba(0x8a8a10ff).into(),
        git_ignored: rgba(0x628496ff).into(),
        git_renamed: rgba(0x8a8a10ff).into(),
        players: [
            PlayerTheme {
                cursor: rgba(0x267eadff).into(),
                selection: rgba(0x267ead3d).into(),
            },
            PlayerTheme {
                cursor: rgba(0x568c3bff).into(),
                selection: rgba(0x568c3b3d).into(),
            },
            PlayerTheme {
                cursor: rgba(0xb72ed2ff).into(),
                selection: rgba(0xb72ed23d).into(),
            },
            PlayerTheme {
                cursor: rgba(0x935c25ff).into(),
                selection: rgba(0x935c253d).into(),
            },
            PlayerTheme {
                cursor: rgba(0x6c6ab7ff).into(),
                selection: rgba(0x6c6ab73d).into(),
            },
            PlayerTheme {
                cursor: rgba(0x2e8f6eff).into(),
                selection: rgba(0x2e8f6e3d).into(),
            },
            PlayerTheme {
                cursor: rgba(0xd22e71ff).into(),
                selection: rgba(0xd22e713d).into(),
            },
            PlayerTheme {
                cursor: rgba(0x8a8a10ff).into(),
                selection: rgba(0x8a8a103d).into(),
            },
        ],
    }
}