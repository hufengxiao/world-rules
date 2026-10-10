//! 消化过程
//!
//! 食物从口腔到肠道的消化吸收全过程

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: DigestionProcessStepsRules,
    name: "消化过程",
    desc: "食物从口腔到肠道的消化吸收全过程",
    origin: "科学常识",
    tags: ["生理", "消化", "过程"]
}

impl DigestionProcessStepsRules {
    /// 口腔阶段
    pub fn mouth(&self) -> Vec<&'static str> {
        vec![
            "牙齿咀嚼磨碎食物",
            "唾液淀粉酶分解淀粉",
            "唾液润滑便于吞咽",
            "形成食团下咽",
        ]
    }

    /// 胃内消化
    pub fn stomach(&self) -> Vec<&'static str> {
        vec![
            "胃壁肌肉搅拌食物",
            "胃酸激活胃蛋白酶",
            "蛋白质开始分解",
            "形成食糜送入小肠",
        ]
    }

    /// 小肠吸收
    pub fn small_intestine(&self) -> Vec<&'static str> {
        vec![
            "胰液胆汁助消化",
            "三大营养物分解",
            "绒毛吸收养分入血",
            "是主要消化吸收场所",
        ]
    }

    /// 大肠排遗
    pub fn large_intestine(&self) -> Vec<&'static str> {
        vec![
            "吸收水分与电解质",
            "肠道菌群发酵纤维",
            "形成粪便储存",
            "经直肠排出体外",
        ]
    }
}

impl Rule for DigestionProcessStepsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("digestion_process_steps")
    }

    fn explain(&self) -> String {
        format!(
            "【消化过程】\n{}",
            [
                format!(
                    "口腔阶段：\\n{}",
                    self.mouth()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "胃内消化：\\n{}",
                    self.stomach()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "小肠吸收：\\n{}",
                    self.small_intestine()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "大肠排遗：\\n{}",
                    self.large_intestine()
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
    fn test_digestionprocessstepsrules_basic() {
        let rules = DigestionProcessStepsRules::new();
        assert_eq!(rules.metadata().name, "消化过程");
        assert!(!rules.mouth().is_empty());
        assert!(!rules.stomach().is_empty());
        assert!(!rules.small_intestine().is_empty());
        assert!(!rules.large_intestine().is_empty());
    }

    #[test]
    fn test_digestionprocessstepsrules_validation() {
        let rules = DigestionProcessStepsRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::science("digestion_process_steps")
        );
    }

    #[test]
    fn test_digestionprocessstepsrules_explain() {
        let rules = DigestionProcessStepsRules::new();
        let e = rules.explain();
        assert!(e.contains("口腔阶段"));
        assert!(e.contains("胃内消化"));
        assert!(e.contains("小肠吸收"));
    }
}
