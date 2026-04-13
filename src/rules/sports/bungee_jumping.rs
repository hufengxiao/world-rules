//! 蹦极规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 蹦极规则
pub struct BungeeJumpingRules {
    metadata: RuleMetadata,
}

impl BungeeJumpingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "蹦极规则",
                "蹦极运动规则"
            )
            .with_origin("新西兰")
            .with_tags(vec!["体育".into(), "极限".into()]),
        }
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "绳索检查",
            "装备检查",
            "重量限制",
            "健康要求",
            "天气条件",
        ]
    }

    /// 技术动作
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "跳跃姿势",
            "反弹控制",
            "着陆技术",
            "安全姿势",
            "应急处理",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "蹦极绳索",
            "安全绑带",
            "防护头盔",
            "附属装备",
            "安全系统",
        ]
    }

    /// 重量规定
    pub fn weight_requirements(&self) -> Vec<&'static str> {
        vec![
            "最小重量限制",
            "最大重量限制",
            "绳索选择",
            "重量测量",
            "安全计算",
        ]
    }

    /// 健康要求
    pub fn health_requirements(&self) -> Vec<&'static str> {
        vec![
            "心脏病禁跳",
            "高血压禁跳",
            "孕妇禁跳",
            "年龄限制",
            "健康声明",
        ]
    }

    /// 高度分类
    pub fn height_categories(&self) -> Vec<&'static str> {
        vec![
            "低高度: 30米以下",
            "中等高度: 30-60米",
            "高高度: 60米以上",
            "极限高度",
            "高度选择",
        ]
    }

    /// 法律规则
    pub fn legal_rules(&self) -> Vec<&'static str> {
        vec![
            "保险要求",
            "责任声明",
            "场地许可",
            "安全认证",
            "合规要求",
        ]
    }
}

impl Default for BungeeJumpingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for BungeeJumpingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("bungee_jumping")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【蹦极规则】\n\n\
            安全规则:\n{}\n\n\
            装备要求:\n{}\n\n\
            健康要求:\n{}\n\n\
            高度分类:\n{}\n",
            self.safety_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.equipment().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.health_requirements().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.height_categories().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bungee_jumping_rules() {
        let rules = BungeeJumpingRules::new();
        assert!(!rules.safety_rules().is_empty());
    }
}