use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(PartialEq, Eq, Clone, Copy, Component)]
enum CustomFilterTag {
    GroupA,
    GroupB,
}

// A custom filter that allows contacts only between rigid-bodies with the
// same user_data value.
// Note that using collision groups would be a more efficient way of doing
// this, but we use custom filters instead for demonstration purpose.
struct SameUserDataFilter;
impl<'a> PhysicsHooksWithQuery<&'a CustomFilterTag> for SameUserDataFilter {
    fn filter_contact_pair(
        &self,
        context: PairFilterContextView,
        tags: &Query<&'a CustomFilterTag>,
    ) -> Option<SolverFlags> {
        if tags.get(context.collider1()).ok().copied()
            == tags.get(context.collider2()).ok().copied()
        {
            Some(SolverFlags::COMPUTE_IMPULSES)
        } else {
            None
        }
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(
            0xF9 as f32 / 255.0,
            0xF9 as f32 / 255.0,
            0xFF as f32 / 255.0,
        )))
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<&CustomFilterTag>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_physics)
        .run();
}

fn setup_graphics(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-30.0, 30.0, 100.0)
            .looking_at(DVec3::new(0.0, 10.0, 0.0), DVec3::Y),
        ..Default::default()
    });
}

pub fn setup_physics(mut commands: Commands) {
    /*
     * Ground
     */
    commands.insert_resource(PhysicsHooksWithQueryResource(Box::new(
        SameUserDataFilter {},
    )));

    let ground_size = 10.0;

    commands.spawn((
        TransformBundle::from(Transform::from_xyz(0.0, -10.0, 0.0)),
        Collider::cuboid(ground_size, 1.2, ground_size),
        CustomFilterTag::GroupA,
    ));

    commands.spawn((
        TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)),
        Collider::cuboid(ground_size, 1.2, ground_size),
        CustomFilterTag::GroupB,
    ));

    /*
     * Create the cubes
     */
    let num = 4;
    let rad = 0.5;

    let shift = rad * 2.0;
    let centerx = shift * (num as f64) / 2.0;
    let centery = shift / 2.0;
    let mut group_id = 0;
    let tags = [CustomFilterTag::GroupA, CustomFilterTag::GroupB];
    let colors = [Color::hsl(220.0, 1.0, 0.3), Color::hsl(260.0, 1.0, 0.7)];

    for i in 0..num {
        for j in 0usize..num * 5 {
            let x = (i as f64 + j as f64 * 0.2) * shift - centerx;
            let y = j as f64 * shift + centery + 2.0;
            group_id += 1;

            commands.spawn((
                TransformBundle::from(Transform::from_xyz(x, y, 0.0)),
                RigidBody::Dynamic,
                Collider::cuboid(rad, rad, rad),
                ActiveHooks::FILTER_CONTACT_PAIRS,
                tags[group_id % 2],
                ColliderDebugColor(colors[group_id % 2]),
            ));
        }
    }
}
