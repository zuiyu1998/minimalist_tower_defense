use std::f32;

///属性
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
}

impl Default for SkillAttribute {
    fn default() -> Self {
        Self {
            min_value: -f32::INFINITY,
            max_value: f32::INFINITY,
            base_value: 0.0,
            current_value: 0.0,
            modifiers: vec![],
        }
    }
}

impl SkillAttribute {
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
    use super::{SkillAttribute, SkillAttributeModifier, SkillAttributeModifierOperation};

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
    }
}
