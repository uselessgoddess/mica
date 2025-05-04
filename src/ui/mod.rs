use {
  crate::prelude::*,
  bevy::window::SystemCursorIcon,
  std::{hash::BuildHasher, time::Duration},
};

pub fn plugin(app: &mut App) {
  app.add_systems(OnEnter(Pause::Pause), spawn).add_systems(
    Update,
    component_animator_system::<Text2d>
      .in_set(AnimationSystem::AnimationUpdate),
  );
}

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
        ui.spawn((
          UiLayout::solid()
            .size((1920.0, 1080.0))
            .scaling(Scaling::Fill)
            .pack(),
          Sprite::from_image(assets.load("images/ui/background.png")),
        ));

        ui.spawn((UiLayout::window()
          .pos(Rl((22.0, 33.0)))
          .size(Rl((55.0, 34.0)))
          .pack(),))
          .with_children(ui_fn);
      })
      .id();
    commands.entity(entity).add_child(root);
  }
}

const BLUE: Color = Color::srgb(8.0 / 255.0, 226.0 / 255.0, 252.0 / 255.0);
const RED: Color = Color::srgb(1.0, 98.0 / 255.0, 81.0 / 255.0);
const RED_DIM: Color = Color::srgb(172.0 / 255.0, 64.0 / 255.0, 63.0 / 255.0);

struct TextLens {
  text: String,
  anim: fn(&str, f32) -> String,
}

impl Default for TextLens {
  fn default() -> Self {
    Self { text: String::new(), anim: typing }
  }
}

impl TextLens {
  pub fn new(text: impl ToString) -> Self {
    Self { text: text.to_string(), ..default() }
  }

  pub fn anim(mut self, anim: fn(&str, f32) -> String) -> Self {
    self.anim = anim;
    self
  }
}

impl Lens<Text2d> for TextLens {
  fn lerp(&mut self, target: &mut dyn Targetable<Text2d>, ratio: f32) {
    target.0 = (self.anim)(&self.text, ratio);
  }
}

/// Simulates typing animation with an underscore cursor
fn typing(text: &str, ratio: f32) -> String {
  let visible = (ratio * text.len() as f32).floor() as usize;
  let visible = visible.min(text.len());

  if visible < text.len() {
    format!("{}{}", &text[..visible], "_")
  } else {
    text.to_string()
  }
}

/// Creates a decryption effect where random symbols gradually become the actual text
pub fn decryption(text: &str, ratio: f32) -> String {
  use {rand::prelude::*, std::hash::RandomState};

  let symbols = "!@#$%^&*()_+-=[]{}|;:'\",.<>/?`~";

  // Create unique reproducible RNG from time
  let mut rng = StdRng::seed_from_u64(
    RandomState::new().hash_one(text) + (ratio * 60.0).round() as u64,
  );

  let mut result = String::with_capacity(text.len());

  for (i, c) in text.chars().enumerate() {
    let char_progress = (ratio * text.len() as f32) - i as f32;

    if char_progress < 0.0 {
      // Not yet started decrypting this character
      result
        .push(symbols.chars().nth(rng.random_range(0..symbols.len())).unwrap());
    } else if char_progress >= 1.0 {
      // This character is fully decrypted
      result.push(c);
    } else {
      // This character is in the process of being decrypted
      // 80% chance of showing the real character as we get closer to 1.0
      if rng.random::<f32>() < char_progress {
        result.push(c);
      } else {
        result.push(
          symbols.chars().nth(rng.random_range(0..symbols.len())).unwrap(),
        );
      }
    }
  }

  result
}

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
              TextLens::new(button.to_ascii_uppercase()).anim(decryption),
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
