mod bullet;

pub use bullet::*;

use bevy::prelude::*;

use crate::skill::{
    FromSkill, Skill, SkillResponse, SkillRunContext, SkillRunContextData, SkillSystems,
    process_skill_effct,
};

fn process_bullet_skill_effct_system(
    mut processor: BulletSystemParam,
    skill_effct_q: Query<(&BulletSkillEffect, &mut SkillRunContext, &mut SkillResponse)>,
) {
    process_skill_effct::<BulletSystemParam>(&mut processor, skill_effct_q);
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Last,
        process_bullet_skill_effct_system.in_set(SkillSystems::Update),
    );
}

pub fn execute_skill(
    commands: &mut Commands,
    skill: &Skill,
    caster: Entity,
    targets: Vec<Entity>,
    source: Option<Entity>,
    data: SkillRunContextData,
) {
    for target in targets.iter() {
        let bullet_skill_effect = BulletSkillEffect::from_skill(skill);

        commands.spawn((
            bullet_skill_effect,
            SkillRunContext {
                source: source.clone(),
                data: data.clone(),
                caster,
                target: *target,
            },
            SkillResponse::empty(),
        ));
    }
}
