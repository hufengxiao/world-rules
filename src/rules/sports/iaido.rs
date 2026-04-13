//! 居合道规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 居合道规则
pub struct IaidoRules {
    metadata: RuleMetadata,
}

impl IaidoRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "居合道规则",
                "居合道比赛基本规则"
            )
            .with_origin("日本")
            .with_tags(vec!["体育".into(), "武术".into()]),
        }
    }

    /// 比赛形式
    pub fn competition_formats(&self) -> Vec<&'static str> {
        vec![
            "演武表演",
            "技法展示",
            "级别评定",
            "比赛较少",
            "团体演武",
        ]
    }

    /// 基本技法
    pub fn basic_techniques(&self) -> Vec<&'static str> {
        vec![
            "拔刀: 刀剑出鞘",
            "斩击: 切断动作",
            "血振: 清除血迹",
            "纳刀: 收刀入鞘",
            "完整套路",
        ]
    }

    /// 套剑规定
    pub fn kata_sets(&self) -> Vec<&'static str> {
        vec![
            "全日本剑道连盟居合",
            "十二本套剑",
            "各流派套剑",
            "技法规定",
            "顺序要求",
        ]
    }

    /// 级位制度
    pub fn ranking_system(&self) -> Vec<&'static str> {
        vec![
            "初段至八段",
            "级别考试",
            "演武考核",
            "技术要求",
            "精神修养",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "居合刀: 模拟刀剑",
            "刀袋",
            "道服",
            "袴",
            "腰带",
        ]
    }

    /// 场地要求
    pub fn dojo_requirements(&self) -> Vec<&'static str> {
        vec![
            "道场铺设榻榻米",
            "演武空间充足",
            "安全设施",
            "通风良好",
            "祭坛设置",
        ]
    }

    /// 评分标准
    pub fn scoring_criteria(&self) -> Vec<&'static str> {
        vec![
            "技法正确性",
            "气势",
            "姿势",
            "节奏",
            "精神状态",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "刀剑安全使用",
            "演武控制",
            "距离保持",
            "观众安全",
            "医疗支持",
        ]
    }
}

impl Default for IaidoRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for IaidoRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("iaido")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【居合道规则】\n\n\
            基本技法:\n{}\n\n\
            级位制度:\n{}\n\n\
            装备要求:\n{}\n\n\
            评分标准:\n{}\n",
            self.basic_techniques().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.ranking_system().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.equipment().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring_criteria().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iaido_rules() {
        let rules = IaidoRules::new();
        assert!(!rules.basic_techniques().is_empty());
    }
}