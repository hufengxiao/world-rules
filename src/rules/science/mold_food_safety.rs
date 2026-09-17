//! 发霉食品科学常识
//!
//! 食物发霉变质的识别、保存与安全处理的科学常识

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: MoldFoodSafetyRules,
    name: "发霉食品科学常识",
    desc: "食物发霉变质的识别、保存与安全处理的科学常识",
    origin: "国际",
    tags: ["科学", "发霉", "食品", "安全"]
}

impl MoldFoodSafetyRules {
    /// 识别发霉
    pub fn detect(&self) -> Vec<&'static str> {
        vec![
            "观察表面霉斑异味",
            "霉菌在温湿生长",
            "隐性毒素难去除",
            "发霉多不可只切掉",
        ]
    }

    /// 变质判定
    pub fn spoil(&self) -> Vec<&'static str> {
        vec![
            "水分多的食物易变质",
            "腐败气味即弃",
            "变色粘滑谨慎",
            "不冒险食用",
        ]
    }

    /// 正确保存
    pub fn store(&self) -> Vec<&'static str> {
        vec![
            "冷藏冷冻抑制霉变",
            "密封防潮",
            "及时食用保质",
            "剩菜及时处理",
        ]
    }

    /// 安全处置
    pub fn dispose(&self) -> Vec<&'static str> {
        vec![
            "发霉食物丢弃",
            "不削去霉点食用",
            "清理器皿防残留",
            "注意防误食",
        ]
    }
}

impl Rule for MoldFoodSafetyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("mold_food")
    }

    fn explain(&self) -> String {
        format!(
            "【发霉食品科学常识】\n{}",
            [
                format!(
                    "识别发霉：\\n{}",
                    self.detect()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "变质判定：\\n{}",
                    self.spoil()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "正确保存：\\n{}",
                    self.store()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "安全处置：\\n{}",
                    self.dispose()
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
    fn test_moldfoodsafetyrules_basic() {
        let rules = MoldFoodSafetyRules::new();
        assert_eq!(rules.metadata().name, "发霉食品科学常识");
        assert!(!rules.detect().is_empty());
        assert!(!rules.spoil().is_empty());
        assert!(!rules.store().is_empty());
        assert!(!rules.dispose().is_empty());
    }

    #[test]
    fn test_moldfoodsafetyrules_validation() {
        let rules = MoldFoodSafetyRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("mold_food"));
    }

    #[test]
    fn test_moldfoodsafetyrules_explain() {
        let rules = MoldFoodSafetyRules::new();
        let e = rules.explain();
        assert!(e.contains("识别发霉"));
        assert!(e.contains("变质判定"));
        assert!(e.contains("正确保存"));
    }
}
