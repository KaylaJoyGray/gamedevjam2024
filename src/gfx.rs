use bevy::{prelude::*, render::camera::ScalingMode::WindowSize, window::PrimaryWindow};
use std::collections::HashMap;
use std::time::Duration;

pub struct GFXPlugin;

impl Plugin for GFXPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(
                FixedUpdate,
                (
                    update_animations,
                    update_sprite_scaling,
                ),
            );
    }
}

/// Important: this is the sprite size before window scaling is applied
pub const SPRITE_SIZE: f32 = 1.0;

///
/// update_sprite_scaling: Bevy system
///
/// Updates sprite scaling for each Sprite if the window changes
pub fn update_sprite_scaling(
    mut sprites_query: Query<&mut Sprite>,
    window_query: Query<&Window, (With<PrimaryWindow>, Changed<Window>)>,
) {
    if window_query.is_empty() {
        return;
    }
    sprites_query.iter_mut().for_each(|mut sprite| {
        sprite.custom_size = Some(Vec2::new(
            SPRITE_SIZE * window_query.single().scale_factor(),
            SPRITE_SIZE * window_query.single().scale_factor(),
        ));
    });
}

///
/// AnimationType
///
/// * Once: plays once and stops on the last frame
/// * Repeat: loops indefinitely
/// * Despawn: despawns the entity on completion
#[derive(Debug, Clone, PartialEq)]
pub enum AnimationType {
    Once,
    Repeat,
    Despawn,
}

#[derive(Component)]
pub struct Animation {
    index: usize,
    atlas: Handle<TextureAtlas>,
    frames: Vec<usize>,
    timer: Timer,
    animation_type: AnimationType,
    finished: bool,
}

impl Animation {
    pub fn new(
        atlas: Handle<TextureAtlas>,
        frames: Vec<usize>,
        frame_time: f32,
        animation_type: AnimationType,
    ) -> Self {
        Animation {
            index: 0,
            atlas,
            frames,
            timer: Timer::from_seconds(frame_time, TimerMode::Once),
            animation_type,
            finished: false,
        }
    }

    fn advance_frame(&mut self) {
        if self.animation_type.eq(&AnimationType::Repeat) {
            self.index = (self.index + 1) % self.frames.len();
            self.timer.reset();
            return;
        }

        // non-repeating animation
        if self.index < self.frames.len() - 1 {
            self.index += 1;
            self.timer.reset();
        } else {
            self.finished = true;
        }
    }

    /// Advances the timer and returns the index of the current frame
    pub fn tick(&mut self, delta: f32) -> usize {
        self.timer.tick(Duration::from_secs_f32(delta));
        if self.timer.finished() {
            self.advance_frame();
        }
        self.frames[self.index].clone()
    }

    pub fn get_type(&self) -> AnimationType {
        self.animation_type.clone()
    }

    pub fn finished(&self) -> bool {
        self.finished
    }
}

#[derive(Resource)]
pub struct AnimationResource {
    map: HashMap<String, Animation>,
}

impl AnimationResource {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    /// Insert a new Animation
    pub fn insert(&mut self, name: String, animation: Animation) {
        self.map.insert(name, animation.clone());
    }

    /// Get an Animation
    pub fn get(&self, name: &str) -> Option<Animation> {
        self.map.get(name).cloned()
    }
}

pub fn update_animations(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut TextureAtlas, &mut Animation)>,
) {
    for (entity, mut sprite, mut animation) in query.iter_mut() {
        let next_index = animation.tick(time.delta_seconds());
        if next_index.ne(&sprite.index) {
            sprite.index = next_index;
        }

        if animation.finished() {
            match animation.get_type() {
                AnimationType::Once => {
                    commands.entity(entity).remove::<Animation>();
                }
                AnimationType::Despawn => {
                    commands.entity(entity).despawn();
                }
                _ => {}
            }
        }
    }
}

#[derive(Debug, Component)]
pub struct MainCamera {}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        MainCamera {},
        Camera2dBundle {
            projection: OrthographicProjection {
                near: -1000.0,
                far: 1000.0,
                scaling_mode: WindowSize(16.0), // 16 pixels per game unit
                ..default()
            },
            camera: Camera {
                clear_color: ClearColorConfig::from(Color::rgb(0.0, 0.0, 0.0)),
                ..default()
            },
            ..default()
        },
    ));
}
