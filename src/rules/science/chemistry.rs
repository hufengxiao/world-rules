//! 化学元素周期表

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 元素分类
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementCategory {
    /// 碱金属
    AlkaliMetal,
    /// 碱土金属
    AlkalineEarthMetal,
    /// 过渡金属
    TransitionMetal,
    /// 其他金属
    OtherMetal,
    /// 非金属
    Nonmetal,
    /// 稀有气体
    NobleGas,
    /// 卤素
    Halogen,
    /// 镧系元素
    Lanthanide,
    /// 锕系元素
    Actinide,
}

impl ElementCategory {
    pub fn name(&self) -> &'static str {
        match self {
            ElementCategory::AlkaliMetal => "碱金属",
            ElementCategory::AlkalineEarthMetal => "碱土金属",
            ElementCategory::TransitionMetal => "过渡金属",
            ElementCategory::OtherMetal => "其他金属",
            ElementCategory::Nonmetal => "非金属",
            ElementCategory::NobleGas => "稀有气体",
            ElementCategory::Halogen => "卤素",
            ElementCategory::Lanthanide => "镧系元素",
            ElementCategory::Actinide => "锕系元素",
        }
    }
}

/// 元素信息
#[derive(Debug, Clone)]
pub struct Element {
    /// 原子序数
    pub atomic_number: u8,
    /// 元素符号
    pub symbol: &'static str,
    /// 中文名
    pub chinese_name: &'static str,
    /// 英文名
    pub english_name: &'static str,
    /// 原子量 (近似)
    pub atomic_mass: f32,
    /// 分类
    pub category: ElementCategory,
    /// 电子层数
    pub electron_shells: u8,
}

impl Element {
    pub fn name(&self) -> &'static str {
        self.chinese_name
    }
}

/// 化学规则
pub struct ChemistryRules {
    metadata: RuleMetadata,
}

impl ChemistryRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "化学元素周期表",
                "元素周期表基础规则"
            )
            .with_origin("科学")
            .with_tags(vec!["科学".into(), "化学".into()]),
        }
    }

    /// 获取部分常见元素
    pub fn common_elements() -> Vec<Element> {
        vec![
            Element { atomic_number: 1, symbol: "H", chinese_name: "氢", english_name: "Hydrogen", atomic_mass: 1.008, category: ElementCategory::Nonmetal, electron_shells: 1 },
            Element { atomic_number: 2, symbol: "He", chinese_name: "氦", english_name: "Helium", atomic_mass: 4.003, category: ElementCategory::NobleGas, electron_shells: 1 },
            Element { atomic_number: 6, symbol: "C", chinese_name: "碳", english_name: "Carbon", atomic_mass: 12.011, category: ElementCategory::Nonmetal, electron_shells: 2 },
            Element { atomic_number: 7, symbol: "N", chinese_name: "氮", english_name: "Nitrogen", atomic_mass: 14.007, category: ElementCategory::Nonmetal, electron_shells: 2 },
            Element { atomic_number: 8, symbol: "O", chinese_name: "氧", english_name: "Oxygen", atomic_mass: 15.999, category: ElementCategory::Nonmetal, electron_shells: 2 },
            Element { atomic_number: 11, symbol: "Na", chinese_name: "钠", english_name: "Sodium", atomic_mass: 22.990, category: ElementCategory::AlkaliMetal, electron_shells: 3 },
            Element { atomic_number: 12, symbol: "Mg", chinese_name: "镁", english_name: "Magnesium", atomic_mass: 24.305, category: ElementCategory::AlkalineEarthMetal, electron_shells: 3 },
            Element { atomic_number: 26, symbol: "Fe", chinese_name: "铁", english_name: "Iron", atomic_mass: 55.845, category: ElementCategory::TransitionMetal, electron_shells: 4 },
            Element { atomic_number: 29, symbol: "Cu", chinese_name: "铜", english_name: "Copper", atomic_mass: 63.546, category: ElementCategory::TransitionMetal, electron_shells: 4 },
            Element { atomic_number: 30, symbol: "Zn", chinese_name: "锌", english_name: "Zinc", atomic_mass: 65.38, category: ElementCategory::TransitionMetal, electron_shells: 4 },
            Element { atomic_number: 47, symbol: "Ag", chinese_name: "银", english_name: "Silver", atomic_mass: 107.87, category: ElementCategory::TransitionMetal, electron_shells: 5 },
            Element { atomic_number: 79, symbol: "Au", chinese_name: "金", english_name: "Gold", atomic_mass: 196.97, category: ElementCategory::TransitionMetal, electron_shells: 6 },
        ]
    }

    /// 周期表规律
    pub fn periodic_laws(&self) -> Vec<&'static str> {
        vec![
            "原子序数递增排列",
            "同一周期: 电子层数相同",
            "同一族: 化学性质相似",
            "原子半径: 同周期递减，同族递增",
            "电离能: 同周期递增，同族递减",
            "金属性: 同周期递减，同族递增",
        ]
    }

    /// 化学反应类型
    pub fn reaction_types(&self) -> Vec<&'static str> {
        vec![
            "化合反应: A + B → AB",
            "分解反应: AB → A + B",
            "置换反应: A + BC → AC + B",
            "复分解反应: AB + CD → AD + CB",
            "氧化还原反应: 电子转移",
        ]
    }

    /// 常见化学定律
    pub fn chemical_laws(&self) -> Vec<&'static str> {
        vec![
            "质量守恒定律: 反应前后总质量不变",
            "能量守恒定律: 反应前后总能量不变",
            "电荷守恒定律: 反应前后总电荷不变",
            "阿伏伽德罗定律: 同温同压同体积气体分子数相同",
        ]
    }
}

impl Default for ChemistryRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ChemistryRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("chemistry")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        let elements = Self::common_elements();
        format!(
            "【化学元素周期表】\n\n\
            常见元素:\n{}\n\n\
            周期表规律:\n{}\n\n\
            化学反应类型:\n{}\n\n\
            化学定律:\n{}\n",
            elements.iter()
                .map(|e| format!("  • {}({}): {}号元素, {:.3}", e.symbol, e.name(), e.atomic_number, e.atomic_mass))
                .collect::<Vec<_>>()
                .join("\n"),
            self.periodic_laws().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.reaction_types().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.chemical_laws().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_elements() {
        let elements = ChemistryRules::common_elements();
        assert_eq!(elements[0].symbol, "H");
        assert_eq!(elements.len(), 12);
    }
}