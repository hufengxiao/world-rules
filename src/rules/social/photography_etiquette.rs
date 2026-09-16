//! 摄影礼仪
//!
//! 街拍、活动与公共场合摄影的隐私与版权礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: PhotographyEtiquetteRules,
    name: "摄影礼仪",
    desc: "街拍、活动与公共场合摄影的隐私与版权礼仪",
    origin: "国际",
    tags: ["社交", "礼仪", "摄影", "隐私"]
}

impl PhotographyEtiquetteRules {
    /// 拍摄他人
    pub fn portrait(&self) -> Vec<&'static str> {
        vec![
            "拍摄他人前征得同意",
            "尊重不愿被拍的意愿",
            "不偷拍私密场合",
            "人流中尽量避开正面特写",
        ]
    }

    /// 公共场所
    pub fn public(&self) -> Vec<&'static str> {
        vec![
            "在允许摄影的场所拍摄",
            "使用闪光灯注意他人",
            "拍摄前了解场地规定",
            "不遮挡重要展品或他人视线",
        ]
    }

    /// 版权与发布
    pub fn copyright(&self) -> Vec<&'static str> {
        vec![
            "商用照片取得模特授权",
            "分享时标注来源作者",
            "尊重创作者署名权",
            "不冒认他人作品",
        ]
    }

    /// 编辑与使用
    pub fn usage(&self) -> Vec<&'static str> {
        vec![
            "编辑照片尊重客观事实",
            "未经许可不使用他人作品",
            "涉及未成年人注意隐私",
        ]
    }
}

impl Rule for PhotographyEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("photography")
    }

    fn explain(&self) -> String {
        format!(
            "【摄影礼仪】\n{}",
            [
                format!(
                    "拍摄他人：\\n{}",
                    self.portrait()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "公共场所：\\n{}",
                    self.public()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "版权与发布：\\n{}",
                    self.copyright()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "编辑与使用：\\n{}",
                    self.usage()
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
    fn test_photographyetiquetterules_basic() {
        let rules = PhotographyEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "摄影礼仪");
        assert!(!rules.portrait().is_empty());
        assert!(!rules.public().is_empty());
        assert!(!rules.copyright().is_empty());
        assert!(!rules.usage().is_empty());
    }

    #[test]
    fn test_photographyetiquetterules_validation() {
        let rules = PhotographyEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("photography"));
    }

    #[test]
    fn test_photographyetiquetterules_explain() {
        let rules = PhotographyEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("拍摄他人"));
        assert!(e.contains("公共场所"));
        assert!(e.contains("版权与发布"));
    }
}
