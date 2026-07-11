//! 法国踢腿术规则
//!
//! 法式踢腿术(Savate)是法国传统武术，融合拳击和踢腿技术

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 法国踢腿术规则
pub struct SavateRules {
    metadata: RuleMetadata,
}

impl SavateRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("法国踢腿术规则", "法式踢腿术(Savate)竞赛规则")
                .with_origin("法国")
                .with_tags(vec!["体育".into(), "武术".into(), "踢拳".into()]),
        }
    }

    /// 比赛级别
    pub fn competition_levels(&self) -> Vec<&'static str> {
        vec![
            "初学者: 无实战经验",
            "学徒级: 1年以下经验",
            "挑战级: 1-2年经验",
            "技师级: 2-3年经验",
            "专家级: 3年以上经验",
            "大师级: 最高级别",
        ]
    }

    /// 重量级别
    pub fn weight_classes(&self) -> Vec<&'static str> {
        vec![
            "蝇量级: 56kg以下",
            "羽量级: 56-60kg",
            "轻量级: 60-65kg",
            "次中量级: 65-70kg",
            "中量级: 70-75kg",
            "次重量级: 75-80kg",
            "重量级: 80-85kg",
            "超重量级: 85kg以上",
        ]
    }

    /// 回合制度
    pub fn round_system(&self) -> Vec<&'static str> {
        vec![
            "初学者: 3回合，每回合1.5分钟",
            "挑战级: 3回合，每回合2分钟",
            "技师级: 4回合，每回合2分钟",
            "专家级: 5回合，每回合2分钟",
            "大师级: 5回合，每回合3分钟",
            "回合间休息1分钟",
        ]
    }

    /// 允许技术
    pub fn permitted_techniques(&self) -> Vec<&'static str> {
        vec![
            "拳法: 直拳、摆拳、上勾拳",
            "前踢: 用脚尖或脚背",
            "侧踢: 用脚外侧",
            "低扫踢: 击打腿部外侧",
            "中段踢: 击打躯干",
            "高段踢: 击打头部",
            "勾踢: 弧线踢法",
        ]
    }

    /// 禁止技术
    pub fn prohibited_techniques(&self) -> Vec<&'static str> {
        vec![
            "胫骨踢: 用胫骨攻击",
            "膝击: 任何膝部攻击",
            "肘击: 任何肘部攻击",
            "头撞",
            "攻击后脑",
            "攻击关节",
            "攻击已倒地对手",
            "擒抱后攻击",
        ]
    }

    /// 得分规则
    pub fn scoring_system(&self) -> Vec<&'static str> {
        vec![
            "击中头部: 1点",
            "击中躯干: 1点",
            "扫踢使对手失去平衡: 2点",
            "击倒对手: 4点",
            "干净利落的踢击加分",
            "技术性得点优先",
        ]
    }

    /// 护具要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "拳击手套: 专为Savate设计",
            "特制踢鞋: 必须穿戴",
            "护齿: 必须佩戴",
            "护裆: 可选但推荐",
            "紧身比赛服",
            "头盔: 初学者必须",
        ]
    }

    /// 比赛场地
    pub fn competition_venue(&self) -> Vec<&'static str> {
        vec![
            "标准擂台: 6米×6米至7.2米×7.2米",
            "围绳高度: 4条绳索",
            "地面软垫",
            "清洁的比赛环境",
            "两个选手角落",
            "中立角标识",
        ]
    }

    /// 犯规处罚
    pub fn foul_penalties(&self) -> Vec<&'static str> {
        vec!["口头警告", "扣1分", "扣2分", "取消资格", "犯规累积原则"]
    }
}

impl Default for SavateRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SavateRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("savate")
    }

    fn explain(&self) -> String {
        format!(
            "【法国踢腿术规则】\n\n\
            比赛级别:\n{}\n\n\
            允许技术:\n{}\n\n\
            得分规则:\n{}\n\n\
            护具要求:\n{}\n",
            self.competition_levels()
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
    fn test_savate_rules() {
        let rules = SavateRules::new();
        assert_eq!(rules.metadata().name, "法国踢腿术规则");
        assert!(!rules.weight_classes().is_empty());
        assert!(!rules.permitted_techniques().is_empty());
    }

    #[test]
    fn test_savate_scoring() {
        let rules = SavateRules::new();
        let scoring = rules.scoring_system();
        assert!(scoring.iter().any(|s| s.contains("头部")));
        assert!(scoring.iter().any(|s| s.contains("击倒")));
    }

    #[test]
    fn test_savate_equipment() {
        let rules = SavateRules::new();
        let equip = rules.equipment();
        assert!(equip.iter().any(|e| e.contains("踢鞋")));
        assert!(equip
            .iter()
            .any(|e| e.contains("拳套") || e.contains("手套")));
    }
}
