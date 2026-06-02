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

    /// 溶液化学规则
    pub fn solution_rules(&self) -> Vec<&'static str> {
        vec![
            "溶液: 溶质溶解在溶剂中形成的均一混合物",
            "溶解度: 一定温度下溶质在溶剂中最大溶解量",
            "饱和溶液: 溶解度达到最大值的溶液",
            "不饱和溶液: 溶解度未达最大值的溶液",
            "结晶: 溶质从溶液中析出形成晶体",
            "重结晶: 通过溶解结晶纯化物质",
            "蒸馏: 利用沸点差异分离液体混合物",
        ]
    }

    /// 气体化学规则
    pub fn gas_rules(&self) -> Vec<&'static str> {
        vec![
            "理想气体: PV = nRT",
            "道尔顿分压定律: 混合气体总压等于各组分分压之和",
            "格拉罕姆扩散定律: 气体扩散速率与分子量平方根成反比",
            "查理定律: 一定质量气体体积与温度成正比",
            "波义耳定律: 一定质量气体压强与体积成反比",
            "阿伏伽德罗定律: 同温同压下同体积气体分子数相同",
            "临界温度: 气体可液化的最高温度",
        ]
    }

    /// 电化学规则
    pub fn electrochemistry_rules(&self) -> Vec<&'static str> {
        vec![
            "原电池: 化学能转变为电能的装置",
            "电解池: 电能转变为化学能的装置",
            "阳极: 发生氧化反应的电极",
            "阴极: 发生还原反应的电极",
            "电解质: 在水溶液或熔融状态下能导电的化合物",
            "法拉第电解定律: 电解产物质量与电量成正比",
            "电镀: 利用电解在金属表面镀上其他金属",
        ]
    }

    /// 热化学规则
    pub fn thermochemistry_rules(&self) -> Vec<&'static str> {
        vec![
            "焓变: 化学反应吸收或放出的热量",
            "放热反应: 反应放出热量 ΔH < 0",
            "吸热反应: 反应吸收热量 ΔH > 0",
            "盖斯定律: 化学反应热效应只与始末状态有关",
            "燃烧热: 1mol可燃物完全燃烧放出的热量",
            "中和热: 强酸强碱稀溶液反应放热",
            "键能: 化学键断裂吸收的能量",
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

    /// 化学键
    pub fn chemical_bonding(&self) -> Vec<&'static str> {
        vec![
            "离子键: 正负离子之间的静电引力",
            "共价键: 原子间共用电子对形成的化学键",
            "金属键: 金属阳离子与自由电子之间的相互作用",
            "氢键: 电负性原子与已键合氢之间的弱相互作用",
            "范德华力: 分子间普遍存在的弱相互作用",
            "杂化轨道: 原子轨道混合形成新的等价轨道",
            "分子轨道理论: 原子轨道线性组合形成分子轨道",
        ]
    }

    /// 化学平衡
    pub fn chemical_equilibrium(&self) -> Vec<&'static str> {
        vec![
            "勒夏特列原理: 平衡系统对外界改变产生抵消性移动",
            "质量作用定律: 平衡常数K等于产物浓度幂之积除以反应物",
            "范特霍夫方程: 温度对平衡常数的影响",
            "溶度积: 难溶电解质饱和溶液中离子浓度幂之积",
            "酸碱平衡: 酸碱质子理论和电离平衡",
            "缓冲溶液: 能抵抗外加酸碱改变pH的溶液",
            "电化学: 化学能与电能相互转化的规律",
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
            化学定律:\n{}\n\n\
            溶液化学规则:\n{}\n\n\
            气体化学规则:\n{}\n\n\
            电化学规则:\n{}\n\n\
            热化学规则:\n{}\n",
            elements.iter()
                .map(|e| format!("  • {}({}): {}号元素, {:.3}", e.symbol, e.name(), e.atomic_number, e.atomic_mass))
                .collect::<Vec<_>>()
                .join("\n"),
            self.periodic_laws().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.reaction_types().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.chemical_laws().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.solution_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.gas_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.electrochemistry_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.thermochemistry_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
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