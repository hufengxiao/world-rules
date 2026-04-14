//! 混沌理论定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 混沌理论定律集合
pub struct ChaosTheoryLaws {
    metadata: RuleMetadata,
}

impl ChaosTheoryLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "混沌理论定律",
                "混沌理论基本定律"
            )
            .with_origin("科学")
            .with_tags(vec!["科学".into(), "混沌".into()]),
        }
    }

    /// 混沌特性定律
    pub fn characteristics_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("初始敏感性定律", "蝴蝶效应", "初始微小变化巨大影响"),
            ("确定性混沌定律", "确定系统", "确定性系统产生混沌"),
            ("不可预测定律", "长期不可预测", "混沌长期不可预测"),
            ("非线性定律", "非线性关系", "混沌源于非线性"),
            ("边界定律", "混沌边界", "混沌发生边界"),
            ("奇异吸引子定律", "吸引结构", "混沌系统吸引子"),
            ("遍历定律", "遍历特性", "混沌系统遍历性"),
            ("混合定律", "状态混合", "混沌状态混合"),
        ]
    }

    /// 分形定律
    pub fn fractal_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("自相似定律", "结构重复", "分形结构自相似"),
            ("分形维定律", "非整数维", "分形维度计算"),
            ("迭代定律", "迭代生成", "分形迭代生成"),
            ("尺度定律", "尺度不变", "分形尺度不变性"),
            ("复杂定律", "复杂结构", "分形复杂结构"),
            ("边界定律", "分形边界", "分形边界特性"),
        ]
    }

    /// 倍周期定律
    pub fn bifurcation_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("倍周期分岔定律", "周期加倍", "混沌倍周期路径"),
            ("费根鲍姆定律", "常数关系", "分岔通用常数"),
            ("周期定律", "周期窗口", "混沌周期窗口"),
            ("参数定律", "参数变化", "参数与混沌关系"),
            ("分岔定律", "分岔点", "系统分岔过程"),
            ("混沌边界定律", "混沌边界", "进入混沌的边界"),
        ]
    }

    /// 吸引子定律
    pub fn attractor_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("吸引子定律", "吸引状态", "系统趋向吸引子"),
            ("固定点吸引子", "单点吸引", "收敛到固定点"),
            ("周期吸引子定律", "周期吸引", "收敛到周期轨道"),
            ("奇异吸引子定律", "混沌吸引", "混沌奇异吸引子"),
            ("洛伦兹吸引子定律", "洛伦兹系统", "洛伦兹系统吸引子"),
            ("李雅普诺夫指数定律", "稳定性度量", "李雅普诺夫指数"),
        ]
    }

    /// 混沌系统
    pub fn systems(&self) -> Vec<&'static str> {
        vec![
            "洛伦兹系统",
            "Rossler系统",
            "Logistic映射",
            "Henon映射",
            "双摆系统",
            "流体湍流",
            "心脏跳动",
            "天气系统",
        ]
    }

    /// 混沌应用
    pub fn applications(&self) -> Vec<&'static str> {
        vec![
            "天气预报",
            "流体力学",
            "经济学",
            "生态学",
            "生物学",
            "医学",
            "通信加密",
            "艺术创作",
        ]
    }
}

impl Default for ChaosTheoryLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ChaosTheoryLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("chaos_theory")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【混沌理论定律】\n\n特性定律:\n{}\n\n分形定律:\n{}\n\n倍周期定律:\n{}\n",
            self.characteristics_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.fractal_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.bifurcation_laws().iter()
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
    fn test_chaos_theory_laws() {
        let laws = ChaosTheoryLaws::new();
        assert!(!laws.characteristics_laws().is_empty());
        assert!(!laws.fractal_laws().is_empty());
    }
}