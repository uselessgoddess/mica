use crate::prelude::*;

pub fn plugin(app: &mut App) {
  app.add_systems(Update, spawn);
}

fn spawn(
  query: Query<Entity, Added<UiSourceCamera<0>>>,
  mut commands: Commands,
) {
  for entity in query.iter() {
    let root = commands
      .spawn(StateScoped(Game::Gameplay))
      .insert((UiLayoutRoot::new_2d(), UiFetchFromCamera::<0>))
      .with_children(|ui| {
        ui.spawn(UiLayout::window().full().pack()).with_children(|ui| {
          ui.spawn((
            Text2d::new("HUD"),
            UiLayout::window()
              .pos((Rh(10.0), Rl(10.0)))
              .anchor(Anchor::Center)
              .pack(),
            UiTextSize::from(Rh(10.0)),
          ));
        });
      })
      .id();
    commands.entity(entity).add_child(root);
  }
}
