//! 电路基础安全
//!
//! 欧姆定律、串联并联与用电安全基础

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ElectricCircuitBasicsRules,
    name: "电路基础安全",
    desc: "欧姆定律、串联并联与用电安全基础",
    origin: "国际",
    tags: ["科学", "电路", "电学", "安全"]
}

impl ElectricCircuitBasicsRules {
    /// 基本定律
    pub fn ohms(&self) -> Vec<&'static str> {
        vec![
            "电压电流电阻关系",
            "欧姆定律是基础",
            "串联分压电流相同",
            "并联分电流电压相同",
        ]
    }

    /// 功率与能耗
    pub fn power(&self) -> Vec<&'static str> {
        vec![
            "功率等于电压乘电流",
            "能耗是功率乘时间",
            "合理节约用电",
            "注意额定功率",
        ]
    }

    /// 用电安全
    pub fn safety(&self) -> Vec<&'static str> {
        vec![
            "不用湿手碰电器",
            "避免插座过载",
            "破损电线及时更换",
            "不使用三无电器",
        ]
    }

    /// 规范操作
    pub fn practice(&self) -> Vec<&'static str> {
        vec![
            "断电后再检修电路",
            "保护开关装设正确",
            "了解漏电与短路风险",
            "重大电气问题求助电工",
        ]
    }
}

impl Rule for ElectricCircuitBasicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("electric_circuit")
    }

    fn explain(&self) -> String {
        format!(
            "【电路基础安全】\n{}",
            [
                format!(
                    "基本定律：\\n{}",
                    self.ohms()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "功率与能耗：\\n{}",
                    self.power()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "用电安全：\\n{}",
                    self.safety()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "规范操作：\\n{}",
                    self.practice()
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
    fn test_electriccircuitbasicsrules_basic() {
        let rules = ElectricCircuitBasicsRules::new();
        assert_eq!(rules.metadata().name, "电路基础安全");
        assert!(!rules.ohms().is_empty());
        assert!(!rules.power().is_empty());
        assert!(!rules.safety().is_empty());
        assert!(!rules.practice().is_empty());
    }

    #[test]
    fn test_electriccircuitbasicsrules_validation() {
        let rules = ElectricCircuitBasicsRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("electric_circuit"));
    }

    #[test]
    fn test_electriccircuitbasicsrules_explain() {
        let rules = ElectricCircuitBasicsRules::new();
        let e = rules.explain();
        assert!(e.contains("基本定律"));
        assert!(e.contains("功率与能耗"));
        assert!(e.contains("用电安全"));
    }
}
