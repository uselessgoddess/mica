use {crate::prelude::*, std::hash::BuildHasher};

pub fn plugin(app: &mut App) {
  app.add_systems(
    Update,
    component_animator_system::<Text2d>
      .in_set(AnimationSystem::AnimationUpdate),
  );
}

pub struct TextLens {
  text: String,
  anim: fn(&str, f32) -> String,
}

impl Default for TextLens {
  fn default() -> Self {
    Self { text: String::new(), anim: typing_animation }
  }
}

impl TextLens {
  pub fn new(text: impl ToString) -> Self {
    Self { text: text.to_string(), ..default() }
  }

  pub fn animation(mut self, anim: fn(&str, f32) -> String) -> Self {
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
pub fn typing_animation(text: &str, ratio: f32) -> String {
  let visible = (ratio * text.len() as f32).floor() as usize;
  let visible = visible.min(text.len());

  if visible < text.len() {
    format!("{}{}", &text[..visible], "_")
  } else {
    text.to_string()
  }
}

/// Creates a decryption effect where random symbols gradually become the actual text
pub fn decryption_animation(text: &str, ratio: f32) -> String {
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
