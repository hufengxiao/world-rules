//! 散打规则
//!
//! 中国武术散打竞赛规则，融合踢、打、摔技术

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 散打规则
pub struct SandaRules {
    metadata: RuleMetadata,
}

impl SandaRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("散打规则", "中国武术散打竞赛规则")
                .with_origin("中国")
                .with_tags(vec!["体育".into(), "武术".into(), "散打".into()]),
        }
    }

    /// 比赛场地
    pub fn competition_venue(&self) -> Vec<&'static str> {
        vec![
            "标准擂台: 8米×8米",
            "擂台高度: 0.6米",
            "护栏高度: 0.8米",
            "软垫厚度: 最小5厘米",
            "场地标识清晰",
        ]
    }

    /// 重量级别
    pub fn weight_classes(&self) -> Vec<&'static str> {
        vec![
            "48公斤级: 48kg以下",
            "52公斤级: 48-52kg",
            "56公斤级: 52-56kg",
            "60公斤级: 56-60kg",
            "65公斤级: 60-65kg",
            "70公斤级: 65-70kg",
            "75公斤级: 70-75kg",
            "80公斤级: 75-80kg",
            "85公斤级: 80-85kg",
            "90公斤级: 85-90kg",
            "100公斤级: 90-100kg",
            "100公斤以上级",
        ]
    }

    /// 回合制度
    pub fn round_system(&self) -> Vec<&'static str> {
        vec![
            "每场比赛3回合",
            "每回合2分钟",
            "回合间休息1分钟",
            "可延长时间决胜",
            "青少年比赛: 每回合1.5分钟",
        ]
    }

    /// 得分部位
    pub fn scoring_areas(&self) -> Vec<&'static str> {
        vec![
            "头部: 2分",
            "躯干: 1分",
            "大腿: 1分",
            "有效摔倒: 2分",
            "摔倒对方且站立: 3分",
            "下台技术: 3分",
        ]
    }

    /// 允许技法
    pub fn permitted_techniques(&self) -> Vec<&'static str> {
        vec![
            "拳法: 冲拳、掼拳、抄拳、鞭拳",
            "腿法: 蹬腿、踹腿、扫腿、勾腿",
            "摔法: 贴身摔、接腿摔",
            "组合技法连续攻击",
            "防守技术",
        ]
    }

    /// 禁止技法
    pub fn prohibited_techniques(&self) -> Vec<&'static str> {
        vec![
            "击打后脑",
            "击打颈部",
            "击打裆部",
            "使用头、肘、膝攻击",
            "攻击已倒地对手",
            "搂抱后连击",
            "背向逃跑",
            "故意拖延时间",
        ]
    }

    /// 护具要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "头盔: 必须佩戴",
            "拳套: 必须佩戴(不同级别不同重量)",
            "护齿: 必须佩戴",
            "护胸: 必须佩戴",
            "护裆: 必须佩戴",
            "短裤: 散打专用",
        ]
    }

    /// 胜利方式
    pub fn victory_methods(&self) -> Vec<&'static str> {
        vec![
            "优势胜利: 双方实力悬殊",
            "得分胜利: 总分领先",
            "对方弃权",
            "对方被取消资格",
            "对方受伤无法继续",
            "KO胜利",
        ]
    }

    /// 犯规处罚
    pub fn foul_penalties(&self) -> Vec<&'static str> {
        vec![
            "警告: 轻微犯规",
            "扣分: 严重犯规",
            "取消资格: 严重或屡次犯规",
            "技术犯规扣1分",
            "侵人犯规扣2分",
            "累计3次警告取消资格",
        ]
    }
}

impl Default for SandaRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SandaRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("sanda")
    }

    fn explain(&self) -> String {
        format!(
            "【散打规则】\n\n\
            重量级别:\n{}\n\n\
            得分部位:\n{}\n\n\
            允许技法:\n{}\n\n\
            禁止技法:\n{}\n",
            self.weight_classes()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.scoring_areas()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.permitted_techniques()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.prohibited_techniques()
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
    fn test_sanda_rules() {
        let rules = SandaRules::new();
        assert_eq!(rules.metadata().name, "散打规则");
        assert!(!rules.weight_classes().is_empty());
        assert!(!rules.permitted_techniques().is_empty());
    }

    #[test]
    fn test_sanda_scoring() {
        let rules = SandaRules::new();
        let areas = rules.scoring_areas();
        assert!(areas.iter().any(|a| a.contains("头部")));
        assert!(areas.iter().any(|a| a.contains("摔倒")));
    }

    #[test]
    fn test_sanda_equipment() {
        let rules = SandaRules::new();
        let equip = rules.equipment();
        assert!(equip.iter().any(|e| e.contains("头盔")));
        assert!(equip.iter().any(|e| e.contains("拳套")));
    }
}
