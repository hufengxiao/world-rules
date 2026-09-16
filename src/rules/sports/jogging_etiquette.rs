//! 慢跑健身礼仪
//!
//! 慢跑、健步等有氧运动的公共礼仪与安全

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: JoggingEtiquetteRules,
    name: "慢跑健身礼仪",
    desc: "慢跑、健步等有氧运动的公共礼仪与安全",
    origin: "大众",
    tags: ["体育", "慢跑", "健身", "礼仪", "安全"]
}

impl JoggingEtiquetteRules {
    /// 热身准备
    pub fn warmup(&self) -> Vec<&'static str> {
        vec![
            "跑步前充分热身激活",
            "穿着合脚运动鞋",
            "补水并注意环境温度",
            "轻度运动后可逐渐加速",
        ]
    }

    /// 道路礼仪
    pub fn road(&self) -> Vec<&'static str> {
        vec![
            "在人行步道跑步",
            "避让行人主动让道",
            "过马路看清红灯",
            "避免并排占据整条路",
        ]
    }

    /// 安全要点
    pub fn safety(&self) -> Vec<&'static str> {
        vec![
            "夜间穿反光装备",
            "控制合适配速不过冲",
            "留意路面坑洼与交通",
            "突发不适及时停下休息",
        ]
    }

    /// 健身规律
    pub fn routine(&self) -> Vec<&'static str> {
        vec![
            "循序渐进增加路远",
            "合理安排休息与恢复",
            "配合拉伸缓解肌疲劳",
            "坚持适度不过度",
        ]
    }
}

impl Rule for JoggingEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("jogging")
    }

    fn explain(&self) -> String {
        format!(
            "【慢跑健身礼仪】\n{}",
            [
                format!(
                    "热身准备：\\n{}",
                    self.warmup()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "道路礼仪：\\n{}",
                    self.road()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "安全要点：\\n{}",
                    self.safety()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "健身规律：\\n{}",
                    self.routine()
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
    fn test_joggingetiquetterules_basic() {
        let rules = JoggingEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "慢跑健身礼仪");
        assert!(!rules.warmup().is_empty());
        assert!(!rules.road().is_empty());
        assert!(!rules.safety().is_empty());
        assert!(!rules.routine().is_empty());
    }

    #[test]
    fn test_joggingetiquetterules_validation() {
        let rules = JoggingEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::sports("jogging"));
    }

    #[test]
    fn test_joggingetiquetterules_explain() {
        let rules = JoggingEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("热身准备"));
        assert!(e.contains("道路礼仪"));
        assert!(e.contains("安全要点"));
    }
}
