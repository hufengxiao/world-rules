//! 普通感冒护理
//!
//! 普通感冒的居家护理、缓解症状与预防传播规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: CommonColdCareRules,
    name: "普通感冒护理",
    desc: "普通感冒的居家护理、缓解症状与预防传播规则",
    origin: "医学",
    tags: ["健康", "感冒", "护理", "症状缓解"]
}

impl CommonColdCareRules {
    /// 休息与补水
    pub fn rest(&self) -> Vec<&'static str> {
        vec![
            "保证充足休息避免劳累",
            "多喝温水补充水分",
            "保持居室通风适量",
            "发热时适当减少衣物散热",
        ]
    }

    /// 症状缓解
    pub fn relief(&self) -> Vec<&'static str> {
        vec![
            "鼻塞可用生理盐水冲洗鼻腔",
            "咽喉不适可喝温水或含润喉",
            "发烧持续且高烧时及时就医",
            "不自行滥用抗生素",
        ]
    }

    /// 预防传播
    pub fn prevention(&self) -> Vec<&'static str> {
        vec![
            "打喷嚏咳嗽用手肘遮挡",
            "勤洗手并避免触摸口鼻",
            "居家可适当分开餐具",
            "身感不适时避免去人群密集处",
        ]
    }

    /// 就医信号
    pub fn seek_care(&self) -> Vec<&'static str> {
        vec![
            "持续高烧数天不退就医",
            "出现明显胸痛或呼吸困难",
            "伴有严重嗜睡或意识改变",
            "老人幼儿基础病者早就医",
        ]
    }
}

impl Rule for CommonColdCareRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("common_cold")
    }

    fn explain(&self) -> String {
        format!(
            "【普通感冒护理】\n{}",
            [
                format!(
                    "休息与补水：\\n{}",
                    self.rest()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "症状缓解：\\n{}",
                    self.relief()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "预防传播：\\n{}",
                    self.prevention()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "就医信号：\\n{}",
                    self.seek_care()
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
    fn test_commoncoldcarerules_basic() {
        let rules = CommonColdCareRules::new();
        assert_eq!(rules.metadata().name, "普通感冒护理");
        assert!(!rules.rest().is_empty());
        assert!(!rules.relief().is_empty());
        assert!(!rules.prevention().is_empty());
        assert!(!rules.seek_care().is_empty());
    }

    #[test]
    fn test_commoncoldcarerules_validation() {
        let rules = CommonColdCareRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("common_cold"));
    }

    #[test]
    fn test_commoncoldcarerules_explain() {
        let rules = CommonColdCareRules::new();
        let e = rules.explain();
        assert!(e.contains("休息与补水"));
        assert!(e.contains("症状缓解"));
        assert!(e.contains("预防传播"));
    }
}
