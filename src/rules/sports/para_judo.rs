//! 残疾人盲人柔道规则
//!
//! 盲人柔道是残奥会专门项目，针对视力残疾运动员。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 残疾人盲人柔道规则
pub struct ParaJudoRules {
    metadata: RuleMetadata,
}

impl ParaJudoRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("残疾人盲人柔道规则", "残疾人盲人柔道比赛规则")
                .with_origin("IBSA/IJF")
                .with_tags(vec!["体育".into(), "柔道".into(), "残奥".into()]),
        }
    }

    /// 运动分级
    pub fn classification(&self) -> Vec<&'static str> {
        vec![
            "J1: 无光感或极少光感",
            "J2: 可辨别手形",
            "J3: 可辨别物体（非残奥）",
            "体重分级: 男子7级",
            "体重分级: 女子6级",
            "分级评估: 视力测试",
        ]
    }

    /// 比赛项目
    pub fn events(&self) -> Vec<&'static str> {
        vec![
            "残奥会: J1/J2级",
            "世界锦标赛",
            "洲际锦标赛",
            "盲人柔道世界杯",
            "比赛时间: 男5分钟/女4分钟",
            "团体赛: 按体重级别",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "柔道服: IJF认证",
            "腰带: 白色/蓝色",
            "护具: 不允许",
            "眼部保护: 强制",
            "贴布: 眼部保护贴",
            "禁止: 硬质物品",
        ]
    }

    /// 技术规则
    pub fn technique(&self) -> Vec<&'static str> {
        vec![
            "起始握把: 双方先接触",
            "主裁判口令引导",
            "得分: 一本/技有/有效",
            "寝技: 允许地面技术",
            "固技: 20秒一本",
            "绞技/关节技: 允许",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "脱离握把",
            "消极比赛",
            "危险动作",
            "界外出逃",
            "违反礼仪",
            "不接受裁判指令",
        ]
    }

    /// 适应性规则
    pub fn adaptations(&self) -> Vec<&'static str> {
        vec![
            "起始握把: 必须接触",
            "声音引导: 允许",
            "裁判口令: 清晰大声",
            "对手方位: 触觉提示",
            "眼部保护: 强制",
            "教练位置: 允许靠近",
        ]
    }

    /// 参赛资格
    pub fn eligibility(&self) -> Vec<&'static str> {
        vec![
            "IBSA视力认证",
            "最低视力残疾标准",
            "IJF注册",
            "体重认证",
            "国家级分级证书",
        ]
    }
}

impl Default for ParaJudoRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ParaJudoRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("para_judo")
    }

    fn explain(&self) -> String {
        format!(
            "【残疾人盲人柔道规则】\n\n\
            运动分级:\n{}\n\n\
            比赛项目:\n{}\n\n\
            装备要求:\n{}\n\n\
            技术规则:\n{}",
            self.classification()
                .iter()
                .map(|c| format!("  • {}", c))
                .collect::<Vec<_>>()
                .join("\n"),
            self.events()
                .iter()
                .map(|e| format!("  • {}", e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.equipment()
                .iter()
                .map(|eq| format!("  • {}", eq))
                .collect::<Vec<_>>()
                .join("\n"),
            self.technique()
                .iter()
                .map(|t| format!("  • {}", t))
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_para_judo_rules_basic() {
        let rules = ParaJudoRules::new();
        assert_eq!(rules.metadata().name, "残疾人盲人柔道规则");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_para_judo_classification() {
        let rules = ParaJudoRules::new();
        let classification = rules.classification();
        assert!(classification.iter().any(|c| c.contains("J1")));
        assert!(classification.iter().any(|c| c.contains("J2")));
        assert!(classification.iter().any(|c| c.contains("视力")));
        assert!(classification.len() >= 4);
    }

    #[test]
    fn test_para_judo_events() {
        let rules = ParaJudoRules::new();
        let events = rules.events();
        assert!(events.iter().any(|e| e.contains("残奥会")));
        assert!(events.iter().any(|e| e.contains("世界锦标赛")));
        assert!(events.len() >= 4);
    }

    #[test]
    fn test_para_judo_technique() {
        let rules = ParaJudoRules::new();
        let technique = rules.technique();
        assert!(technique.iter().any(|t| t.contains("一本")));
        assert!(technique.iter().any(|t| t.contains("握把")));
        assert!(technique.len() >= 4);
    }

    #[test]
    fn test_para_judo_category() {
        let rules = ParaJudoRules::new();
        assert!(matches!(rules.category(), RuleCategory::Sports(_)));
    }
}
