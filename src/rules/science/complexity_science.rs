//! 复杂性科学定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 复杂性科学定律集合
pub struct ComplexityScienceLaws {
    metadata: RuleMetadata,
}

impl ComplexityScienceLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "复杂性科学定律",
                "复杂性科学基本定律"
            )
            .with_origin("科学")
            .with_tags(vec!["科学".into(), "复杂".into()]),
        }
    }

    /// 复杂性定律
    pub fn complexity_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("复杂性定律", "复杂程度", "系统复杂性度量"),
            ("算法复杂性定律", "柯尔莫哥洛夫", "最短程序描述"),
            ("计算复杂性定律", "P-NP问题", "计算复杂度分类"),
            ("信息复杂性定律", "信息量", "信息复杂度量"),
            ("结构复杂性定律", "结构复杂", "结构复杂性分析"),
            ("动态复杂性定律", "动态复杂", "动态行为复杂"),
            ("涌现定律", "涌现现象", "整体涌现新特性"),
            ("自组织定律", "自发组织", "系统自发组织"),
        ]
    }

    /// 复杂系统定律
    pub fn system_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("多组分定律", "众多组成", "大量组分相互作用"),
            ("非线性定律", "非线性相互作用", "组分非线性关系"),
            ("反馈定律", "正负反馈", "系统反馈回路"),
            ("适应定律", "适应性", "系统适应环境"),
            ("演化定律", "系统演化", "复杂系统演化"),
            ("层次定律", "层次结构", "系统层次组织"),
            ("网络定律", "网络结构", "组分网络连接"),
            ("开放定律", "开放系统", "系统与环境交互"),
        ]
    }

    /// 复杂行为定律
    pub fn behavior_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("涌现行为定律", "涌现特性", "整体涌现特性"),
            ("相变定律", "临界相变", "系统相变现象"),
            ("临界定律", "临界状态", "系统临界点"),
            ("分岔定律", "行为分岔", "行为分岔转变"),
            ("吸引定律", "吸引子", "行为趋向吸引子"),
            ("波动定律", "波动行为", "系统波动现象"),
            ("周期定律", "周期行为", "周期性行为"),
            ("混沌定律", "混沌行为", "混沌复杂行为"),
        ]
    }

    /// 复杂性方法
    pub fn methods(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("模拟定律", "模拟方法", "计算机模拟"),
            ("建模定律", "建模方法", "复杂系统建模"),
            ("分析定律", "分析方法", "复杂系统分析"),
            ("控制定律", "控制方法", "复杂系统控制"),
            ("预测定律", "预测方法", "复杂行为预测"),
            ("优化定律", "优化方法", "复杂系统优化"),
        ]
    }

    /// 复杂系统类型
    pub fn system_types(&self) -> Vec<&'static str> {
        vec![
            "生态系统",
            "经济系统",
            "社会系统",
            "大脑系统",
            "免疫系统",
            "城市系统",
            "交通系统",
            "互联网",
        ]
    }

    /// 复杂性度量
    pub fn measures(&self) -> Vec<&'static str> {
        vec![
            "熵",
            "信息量",
            "维度",
            "连接度",
            "分形维",
            "复杂度指数",
            "关联长度",
            "相干长度",
        ]
    }
}

impl Default for ComplexityScienceLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ComplexityScienceLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("complexity_science")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【复杂性科学定律】\n\n复杂性定律:\n{}\n\n系统定律:\n{}\n\n行为定律:\n{}\n",
            self.complexity_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.system_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.behavior_laws().iter()
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
    fn test_complexity_science_laws() {
        let laws = ComplexityScienceLaws::new();
        assert!(!laws.complexity_laws().is_empty());
        assert!(!laws.system_laws().is_empty());
    }
}