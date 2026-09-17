//! 交通记分处理
//!
//! 交通违法记分规则、累积与满分处理要点

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: TrafficPointsDemeritRules,
    name: "交通记分处理",
    desc: "交通违法记分规则、累积与满分处理要点",
    origin: "中国",
    tags: ["法律", "交通", "记分", "违章"]
}

impl TrafficPointsDemeritRules {
    /// 记分类型
    pub fn points(&self) -> Vec<&'static str> {
        vec![
            "不同违法记不同分",
            "记分周期有规定",
            "累计满有处理",
            "记分公开可查",
        ]
    }

    /// 期满处理
    pub fn clearing(&self) -> Vec<&'static str> {
        vec!["周期末及时缴清", "及时学习消记", "满分会停驶", "依规重考"]
    }

    /// 查询核对
    pub fn verify(&self) -> Vec<&'static str> {
        vec![
            "定期查询记分记录",
            "对照处罚核对",
            "有异议及时反映",
            "记录完整",
        ]
    }

    /// 守法驾驶
    pub fn obey(&self) -> Vec<&'static str> {
        vec![
            "文明驾驶规避罚分",
            "不借分转让",
            "守规安全出行",
            "余额即时了解",
        ]
    }
}

impl Rule for TrafficPointsDemeritRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("traffic_points")
    }

    fn explain(&self) -> String {
        format!(
            "【交通记分处理】\n{}",
            [
                format!(
                    "记分类型：\\n{}",
                    self.points()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "期满处理：\\n{}",
                    self.clearing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "查询核对：\\n{}",
                    self.verify()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "守法驾驶：\\n{}",
                    self.obey()
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
    fn test_trafficpointsdemeritrules_basic() {
        let rules = TrafficPointsDemeritRules::new();
        assert_eq!(rules.metadata().name, "交通记分处理");
        assert!(!rules.points().is_empty());
        assert!(!rules.clearing().is_empty());
        assert!(!rules.verify().is_empty());
        assert!(!rules.obey().is_empty());
    }

    #[test]
    fn test_trafficpointsdemeritrules_validation() {
        let rules = TrafficPointsDemeritRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("traffic_points"));
    }

    #[test]
    fn test_trafficpointsdemeritrules_explain() {
        let rules = TrafficPointsDemeritRules::new();
        let e = rules.explain();
        assert!(e.contains("记分类型"));
        assert!(e.contains("期满处理"));
        assert!(e.contains("查询核对"));
    }
}
