//! 一色围棋规则（One-Color Go）
//!
//! 双方使用同色棋子，凭记忆和逻辑判断归属。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: OneColorGoRules,
    name: "一色围棋规则",
    desc: "一色围棋（同色棋子）对弈规则",
    origin: "日本",
    tags: ["游戏", "棋类", "围棋", "变体", "挑战"]
}

impl OneColorGoRules {
    /// 获取基本规则章节
    pub fn section_basic(&self) -> Vec<&'static str> {
        vec![
            "棋盘: 标准19×19",
            "棋子: 双方使用相同颜色的棋子",
            "落子: 黑先白后，但棋子外观相同",
            "区分方式: 仅凭记忆和落子顺序",
            "目的: 训练记忆力和棋局推理能力",
        ]
    }

    /// 获取对弈规则章节
    pub fn section_play(&self) -> Vec<&'static str> {
        vec![
            "落子声明: 每次落子需明确声明\"我方落子\"",
            "棋局记录: 保持完整记录以验证归属",
            "提子判断: 依赖记录确认被提棋子归属",
            "争议处理: 依靠对局记录解决归属争议",
            "pass声明: 明确声明pass并记录",
        ]
    }

    /// 获取计分规则章节
    pub fn section_scoring(&self) -> Vec<&'static str> {
        vec![
            "终局判定: 双方pass后，根据记录区分",
            "棋子归属: 根据落子顺序逐一确定",
            "计分方式: 与标准围棋相同",
            "贴目: 适用于一色围棋",
            "验证: 终局时在棋盘上标记双方棋子",
        ]
    }

    /// 获取训练价值章节
    pub fn section_training(&self) -> Vec<&'static str> {
        vec![
            "提升记忆: 强迫记住每步棋",
            "逻辑推理: 从棋形推断棋子归属",
            "棋形识别: 训练对棋形的敏感度",
            "大师训练: 专业棋手常用训练方法",
            "难度等级: 高于盲棋的极限挑战",
        ]
    }
}

impl Rule for OneColorGoRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("one_color_go")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "一色围棋规则",
            &[
                ("基本规则", &self.section_basic()),
                ("对弈规则", &self.section_play()),
                ("计分规则", &self.section_scoring()),
                ("训练价值", &self.section_training()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_color_go_rules() {
        let rules = OneColorGoRules::new();
        assert_eq!(rules.metadata().name, "一色围棋规则");
        assert!(!rules.explain().is_empty());
        assert!(rules.explain().contains("同色"));
        assert!(rules.explain().contains("记忆"));
    }

    #[test]
    fn test_one_color_go_play_rules() {
        let rules = OneColorGoRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("落子声明"));
        assert!(explanation.contains("记录"));
    }

    #[test]
    fn test_one_color_go_training_value() {
        let rules = OneColorGoRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("记忆"));
        assert!(explanation.contains("推理"));
    }
}