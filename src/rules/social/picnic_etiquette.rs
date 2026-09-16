//! 野餐礼仪
//!
//! 公园或郊外野餐时的场地共享、食物分享与环保礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: PicnicEtiquetteRules,
    name: "野餐礼仪",
    desc: "公园或郊外野餐时的场地共享、食物分享与环保礼仪",
    origin: "休闲",
    tags: ["社交", "礼仪", "野餐", "户外", "环保"]
}

impl PicnicEtiquetteRules {
    /// 场地选择
    pub fn site(&self) -> Vec<&'static str> {
        vec![
            "选择允许野餐的开放区域",
            "与相邻人群保持合适距离",
            "避开主要通行步道与车道",
            "在阴凉背风处铺开餐布",
        ]
    }

    /// 分享与空间
    pub fn sharing(&self) -> Vec<&'static str> {
        vec![
            "食物带到中间供大家分享",
            "使用公筷公勺分取餐点",
            "邻座游客交换前先征求同意",
            "分享食物顾及他人忌口",
        ]
    }

    /// 环保收场
    pub fn cleanup(&self) -> Vec<&'static str> {
        vec![
            "离开带走产生的全部垃圾",
            "不遗留食物残渣吸引虫蚁",
            "归还或收好野餐垫与器具",
            "留下干净整洁的场地",
        ]
    }

    /// 同伴礼仪
    pub fn companions(&self) -> Vec<&'static str> {
        vec![
            "照顾同行老幼与残障者就位",
            "不擅自离队久不归队",
            "饮用水彼此分享",
            "餐后收拾帮忙共同清理",
        ]
    }
}

impl Rule for PicnicEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("picnic")
    }

    fn explain(&self) -> String {
        format!(
            "【野餐礼仪】\n{}",
            [
                format!(
                    "场地选择：\\n{}",
                    self.site()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "分享与空间：\\n{}",
                    self.sharing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "环保收场：\\n{}",
                    self.cleanup()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "同伴礼仪：\\n{}",
                    self.companions()
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
    fn test_picnicetiquetterules_basic() {
        let rules = PicnicEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "野餐礼仪");
        assert!(!rules.site().is_empty());
        assert!(!rules.sharing().is_empty());
        assert!(!rules.cleanup().is_empty());
        assert!(!rules.companions().is_empty());
    }

    #[test]
    fn test_picnicetiquetterules_validation() {
        let rules = PicnicEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("picnic"));
    }

    #[test]
    fn test_picnicetiquetterules_explain() {
        let rules = PicnicEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("场地选择"));
        assert!(e.contains("分享与空间"));
        assert!(e.contains("环保收场"));
    }
}
