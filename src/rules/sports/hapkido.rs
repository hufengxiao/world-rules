//! 韩国合气道规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 韩国合气道规则
pub struct HapkidoRules {
    metadata: RuleMetadata,
}

impl HapkidoRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "韩国合气道规则",
                "韩国武术合气道规则"
            )
            .with_origin("韩国")
            .with_tags(vec!["体育".into(), "武术".into()]),
        }
    }

    /// 技术体系
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "投掷技术",
            "擒拿技术",
            "打击技术",
            "压力点技术",
            "武器防御",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "表演比赛",
            "对练比赛",
            "技术展示",
            "评分标准",
            "安全规则",
        ]
    }

    /// 得分标准
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "技术难度",
            "执行质量",
            "流畅表现",
            "控制能力",
            "防御展示",
        ]
    }

    /// 武器防御
    pub fn weapon_defense(&self) -> Vec<&'static str> {
        vec![
            "刀具防御",
            "棍棒防御",
            "武器夺取",
            "多人防御",
            "日常应用",
        ]
    }

    /// 级别体系
    pub fn belt_system(&self) -> Vec<&'static str> {
        vec![
            "白带: 初级",
            "黄带: 进阶",
            "蓝带: 中级",
            "红带: 高级",
            "黑带: 专家",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "控制力度",
            "护具佩戴",
            "教练指导",
            "循序渐进",
            "医疗支持",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "武术服装",
            "腰带标识",
            "防护手套",
            "训练武器",
            "比赛场地",
        ]
    }
}

impl Default for HapkidoRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for HapkidoRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("hapkido")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【韩国合气道规则】\n\n\
            技术体系:\n{}\n\n\
            武器防御:\n{}\n\n\
            级别体系:\n{}\n\n\
            装备要求:\n{}\n",
            self.techniques().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.weapon_defense().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.belt_system().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.equipment().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hapkido_rules() {
        let rules = HapkidoRules::new();
        assert!(!rules.techniques().is_empty());
    }
}