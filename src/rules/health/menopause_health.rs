//! 更年期健康
//!
//! 更年期症状应对、内分泌与生活方式调理

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: MenopauseHealthRules,
    name: "更年期健康",
    desc: "更年期症状应对、内分泌与生活方式调理",
    origin: "医学",
    tags: ["健康", "更年期", "内分泌", "女性"]
}

impl MenopauseHealthRules {
    /// 认识改变
    pub fn understand(&self) -> Vec<&'static str> {
        vec![
            "了解潮热盗汗等常见症状",
            "正视情绪与睡眠变化",
            "月经周期变化为自然",
            "不因症状而困扰自卑",
        ]
    }

    /// 骨钙与运动
    pub fn bone(&self) -> Vec<&'static str> {
        vec![
            "重视补钙与维D",
            "负重锻炼护骨密度",
            "适度力量运动",
            "规律活动减少跌倒",
        ]
    }

    /// 情绪饮食
    pub fn lifestyle(&self) -> Vec<&'static str> {
        vec![
            "均衡饮食控脂限糖",
            "缓解压力调节情绪",
            "保证充足睡眠",
            "规律作息稳定状态",
        ]
    }

    /// 就医调理
    pub fn care(&self) -> Vec<&'static str> {
        vec![
            "症状明显影响生活就医",
            "遵医嘱考虑规范干预",
            "定期体检关注心脏骨密度",
            "用药须经专业评估",
        ]
    }
}

impl Rule for MenopauseHealthRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("menopause")
    }

    fn explain(&self) -> String {
        format!(
            "【更年期健康】\n{}",
            [
                format!(
                    "认识改变：\\n{}",
                    self.understand()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "骨钙与运动：\\n{}",
                    self.bone()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "情绪饮食：\\n{}",
                    self.lifestyle()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "就医调理：\\n{}",
                    self.care()
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
    fn test_menopausehealthrules_basic() {
        let rules = MenopauseHealthRules::new();
        assert_eq!(rules.metadata().name, "更年期健康");
        assert!(!rules.understand().is_empty());
        assert!(!rules.bone().is_empty());
        assert!(!rules.lifestyle().is_empty());
        assert!(!rules.care().is_empty());
    }

    #[test]
    fn test_menopausehealthrules_validation() {
        let rules = MenopauseHealthRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("menopause"));
    }

    #[test]
    fn test_menopausehealthrules_explain() {
        let rules = MenopauseHealthRules::new();
        let e = rules.explain();
        assert!(e.contains("认识改变"));
        assert!(e.contains("骨钙与运动"));
        assert!(e.contains("情绪饮食"));
    }
}
