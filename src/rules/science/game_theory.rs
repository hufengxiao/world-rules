//! 博弈论定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 博弈论定律集合
pub struct GameTheoryLaws {
    metadata: RuleMetadata,
}

impl GameTheoryLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "博弈论定律",
                "博弈论基本定律"
            )
            .with_origin("数学")
            .with_tags(vec!["科学".into(), "数学".into(), "博弈".into()]),
        }
    }

    /// 基本定律
    pub fn basic_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("纳什均衡定律", "均衡状态", "无人单方面改变"),
            ("帕累托最优定律", "最优状态", "无法在不损害他人下改进"),
            ("零和博弈定律", "总和为零", "一方所得即另一方所失"),
            ("非零和博弈定律", "总和不定", "可能双赢或双输"),
            ("囚徒困境定律", "个体理性导致集体非理性", "两囚徒背叛"),
            ("理性选择定律", "理性假设", "参与者理性决策"),
            ("信息完全定律", "完全信息", "信息对称博弈"),
            ("信息不完全定律", "不完全信息", "信息不对称博弈"),
        ]
    }

    /// 策略定律
    pub fn strategy_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("最优策略定律", "策略选择", "最优响应策略"),
            ("混合策略定律", "概率组合", "以概率选择策略"),
            ("纯策略定律", "确定选择", "确定性策略"),
            ("占优策略定律", "绝对最优", "无论对手如何最优"),
            ("劣势策略定律", "劣势淘汰", "淘汰劣势策略"),
            ("威胁定律", "可信威胁", "可信威胁策略"),
            ("承诺定律", "可信承诺", "可信承诺机制"),
        ]
    }

    /// 合作博弈定律
    pub fn cooperative_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("合作博弈定律", "合作联盟", "合作形成联盟"),
            ("核心定律", "核心分配", "联盟稳定分配"),
            ("夏普利值定律", "贡献度量", "公平贡献分配"),
            ("核仁定律", "核仁解", "最小最大不满意"),
            ("联盟形成定律", "联盟博弈", "联盟结构形成"),
            ("收益分配定律", "分配机制", "联盟收益分配"),
        ]
    }

    /// 动态博弈定律
    pub fn dynamic_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("子博弈完美定律", "完美均衡", "每个子博弈纳什均衡"),
            ("逆向归纳定律", "逆向推理", "从终点逆向推理"),
            ("重复博弈定律", "重复进行", "同一博弈重复"),
            ("触发策略定律", "触发惩罚", "偏离触发惩罚"),
            ("序贯博弈定律", "顺序进行", "参与者轮流决策"),
            ("无限博弈定律", "无限重复", "无终止博弈"),
        ]
    }

    /// 博弈类型
    pub fn game_types(&self) -> Vec<&'static str> {
        vec![
            "静态博弈",
            "动态博弈",
            "完全信息博弈",
            "不完全信息博弈",
            "合作博弈",
            "非合作博弈",
            "重复博弈",
            "随机博弈",
        ]
    }

    /// 博弈应用
    pub fn applications(&self) -> Vec<&'static str> {
        vec![
            "经济学",
            "政治学",
            "国际关系",
            "生物学",
            "计算机科学",
            "军事战略",
            "商业竞争",
            "体育比赛",
        ]
    }
}

impl Default for GameTheoryLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for GameTheoryLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("game_theory")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【博弈论定律】\n\n基本定律:\n{}\n\n策略定律:\n{}\n\n合作定律:\n{}\n",
            self.basic_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.strategy_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.cooperative_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_theory_laws() {
        let laws = GameTheoryLaws::new();
        assert!(!laws.basic_laws().is_empty());
        assert!(!laws.strategy_laws().is_empty());
    }
}