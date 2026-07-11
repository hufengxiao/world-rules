//! Pankration规则
//!
//! 古希腊Pankration格斗规则，古代奥林匹克格斗项目

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// Pankration规则
pub struct PankrationRules {
    metadata: RuleMetadata,
}

impl PankrationRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("Pankration规则", "古希腊Pankration格斗竞赛规则")
                .with_origin("古希腊")
                .with_tags(vec!["体育".into(), "格斗".into(), "历史格斗".into()]),
        }
    }

    /// 历史特点
    pub fn historical_characteristics(&self) -> Vec<&'static str> {
        vec![
            "古代奥林匹克项目",
            "公元前648年加入奥运会",
            "融合拳击与摔跤",
            "古希腊最高格斗技术",
            "现代MMA原型",
        ]
    }

    /// 历史比赛规则
    pub fn ancient_rules(&self) -> Vec<&'static str> {
        vec![
            "无时间限制",
            "投降判定胜利",
            "KO判定胜利",
            "仅禁止咬人和插眼",
            "无重量级别",
            "无护具比赛",
        ]
    }

    /// 允许技法(历史)
    pub fn permitted_techniques_ancient(&self) -> Vec<&'static str> {
        vec![
            "拳法: 各种拳击",
            "踢击: 各种腿法",
            "摔法: 各种摔投",
            "地面攻击",
            "缠抱攻击",
            "关节技",
            "窒息技术",
            "站立打击",
        ]
    }

    /// 禁止技法(历史)
    pub fn prohibited_techniques_ancient(&self) -> Vec<&'static str> {
        vec![
            "咬人: 严格禁止",
            "手指插入眼睛: 严格禁止",
            "撕扯头发",
            "攻击生殖器",
        ]
    }

    /// 现代复兴规则
    pub fn modern_revision(&self) -> Vec<&'static str> {
        vec![
            "现代Pankration复兴",
            "增加安全规则",
            "护具要求",
            "时间限制",
            "重量级别划分",
            "裁判监督",
        ]
    }

    /// 现代比赛回合
    pub fn modern_round_system(&self) -> Vec<&'static str> {
        vec![
            "现代比赛: 3回合",
            "每回合3分钟",
            "回合间休息1分钟",
            "可延长时间",
            "青年比赛: 缩短时间",
        ]
    }

    /// 现代允许技法
    pub fn modern_permitted_techniques(&self) -> Vec<&'static str> {
        vec![
            "拳法: 各种拳击",
            "腿法: 各种踢击",
            "摔法: 各种摔投",
            "地面控制",
            "关节技(部分)",
            "站立缠抱",
            "组合攻击",
        ]
    }

    /// 现代禁止技法
    pub fn modern_prohibited_techniques(&self) -> Vec<&'static str> {
        vec![
            "肘击",
            "头部打击地面",
            "攻击后脑",
            "攻击眼睛",
            "攻击裆部",
            "咬人",
            "撕扯头发",
            "小关节技",
            "脊柱攻击",
        ]
    }

    /// 现代重量级别
    pub fn modern_weight_classes(&self) -> Vec<&'static str> {
        vec![
            "轻量级: 65kg以下",
            "中量级: 65-75kg",
            "重量级: 75-85kg",
            "超重量级: 85kg以上",
        ]
    }

    /// 现代护具要求
    pub fn modern_equipment(&self) -> Vec<&'static str> {
        vec![
            "拳套: 现代比赛",
            "护齿: 必须佩戴",
            "护裆: 必须佩戴",
            "头盔: 青年比赛",
            "护腿: 推荐使用",
        ]
    }

    /// 历史胜利方式
    pub fn ancient_victory(&self) -> Vec<&'static str> {
        vec![
            "投降: 对手举手投降",
            "KO: 对手无法继续",
            "对手死亡(历史极端)",
            "裁判判定(历史)",
        ]
    }

    /// 现代胜利条件
    pub fn modern_victory(&self) -> Vec<&'static str> {
        vec![
            "KO胜利",
            "TKO胜利",
            "投降胜利",
            "判定胜利",
            "对手弃权",
            "裁判终止",
        ]
    }
}

impl Default for PankrationRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for PankrationRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("pankration")
    }

    fn explain(&self) -> String {
        format!(
            "【Pankration规则】\n\n\
            历史特点:\n{}\n\n\
            历史比赛规则:\n{}\n\n\
            历史允许技法:\n{}\n\n\
            现代复兴规则:\n{}\n",
            self.historical_characteristics()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.ancient_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.permitted_techniques_ancient()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.modern_revision()
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
    fn test_pankration_rules() {
        let rules = PankrationRules::new();
        assert_eq!(rules.metadata().name, "Pankration规则");
        assert!(!rules.historical_characteristics().is_empty());
    }

    #[test]
    fn test_pankration_olympic_origin() {
        let rules = PankrationRules::new();
        let history = rules.historical_characteristics();
        assert!(history.iter().any(|h| h.contains("奥林匹克")));
        assert!(history.iter().any(|h| h.contains("古希腊")));
    }

    #[test]
    fn test_pankration_biting_prohibited() {
        let rules = PankrationRules::new();
        let prohibited = rules.prohibited_techniques_ancient();
        assert!(prohibited.iter().any(|p| p.contains("咬人")));
    }

    #[test]
    fn test_pankration_mma_origin() {
        let rules = PankrationRules::new();
        let history = rules.historical_characteristics();
        assert!(history.iter().any(|h| h.contains("MMA原型")));
    }

    #[test]
    fn test_pankration_modern_rules() {
        let rules = PankrationRules::new();
        let modern = rules.modern_revision();
        assert!(modern.iter().any(|m| m.contains("安全规则")));
    }
}