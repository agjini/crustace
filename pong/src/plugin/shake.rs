use bevy::math::EulerRot;
use bevy::prelude::{
    App, Component, IntoSystemConfigs, Plugin, PostUpdate, PreUpdate, Quat, Query, Reflect, Res,
    Time, Transform, Vec3,
};
use bevy::transform::systems::{propagate_transforms, sync_simple_transforms};
use noise::{NoiseFn, Perlin};

pub(crate) struct ShakePlugin;

impl Plugin for ShakePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Shake>()
            .add_systems(PreUpdate, restore)
            .add_systems(
                PostUpdate,
                shake
                    .before(propagate_transforms)
                    .before(sync_simple_transforms),
            );
    }
}

#[derive(Component, Reflect)]
pub struct Shake {
    time: f32,
    amplitude: f32,
    frequency: f32,
    original_transform: Option<Transform>,
}

impl Shake {
    pub fn new(time: f32, amplitude: f32, frequency: f32) -> Self {
        Self {
            time,
            amplitude,
            frequency,
            original_transform: None,
        }
    }

    pub fn add_time(&mut self, seconds: f32) {
        self.time += seconds;
    }
}

fn restore(mut shakes: Query<(&mut Shake, &mut Transform)>) {
    for (mut shake, mut transform) in &mut shakes {
        if shake.original_transform.is_some() {
            let translation = shake.original_transform.take().unwrap();
            *transform = translation;
        }
    }
}

const MAX_YAW: f32 = 0.01;
const MAX_PITCH: f32 = 0.01;
const MAX_ROLL: f32 = 0.01;

fn shake(time: Res<Time>, mut query: Query<(&mut Shake, &mut Transform)>) {
    let perlin1 = Perlin::new(1);
    let perlin2 = Perlin::new(2);
    let perlin3 = Perlin::new(3);

    for (mut shake, mut transform) in query.iter_mut() {
        shake.time -= time.delta_seconds();
        shake.time = shake.time.max(0.0);
        if shake.time == 0.0 {
            continue;
        }
        shake.original_transform = Some(transform.clone());

        let perlin_param = [(shake.time * shake.frequency) as f64];
        let rotation = Vec3::new(
            MAX_YAW * shake.amplitude * perlin1.get(perlin_param) as f32,
            MAX_PITCH * shake.amplitude * perlin2.get(perlin_param) as f32,
            MAX_ROLL * shake.amplitude * perlin3.get(perlin_param) as f32,
        );
        transform.rotation *= Quat::from_euler(EulerRot::YXZ, rotation.y, rotation.x, rotation.z);
    }
}
