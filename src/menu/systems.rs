use bevy::prelude::*;

use crate::GameState;
use crate::MenuData;

const NORMAL_BUTTON_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);
const HOVERED_BUTTON_COLOR: Color = Color::srgb(0.7, 0.7, 0.7);
const PRESSED_BUTTON_COLOR: Color = Color::srgb(0.3, 0.3, 0.3);

pub fn menu(
   mut next_state: ResMut<NextState<GameState>>,
   mut interaction_query: Query<
      (&Interaction, &mut BackgroundColor),
      (Changed<Interaction>, With<Button>),
   >,
) {
   for (inter, mut color) in &mut interaction_query {
      match *inter {
         Interaction::Pressed => {
            *color = PRESSED_BUTTON_COLOR.into();
            next_state.set(GameState::InGame);
         }
         Interaction::Hovered => {
            *color = HOVERED_BUTTON_COLOR.into();
         }
         Interaction::None => {
            *color = NORMAL_BUTTON_COLOR.into();
         }
      }
   }
}

pub fn setting_button() {}

pub fn setup_menu(mut cmd: Commands) {
   // let play_button_entity = cmd
   //    .spawn(Node {
   //       // center button
   //       width: Val::Percent(100.0),
   //       height: Val::Percent(100.0),
   //       justify_content: JustifyContent::Center,
   //       align_items: AlignItems::Center,
   //       ..Default::default()
   //    })
   //    .with_children(|parent| {
   //       parent
   //          .spawn((
   //             Button,
   //             Node {
   //                width: Val::Px(150.0),
   //                height: Val::Px(65.0),
   //                justify_content: JustifyContent::Center,
   //                align_items: AlignItems::Center,
   //                ..Default::default()
   //             },
   //             BackgroundColor(NORMAL_BUTTON_COLOR),
   //          ))
   //          .with_children(|parent| {
   //             parent.spawn((
   //                Text::new("Play"),
   //                TextFont { font_size: 30.0, ..Default::default() },
   //                TextColor(Color::WHITE),
   //             ));
   //          });
   //    })
   //    .id();
   let menu_horizontal_container_e = cmd
      .spawn(Node {
         // root container
         width: Val::Percent(100.0),
         height: Val::Percent(100.0),
         justify_content: JustifyContent::Center,
         align_items: AlignItems::Center,
         ..Default::default()
      })
      .with_children(|parent| {
         parent
            .spawn(
               (Node {
                  // menu container
                  width: Val::Percent(100.0),
                  height: Val::Percent(100.0),
                  justify_content: JustifyContent::Center,
                  align_items: AlignItems::Center,
                  flex_direction: FlexDirection::Column,
                  ..Default::default()
               }),
            )
            .with_children(|parent| {
               parent
                  .spawn((
                     Button,
                     Node {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                     },
                     BackgroundColor(NORMAL_BUTTON_COLOR),
                  ))
                  .with_children(|parent| {
                     parent.spawn((
                        Text::new("Play"),
                        TextFont { font_size: 30.0, ..Default::default() },
                        TextColor(Color::WHITE),
                     ));
                  });
               parent
                  .spawn((
                     Button,
                     Node {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                     },
                     BackgroundColor(NORMAL_BUTTON_COLOR),
                  ))
                  .with_children(|parent| {
                     parent.spawn((
                        Text::new("Toggle Fullscreen"),
                        TextFont { font_size: 30.0, ..Default::default() },
                        TextColor(Color::WHITE),
                     ));
                  });
               parent
                  .spawn((
                     Button,
                     Node {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                     },
                     BackgroundColor(NORMAL_BUTTON_COLOR),
                  ))
                  .with_children(|parent| {
                     parent.spawn((
                        Text::new("Exit"),
                        TextFont { font_size: 30.0, ..Default::default() },
                        TextColor(Color::WHITE),
                     ));
                  });
            });
      })
      .id();
   cmd.insert_resource(MenuData { menu_entity: menu_horizontal_container_e });
}

pub fn cleanup_menu(
   mut cmd: Commands,
   menu_data: Res<MenuData>,
) {
   cmd.entity(menu_data.menu_entity).despawn_recursive();
}
