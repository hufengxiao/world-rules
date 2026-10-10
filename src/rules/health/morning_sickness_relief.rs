//! 孕吐缓解
//!
//! 孕期恶心呕吐的饮食与生活缓解方法

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: MorningSicknessReliefRules,
    name: "孕吐缓解",
    desc: "孕期恶心呕吐的饮食与生活缓解方法",
    origin: "中国",
    tags: ["孕产", "缓解", "女性"]
}

impl MorningSicknessReliefRules {
    /// 饮食调整
    pub fn diet(&self) -> Vec<&'static str> {
        vec![
            "少食多餐避免空腹",
            "清淡易消化为宜",
            "晨起先吃几片饼干",
            "避免油腻与刺激气味",
        ]
    }

    /// 生活调理
    pub fn lifestyle(&self) -> Vec<&'static str> {
        vec![
            "充分休息减少疲劳",
            "闻柠檬姜味缓解恶心",
            "室内通风减少异味",
            "起床动作缓慢",
        ]
    }

    /// 缓解技巧
    pub fn tips(&self) -> Vec<&'static str> {
        vec![
            "含姜糖或喝姜茶",
            "小口多次补水",
            "避免过饱或过饥",
            "适度散步分散注意",
        ]
    }

    /// 就医提示
    pub fn warning(&self) -> Vec<&'static str> {
        vec![
            "无法进食须就医",
            "体重明显下降就诊",
            "严重脱水应输液",
            "剧吐需住院治疗",
        ]
    }
}

impl Rule for MorningSicknessReliefRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("morning_sickness_relief")
    }

    fn explain(&self) -> String {
        format!(
            "【孕吐缓解】\n{}",
            [
                format!(
                    "饮食调整：\\n{}",
                    self.diet()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "生活调理：\\n{}",
                    self.lifestyle()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "缓解技巧：\\n{}",
                    self.tips()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "就医提示：\\n{}",
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
    fn test_morningsicknessreliefrules_basic() {
        let rules = MorningSicknessReliefRules::new();
        assert_eq!(rules.metadata().name, "孕吐缓解");
        assert!(!rules.diet().is_empty());
        assert!(!rules.lifestyle().is_empty());
        assert!(!rules.tips().is_empty());
        assert!(!rules.warning().is_empty());
    }

    #[test]
    fn test_morningsicknessreliefrules_validation() {
        let rules = MorningSicknessReliefRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::health("morning_sickness_relief")
        );
    }

    #[test]
    fn test_morningsicknessreliefrules_explain() {
        let rules = MorningSicknessReliefRules::new();
        let e = rules.explain();
        assert!(e.contains("饮食调整"));
        assert!(e.contains("生活调理"));
        assert!(e.contains("缓解技巧"));
    }
}
