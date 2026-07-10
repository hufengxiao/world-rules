//! 盲围棋规则（Blind Go）
//!
//! 双方在盲棋状态下对弈，记忆与推理的极限挑战。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: BlindGoRules,
    name: "盲围棋规则",
    desc: "盲围棋（盲棋）对弈规则",
    origin: "中国",
    tags: ["游戏", "棋类", "围棋", "盲棋", "挑战"]
}

impl BlindGoRules {
    /// 获取基本规则章节
    pub fn section_basic(&self) -> Vec<&'static str> {
        vec![
            "使用标准19×19棋盘",
            "双方均不看棋盘，完全凭记忆下棋",
            "黑先白后，用口头坐标报点落子",
            "坐标格式: 字母+数字（如 Q16）",
            "裁判或助手负责在棋盘上落子",
        ]
    }

    /// 获取特殊规则章节
    pub fn section_special(&self) -> Vec<&'static str> {
        vec![
            "非法落子: 若落点已有子，需重新报点",
            "提子后裁判告知双方被提棋子位置",
            "pass: 明确声明\"pass\"或\"虚手\"",
            "记忆辅助: 部分版本允许口头询问棋局状态",
            "严格版本: 禁止询问，完全依靠记忆",
        ]
    }

    /// 获取挑战等级章节
    pub fn section_challenge(&self) -> Vec<&'static str> {
        vec![
            "普通盲棋: 可询问棋盘某点状态",
            "完全盲棋: 禁止任何询问",
            "大师级盲棋: 不告知提子位置",
            "世界纪录: 专业棋手可盲下多盘",
            "武宫正树曾盲下19×19完整一局",
        ]
    }

    /// 获取赛事规则章节
    pub fn section_competition(&self) -> Vec<&'static str> {
        vec![
            "裁判职责: 验证落子合法性、记录棋局",
            "时间限制: 每方通常更长时间思考",
            "犯规: 非法落子可能受罚",
            "对局记录: 裁判全程记录并告知结果",
            "争议处理: 裁判有权暂停并核实棋局",
        ]
    }
}

impl Rule for BlindGoRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("blind_go")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "盲围棋规则",
            &[
                ("基本规则", &self.section_basic()),
                ("特殊规则", &self.section_special()),
                ("挑战等级", &self.section_challenge()),
                ("赛事规则", &self.section_competition()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blind_go_rules() {
        let rules = BlindGoRules::new();
        assert_eq!(rules.metadata().name, "盲围棋规则");
        assert!(!rules.explain().is_empty());
        assert!(rules.explain().contains("19×19"));
        assert!(rules.explain().contains("记忆"));
        assert!(rules.explain().contains("坐标"));
    }

    #[test]
    fn test_blind_go_special_rules() {
        let rules = BlindGoRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("非法落子"));
        assert!(explanation.contains("pass"));
    }
}
