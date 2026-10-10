//! 电池化学原理
//!
//! 化学电池将化学能转化为电能的原理

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: BatteryChemistryRules,
    name: "电池化学原理",
    desc: "化学电池将化学能转化为电能的原理",
    origin: "科学常识",
    tags: ["化学", "电化学", "能源"]
}

impl BatteryChemistryRules {
    /// 工作原理
    pub fn principle(&self) -> Vec<&'static str> {
        vec![
            "氧化还原反应分离进行",
            "负极失电子被氧化",
            "正极得电子被还原",
            "电子经外电路流动",
        ]
    }

    /// 电极构成
    pub fn electrodes(&self) -> Vec<&'static str> {
        vec![
            "负极活性物质易失电子",
            "正极活性物质易得电子",
            "电解液传导离子",
            "隔膜防止短路",
        ]
    }

    /// 常见类型
    pub fn types(&self) -> Vec<&'static str> {
        vec![
            "干电池一次性使用",
            "锂电池能量密度高",
            "铅酸电池用于汽车",
            "燃料电池持续供能",
        ]
    }

    /// 充放电
    pub fn cycling(&self) -> Vec<&'static str> {
        vec![
            "充电为逆反应",
            "放电为自发反应",
            "可逆电池可反复使用",
            "循环次数有限",
        ]
    }
}

impl Rule for BatteryChemistryRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("battery_chemistry")
    }

    fn explain(&self) -> String {
        format!(
            "【电池化学原理】\n{}",
            [
                format!(
                    "工作原理：\\n{}",
                    self.principle()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "电极构成：\\n{}",
                    self.electrodes()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "常见类型：\\n{}",
                    self.types()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "充放电：\\n{}",
                    self.cycling()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
            ]
            .join("\n\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::core::ValidateContext;

    #[test]
    fn test_batterychemistryrules_basic() {
        let rules = BatteryChemistryRules::new();
        assert_eq!(rules.metadata().name, "电池化学原理");
        assert!(!rules.principle().is_empty());
        assert!(!rules.electrodes().is_empty());
        assert!(!rules.types().is_empty());
        assert!(!rules.cycling().is_empty());
    }

    #[test]
    fn test_batterychemistryrules_validation() {
        let rules = BatteryChemistryRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("battery_chemistry"));
    }

    #[test]
    fn test_batterychemistryrules_explain() {
        let rules = BatteryChemistryRules::new();
        let e = rules.explain();
        assert!(e.contains("工作原理"));
        assert!(e.contains("电极构成"));
        assert!(e.contains("常见类型"));
    }
}
