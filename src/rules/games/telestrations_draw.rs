//! 传画猜词（Telestrations）
//!
//! 传画猜词是聚会传阅游戏，玩家依次把词语画成图、再据图猜词，如此交替传递，最终对比首尾内容引发笑料。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: TelestrationsDrawRules,
    name: "传画猜词（Telestrations）",
    desc: "传画猜词是聚会传阅游戏，玩家依次把词语画成图、再据图猜词，如此交替传递，最终对比首尾内容引发笑料。",
    origin: "由美式聚会游戏「电话图画」演化而来",
    tags: ["聚会", "绘画", "猜词", "桌面游戏"]
}

impl TelestrationsDrawRules {
    /// 准备
    pub fn setup(&self) -> Vec<&'static str> {
        vec![
            "每人一本可翻页画册",
            "各自抽一个词语",
            "在首页写下或默记词语",
            "画册按序向右传递",
        ]
    }

    /// 传阅流程
    pub fn passing(&self) -> Vec<&'static str> {
        vec![
            "据他人画作猜词写在下一页",
            "据他人词语作画在下一页",
            "画与猜交替进行",
            "翻页遮住前文只看得上一步",
        ]
    }

    /// 作画要求
    pub fn drawing(&self) -> Vec<&'static str> {
        vec![
            "不得书写文字或数字",
            "只能以图画表达含义",
            "限时作画不磨蹭",
            "画技高低不影响得分",
        ]
    }

    /// 揭晓与计分
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "画册回到本人逐页揭晓",
            "对比首词与末猜是否一致",
            "猜对可得分",
            "笑点越离谱越有趣",
        ]
    }
}

impl Rule for TelestrationsDrawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("telestrations")
    }

    fn explain(&self) -> String {
        format!(
            "【传画猜词（Telestrations）】\n{}",
            [
                format!(
                    "准备：\\n{}",
                    self.setup()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "传阅流程：\\n{}",
                    self.passing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "作画要求：\\n{}",
                    self.drawing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "揭晓与计分：\\n{}",
                    self.scoring()
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
    fn test_telestrationsdrawrules_basic() {
        let rules = TelestrationsDrawRules::new();
        assert_eq!(rules.metadata().name, "传画猜词（Telestrations）");
        assert!(!rules.setup().is_empty());
        assert!(!rules.passing().is_empty());
        assert!(!rules.drawing().is_empty());
        assert!(!rules.scoring().is_empty());
    }

    #[test]
    fn test_telestrationsdrawrules_validation() {
        let rules = TelestrationsDrawRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("telestrations"));
    }

    #[test]
    fn test_telestrationsdrawrules_explain() {
        let rules = TelestrationsDrawRules::new();
        let e = rules.explain();
        assert!(e.contains("准备"));
        assert!(e.contains("传阅流程"));
        assert!(e.contains("作画要求"));
    }
}
