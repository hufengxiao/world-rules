//! Pancrase规则
//!
//! 日本Pancrase综合格斗规则，融合摔跤和打击技术

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// Pancrase规则
pub struct PancraseRules {
    metadata: RuleMetadata,
}

impl PancraseRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("Pancrase规则", "日本Pancrase综合格斗竞赛规则")
                .with_origin("日本")
                .with_tags(vec!["体育".into(), "格斗".into(), "MMA".into()]),
        }
    }

    /// Pancrase历史特点
    pub fn historical_characteristics(&self) -> Vec<&'static str> {
        vec![
            "创立于1993年",
            "日本早期MMA组织",
            "融合摔跤与打击",
            "开放规则体系",
            "职业摔跤演化",
        ]
    }

    /// 比赛回合
    pub fn round_system(&self) -> Vec<&'static str> {
        vec![
            "标准比赛: 3回合",
            "冠军赛: 5回合",
            "每回合5分钟",
            "回合间休息1分钟",
            "无时间限制比赛(历史)",
        ]
    }

    /// 允许技法
    pub fn permitted_techniques(&self) -> Vec<&'static str> {
        vec![
            "拳法: 各种拳击技术",
            "腿法: 各种踢击技术",
            "膝击: 站立和地面允许",
            "摔法: 各种摔投技术",
            "地面控制: 擒拿技术",
            "关节技: 手臂、腿部",
            "窒息技术",
            "缠抱攻击",
        ]
    }

    /// 禁止技法
    pub fn prohibited_techniques(&self) -> Vec<&'static str> {
        vec![
            "肘击: 垂直肘禁止",
            "攻击后脑",
            "攻击眼睛",
            "攻击裆部",
            "咬人",
            "撕扯头发",
            "手指插入",
            "攻击已倒地对手头部",
            "小关节技",
            "脊柱扭转",
        ]
    }

    /// 关节技规则
    pub fn submission_rules(&self) -> Vec<&'static str> {
        vec![
            "手臂关节: 允许",
            "腿部关节: 允许",
            "窒息技术: 允许",
            "小关节: 禁止",
            "脊柱攻击: 禁止",
            "颈部扭转: 禁止",
        ]
    }

    /// 重量级别
    pub fn weight_classes(&self) -> Vec<&'static str> {
        vec![
            "蝇量级: 57kg以下",
            "羽量级: 57-66kg",
            "轻量级: 66-70kg",
            "次中量级: 70-77kg",
            "中量级: 77-84kg",
            "轻重量级: 84-93kg",
            "重量级: 93kg以上",
        ]
    }

    /// 胜利条件
    pub fn victory_conditions(&self) -> Vec<&'static str> {
        vec![
            "KO胜利",
            "TKO胜利",
            "投降胜利: 关节技",
            "窒息投降",
            "判定胜利",
            "对手弃权",
            "裁判终止",
        ]
    }

    /// 护具要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "拳套: 开放式或小型",
            "护齿: 必须佩戴",
            "护裆: 必须佩戴",
            "短裤: Pancrase专用",
            "无鞋比赛",
            "缠手带",
        ]
    }

    /// 比赛场地
    pub fn competition_venue(&self) -> Vec<&'static str> {
        vec![
            "标准擂台: 8米×8米",
            "围绳高度: 1.2米",
            "地面软垫",
            "两个选手角落",
            "裁判区域",
        ]
    }

    /// 拍地投降规则
    pub fn tapout_rules(&self) -> Vec<&'static str> {
        vec![
            "拍地投降: 有效",
            "口头投降: 有效",
            "裁判干预投降",
            "团队抛毛巾投降",
            "窒息昏迷判定负",
            "关节技保护",
        ]
    }

    /// 评分标准
    pub fn scoring_criteria(&self) -> Vec<&'static str> {
        vec![
            "有效打击",
            "摔法成功",
            "地面控制",
            "主动攻击",
            "比赛掌控",
            "10-10评分",
        ]
    }
}

impl Default for PancraseRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for PancraseRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("pancrase")
    }

    fn explain(&self) -> String {
        format!(
            "【Pancrase规则】\n\n\
            历史特点:\n{}\n\n\
            允许技法:\n{}\n\n\
            关节技规则:\n{}\n\n\
            胜利条件:\n{}\n",
            self.historical_characteristics()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.permitted_techniques()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.submission_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.victory_conditions()
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
    fn test_pancrase_rules() {
        let rules = PancraseRules::new();
        assert_eq!(rules.metadata().name, "Pancrase规则");
        assert!(!rules.historical_characteristics().is_empty());
    }

    #[test]
    fn test_pancrase_submissions() {
        let rules = PancraseRules::new();
        let submissions = rules.permitted_techniques();
        assert!(submissions.iter().any(|s| s.contains("关节技")));
        assert!(submissions.iter().any(|s| s.contains("窒息")));
    }

    #[test]
    fn test_pancrase_no_small_joints() {
        let rules = PancraseRules::new();
        let prohibited = rules.prohibited_techniques();
        assert!(prohibited.iter().any(|p| p.contains("小关节")));
    }

    #[test]
    fn test_pancrase_rounds() {
        let rules = PancraseRules::new();
        let rounds = rules.round_system();
        assert!(rounds.iter().any(|r| r.contains("5分钟")));
    }
}
