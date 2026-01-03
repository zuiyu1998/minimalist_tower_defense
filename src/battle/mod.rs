mod bullet;

pub use bullet::*;

use bevy::prelude::*;

use crate::skill::{
    FromSkill, Skill, SkillResponse, SkillRunContext, SkillSystems, process_skill_effct,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Last,
        process_skill_effct::<BulletSystemParam>.in_set(SkillSystems::Update),
    );
}

pub fn execute_skill(
    commands: &mut Commands,
    skill: &Skill,
    caster: Entity,
    targets: Vec<Entity>,
    source: Option<Entity>,
) {
    for target in targets.iter() {
        let bullet_skill_effect = BulletSkillEffect::from_skill(skill);

        commands.spawn((
            bullet_skill_effect,
            SkillRunContext {
                source: source.clone(),
                payload: Default::default(),
                caster,
                target: *target,
            },
            SkillResponse::empty(),
        ));
    }
}
