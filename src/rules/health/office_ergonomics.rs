//! 办公室人体工学
//!
//! 工位设置与坐姿以减少久坐损害的人体工学规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: OfficeErgonomicsRules,
    name: "办公室人体工学",
    desc: "工位设置与坐姿以减少久坐损害的人体工学规则",
    origin: "国际",
    tags: ["健康", "人体工学", "坐姿", "工位", "久坐"]
}

impl OfficeErgonomicsRules {
    /// 座椅与姿态
    pub fn posture(&self) -> Vec<&'static str> {
        vec![
            "坐姿腰背挺直脚踏实地",
            "调节座椅高度使膝约直角",
            "屏幕与眼睛保持适度距离",
            "手臂自然承托不耸肩",
        ]
    }

    /// 工位布置
    pub fn setup(&self) -> Vec<&'static str> {
        vec![
            "显示器顶端与视线平",
            "键盘置于手肘自然高度",
            "鼠标靠近在手边",
            "用支架减少低头",
        ]
    }

    /// 用屏习惯
    pub fn screen(&self) -> Vec<&'static str> {
        vec![
            "保持适当视距与亮度",
            "避免长时间固定姿势",
            "穿插站立或走动",
            "减少不必要炫目的反光",
        ]
    }

    /// 健康预警
    pub fn warning(&self) -> Vec<&'static str> {
        vec![
            "肩颈腕持续酸痛适时调整",
            "出现麻木或刺痛就医",
            "久坐人群落实定时活动",
            "留意不适并主动休息",
        ]
    }
}

impl Rule for OfficeErgonomicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("office_ergonomics")
    }

    fn explain(&self) -> String {
        format!(
            "【办公室人体工学】\n{}",
            [
                format!(
                    "座椅与姿态：\\n{}",
                    self.posture()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "工位布置：\\n{}",
                    self.setup()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "用屏习惯：\\n{}",
                    self.screen()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "健康预警：\\n{}",
                    self.warning()
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
    fn test_officeergonomicsrules_basic() {
        let rules = OfficeErgonomicsRules::new();
        assert_eq!(rules.metadata().name, "办公室人体工学");
        assert!(!rules.posture().is_empty());
        assert!(!rules.setup().is_empty());
        assert!(!rules.screen().is_empty());
        assert!(!rules.warning().is_empty());
    }

    #[test]
    fn test_officeergonomicsrules_validation() {
        let rules = OfficeErgonomicsRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("office_ergonomics"));
    }

    #[test]
    fn test_officeergonomicsrules_explain() {
        let rules = OfficeErgonomicsRules::new();
        let e = rules.explain();
        assert!(e.contains("座椅与姿态"));
        assert!(e.contains("工位布置"));
        assert!(e.contains("用屏习惯"));
    }
}
