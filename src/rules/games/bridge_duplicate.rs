//! 复式桥牌规则 (Duplicate Bridge)
//!
//! 复式桥牌是正式比赛的标准形式，多桌同时进行相同牌局的比赛。
//! 每副牌在不同桌重复进行，比较各桌成绩来决定胜负。
//!
//! # Examples
//!
//! ```rust
//! use world_rules::rules::games::bridge_duplicate::BridgeDuplicateRules;
//! use world_rules::rules::core::Rule;
//!
//! let rules = BridgeDuplicateRules::new();
//! assert_eq!(rules.name(), "复式桥牌规则");
//! assert!(!rules.explain().is_empty());
//! ```

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: BridgeDuplicateRules,
    name: "复式桥牌规则",
    desc: "复式桥牌（Duplicate Bridge）竞赛规则",
    origin: "国际",
    tags: ["游戏", "卡牌", "桥牌", "竞技"],
}

impl BridgeDuplicateRules {
    /// 比赛概述
    pub fn overview(&self) -> Vec<&'static str> {
        vec![
            "复式桥牌是正式比赛的标准形式",
            "相同牌局在多桌重复进行",
            "比较各桌成绩决定胜负",
            "减少运气的因素，强调技术",
        ]
    }

    /// 比赛形式
    pub fn tournament_formats(&self) -> Vec<&'static str> {
        vec![
            "队式赛(Team): 两队对抗，计算IMP分差",
            "双人赛(Pairs): 多对组合排名，按比赛分计分",
            "瑞士移位赛: 根据积分动态配对",
            "淘汰赛: 单败淘汰制",
        ]
    }

    /// 计分方式
    pub fn scoring_methods(&self) -> Vec<&'static str> {
        vec![
            "IMP制(International Match Points): 换算国际比赛分",
            "比赛分制(Match Points): 百分比排名",
            "总墩分制: 按原始墩分计算",
            "VP制(Victory Points): 队式赛专用换算",
        ]
    }

    /// 比赛程序
    pub fn procedures(&self) -> Vec<&'static str> {
        vec![
            "使用预制牌或电脑发牌",
            "牌面记录表记录每副牌结果",
            "叫牌须使用叫牌卡或叫牌盒",
            "开室和闭室同时进行",
            "比赛结束后统一计分",
        ]
    }

    /// 叫牌体系规定
    pub fn system_regulations(&self) -> Vec<&'static str> {
        vec![
            "叫牌体系须预先注册",
            "约定叫须有解释义务",
            "心理叫牌有限制",
            "禁止非法信号系统",
        ]
    }

    /// 时限规定
    pub fn time_controls(&self) -> Vec<&'static str> {
        vec![
            "每副牌限时7-9分钟",
            "叫牌和打牌共用时间",
            "超时可能被判罚分",
            "比赛总时长视赛制而定",
        ]
    }
}

impl Rule for BridgeDuplicateRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("bridge_duplicate")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "复式桥牌规则 (Duplicate Bridge)",
            &[
                ("概述", &self.overview()),
                ("比赛形式", &self.tournament_formats()),
                ("计分方式", &self.scoring_methods()),
                ("比赛程序", &self.procedures()),
                ("叫牌体系规定", &self.system_regulations()),
                ("时限规定", &self.time_controls()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bridge_duplicate_rules() {
        let rules = BridgeDuplicateRules::new();
        assert_eq!(rules.metadata().name, "复式桥牌规则");
        assert!(!rules.explain().is_empty());
        assert!(!rules.tournament_formats().is_empty());
    }
}
