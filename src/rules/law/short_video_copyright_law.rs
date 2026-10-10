//! 短视频版权
//!
//! 短视频创作与传播中的著作权使用与侵权边界

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ShortVideoCopyrightLawRules,
    name: "短视频版权",
    desc: "短视频创作与传播中的著作权使用与侵权边界",
    origin: "中国",
    tags: ["法律", "版权", "短视频"]
}

impl ShortVideoCopyrightLawRules {
    /// 权利归属
    pub fn ownership(&self) -> Vec<&'static str> {
        vec![
            "原创视频享著作权",
            "拍摄者享相关权利",
            "音乐画面需授权",
            "职务作品依约定",
        ]
    }

    /// 合理使用
    pub fn fair_use(&self) -> Vec<&'static str> {
        vec![
            "适当引用可构成合理使用",
            "评论说明为目的",
            "不得影响原作正常使用",
            "不得损害权利人利益",
        ]
    }

    /// 常见侵权
    pub fn infringement(&self) -> Vec<&'static str> {
        vec![
            "搬运他人视频",
            "擅自使用背景音乐",
            "二次剪辑商用",
            "盗用他人素材",
        ]
    }

    /// 维权方式
    pub fn remedy(&self) -> Vec<&'static str> {
        vec![
            "通知平台下架删除",
            "要求赔偿损失",
            "保留原创证据",
            "必要时提起诉讼",
        ]
    }
}

impl Rule for ShortVideoCopyrightLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("short_video_copyright_law")
    }

    fn explain(&self) -> String {
        format!(
            "【短视频版权】\n{}",
            [
                format!(
                    "权利归属：\\n{}",
                    self.ownership()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "合理使用：\\n{}",
                    self.fair_use()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "常见侵权：\\n{}",
                    self.infringement()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "维权方式：\\n{}",
                    self.remedy()
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
    fn test_shortvideocopyrightlawrules_basic() {
        let rules = ShortVideoCopyrightLawRules::new();
        assert_eq!(rules.metadata().name, "短视频版权");
        assert!(!rules.ownership().is_empty());
        assert!(!rules.fair_use().is_empty());
        assert!(!rules.infringement().is_empty());
        assert!(!rules.remedy().is_empty());
    }

    #[test]
    fn test_shortvideocopyrightlawrules_validation() {
        let rules = ShortVideoCopyrightLawRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::law("short_video_copyright_law")
        );
    }

    #[test]
    fn test_shortvideocopyrightlawrules_explain() {
        let rules = ShortVideoCopyrightLawRules::new();
        let e = rules.explain();
        assert!(e.contains("权利归属"));
        assert!(e.contains("合理使用"));
        assert!(e.contains("常见侵权"));
    }
}
