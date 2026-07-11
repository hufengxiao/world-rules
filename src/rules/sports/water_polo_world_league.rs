//! 水球世界联赛规则
//!
//! 水球世界联赛是世界泳联最高水平水球联赛。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 水球世界联赛规则
pub struct WaterPoloWorldLeagueRules {
    metadata: RuleMetadata,
}

impl WaterPoloWorldLeagueRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("水球世界联赛规则", "世界泳联水球世界联赛竞赛规则")
                .with_origin("World Aquatics")
                .with_tags(vec!["体育".into(), "水上".into(), "水球".into()]),
        }
    }

    /// 联赛结构
    pub fn league_structure(&self) -> Vec<&'static str> {
        vec![
            "男子世界联赛: 16支球队",
            "女子世界联赛: 12支球队",
            "分组赛制: 4组每组4队",
            "淘汰赛: 四分之一决赛",
            "决赛阶段: 半决赛和决赛",
            "年度积分排名",
        ]
    }

    /// 比赛时间
    pub fn match_duration(&self) -> Vec<&'static str> {
        vec![
            "四节比赛: 每节8分钟",
            "有效时间制(停表)",
            "节间休息: 2分钟",
            "中场休息: 3分钟",
            "加时赛: 两节各3分钟",
            "点球决胜: 5轮点球",
        ]
    }

    /// 场地规格
    pub fn pool_dimensions(&self) -> Vec<&'static str> {
        vec![
            "泳池长度: 25-30米",
            "泳池宽度: 20米",
            "水深: 至少2米",
            "球门高度: 90厘米",
            "球门宽度: 3米",
            "标记线: 2米、5米、7米",
        ]
    }

    /// 队员配置
    pub fn team_composition(&self) -> Vec<&'static str> {
        vec![
            "场上队员: 7人(含守门员)",
            "替补队员: 最多6人",
            "换人次数不限",
            "可在比赛进行中换人",
            "守门员帽: 红色",
            "队员帽号: 1-13号",
        ]
    }

    /// 得分规则
    pub fn scoring_rules(&self) -> Vec<&'static str> {
        vec![
            "球完全越过球门线得1分",
            "射门前最多持球35秒",
            "30秒进攻时限",
            "可在任何位置射门",
            "得分多者获胜",
            "加时赛计分相同",
        ]
    }

    /// 犯规与处罚
    pub fn fouls_penalties(&self) -> Vec<&'static str> {
        vec![
            "普通犯规: 自由球",
            "严重犯规: 罚出场20秒",
            "暴力犯规: 红牌驱逐",
            "5米内犯规: 点球",
            "累计3次严重犯规: 罚出场",
            "犯规累计制",
        ]
    }

    /// 联赛积分
    pub fn points_system(&self) -> Vec<&'static str> {
        vec![
            "胜: 3分",
            "平: 1分",
            "负: 0分",
            "加时胜: 2分",
            "加时负: 1分",
            "总积分决定排名",
        ]
    }

    /// 晋级规则
    pub fn advancement_rules(&self) -> Vec<&'static str> {
        vec![
            "小组前2名晋级淘汰赛",
            "积分相同看胜负关系",
            "次看净胜球",
            "最后看总得分",
            "淘汰赛单场决胜",
            "决赛中立场地",
        ]
    }

    /// 资格要求
    pub fn qualification_requirements(&self) -> Vec<&'static str> {
        vec![
            "国家级协会认证",
            "世界排名或资格赛",
            "地区名额分配",
            "上届成绩保障名额",
            "主办国自动资格",
            "年龄限制: 18岁以上",
        ]
    }
}

impl Default for WaterPoloWorldLeagueRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for WaterPoloWorldLeagueRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("water_polo_world_league")
    }

    fn explain(&self) -> String {
        format!(
            "【水球世界联赛规则】\n\n\
            联赛结构:\n{}\n\n\
            比赛时间:\n{}\n\n\
            积分系统:\n{}\n",
            self.league_structure()
                .iter()
                .map(|l| format!("  • {}", l))
                .collect::<Vec<_>>()
                .join("\n"),
            self.match_duration()
                .iter()
                .map(|m| format!("  • {}", m))
                .collect::<Vec<_>>()
                .join("\n"),
            self.points_system()
                .iter()
                .map(|p| format!("  • {}", p))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn water_polo_world_league_rules_basic() {
        let rules = WaterPoloWorldLeagueRules::new();
        assert_eq!(rules.metadata().name, "水球世界联赛规则");
        assert!(!rules.explain().is_empty());
        assert!(matches!(rules.category(), RuleCategory::Sports(_)));
    }

    #[test]
    fn water_polo_world_league_structure() {
        let rules = WaterPoloWorldLeagueRules::new();
        let structure = rules.league_structure();
        assert!(structure.iter().any(|s| s.contains("男子")));
        assert!(structure.iter().any(|s| s.contains("女子")));
        assert!(structure.len() >= 6);
    }

    #[test]
    fn water_polo_world_league_points() {
        let rules = WaterPoloWorldLeagueRules::new();
        let points = rules.points_system();
        assert!(points.iter().any(|p| p.contains("胜")));
        assert!(points.iter().any(|p| p.contains("3分")));
        assert!(points.len() >= 6);
    }
}