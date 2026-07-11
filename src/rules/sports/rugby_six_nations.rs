//! 橄榄球六国赛规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

/// 六国赛参赛国家
#[derive(Debug, Clone, Copy)]
pub enum SixNationsTeam {
    /// 英格兰
    England,
    /// 法国
    France,
    /// 爱尔兰
    Ireland,
    /// 意大利
    Italy,
    /// 苏格兰
    Scotland,
    /// 威尔士
    Wales,
}

simple_rule! {
    struct: RugbySixNationsRules,
    name: "橄榄球六国赛规则",
    desc: "欧洲六国橄榄球锦标赛规则",
    origin: "Six Nations",
    tags: ["体育", "橄榄球", "六国赛"]
}

impl RugbySixNationsRules {
    /// 参赛队伍数量
    pub fn team_count(&self) -> u8 {
        6
    }

    /// 参赛国家列表
    pub fn participating_nations(&self) -> Vec<&'static str> {
        vec!["英格兰", "法国", "爱尔兰", "意大利", "苏格兰", "威尔士"]
    }

    /// 比赛赛制
    pub fn match_format(&self) -> Vec<&'static str> {
        vec![
            "每队比赛5场",
            "循环赛制",
            "主场客场轮换",
            "每年2-3月举办",
            "周末比赛",
        ]
    }

    /// 比赛规则
    pub fn game_rules(&self) -> Vec<&'static str> {
        vec![
            "正规时间80分钟",
            "上下半场各40分钟",
            "15人制橄榄球",
            "得分: 达阵5分",
            "追加射门2分",
            "罚踢3分",
            "落踢3分",
        ]
    }

    /// 积分规则
    pub fn points_rules(&self) -> Vec<&'static str> {
        vec![
            "胜: 4分",
            "平: 2分",
            "负: 0分",
            "达阵4次以上奖励1分",
            "失利7分以内奖励1分",
            "大满贯额外3分",
        ]
    }

    /// 冠军判定
    pub fn championship_rules(&self) -> Vec<&'static str> {
        vec![
            "积分最高者夺冠",
            "同分看胜负关系",
            "同分同胜负看达阵数",
            "大满贯(全胜)最高荣誉",
        ]
    }

    /// 特殊奖项
    pub fn special_awards(&self) -> Vec<&'static str> {
        vec![
            "大满贯: 全胜夺冠",
            "三冠: 胜英苏威",
            "木勺: 最后一名",
            " Millennium Trophy",
            "Giuseppe Garibaldi Trophy",
        ]
    }

    /// 获取国家名称
    pub fn team_name(&self, team: SixNationsTeam) -> &'static str {
        match team {
            SixNationsTeam::England => "英格兰",
            SixNationsTeam::France => "法国",
            SixNationsTeam::Ireland => "爱尔兰",
            SixNationsTeam::Italy => "意大利",
            SixNationsTeam::Scotland => "苏格兰",
            SixNationsTeam::Wales => "威尔士",
        }
    }

    /// 计算基础积分
    pub fn calculate_base_points(&self, wins: u8, draws: u8) -> u16 {
        wins as u16 * 4 + draws as u16 * 2
    }

    /// 检查是否有大满贯
    pub fn check_grand_slam(&self, wins: u8) -> bool {
        wins == 5
    }

    /// 检查是否有三冠
    pub fn check_triple_crown(
        &self,
        beat_english: bool,
        beat_scotland: bool,
        beat_wales: bool,
    ) -> bool {
        beat_english && beat_scotland && beat_wales
    }
}

impl Rule for RugbySixNationsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("rugby_six_nations")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "橄榄球六国赛规则",
            &[
                ("参赛国家", &self.participating_nations()),
                ("比赛赛制", &self.match_format()),
                ("比赛规则", &self.game_rules()),
                ("积分规则", &self.points_rules()),
                ("冠军判定", &self.championship_rules()),
                ("特殊奖项", &self.special_awards()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let rules = RugbySixNationsRules::new();
        assert_eq!(rules.metadata().name, "橄榄球六国赛规则");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_team_count() {
        let rules = RugbySixNationsRules::new();
        assert_eq!(rules.team_count(), 6);
        assert_eq!(rules.participating_nations().len(), 6);
    }

    #[test]
    fn test_points_calculation() {
        let rules = RugbySixNationsRules::new();
        assert_eq!(rules.calculate_base_points(4, 1), 18);
        assert_eq!(rules.calculate_base_points(5, 0), 20);
    }

    #[test]
    fn test_grand_slam() {
        let rules = RugbySixNationsRules::new();
        assert!(rules.check_grand_slam(5));
        assert!(!rules.check_grand_slam(4));
    }

    #[test]
    fn test_triple_crown() {
        let rules = RugbySixNationsRules::new();
        assert!(rules.check_triple_crown(true, true, true));
        assert!(!rules.check_triple_crown(true, false, true));
    }
}
