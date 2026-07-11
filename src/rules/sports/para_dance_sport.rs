//! 残疾人舞蹈运动规则
//!
//! 残疾人舞蹈运动是国际认可的残疾人体育运动。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 残疾人舞蹈运动规则
pub struct ParaDanceSportRules {
    metadata: RuleMetadata,
}

impl ParaDanceSportRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("残疾人舞蹈运动规则", "残疾人舞蹈运动比赛规则")
                .with_origin("WDSF/IPC")
                .with_tags(vec!["体育".into(), "舞蹈".into(), "残奥".into()]),
        }
    }

    /// 运动分级
    pub fn classification(&self) -> Vec<&'static str> {
        vec![
            "轮椅1级: 较严重残疾",
            "轮椅2级: 较轻残疾",
            "站立1级: 较严重残疾",
            "站立2级: 较轻残疾",
            "双人: 轮椅+站立",
            "分级评估: 功能测试",
        ]
    }

    /// 比赛项目
    pub fn events(&self) -> Vec<&'static str> {
        vec![
            "标准舞: 华尔兹、探戈等",
            "拉丁舞: 恰恰、桑巴等",
            "单人组: 独舞",
            "双人组: 混合组合",
            "团体舞: 队形舞",
            "世界锦标赛",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "轮椅: 竞技轮椅",
            "服装: 舞蹈专用",
            "舞鞋: 站立组",
            "禁止: 装饰性道具",
            "音乐: 自选音乐",
            "轮椅装饰: 允许",
        ]
    }

    /// 评分规则
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "技术质量: 40%",
            "音乐表现: 30%",
            "编舞创意: 20%",
            "整体印象: 10%",
            "裁判组: 5-7人",
            "评分系统: 10分制",
        ]
    }

    /// 技术规则
    pub fn technique(&self) -> Vec<&'static str> {
        vec![
            "时间: 1.5-2.5分钟",
            "空间利用: 舞池面积",
            "舞蹈元素: 必须包含",
            "轮椅技巧: 转向、旋转",
            "站立技巧: 移动、平衡",
            "禁止: 危险动作",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "超时",
            "越出舞池",
            "服装违规",
            "音乐违规",
            "危险动作",
            "不当行为",
        ]
    }

    /// 适应性规则
    pub fn adaptations(&self) -> Vec<&'static str> {
        vec![
            "轮椅技术评分",
            "上肢功能评分",
            "舞蹈改编: 允许",
            "节奏适配: 允许",
            "搭档辅助: 允许",
            "分级组合比赛",
        ]
    }

    /// 参赛资格
    pub fn eligibility(&self) -> Vec<&'static str> {
        vec![
            "WDSF分级认证",
            "最低残疾标准",
            "国际注册",
            "舞蹈培训证明",
            "体检合格证明",
        ]
    }
}

impl Default for ParaDanceSportRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ParaDanceSportRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("para_dance_sport")
    }

    fn explain(&self) -> String {
        format!(
            "【残疾人舞蹈运动规则】\n\n\
            运动分级:\n{}\n\n\
            比赛项目:\n{}\n\n\
            评分规则:\n{}\n\n\
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
            self.scoring()
                .iter()
                .map(|s| format!("  • {}", s))
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
    fn test_para_dance_sport_rules_basic() {
        let rules = ParaDanceSportRules::new();
        assert_eq!(rules.metadata().name, "残疾人舞蹈运动规则");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_para_dance_sport_classification() {
        let rules = ParaDanceSportRules::new();
        let classification = rules.classification();
        assert!(classification.iter().any(|c| c.contains("轮椅")));
        assert!(classification.iter().any(|c| c.contains("站立")));
        assert!(classification.len() >= 4);
    }

    #[test]
    fn test_para_dance_sport_events() {
        let rules = ParaDanceSportRules::new();
        let events = rules.events();
        assert!(events.iter().any(|e| e.contains("标准舞")));
        assert!(events.iter().any(|e| e.contains("拉丁舞")));
        assert!(events.len() >= 4);
    }

    #[test]
    fn test_para_dance_sport_scoring() {
        let rules = ParaDanceSportRules::new();
        let scoring = rules.scoring();
        assert!(scoring.iter().any(|s| s.contains("技术")));
        assert!(scoring.iter().any(|s| s.contains("音乐")));
        assert!(scoring.len() >= 4);
    }

    #[test]
    fn test_para_dance_sport_category() {
        let rules = ParaDanceSportRules::new();
        assert!(matches!(rules.category(), RuleCategory::Sports(_)));
    }
}
