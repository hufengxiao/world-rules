//! 烫伤烧伤急救
//!
//! 烫烧伤的降温、处理与就医判断

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: BurnScaldingAidRules,
    name: "烫伤烧伤急救",
    desc: "烫烧伤的降温、处理与就医判断",
    origin: "医学",
    tags: ["健康", "烫伤", "急救", "烧伤"]
}

impl BurnScaldingAidRules {
    /// 立即降温
    pub fn cool(&self) -> Vec<&'static str> {
        vec![
            "冷水冲洗伤口",
            "持续流动水降温",
            "时长足够缓解",
            "勿冰敷直贴",
        ]
    }

    /// 正确护理
    pub fn care(&self) -> Vec<&'static str> {
        vec!["不弄破水泡", "不涂牙膏酱", "盖无菌纱布", "保持清洁"]
    }

    /// 就医判断
    pub fn visit(&self) -> Vec<&'static str> {
        vec![
            "面积大及时就医",
            "深度烧伤就医",
            "面部手踝就医",
            "感染迹象就医",
        ]
    }

    /// 预防烫伤
    pub fn prevent(&self) -> Vec<&'static str> {
        vec!["热液远离儿童", "餐具防滑稳", "厨房小心热油", "热水试温再动"]
    }
}

impl Rule for BurnScaldingAidRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("burn_aid")
    }

    fn explain(&self) -> String {
        format!(
            "【烫伤烧伤急救】\n{}",
            [
                format!(
                    "立即降温：\\n{}",
                    self.cool()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "正确护理：\\n{}",
                    self.care()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "就医判断：\\n{}",
                    self.visit()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "预防烫伤：\\n{}",
                    self.prevent()
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
    fn test_burnscaldingaidrules_basic() {
        let rules = BurnScaldingAidRules::new();
        assert_eq!(rules.metadata().name, "烫伤烧伤急救");
        assert!(!rules.cool().is_empty());
        assert!(!rules.care().is_empty());
        assert!(!rules.visit().is_empty());
        assert!(!rules.prevent().is_empty());
    }

    #[test]
    fn test_burnscaldingaidrules_validation() {
        let rules = BurnScaldingAidRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("burn_aid"));
    }

    #[test]
    fn test_burnscaldingaidrules_explain() {
        let rules = BurnScaldingAidRules::new();
        let e = rules.explain();
        assert!(e.contains("立即降温"));
        assert!(e.contains("正确护理"));
        assert!(e.contains("就医判断"));
    }
}
