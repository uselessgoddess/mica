mod hud;

use {crate::prelude::*, bevy::window::SystemCursorIcon, std::time::Duration};

pub fn plugin(app: &mut App) {
  app.add_plugins(hud::plugin).add_systems(OnEnter(Pause::Pause), spawn);
}

pub const DEPTH: f32 = 128.0;

fn spawn(
  query: Query<Entity, With<UiSourceCamera<0>>>,
  mut commands: Commands,
  assets: Res<AssetServer>,
) {
  for entity in query.iter() {
    let root = commands
      .spawn(StateScoped(Pause::Pause))
      .insert((UiLayoutRoot::new_2d(), UiFetchFromCamera::<0>))
      .with_children(|ui| {
        ui.spawn((UiLayout::solid().pack(), UiDepth::Add(DEPTH)))
          .with_children(|ui| {
            ui.spawn((
              Sprite::from_image(assets.load("images/ui/background.png")),
              UiLayout::solid()
                .size((1920.0, 1080.0))
                .scaling(Scaling::Fill)
                .pack(),
            ));

            ui.spawn(
              UiLayout::window()
                .pos(Rl((22.0, 33.0)))
                .size(Rl((55.0, 34.0)))
                .pack(),
            )
            .with_children(ui_fn);
          });
      })
      .id();
    commands.entity(entity).add_child(root);
  }
}

const BLUE: Color = Color::srgb(8.0 / 255.0, 226.0 / 255.0, 252.0 / 255.0);
const RED: Color = Color::srgb(1.0, 98.0 / 255.0, 81.0 / 255.0);
const RED_DIM: Color = Color::srgb(172.0 / 255.0, 64.0 / 255.0, 63.0 / 255.0);

fn ui_fn(ui: &mut ChildBuilder) {
  let gap = 3.0;
  let size = 14.0;
  let mut offset = 0.0;
  for button in
    ["Continue", "New Game", "Load Game", "Settings", "Credits", "Quit Game"]
  {
    let mut entity = ui.spawn((
      Name::new(button),
      UiLayout::window().y(Rl(offset)).size(Rl((100.0, size))).pack(),
      Visibility::Visible,
      OnHoverSetCursor::new(SystemCursorIcon::Pointer),
    ));
    entity
      .with_children(|ui| {
        ui.spawn((
          UiLayout::new(vec![
            (UiBase::id(), UiLayout::window().full()),
            (UiHover::id(), UiLayout::window().x(Rl(10.0)).full()),
          ]),
          UiHover::new().forward_speed(20.0).backward_speed(4.0),
          UiColor::new(vec![(UiBase::id(), RED_DIM), (UiHover::id(), RED)]),
          PickingBehavior::IGNORE,
        ))
        .with_children(|ui| {
          ui.spawn((
            UiLayout::window()
              .pos((Rh(40.0), Rl(50.0)))
              .anchor(Anchor::CenterLeft)
              .pack(),
            UiColor::new(vec![(UiBase::id(), RED_DIM), (UiHover::id(), RED)]),
            UiHover::new().forward_speed(20.0).backward_speed(4.0),
            UiTextSize::from(Rh(60.0)),
            Text2d::default(),
            Animator::new(Tween::new(
              EaseFunction::Linear,
              Duration::from_secs_f32(0.5),
              TextLens::new(button.to_ascii_uppercase())
                .animation(decryption_animation),
            )),
            TextFont { font_size: 64.0, ..default() },
            PickingBehavior::IGNORE,
          ));

          ui.spawn((
            UiLayout::window()
              .pos(Rl((90.0, 50.0)))
              .anchor(Anchor::CenterRight)
              .pack(),
            UiColor::new(vec![(UiBase::id(), BLUE), (UiHover::id(), RED)]),
            UiHover::new().forward_speed(20.0).backward_speed(4.0),
            UiTextSize::from(Rh(60.0)),
            Text2d::new("<-"),
            TextFont { font_size: 64.0, ..default() },
            PickingBehavior::IGNORE,
          ));
        });
      })
      .observe(hover_set::<Pointer<Over>, true>)
      .observe(hover_set::<Pointer<Out>, false>);

    offset += gap + size;
  }
}
