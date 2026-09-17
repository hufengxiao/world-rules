//! 痛风管理与尿酸
//!
//! 高尿酸与痛风的饮食、监测与发作应对规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: GoutManagementRules,
    name: "痛风管理与尿酸",
    desc: "高尿酸与痛风的饮食、监测与发作应对规则",
    origin: "医学",
    tags: ["健康", "痛风", "尿酸", "关节", "饮食"]
}

impl GoutManagementRules {
    /// 饮食控制
    pub fn diet(&self) -> Vec<&'static str> {
        vec![
            "限制高嘌呤食物摄入",
            "减少海鲜内脏浓汤等",
            "限制酒精尤其啤酒",
            "多饮水促尿酸排出",
        ]
    }

    /// 监测认知
    pub fn monitor(&self) -> Vec<&'static str> {
        vec![
            "定期检测血尿酸水平",
            "识别脚趾关节红肿热痛",
            "区分只是高尿酸还是痛风",
            "遵医嘱管理目标值",
        ]
    }

    /// 发作应对
    pub fn flare(&self) -> Vec<&'static str> {
        vec![
            "发作期减少行走负重",
            "抬高患肢并冷敷",
            "遵医嘱用药缓解",
            "不自行乱吃降酸药加重发作",
        ]
    }

    /// 长期管理
    pub fn long_term(&self) -> Vec<&'static str> {
        vec![
            "控制体重与合理饮食",
            "坚持规范用药遵医嘱",
            "规律运动但不过度",
            "定期随访评估肾功能",
        ]
    }
}

impl Rule for GoutManagementRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("gout_management")
    }

    fn explain(&self) -> String {
        format!(
            "【痛风管理与尿酸】\n{}",
            [
                format!(
                    "饮食控制：\\n{}",
                    self.diet()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "监测认知：\\n{}",
                    self.monitor()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "发作应对：\\n{}",
                    self.flare()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "长期管理：\\n{}",
                    self.long_term()
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
    fn test_goutmanagementrules_basic() {
        let rules = GoutManagementRules::new();
        assert_eq!(rules.metadata().name, "痛风管理与尿酸");
        assert!(!rules.diet().is_empty());
        assert!(!rules.monitor().is_empty());
        assert!(!rules.flare().is_empty());
        assert!(!rules.long_term().is_empty());
    }

    #[test]
    fn test_goutmanagementrules_validation() {
        let rules = GoutManagementRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("gout_management"));
    }

    #[test]
    fn test_goutmanagementrules_explain() {
        let rules = GoutManagementRules::new();
        let e = rules.explain();
        assert!(e.contains("饮食控制"));
        assert!(e.contains("监测认知"));
        assert!(e.contains("发作应对"));
    }
}
