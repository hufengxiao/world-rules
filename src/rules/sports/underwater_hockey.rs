//! 水下曲棍球规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 水下曲棍球规则 (CMAS)
pub struct UnderwaterHockeyRules {
    metadata: RuleMetadata,
}

impl UnderwaterHockeyRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("水下曲棍球规则", "CMAS水下曲棍球竞赛规则")
                .with_origin("国际")
                .with_tags(vec![
                    "体育".into(),
                    "水上".into(),
                    "潜水".into(),
                    "团队".into(),
                ]),
        }
    }

    /// 比赛场地
    pub fn playing_area(&self) -> Vec<&'static str> {
        vec![
            "泳池长度: 21-25米",
            "泳池宽度: 12-15米",
            "水深: 2.0-3.65米",
            "球门: 两端各一个，宽3米",
            "底线标志: 明确标示",
        ]
    }

    /// 比赛时间
    pub fn match_duration(&self) -> Vec<&'static str> {
        vec![
            "上半场: 15分钟",
            "中场休息: 3分钟",
            "下半场: 15分钟",
            "加时赛: 必要时进行",
            "暂停: 每队每半场1次",
        ]
    }

    /// 球队组成
    pub fn team_composition(&self) -> Vec<&'static str> {
        vec![
            "场上队员: 每队6人",
            "替补队员: 最多4人",
            "无限次换人: 随时可换",
            "换人区: 指定区域换人",
            "队长: 指定1人",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "面镜: 透明镜片",
            "呼吸管: 简单设计，无阀门",
            "脚蹼: 统一规格",
            "球杆: 长度不超过350mm",
            "手套: 保护性手套",
            "泳帽: 带护耳",
            "水球: 铅芯塑料球",
        ]
    }

    /// 比赛规则
    pub fn game_rules(&self) -> Vec<&'static str> {
        vec![
            "目标: 将球推入对方球门",
            "得分: 球完全越过球门线",
            "开球: 轮流开球",
            "不得抓球: 必须用球杆击球",
            "不得阻挡: 禁止身体阻挡",
        ]
    }

    /// 犯规行为
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "抓球: 用手或身体控球",
            "阻挡: 故意阻挡对手",
            "攻击性接触: 危险动作",
            "干扰装备: 干扰对手装备",
            "过度停留: 长时间不下潜",
        ]
    }

    /// 处罚规则
    pub fn penalties(&self) -> Vec<&'static str> {
        vec![
            "轻微犯规: 交换球权",
            "重大犯规: 短暂罚下(2分钟)",
            "严重犯规: 直接罚下",
            "累积犯规: 队员警告",
            "不当行为: 技术犯规",
        ]
    }

    /// 裁判职责
    pub fn referees(&self) -> Vec<&'static str> {
        vec![
            "主裁判: 水下和水面各1人",
            "边裁判: 2人协助判断",
            "计时员: 记录比赛时间",
            "记分员: 记录比分和犯规",
            "视频回放: 可用于争议判罚",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "禁止屏气过久: 安全第一",
            "紧急上浮: 允许随时上浮",
            "医疗待命: 现场医疗支持",
            "装备检查: 赛前强制检查",
            "替换规则: 允许受伤替换",
        ]
    }
}

impl Default for UnderwaterHockeyRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for UnderwaterHockeyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("underwater_hockey")
    }

    fn explain(&self) -> String {
        format!(
            "【水下曲棍球规则】\n\n\
            比赛场地:\n{}\n\n\
            球队组成:\n{}\n\n\
            比赛规则:\n{}\n\n\
            犯规行为:\n{}",
            self.playing_area()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.team_composition()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.game_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.fouls()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_underwater_hockey_rules() {
        let rules = UnderwaterHockeyRules::new();
        assert_eq!(rules.metadata().name, "水下曲棍球规则");
        assert!(!rules.explain().is_empty());
        assert!(matches!(rules.category(), RuleCategory::Sports(_)));
    }

    #[test]
    fn test_playing_area() {
        let rules = UnderwaterHockeyRules::new();
        let area = rules.playing_area();
        assert!(area.iter().any(|a| a.contains("21-25米")));
        assert!(area.iter().any(|a| a.contains("水深")));
    }

    #[test]
    fn test_team_composition() {
        let rules = UnderwaterHockeyRules::new();
        let team = rules.team_composition();
        assert!(team.iter().any(|t| t.contains("6人")));
        assert!(team.iter().any(|t| t.contains("替补")));
    }

    #[test]
    fn test_equipment() {
        let rules = UnderwaterHockeyRules::new();
        let equipment = rules.equipment();
        assert!(equipment.iter().any(|e| e.contains("面镜")));
        assert!(equipment.iter().any(|e| e.contains("脚蹼")));
        assert!(equipment.iter().any(|e| e.contains("球杆")));
    }

    #[test]
    fn test_game_rules() {
        let rules = UnderwaterHockeyRules::new();
        let game_rules = rules.game_rules();
        assert!(game_rules.iter().any(|r| r.contains("球门")));
        assert!(game_rules.iter().any(|r| r.contains("得分")));
    }

    #[test]
    fn test_fouls() {
        let rules = UnderwaterHockeyRules::new();
        let fouls = rules.fouls();
        assert!(fouls.iter().any(|f| f.contains("抓球")));
        assert!(fouls.iter().any(|f| f.contains("阻挡")));
    }

    #[test]
    fn test_penalties() {
        let rules = UnderwaterHockeyRules::new();
        let penalties = rules.penalties();
        assert!(penalties.iter().any(|p| p.contains("罚下")));
        assert!(penalties.len() >= 5);
    }

    #[test]
    fn test_referees() {
        let rules = UnderwaterHockeyRules::new();
        let referees = rules.referees();
        assert!(referees.iter().any(|r| r.contains("主裁判")));
        assert!(referees.iter().any(|r| r.contains("计时员")));
    }

    #[test]
    fn test_safety_rules() {
        let rules = UnderwaterHockeyRules::new();
        let safety = rules.safety_rules();
        assert!(safety.iter().any(|s| s.contains("屏气")));
        assert!(safety.iter().any(|s| s.contains("医疗")));
    }
}
