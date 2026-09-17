//! 心肺复苏CPR应急
//!
//! 发现心跳骤停等紧急情况时识别、呼救与实施CPR的规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: CprEmergencyRules,
    name: "心肺复苏CPR应急",
    desc: "发现心跳骤停等紧急情况时识别、呼救与实施CPR的规则",
    origin: "医学",
    tags: ["健康", "急救", "CPR", "心肺", "人工呼吸"]
}

impl CprEmergencyRules {
    /// 识别判断
    pub fn assess(&self) -> Vec<&'static str> {
        vec![
            "先确认环境安全",
            "轻拍肩部大声呼唤",
            "观察有无呼吸",
            "判断是否为晕厥危象",
        ]
    }

    /// 呼救施救位
    pub fn call(&self) -> Vec<&'static str> {
        vec![
            "立即拨打急救电话",
            "清楚说明地点与情况",
            "呼救他人协助取AED",
            "将患者放平仰卧于硬地",
        ]
    }

    /// 实施按压
    pub fn compress(&self) -> Vec<&'static str> {
        vec![
            "双手交叠于胸骨下半",
            "成人按压深度约5-6厘米",
            "频率每分钟100-120次",
            "充分回弹保持连贯",
        ]
    }

    /// 配合呼吸
    pub fn airway(&self) -> Vec<&'static str> {
        vec![
            "熟练者进行按压与人工呼吸",
            "非专业可专注按压",
            "使用AED按提示操作",
            "患者苏醒或救援到达再停",
        ]
    }
}

impl Rule for CprEmergencyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("cpr_emergency")
    }

    fn explain(&self) -> String {
        format!(
            "【心肺复苏CPR应急】\n{}",
            [
                format!(
                    "识别判断：\\n{}",
                    self.assess()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "呼救施救位：\\n{}",
                    self.call()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "实施按压：\\n{}",
                    self.compress()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "配合呼吸：\\n{}",
                    self.airway()
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
    fn test_cpremergencyrules_basic() {
        let rules = CprEmergencyRules::new();
        assert_eq!(rules.metadata().name, "心肺复苏CPR应急");
        assert!(!rules.assess().is_empty());
        assert!(!rules.call().is_empty());
        assert!(!rules.compress().is_empty());
        assert!(!rules.airway().is_empty());
    }

    #[test]
    fn test_cpremergencyrules_validation() {
        let rules = CprEmergencyRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("cpr_emergency"));
    }

    #[test]
    fn test_cpremergencyrules_explain() {
        let rules = CprEmergencyRules::new();
        let e = rules.explain();
        assert!(e.contains("识别判断"));
        assert!(e.contains("呼救施救位"));
        assert!(e.contains("实施按压"));
    }
}
