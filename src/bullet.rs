use crate::*;
// use bevy::prelude::*;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Bullet {
    pub speed: f32,
    pub direction: Vec3,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Lifetime {
    pub timer: Timer,
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(lifetime_despawn)
            .add_system(move_bullets)
            .add_system(bullet_collision)
            .register_type::<Lifetime>()
            .register_type::<Bullet>();
    }
}

fn move_bullets(mut bullets: Query<(&Bullet, &mut Transform)>, time: Res<Time>) {
    for (bullet, mut transform) in &mut bullets {
        transform.translation += bullet.direction.normalize() * bullet.speed * time.delta_seconds();
    }
}

fn bullet_collision(
    mut commands: Commands,
    bullets: Query<(Entity, &GlobalTransform), With<Bullet>>,
    mut targets: Query<(&mut Health, &GlobalTransform), With<Target>>,
) {
    for (bullet, bullet_transform) in &bullets {
        for (mut target_health, target_transform) in &mut targets {
            if Vec3::distance(
                bullet_transform.translation(),
                target_transform.translation(),
            ) < 0.2
            {
                commands.entity(bullet).despawn_recursive();
                target_health.value -= 1;
                break;
            }
        }
    }
}

fn lifetime_despawn(
    mut commands: Commands,
    mut entities: Query<(Entity, &mut Lifetime)>,
    time: Res<Time>,
) {
    for (entity, mut lifetime) in &mut entities {
        if lifetime.timer.tick(time.delta()).just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
