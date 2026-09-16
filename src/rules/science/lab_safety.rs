//! 实验室安全规范
//!
//! 科学实验室操作、化学品与设备安全的基本规范

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: LabSafetyRules,
    name: "实验室安全规范",
    desc: "科学实验室操作、化学品与设备安全的基本规范",
    origin: "科学",
    tags: ["科学", "实验室", "安全", "化学", "规范"]
}

impl LabSafetyRules {
    /// 着装防护
    pub fn protective_gear(&self) -> Vec<&'static str> {
        vec![
            "进入实验室穿着实验服",
            "佩戴防护眼镜与手套",
            "长发束起避免接触火焰",
            "不穿拖鞋或凉鞋进入",
        ]
    }

    /// 化学品操作
    pub fn chemical(&self) -> Vec<&'static str> {
        vec![
            "看清标签再取用化学品",
            "不随意闻尝或接触",
            "稀释酸时酸入水",
            "用后盖紧瓶盖归位",
        ]
    }

    /// 设备安全
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "用前检查玻璃器皿完好",
            "加热操作注意远离易燃",
            "离心机配平再运行",
            "用毕及时关闭电源",
        ]
    }

    /// 应急处置
    pub fn emergency(&self) -> Vec<&'static str> {
        vec![
            "化学溅入眼中立即用清水冲淋求助",
            "小面积烧伤用流动清水降温",
            "化学品泄漏按程序清理",
            "火情按下灭警并逃离",
        ]
    }
}

impl Rule for LabSafetyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("lab_safety")
    }

    fn explain(&self) -> String {
        format!(
            "【实验室安全规范】\n{}",
            [
                format!(
                    "着装防护：\\n{}",
                    self.protective_gear()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "化学品操作：\\n{}",
                    self.chemical()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "设备安全：\\n{}",
                    self.equipment()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "应急处置：\\n{}",
                    self.emergency()
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
    fn test_labsafetyrules_basic() {
        let rules = LabSafetyRules::new();
        assert_eq!(rules.metadata().name, "实验室安全规范");
        assert!(!rules.protective_gear().is_empty());
        assert!(!rules.chemical().is_empty());
        assert!(!rules.equipment().is_empty());
        assert!(!rules.emergency().is_empty());
    }

    #[test]
    fn test_labsafetyrules_validation() {
        let rules = LabSafetyRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("lab_safety"));
    }

    #[test]
    fn test_labsafetyrules_explain() {
        let rules = LabSafetyRules::new();
        let e = rules.explain();
        assert!(e.contains("着装防护"));
        assert!(e.contains("化学品操作"));
        assert!(e.contains("设备安全"));
    }
}
