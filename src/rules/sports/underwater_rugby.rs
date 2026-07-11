//! 水下橄榄球规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 水下橄榄球规则 (CMAS)
pub struct UnderwaterRugbyRules {
    metadata: RuleMetadata,
}

impl UnderwaterRugbyRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("水下橄榄球规则", "CMAS水下橄榄球竞赛规则")
                .with_origin("德国")
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
            "泳池长度: 12-18米",
            "泳池宽度: 8-12米",
            "水深: 3.5-5.0米",
            "球门: 底部篮筐，直径400mm",
            "水底标志: 明确标示区域",
        ]
    }

    /// 比赛时间
    pub fn match_duration(&self) -> Vec<&'static str> {
        vec![
            "上半场: 15分钟",
            "中场休息: 5分钟",
            "下半场: 15分钟",
            "有效时间: 不停表计时",
            "加时赛: 必要时进行",
        ]
    }

    /// 球队组成
    pub fn team_composition(&self) -> Vec<&'static str> {
        vec![
            "场上队员: 每队6人",
            "替补队员: 最多6人",
            "换人: 随时可换，不限次数",
            "换人方式: 从换人区入场",
            "队长: 指定1人",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "面镜: 必须佩戴",
            "呼吸管: 简单设计",
            "脚蹼: 统一规格",
            "泳帽: 带护耳和号码",
            "水球: 充水橡胶球，稍沉",
            "手套: 可选保护手套",
            "泳衣: 团队统一颜色",
        ]
    }

    /// 比赛规则
    pub fn game_rules(&self) -> Vec<&'static str> {
        vec![
            "目标: 将球放入对方篮筐",
            "得分: 球完全进入篮筐得1分",
            "开球: 得分后在水中重新开球",
            "传球: 只能向后传球",
            "持球: 可持球游动，但可能被抢",
        ]
    }

    /// 身体接触规则
    pub fn contact_rules(&self) -> Vec<&'static str> {
        vec![
            "允许接触: 可抢球和阻挡",
            "禁止攻击: 不得攻击对方身体",
            "禁止拉扯: 不得拉扯泳衣或装备",
            "头部保护: 不得攻击头部",
            "公平竞争: 遵守体育道德",
        ]
    }

    /// 犯规行为
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "攻击性行为: 危险动作",
            "拉扯装备: 拉对方面镜或呼吸管",
            "阻挡进攻: 非法防守",
            "超时持球: 长时间占据优势",
            "不当行为: 不尊重裁判",
        ]
    }

    /// 处罚规则
    pub fn penalties(&self) -> Vec<&'static str> {
        vec![
            "轻微犯规: 自由球",
            "重大犯规: 2分钟罚下",
            "严重犯规: 红牌直接罚下",
            "团队犯规: 累计处罚",
            "技术犯规: 警告或罚时",
        ]
    }

    /// 裁判职责
    pub fn referees(&self) -> Vec<&'static str> {
        vec![
            "主裁判: 水下和水面各1人",
            "边裁判: 判断进球和犯规",
            "计时员: 控制比赛时间",
            "记分员: 记录比分和犯规",
            "换人监督: 监督换人程序",
        ]
    }

    /// 战术要点
    pub fn tactics(&self) -> Vec<&'static str> {
        vec![
            "阵型部署: 进攻和防守阵型",
            "换人策略: 保持体能优势",
            "深度控制: 利用深度空间",
            "团队配合: 传球和掩护",
            "反击战术: 快速转换",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "医疗支持: 现场医疗待命",
            "装备检查: 赛前强制检查",
            "身体状况: 确保参赛者健康",
            "紧急程序: 制定应急计划",
            "禁赛规定: 身体不适者不得参赛",
        ]
    }
}

impl Default for UnderwaterRugbyRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for UnderwaterRugbyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("underwater_rugby")
    }

    fn explain(&self) -> String {
        format!(
            "【水下橄榄球规则】\n\n\
            比赛场地:\n{}\n\n\
            球队组成:\n{}\n\n\
            比赛规则:\n{}\n\n\
            身体接触规则:\n{}",
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
            self.contact_rules()
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
    fn test_underwater_rugby_rules() {
        let rules = UnderwaterRugbyRules::new();
        assert_eq!(rules.metadata().name, "水下橄榄球规则");
        assert!(!rules.explain().is_empty());
        assert!(matches!(rules.category(), RuleCategory::Sports(_)));
    }

    #[test]
    fn test_playing_area() {
        let rules = UnderwaterRugbyRules::new();
        let area = rules.playing_area();
        assert!(area.iter().any(|a| a.contains("12-18米")));
        assert!(area.iter().any(|a| a.contains("篮筐")));
    }

    #[test]
    fn test_team_composition() {
        let rules = UnderwaterRugbyRules::new();
        let team = rules.team_composition();
        assert!(team.iter().any(|t| t.contains("6人")));
        assert!(team.iter().any(|t| t.contains("替补")));
    }

    #[test]
    fn test_equipment() {
        let rules = UnderwaterRugbyRules::new();
        let equipment = rules.equipment();
        assert!(equipment.iter().any(|e| e.contains("面镜")));
        assert!(equipment.iter().any(|e| e.contains("脚蹼")));
        assert!(equipment.iter().any(|e| e.contains("水球")));
    }

    #[test]
    fn test_game_rules() {
        let rules = UnderwaterRugbyRules::new();
        let game_rules = rules.game_rules();
        assert!(game_rules.iter().any(|r| r.contains("篮筐")));
        assert!(game_rules.iter().any(|r| r.contains("得分")));
    }

    #[test]
    fn test_contact_rules() {
        let rules = UnderwaterRugbyRules::new();
        let contact = rules.contact_rules();
        assert!(contact.iter().any(|c| c.contains("允许")));
        assert!(contact.iter().any(|c| c.contains("禁止")));
    }

    #[test]
    fn test_fouls() {
        let rules = UnderwaterRugbyRules::new();
        let fouls = rules.fouls();
        assert!(fouls.iter().any(|f| f.contains("攻击")));
        assert!(fouls.iter().any(|f| f.contains("装备")));
    }

    #[test]
    fn test_penalties() {
        let rules = UnderwaterRugbyRules::new();
        let penalties = rules.penalties();
        assert!(penalties.iter().any(|p| p.contains("罚下")));
        assert!(penalties.len() >= 5);
    }

    #[test]
    fn test_tactics() {
        let rules = UnderwaterRugbyRules::new();
        let tactics = rules.tactics();
        assert!(tactics.iter().any(|t| t.contains("阵型")));
        assert!(tactics.iter().any(|t| t.contains("配合")));
    }

    #[test]
    fn test_safety_rules() {
        let rules = UnderwaterRugbyRules::new();
        let safety = rules.safety_rules();
        assert!(safety.iter().any(|s| s.contains("医疗")));
        assert!(safety.iter().any(|s| s.contains("装备")));
    }
}
