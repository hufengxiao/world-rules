//! 适应性球类规则
//!
//! 针对不同残疾类型的球类运动适应性规则，涵盖轮椅网球、盲人足球等。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 适应性球类规则
pub struct AdaptiveBallGamesRules {
    metadata: RuleMetadata,
}

impl AdaptiveBallGamesRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("适应性球类规则", "残疾人球类运动适应性规则")
                .with_origin("IPC/ITF/FIFA")
                .with_tags(vec![
                    "体育".into(),
                    "球类".into(),
                    "残奥".into(),
                    "适应性".into(),
                ]),
        }
    }

    /// 运动分级
    pub fn classification(&self) -> Vec<&'static str> {
        vec![
            "轮椅网球: 下肢残疾",
            "Open级: 所有下肢残疾",
            "Quad级: 四肢残疾",
            "盲人足球: B1-B3视力残疾",
            "B1级: 全盲运动员",
            "B2-B3级: 低视力运动员",
            "轮椅篮球: 1.0-4.5分",
            "坐式排球: 下肢残疾",
            "轮椅橄榄球: 0.5-3.5分",
            "盲人门球: B1-B3视力残疾",
        ]
    }

    /// 轮椅网球规则
    pub fn wheelchair_tennis_rules(&self) -> Vec<&'static str> {
        vec![
            "两跳规则: 球可落地两次",
            "轮椅规格: 符合ITF标准",
            "移动规则: 轮椅视为身体一部分",
            "发球规则: 允许轮椅固定",
            "击球规则: 轮椅触球无效",
            "假肢规定: 可佩戴假肢",
            "球场规格: 标准网球场",
            "球拍规格: 标准网球拍",
            "禁止: 电子驱动轮椅",
        ]
    }

    /// 盲人足球规则
    pub fn blind_football_rules(&self) -> Vec<&'static str> {
        vec![
            "球场: 20m×40m硬地",
            "球: 内置发声装置",
            "运动员: 全部B1级（眼罩遮蔽）",
            "守门员: 视力正常或B2-B3级",
            "引导员: 场外声音引导",
            "比赛时间: 2×20分钟",
            "犯规规则: 禁止视觉作弊",
            "换人: 无限次换人",
            "防守: 不得发出干扰声",
            "角球区: 守门员不得出区",
        ]
    }

    /// 轮椅篮球规则
    pub fn wheelchair_basketball_rules(&self) -> Vec<&'static str> {
        vec![
            "分级评分: 1.0-4.5分（14分上限）",
            "轮椅规格: 符合IWBF标准",
            "运球规则: 每推两次必须运球",
            "犯规计算: 轮椅犯规计为个人犯规",
            "三分线: 标准距离",
            "比赛时间: 4×10分钟",
            "进攻时限: 24秒",
            "抬升犯规: 轮椅抬升不得超3秒",
            "阻挡犯规: 建立合法防守位置",
        ]
    }

    /// 坐式排球规则
    pub fn sitting_volleyball_rules(&self) -> Vec<&'static str> {
        vec![
            "球场: 10m×6m",
            "网高: 1.15m（男子）1.05m（女子）",
            "坐姿: 臀部必须接触地面",
            "击球规则: 允许身体任何部位触球",
            "发球规则: 坐姿发球",
            "防守规则: 允许拦网",
            "移动规则: 臀部离地为犯规",
            "比赛制: 五局三胜",
            "每局25分: 决胜局15分",
            "最少队员: 6人上场",
        ]
    }

    /// 盲人门球规则
    pub fn goalball_rules(&self) -> Vec<&'static str> {
        vec![
            "球场: 18m×9m室内场",
            "球: 内置发声装置，重1.25kg",
            "运动员: 全部佩戴眼罩",
            "比赛时间: 2×12分钟",
            "投掷规则: 必须在投掷区内",
            "防守规则: 身体任何部位可挡球",
            "犯规规则: 触摸眼罩犯规",
            "静音规则: 比赛中保持安静",
            "换人: 无限次换人",
            "得分: 球完全过线得分",
        ]
    }

    /// 装备适应性
    pub fn equipment_adaptations(&self) -> Vec<&'static str> {
        vec![
            "轮椅: 运动轮椅，符合规格",
            "眼罩: 黑色眼罩，全遮蔽",
            "发声球: 内置铃铛或电子发声",
            "假肢: 可佩戴但不允许使用",
            "手套: 守门员允许特殊手套",
            "护具: 允许护膝、护肘",
            "鞋子: 可选择性穿着",
            "禁止: 电子辅助设备",
            "禁止: 视觉增强装置",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "轮椅规格违规",
            "眼罩作弊",
            "非法辅助设备",
            "分级总分超标",
            "臀部离地（坐式排球）",
            "干扰声音（盲人运动）",
            "危险移动",
            "违反体育道德行为",
            "技术犯规",
            "非法换人",
        ]
    }
}

impl Default for AdaptiveBallGamesRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for AdaptiveBallGamesRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("adaptive_ball_games")
    }

    fn explain(&self) -> String {
        format!(
            "【适应性球类规则】\n\n\
            运动分级:\n{}\n\n\
            轮椅网球规则:\n{}\n\n\
            盲人足球规则:\n{}\n\n\
            轮椅篮球规则:\n{}\n\n\
            坐式排球规则:\n{}\n\n\
            盲人门球规则:\n{}",
            self.classification()
                .iter()
                .map(|c| format!("  • {}", c))
                .collect::<Vec<_>>()
                .join("\n"),
            self.wheelchair_tennis_rules()
                .iter()
                .map(|w| format!("  • {}", w))
                .collect::<Vec<_>>()
                .join("\n"),
            self.blind_football_rules()
                .iter()
                .map(|b| format!("  • {}", b))
                .collect::<Vec<_>>()
                .join("\n"),
            self.wheelchair_basketball_rules()
                .iter()
                .map(|w| format!("  • {}", w))
                .collect::<Vec<_>>()
                .join("\n"),
            self.sitting_volleyball_rules()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.goalball_rules()
                .iter()
                .map(|g| format!("  • {}", g))
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adaptive_ball_games_rules_basic() {
        let rules = AdaptiveBallGamesRules::new();
        assert_eq!(rules.metadata().name, "适应性球类规则");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_adaptive_ball_games_classification() {
        let rules = AdaptiveBallGamesRules::new();
        let classification = rules.classification();
        assert!(classification.iter().any(|c| c.contains("轮椅网球")));
        assert!(classification.iter().any(|c| c.contains("盲人足球")));
        assert!(classification.iter().any(|c| c.contains("坐式排球")));
        assert!(classification.len() >= 6);
    }

    #[test]
    fn test_adaptive_ball_games_wheelchair_tennis() {
        let rules = AdaptiveBallGamesRules::new();
        let tennis = rules.wheelchair_tennis_rules();
        assert!(tennis.iter().any(|t| t.contains("两跳")));
        assert!(tennis.iter().any(|t| t.contains("轮椅")));
        assert!(tennis.len() >= 6);
    }

    #[test]
    fn test_adaptive_ball_games_blind_football() {
        let rules = AdaptiveBallGamesRules::new();
        let football = rules.blind_football_rules();
        assert!(football.iter().any(|f| f.contains("发声")));
        assert!(football.iter().any(|f| f.contains("眼罩")));
        assert!(football.len() >= 6);
    }

    #[test]
    fn test_adaptive_ball_games_equipment() {
        let rules = AdaptiveBallGamesRules::new();
        let equipment = rules.equipment_adaptations();
        assert!(equipment.iter().any(|e| e.contains("轮椅")));
        assert!(equipment.iter().any(|e| e.contains("眼罩")));
        assert!(equipment.len() >= 6);
    }

    #[test]
    fn test_adaptive_ball_games_category() {
        let rules = AdaptiveBallGamesRules::new();
        assert!(matches!(rules.category(), RuleCategory::Sports(_)));
    }
}
