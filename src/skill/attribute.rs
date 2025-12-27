use std::{f32, mem::swap};

use bevy::{ecs::component::Component, platform::collections::HashMap};

#[derive(Debug, Default, Component)]
pub struct SkillAttributeSet {
    data: HashMap<String, SkillAttribute>,
    dependency_modifiers: HashMap<String, AttributeDependencyModifier>,
}

impl SkillAttributeSet {
    pub fn skill_attribute(&self, attribute_name: &str) -> Option<&SkillAttribute> {
        self.data.get(attribute_name)
    }

    pub fn add_dependency_modifier(&mut self, dependency_modifier: AttributeDependencyModifier) {
        let source = dependency_modifier.get_source();
        self.dependency_modifiers
            .insert(source, dependency_modifier);
    }

    pub fn add_skill_attribute(&mut self, attribute: SkillAttribute) {
        let attribute_name = attribute.name.clone();
        self.data.insert(attribute_name, attribute);
    }

    pub fn update_attribute_base_value(&mut self, attribute_name: &str, value: f32) {
        if let Some(attribute) = self.data.get_mut(attribute_name) {
            let old_value = attribute.get_current_value();

            attribute.update_base_value(value);

            let new_value = attribute.get_current_value();

            if new_value != old_value {
                let dependency_modifiers = self
                    .dependency_modifiers
                    .values()
                    .filter(|dependency_modifier| dependency_modifier.source == attribute_name)
                    .collect::<Vec<_>>();

                for dependency_modifier in dependency_modifiers.iter() {
                    if let Some(target_attribute) = self.data.get_mut(&dependency_modifier.target) {
                        dependency_modifier.update(target_attribute, new_value - old_value);
                    } else {
                        tracing::error!("{} attribute_name not found.", dependency_modifier.target);
                    }
                }
            }
        } else {
            tracing::error!("{} attribute_name not found.", attribute_name);
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct AttributeDependencyModifier {
    source: String,
    target: String,
    value: f32,
}

impl AttributeDependencyModifier {
    pub fn get_source(&self) -> String {
        format!("__attribute_dependency_{}__{}", self.source, self.target)
    }

    pub fn update(&self, target: &mut SkillAttribute, value: i32) {
        let mut modifier = SkillAttributeModifier::default();
        modifier.source = self.get_source();
        modifier.value = self.value * (value as f32);

        target.add_modifier(&modifier);
    }
}

///属性
#[derive(Debug, Clone)]
pub struct SkillAttribute {
    ///物理的最小值
    min_value: f32,
    ///物理的最大值
    max_value: f32,
    ///基础值
    base_value: f32,
    ///实际值
    current_value: f32,
    ///修改器类别
    modifiers: Vec<SkillAttributeModifier>,
    ///属性名
    pub name: String,
}

impl Default for SkillAttribute {
    fn default() -> Self {
        Self {
            min_value: -f32::INFINITY,
            max_value: f32::INFINITY,
            base_value: 0.0,
            current_value: 0.0,
            modifiers: vec![],
            name: Default::default(),
        }
    }
}

impl SkillAttribute {
    pub fn remove_modifier_with_source(&mut self, source: &str) {
        let mut modifiers = vec![];

        swap(&mut self.modifiers, &mut modifiers);

        self.modifiers = modifiers
            .into_iter()
            .filter(|modifier| modifier.source != source)
            .collect::<Vec<_>>();

        self.calculate_current_value();
    }

    pub fn add_modifier(&mut self, modifier: &SkillAttributeModifier) {
        if !self.modifiers.contains(modifier) {
            self.modifiers.push(modifier.clone());
            self.calculate_current_value();
        }
    }

    pub fn remove_modifier(&mut self, modifier: &SkillAttributeModifier) {
        if self.modifiers.contains(modifier) {
            let index = self
                .modifiers
                .iter()
                .position(|item| item == modifier)
                .unwrap();

            self.modifiers.remove(index);

            self.calculate_current_value();
        }
    }

    pub fn base_value(&self) -> i32 {
        self.base_value.clamp(self.min_value, self.max_value) as i32
    }

    pub fn get_current_value(&self) -> i32 {
        self.current_value.clamp(self.min_value, self.max_value) as i32
    }

    fn calculate_current_value(&mut self) {
        let mut current_value = self.base_value;

        let mut absolute_modifiers = vec![];
        let mut percentage_modifiers = vec![];
        let mut overload_modifier = None;

        for modifier in self.modifiers.iter() {
            match modifier.operation {
                SkillAttributeModifierOperation::Absolute => {
                    absolute_modifiers.push(modifier.clone());
                }
                SkillAttributeModifierOperation::Percentage => {
                    percentage_modifiers.push(modifier.clone());
                }
                SkillAttributeModifierOperation::Overload => {
                    if overload_modifier.is_none() {
                        overload_modifier = Some(modifier.clone());
                    } else {
                        let overload_modifier = overload_modifier.as_mut().unwrap();

                        if overload_modifier.priority < modifier.priority {
                            *overload_modifier = modifier.clone()
                        }
                    }
                }
            }
        }

        let mut absolute_value = 0.0;
        for modifier in absolute_modifiers.iter() {
            absolute_value += modifier.value;
        }
        current_value += absolute_value;

        let mut percentage_value = 0.0;
        for modifier in percentage_modifiers.iter() {
            percentage_value += modifier.value * self.base_value;
        }
        current_value += percentage_value;

        if let Some(modifier) = overload_modifier {
            current_value = modifier.value;
        }

        self.current_value = current_value;
    }

    pub fn update_base_value(&mut self, value: f32) {
        self.base_value += value;
        self.calculate_current_value();
    }
}

///更改器类型
#[derive(Debug, Clone, PartialEq, Default)]
pub enum SkillAttributeModifierOperation {
    ///相对值
    #[default]
    Absolute,
    ///百分比
    Percentage,
    ///覆盖
    Overload,
}

///修改器
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SkillAttributeModifier {
    ///对数值的更改方式
    pub operation: SkillAttributeModifierOperation,
    ///优先级
    pub priority: u32,
    ///数值
    pub value: f32,
    pub source: String,
}

#[cfg(test)]
mod test {

    use super::{
        AttributeDependencyModifier, SkillAttribute, SkillAttributeModifier,
        SkillAttributeModifierOperation, SkillAttributeSet,
    };

    #[test]
    fn test_attribute_set() {
        let mut attribute_set = SkillAttributeSet::default();

        let mut attribute = SkillAttribute::default();
        attribute.name = "power".to_string();
        attribute.update_base_value(10.0);

        attribute_set.add_skill_attribute(attribute);

        let mut attribute = SkillAttribute::default();
        attribute.name = "burden".to_string();
        attribute.update_base_value(10.0);

        attribute_set.add_skill_attribute(attribute);

        let mut dependency_modifier = AttributeDependencyModifier::default();

        dependency_modifier.source = "power".to_string();
        dependency_modifier.target = "burden".to_string();
        dependency_modifier.value = 10.0;

        attribute_set.add_dependency_modifier(dependency_modifier);

        attribute_set.update_attribute_base_value("power", 1.0);

        let burden_attribute = attribute_set.skill_attribute("burden").unwrap();

        let burden = burden_attribute.get_current_value();

        assert_eq!(20, burden);
    }

    #[test]
    fn test_attribute() {
        let mut attribute = SkillAttribute::default();
        attribute.update_base_value(10.0);

        let mut absolute_modifier = SkillAttributeModifier::default();
        absolute_modifier.value = 2.0;
        attribute.add_modifier(&absolute_modifier);
        assert_eq!(attribute.get_current_value(), 12);

        let mut percentage_modifier = SkillAttributeModifier::default();
        percentage_modifier.operation = SkillAttributeModifierOperation::Percentage;
        percentage_modifier.value = 0.5;
        attribute.add_modifier(&percentage_modifier);
        assert_eq!(attribute.get_current_value(), 17);

        let mut overload_modifier = SkillAttributeModifier::default();
        overload_modifier.operation = SkillAttributeModifierOperation::Overload;
        overload_modifier.value = 6.0;
        attribute.add_modifier(&overload_modifier);
        assert_eq!(attribute.get_current_value(), 6);

        let mut overload_modifier1 = SkillAttributeModifier::default();
        overload_modifier1.operation = SkillAttributeModifierOperation::Overload;
        overload_modifier1.value = 20.0;
        overload_modifier1.priority = 10;
        attribute.add_modifier(&overload_modifier1);
        assert_eq!(attribute.get_current_value(), 20);

        attribute.remove_modifier_with_source(&overload_modifier1.source);

        assert_eq!(attribute.get_current_value(), 10);
    }
}
