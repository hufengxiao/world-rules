//! 气道异物窒息急救
//!
//! 气道被食物或异物阻塞时的识别与海姆立克急救规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ChokingAirwayReliefRules,
    name: "气道异物窒息急救",
    desc: "气道被食物或异物阻塞时的识别与海姆立克急救规则",
    origin: "医学",
    tags: ["健康", "急救", "海姆立克", "窒息", "气道"]
}

impl ChokingAirwayReliefRules {
    /// 识别窒息
    pub fn signs(&self) -> Vec<&'static str> {
        vec![
            "识别用手抓喉咙紧憋不出声",
            "能咳嗽说话则鼓励继续咳",
            "无法呼吸说话提示阻塞",
            "观察面色发青唇紫",
        ]
    }

    /// 成人海姆立克
    pub fn heimlich(&self) -> Vec<&'static str> {
        vec![
            "站于患者背后环抱其腰部",
            "拳心置于脐上胸腹之间",
            "向内向上快速冲击",
            "反复直到异物排出或苏醒",
        ]
    }

    /// 自救与儿童
    pub fn self_child(&self) -> Vec<&'static str> {
        vec![
            "无他人时借椅背自救冲击",
            "儿童用适宜力量拍背",
            "婴幼儿薄垫拍背压胸",
            "不盲目掏取可观察到异物",
        ]
    }

    /// 后续处理
    pub fn follow(&self) -> Vec<&'static str> {
        vec![
            "排出后仍建议就医复检",
            "昏迷者平放并呼急救",
            "持续无改善立即送医",
            "急救后心理安抚",
        ]
    }
}

impl Rule for ChokingAirwayReliefRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("choking_airway")
    }

    fn explain(&self) -> String {
        format!(
            "【气道异物窒息急救】\n{}",
            [
                format!(
                    "识别窒息：\\n{}",
                    self.signs()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "成人海姆立克：\\n{}",
                    self.heimlich()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "自救与儿童：\\n{}",
                    self.self_child()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "后续处理：\\n{}",
                    self.follow()
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
    fn test_chokingairwayreliefrules_basic() {
        let rules = ChokingAirwayReliefRules::new();
        assert_eq!(rules.metadata().name, "气道异物窒息急救");
        assert!(!rules.signs().is_empty());
        assert!(!rules.heimlich().is_empty());
        assert!(!rules.self_child().is_empty());
        assert!(!rules.follow().is_empty());
    }

    #[test]
    fn test_chokingairwayreliefrules_validation() {
        let rules = ChokingAirwayReliefRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("choking_airway"));
    }

    #[test]
    fn test_chokingairwayreliefrules_explain() {
        let rules = ChokingAirwayReliefRules::new();
        let e = rules.explain();
        assert!(e.contains("识别窒息"));
        assert!(e.contains("成人海姆立克"));
        assert!(e.contains("自救与儿童"));
    }
}
