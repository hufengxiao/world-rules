//! 联棋规则（Pair Go / Rengo）
//!
//! 四人团队合作围棋变体，轮流落子。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: PairGoRules,
    name: "联棋规则",
    desc: "联棋（四人围棋）团队对弈规则",
    origin: "日本",
    tags: ["游戏", "棋类", "围棋", "团队", "变体"]
}

impl PairGoRules {
    /// 获取基本规则章节
    pub fn section_basic(&self) -> Vec<&'static str> {
        vec![
            "棋盘: 标准19×19",
            "参赛者: 四人，分成两队（黑队、白队）",
            "队内配合: 每队两人轮流落子",
            "落子顺序: 黑1→白1→黑2→白2循环",
            "禁止交流: 落子过程不允许语言交流",
        ]
    }

    /// 获取团队规则章节
    pub fn section_team(&self) -> Vec<&'static str> {
        vec![
            "搭档组成: 通常强弱搭配（强+弱）",
            "轮流落子: 队内成员必须轮流",
            "顺序固定: 开局确定顺序后不变",
            "连续落子禁止: 同一人不能连续两步",
            "换人规则: 特殊情况可申请换人",
        ]
    }

    /// 获取交流规则章节
    pub fn section_communication(&self) -> Vec<&'static str> {
        vec![
            "禁止口头交流: 对局中不得讨论棋局",
            "禁止手势: 不得用手势暗示意图",
            "表情中立: 尽量保持表情平静",
            "违规处理: 交流可能判罚或警告",
            "赛后讨论: 终局后可复盘讨论",
        ]
    }

    /// 获取计分规则章节
    pub fn section_scoring(&self) -> Vec<&'static str> {
        vec![
            "计分方式: 与标准围棋相同",
            "贴目: 通常使用标准贴目（6.5/7.5）",
            "团队胜负: 计算团队总得分",
            "积分分配: 联棋赛中每队共享积分",
            "比赛形式: 混双赛（男女搭配）常见",
        ]
    }

    /// 获取赛事规则章节
    pub fn section_competition(&self) -> Vec<&'static str> {
        vec![
            "世界联棋赛: 国际联棋锦标赛",
            "混双赛: 男女搭配的联棋形式",
            "计时规则: 每队共用时间池",
            "暂停规则: 特殊情况可申请暂停",
            "裁判职责: 监控交流违规和落子顺序",
        ]
    }
}

impl Rule for PairGoRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("pair_go")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "联棋规则",
            &[
                ("基本规则", &self.section_basic()),
                ("团队规则", &self.section_team()),
                ("交流规则", &self.section_communication()),
                ("计分规则", &self.section_scoring()),
                ("赛事规则", &self.section_competition()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pair_go_rules() {
        let rules = PairGoRules::new();
        assert_eq!(rules.metadata().name, "联棋规则");
        assert!(!rules.explain().is_empty());
        assert!(rules.explain().contains("四人"));
        assert!(rules.explain().contains("团队"));
    }

    #[test]
    fn test_pair_go_team_rules() {
        let rules = PairGoRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("搭档"));
        assert!(explanation.contains("轮流"));
    }

    #[test]
    fn test_pair_go_communication_rules() {
        let rules = PairGoRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("禁止"));
        assert!(explanation.contains("交流"));
    }

    #[test]
    fn test_pair_go_competition() {
        let rules = PairGoRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("混双"));
    }
}