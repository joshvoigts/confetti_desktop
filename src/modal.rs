use crate::control::ModalEvent;
use bevy::app::AppExit;
use bevy::prelude::*;

#[derive(Component)]
pub struct Modal;

#[derive(Component)]
pub struct QuitButton;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

pub fn handle_modal_button(
   mut interactions: Query<
      (&Interaction, &mut BackgroundColor),
      (Changed<Interaction>, With<QuitButton>),
   >,
   mut exit: EventWriter<AppExit>,
) {
   for (interaction, mut color) in interactions.iter_mut() {
      match *interaction {
         Interaction::Pressed => {
            *color = PRESSED_BUTTON.into();
            exit.send(AppExit::Success);
         }
         Interaction::Hovered => {
            *color = HOVERED_BUTTON.into();
         }
         Interaction::None => {
            *color = NORMAL_BUTTON.into();
         }
      }
   }
}

pub fn handle_modal(
   modals: Query<&Modal>,
   mut commands: Commands,
   mut event: EventReader<ModalEvent>,
) {
   if !event.is_empty() && modals.is_empty() {
      event.clear();
      commands
         .spawn(NodeBundle {
            style: Style {
               position_type: PositionType::Absolute,
               display: Display::Flex,
               flex_direction: FlexDirection::Row,
               justify_content: JustifyContent::Center,
               align_items: AlignItems::Center,
               width: Val::Percent(100.),
               height: Val::Percent(100.),
               ..default()
            },
            z_index: ZIndex::Global(99),
            ..default()
         })
         .with_children(|parent| {
            parent
               .spawn(NodeBundle {
                  style: Style {
                     display: Display::Grid,
                     justify_items: JustifyItems::Center,
                     align_items: AlignItems::Center,
                     padding: UiRect::all(Val::Px(45.))
                        .with_top(Val::Px(35.))
                        .with_bottom(Val::Px(35.)),
                     ..default()
                  },
                  background_color: Color::srgb(0.35, 0.4, 0.6)
                     .into(),
                  ..default()
               })
               .with_children(|parent| {
                  parent.spawn(
                     TextBundle::from_section(
                        "Confetti Desktop",
                        TextStyle {
                           font_size: 30.0,
                           ..default()
                        },
                     )
                     .with_style(Style {
                        margin: UiRect::ZERO
                           .with_bottom(Val::Px(30.)),
                        ..default()
                     }),
                  );
                  parent.spawn(TextBundle::from_section(
                     "A harmless prank app",
                     TextStyle {
                        font_size: 15.0,
                        ..default()
                     },
                  ));
                  parent.spawn(TextBundle::from_section(
                     "Press space to clear",
                     TextStyle {
                        font_size: 15.0,
                        ..default()
                     },
                  ));
                  parent
                     .spawn(ButtonBundle {
                        style: Style {
                           margin: UiRect::ZERO
                              .with_top(Val::Px(30.0)),
                           padding: UiRect::all(Val::Px(10.)),
                           ..default()
                        },
                        background_color: Color::srgb(0.0, 0.0, 0.0)
                           .into(),
                        ..default()
                     })
                     .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                           "Quit",
                           TextStyle {
                              font_size: 15.0,
                              ..default()
                           },
                        ));
                     })
                     .insert(QuitButton);
               });
         })
         .insert(Modal);
   }
}
