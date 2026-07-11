//! Luta Livre规则
//!
//! 巴西Luta Livre摔跤格斗规则，无道服擒拿格斗

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// Luta Livre规则
pub struct LutaLivreRules {
    metadata: RuleMetadata,
}

impl LutaLivreRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("Luta Livre规则", "巴西Luta Livre无道服摔跤格斗规则")
                .with_origin("巴西")
                .with_tags(vec!["体育".into(), "格斗".into(), "摔跤".into()]),
        }
    }

    /// Luta Livre特点
    pub fn unique_characteristics(&self) -> Vec<&'static str> {
        vec![
            "无道服摔跤",
            "站立与地面结合",
            "巴西街头格斗演化",
            "擒拿格斗技术",
            "与BJJ竞争历史",
        ]
    }

    /// 比赛类型
    pub fn competition_types(&self) -> Vec<&'static str> {
        vec![
            "Sport Luta Livre: 体育竞技",
            "Luta Livre Vale Tudo: 全面格斗",
            "Luta Livre Submission: 擒拿比赛",
            "竞技擒拿赛",
            "全面格斗赛",
        ]
    }

    /// 比赛回合
    pub fn round_system(&self) -> Vec<&'static str> {
        vec![
            "擒拿比赛: 无时间限制",
            "竞技赛: 10-15分钟",
            "全面格斗: 3回合",
            "每回合5分钟",
            "回合间休息1分钟",
        ]
    }

    /// 允许技法
    pub fn permitted_techniques(&self) -> Vec<&'static str> {
        vec![
            "摔法: 各种摔投技术",
            "关节技: 手臂、腿部",
            "窒息技术: 各种窒息",
            "地面控制",
            "站立擒拿",
            "缠抱摔投",
            "腿部攻击",
            "扭锁技术",
        ]
    }

    /// 禁止技法
    pub fn prohibited_techniques(&self) -> Vec<&'static str> {
        vec![
            "打击技术(擒拿赛)",
            "攻击后脑",
            "攻击眼睛",
            "攻击裆部",
            "咬人",
            "撕扯头发",
            "小关节技",
            "脊柱扭转",
            "完全格斗赛允许打击",
        ]
    }

    /// 关节技分类
    pub fn submission_categories(&self) -> Vec<&'static str> {
        vec![
            "手臂锁: armbar, kimura",
            "腿部锁: kneebar, heel hook",
            "窒息: guillotine, rear naked",
            "扭转技术",
            "复合锁定",
            "地面控制锁",
        ]
    }

    /// 重量级别
    pub fn weight_classes(&self) -> Vec<&'static str> {
        vec![
            "蝇量级: 57kg以下",
            "羽量级: 57-66kg",
            "轻量级: 66-76kg",
            "中量级: 76-86kg",
            "重量级: 86-99kg",
            "超重量级: 99kg以上",
        ]
    }

    /// 胜利条件
    pub fn victory_conditions(&self) -> Vec<&'static str> {
        vec![
            "投降胜利",
            "窒息投降",
            "关节技投降",
            "KO胜利(全面格斗)",
            "TKO胜利",
            "判定胜利",
            "对手弃权",
        ]
    }

    /// 护具要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "无道服: 短裤或紧身衣",
            "护齿: 必须佩戴",
            "护裆: 必须佩戴",
            "无鞋比赛",
            "拳套(全面格斗)",
            "缠手带",
        ]
    }

    /// 比赛场地
    pub fn competition_venue(&self) -> Vec<&'static str> {
        vec![
            "摔跤垫: 擒拿比赛",
            "擂台: 全面格斗",
            "标准尺寸场地",
            "地面软垫",
            "裁判区域",
        ]
    }

    /// 训练体系
    pub fn training_system(&self) -> Vec<&'static str> {
        vec![
            "站立摔跤技术",
            "地面擒拿技术",
            "体能训练",
            "实战对练",
            "技术演练",
        ]
    }
}

impl Default for LutaLivreRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for LutaLivreRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("luta_livre")
    }

    fn explain(&self) -> String {
        format!(
            "【Luta Livre规则】\n\n\
            特点:\n{}\n\n\
            比赛类型:\n{}\n\n\
            允许技法:\n{}\n\n\
            关节技分类:\n{}\n",
            self.unique_characteristics()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
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
            self.submission_categories()
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
    fn test_luta_livre_rules() {
        let rules = LutaLivreRules::new();
        assert_eq!(rules.metadata().name, "Luta Livre规则");
        assert!(!rules.unique_characteristics().is_empty());
    }

    #[test]
    fn test_luta_livre_no_gi() {
        let rules = LutaLivreRules::new();
        let characteristics = rules.unique_characteristics();
        assert!(characteristics.iter().any(|c| c.contains("无道服")));
    }

    #[test]
    fn test_luta_livre_submissions() {
        let rules = LutaLivreRules::new();
        let submissions = rules.permitted_techniques();
        assert!(submissions.iter().any(|s| s.contains("关节技")));
        assert!(submissions.iter().any(|s| s.contains("窒息")));
    }

    #[test]
    fn test_luta_livre_equipment() {
        let rules = LutaLivreRules::new();
        let equip = rules.equipment();
        assert!(equip.iter().any(|e| e.contains("无道服")));
    }
}
