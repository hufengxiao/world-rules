//! 白拳规则
//!
//! 白拳是越南传统武术，结合了踢、打、摔技术

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 白拳规则
pub struct VoivovamRules {
    metadata: RuleMetadata,
}

impl VoivovamRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("白拳规则", "越南传统武术Binh Dinh/Võ cổ truyền竞赛规则")
                .with_origin("越南")
                .with_tags(vec!["体育".into(), "武术".into(), "格斗".into()]),
        }
    }

    /// 比赛形式
    pub fn competition_types(&self) -> Vec<&'static str> {
        vec![
            "套路比赛: 动作表演评分",
            "实战比赛: 对抗竞技",
            "器械比赛: 传统武器套路",
            "对练比赛: 双人配合演练",
            "团体比赛: 多人表演",
        ]
    }

    /// 重量级别
    pub fn weight_classes(&self) -> Vec<&'static str> {
        vec![
            "蝇量级: 52kg以下",
            "羽量级: 52-57kg",
            "轻量级: 57-62kg",
            "次中量级: 62-67kg",
            "中量级: 67-72kg",
            "次重量级: 72-81kg",
            "重量级: 81kg以上",
        ]
    }

    /// 比赛回合
    pub fn round_system(&self) -> Vec<&'static str> {
        vec![
            "套路比赛: 单人表演",
            "实战比赛: 3回合",
            "每回合2分钟",
            "回合间休息1分钟",
            "决赛可延长至5回合",
        ]
    }

    /// 允许技法
    pub fn permitted_techniques(&self) -> Vec<&'static str> {
        vec![
            "拳法: 直拳、摆拳、勾拳",
            "腿法: 扫腿、蹬腿、勾踢",
            "肘击: 各种肘法",
            "膝击: 正膝、侧膝",
            "摔法: 扫摔、缠摔",
            "擒拿: 关节控制技术",
            "防守: 闪避、格挡",
        ]
    }

    /// 禁止技法
    pub fn prohibited_techniques(&self) -> Vec<&'static str> {
        vec![
            "攻击后脑",
            "攻击眼睛",
            "攻击裆部",
            "攻击关节背面",
            "咬人",
            "攻击已倒地对手",
            "使用隐藏武器",
        ]
    }

    /// 得分规则
    pub fn scoring_system(&self) -> Vec<&'static str> {
        vec![
            "有效踢击躯干: 1分",
            "有效踢击头部: 2分",
            "有效摔法: 2分",
            "击倒对手: 3分",
            "套路评分: 动作规范、力量、节奏",
            "技术难度加分",
        ]
    }

    /// 护具要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "拳套: 现代比赛必须",
            "头盔: 青少年必须",
            "护齿: 必须",
            "护胸: 可选",
            "护胫: 可选",
            "传统服装: 套路比赛",
        ]
    }

    /// 套路要求
    pub fn form_requirements(&self) -> Vec<&'static str> {
        vec![
            "动作规范准确",
            "力量表现充分",
            "节奏变化清晰",
            "精神集中",
            "传统风格体现",
            "难度系数考量",
        ]
    }

    /// 胜利条件
    pub fn victory_conditions(&self) -> Vec<&'static str> {
        vec![
            "得分领先胜利",
            "KO胜利",
            "对手弃权",
            "裁判终止比赛",
            "对手取消资格",
            "套路评分最高",
        ]
    }

    /// 传统武器
    pub fn traditional_weapons(&self) -> Vec<&'static str> {
        vec![
            "刀: 单刀技法",
            "剑: 双手剑法",
            "棍: 长棍技法",
            "枪: 枪法套路",
            "双刀: 双刀技术",
            "盾牌: 盾牌技法",
        ]
    }
}

impl Default for VoivovamRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for VoivovamRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("voivovam")
    }

    fn explain(&self) -> String {
        format!(
            "【白拳规则】\n\n\
            比赛形式:\n{}\n\n\
            允许技法:\n{}\n\n\
            得分规则:\n{}\n\n\
            护具要求:\n{}\n",
            self.competition_types()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.permitted_techniques()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.scoring_system()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.equipment()
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
    fn test_voivovam_rules() {
        let rules = VoivovamRules::new();
        assert_eq!(rules.metadata().name, "白拳规则");
        assert!(!rules.competition_types().is_empty());
    }

    #[test]
    fn test_voivovam_techniques() {
        let rules = VoivovamRules::new();
        let techniques = rules.permitted_techniques();
        assert!(techniques.iter().any(|t| t.contains("拳法")));
        assert!(techniques.iter().any(|t| t.contains("摔法")));
    }

    #[test]
    fn test_voivovam_weapons() {
        let rules = VoivovamRules::new();
        let weapons = rules.traditional_weapons();
        assert!(weapons.iter().any(|w| w.contains("刀")));
        assert!(weapons.iter().any(|w| w.contains("剑")));
    }
}