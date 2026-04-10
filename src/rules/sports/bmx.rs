//! BMX规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// BMX规则
pub struct BmxRules {
    metadata: RuleMetadata,
}

impl BmxRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "BMX规则",
                "BMX比赛基本规则"
            )
            .with_origin("美国")
            .with_tags(vec!["体育".into(), "极限".into()]),
        }
    }

    /// 比赛项目
    pub fn disciplines(&self) -> Vec<&'static str> {
        vec![
            "BMX竞速: 越野比赛",
            "BMX自由式: 技术表演",
            "BMX街式: 城市障碍",
            "BMX平地花式: 地面技巧",
            "BMX垂直: U型场地",
        ]
    }

    /// 竞速规则
    pub fn racing_rules(&self) -> Vec<&'static str> {
        vec![
            "赛道长度: 300-400米",
            "起跳坡道",
            "障碍物和弯道",
            "8人同时比赛",
            "计时制",
        ]
    }

    /// 自由式规则
    pub fn freestyle_rules(&self) -> Vec<&'static str> {
        vec![
            "限时60秒",
            "动作多样性",
            "创新技术加分",
            "落地质量",
            "全场利用",
        ]
    }

    /// 得分标准
    pub fn scoring_criteria(&self) -> Vec<&'static str> {
        vec![
            "难度: 技术挑战度",
            "执行: 完成质量",
            "幅度: 动作高度",
            "流畅度: 连续性",
            "创新: 新技术",
        ]
    }

    /// 技术动作
    pub fn tricks(&self) -> Vec<&'static str> {
        vec![
            "空中动作: 各种旋转",
            "抓车动作: 多种抓法",
            "翻转动作: 空中翻转",
            "地面动作: 平地技巧",
            "组合动作",
        ]
    }

    /// 车辆规格
    pub fn bike_specifications(&self) -> Vec<&'static str> {
        vec![
            "竞速车: 轻量化设计",
            "自由式车: 结实耐用",
            "轮径: 20英寸标准",
            "车架材料: 钢或铝合金",
            "齿轮比例",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "必须佩戴头盔",
            "护具: 膝盖肘部",
            "车辆检查",
            "赛道安全检查",
            "医疗支持在场",
        ]
    }

    /// 比赛形式
    pub fn competition_formats(&self) -> Vec<&'static str> {
        vec![
            "淘汰赛制",
            "积分赛制",
            "预赛和决赛",
            "排名赛",
            "团队赛",
        ]
    }
}

impl Default for BmxRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for BmxRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("bmx")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【BMX规则】\n\n\
            比赛项目:\n{}\n\n\
            得分标准:\n{}\n\n\
            技术动作:\n{}\n\n\
            车辆规格:\n{}\n",
            self.disciplines().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring_criteria().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.tricks().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.bike_specifications().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bmx_rules() {
        let rules = BmxRules::new();
        assert!(!rules.disciplines().is_empty());
    }
}