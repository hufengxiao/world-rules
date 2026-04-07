//! 跳水规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 跳水规则
pub struct DivingRules {
    metadata: RuleMetadata,
}

impl DivingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "跳水规则",
                "FINA 国际泳联跳水标准规则"
            )
            .with_origin("FINA")
            .with_tags(vec!["体育".into(), "跳水".into()]),
        }
    }

    /// 跳水项目
    pub fn events(&self) -> Vec<&'static str> {
        vec![
            "1米跳板",
            "3米跳板",
            "10米跳台",
            "双人3米跳板",
            "双人10米跳台",
        ]
    }

    /// 跳水姿势
    pub fn dive_positions(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            ("A", "直体"),
            ("B", "屈体"),
            ("C", "抱膝"),
            ("D", "任意姿势"),
        ]
    }

    /// 评分规则
    pub fn scoring_rules(&self) -> Vec<&'static str> {
        vec![
            "7名裁判打分(双人11名)",
            "分数范围: 0-10分",
            "去掉两个最高分和两个最低分",
            "总分 = 有效分之和 × 难度系数 × 0.6",
        ]
    }

    /// 难度系数
    pub fn difficulty_factors(&self) -> Vec<&'static str> {
        vec![
            "根据动作组别、翻腾周数、姿势计算",
            "难度范围: 1.2 - 3.8",
            "翻腾越多难度越高",
            "屈体/抱膝比直体难度高",
        ]
    }

    /// 扣分因素
    pub fn deduction_factors(&self) -> Vec<&'static str> {
        vec![
            "入水水花大小",
            "空中姿势",
            "起跳高度",
            "动作完成度",
            "身体角度",
        ]
    }

    /// 动作组别
    pub fn dive_groups(&self) -> Vec<&'static str> {
        vec![
            "第1组: 面对池向前跳水",
            "第2组: 面对板向后跳水",
            "第3组: 面对池反身跳水",
            "第4组: 面对板向内跳水",
            "第5组: 转体跳水",
            "第6组: 臂立跳水(仅跳台)",
        ]
    }
}

impl Default for DivingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for DivingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("diving")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        let positions = self.dive_positions();
        format!(
            "【跳水规则】\n\n\
            跳水项目:\n{}\n\n\
            跳水姿势:\n{}\n\n\
            评分规则:\n{}\n\n\
            动作组别:\n{}\n",
            self.events().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            positions.iter().map(|(c, n)| format!("  • {} - {}", c, n)).collect::<Vec<_>>().join("\n"),
            self.scoring_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.dive_groups().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diving_rules() {
        let rules = DivingRules::new();
        assert_eq!(rules.dive_positions().len(), 4);
    }
}