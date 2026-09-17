//! 车站候车礼仪
//!
//! 车站候车商圈、出行与公共秩序礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: StationHallMannerRules,
    name: "车站候车礼仪",
    desc: "车站候车商圈、出行与公共秩序礼仪",
    origin: "国际",
    tags: ["社交", "礼仪", "车站", "候车"]
}

impl StationHallMannerRules {
    /// 候车规范
    pub fn wait(&self) -> Vec<&'static str> {
        vec![
            "在候车区有序等候",
            "不越过安全黄线",
            "列车到达先下后上",
            "有序排队登车",
        ]
    }

    /// 行李礼节
    pub fn luggage(&self) -> Vec<&'static str> {
        vec![
            "随身行李归置好",
            "不占通道座位",
            "行李不碰他人",
            "大件托运依规",
        ]
    }

    /// 安静文明
    pub fn quiet(&self) -> Vec<&'static str> {
        vec![
            "不大声喧哗播放",
            "保持区域整洁",
            "不乱扔垃圾",
            "垃圾归置妥当",
        ]
    }

    /// 帮助他人
    pub fn help(&self) -> Vec<&'static str> {
        vec!["帮助有困难旅客", "为紧赶者让行", "答疑友善", "体谅他人匆匆"]
    }
}

impl Rule for StationHallMannerRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("station_hall")
    }

    fn explain(&self) -> String {
        format!(
            "【车站候车礼仪】\n{}",
            [
                format!(
                    "候车规范：\\n{}",
                    self.wait()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "行李礼节：\\n{}",
                    self.luggage()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "安静文明：\\n{}",
                    self.quiet()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "帮助他人：\\n{}",
                    self.help()
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
    fn test_stationhallmannerrules_basic() {
        let rules = StationHallMannerRules::new();
        assert_eq!(rules.metadata().name, "车站候车礼仪");
        assert!(!rules.wait().is_empty());
        assert!(!rules.luggage().is_empty());
        assert!(!rules.quiet().is_empty());
        assert!(!rules.help().is_empty());
    }

    #[test]
    fn test_stationhallmannerrules_validation() {
        let rules = StationHallMannerRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("station_hall"));
    }

    #[test]
    fn test_stationhallmannerrules_explain() {
        let rules = StationHallMannerRules::new();
        let e = rules.explain();
        assert!(e.contains("候车规范"));
        assert!(e.contains("行李礼节"));
        assert!(e.contains("安静文明"));
    }
}
