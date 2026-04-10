//! 潜水规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 潜水规则
pub struct DivingSportRules {
    metadata: RuleMetadata,
}

impl DivingSportRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "潜水规则",
                "潜水运动基本规则"
            )
            .with_origin("法国")
            .with_tags(vec!["体育".into(), "水上".into()]),
        }
    }

    /// 潜水类型
    pub fn diving_types(&self) -> Vec<&'static str> {
        vec![
            "自由潜水: 屏气潜水",
            "水肺潜水: 呼吸器潜水",
            "技术潜水: 深水潜水",
            "洞穴潜水: 洞穴探索",
            "竞技潜水: 竞赛潜水",
        ]
    }

    /// 竞技项目
    pub fn competition_events(&self) -> Vec<&'static str> {
        vec![
            "恒重下潜: 鳍泳下潜",
            "动态屏气: 水平距离",
            "静态屏气: 时间挑战",
            "变重下潜: 配重下潜",
            "无限制下潜: 深度挑战",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "必须有潜伴",
            "安全绳连接",
            "潜水计划",
            "安全停留",
            "应急程序",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "面镜: 视力保护",
            "呼吸管: 水面呼吸",
            "脚蹼: 推进力",
            "潜水服: 保暖保护",
            "配重: 下潜辅助",
        ]
    }

    /// 认证等级
    pub fn certification_levels(&self) -> Vec<&'static str> {
        vec![
            "开放水域潜水员",
            "进阶开放水域潜水员",
            "救援潜水员",
            "潜水长",
            "教练等级",
        ]
    }

    /// 深度限制
    pub fn depth_limits(&self) -> Vec<&'static str> {
        vec![
            "休闲潜水: 40米以内",
            "进阶潜水: 30米以内",
            "自由潜水: 个人能力范围",
            "技术潜水: 特殊训练",
            "减压限制",
        ]
    }

    /// 禁止行为
    pub fn prohibited_actions(&self) -> Vec<&'static str> {
        vec![
            "单独潜水",
            "超过训练深度",
            "忽视安全规则",
            "快速上浮",
            "触摸海洋生物",
        ]
    }

    /// 健康要求
    pub fn health_requirements(&self) -> Vec<&'static str> {
        vec![
            "潜水体检",
            "心血管健康",
            "呼吸系统健康",
            "耳鼻喉检查",
            "心理状态良好",
        ]
    }
}

impl Default for DivingSportRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for DivingSportRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("diving_sport")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【潜水规则】\n\n\
            潜水类型:\n{}\n\n\
            安全规则:\n{}\n\n\
            装备要求:\n{}\n\n\
            禁止行为:\n{}\n",
            self.diving_types().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.safety_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.equipment().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.prohibited_actions().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diving_sport_rules() {
        let rules = DivingSportRules::new();
        assert!(!rules.diving_types().is_empty());
    }
}