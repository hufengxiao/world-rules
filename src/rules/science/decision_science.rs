//! 决策科学定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 决策科学定律集合
pub struct DecisionScienceLaws {
    metadata: RuleMetadata,
}

impl DecisionScienceLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "决策科学定律",
                "决策科学基本定律"
            )
            .with_origin("科学")
            .with_tags(vec!["科学".into(), "决策".into()]),
        }
    }

    /// 决策理论定律
    pub fn theory_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("理性决策定律", "理性选择", "理性决策模型"),
            ("最优决策定律", "最优选择", "最优决策标准"),
            ("满意决策定律", "满意标准", "有限理性决策"),
            ("期望效用定律", "效用最大化", "期望效用理论"),
            ("主观概率定律", "主观概率", "主观概率判断"),
            ("偏好定律", "偏好顺序", "偏好关系"),
            ("选择定律", "选择规则", "选择函数"),
        ]
    }

    /// 决策方法定律
    pub fn method_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("分析定律", "决策分析", "系统决策分析"),
            ("模型定律", "决策建模", "决策模型构建"),
            ("优化定律", "决策优化", "最优决策求解"),
            ("评估定律", "决策评估", "方案评估方法"),
            ("权重定律", "权重确定", "准则权重确定"),
            ("排序定律", "方案排序", "决策方案排序"),
        ]
    }

    /// 多准则决策定律
    pub fn multi_criteria_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("多准则定律", "多重准则", "多准则决策问题"),
            ("权重定律", "准则权重", "准则权重分配"),
            ("帕累托定律", "帕累托最优", "多准则帕累托解"),
            ("折衷定律", "准则折衷", "准则冲突折衷"),
            ("层次分析定律", "AHP方法", "层次分析法"),
            ("效用聚合定律", "效用综合", "多准则效用聚合"),
        ]
    }

    /// 群体决策定律
    pub fn group_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("群体决策定律", "集体选择", "群体决策过程"),
            ("投票定律", "投票机制", "投票决策方法"),
            ("共识定律", "共识达成", "共识决策方法"),
            ("协商定律", "协商过程", "协商决策方法"),
            ("阿罗定律", "不可能定理", "阿罗不可能定理"),
            ("偏好聚合定律", "偏好综合", "群体偏好聚合"),
        ]
    }

    /// 决策偏差定律
    pub fn bias_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("偏差定律", "决策偏差", "决策常见偏差"),
            ("锚定定律", "锚定效应", "初始锚定影响"),
            ("框架定律", "框架效应", "问题框架影响"),
            ("确认定律", "确认偏差", "信息确认倾向"),
            ("过度自信定律", "自信偏差", "过度自信倾向"),
            ("损失厌恶定律", "损失厌恶", "损失比收益敏感"),
        ]
    }

    /// 决策类型
    pub fn decision_types(&self) -> Vec<&'static str> {
        vec![
            "战略决策",
            "战术决策",
            "操作决策",
            "个人决策",
            "组织决策",
            "紧急决策",
            "常规决策",
            "风险决策",
        ]
    }

    /// 决策工具
    pub fn tools(&self) -> Vec<&'static str> {
        vec![
            "决策树",
            "期望效用",
            "成本效益分析",
            "风险分析",
            "博弈分析",
            "层次分析法",
            "多属性决策",
            "群体决策系统",
        ]
    }
}

impl Default for DecisionScienceLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for DecisionScienceLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("decision_science")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【决策科学定律】\n\n理论定律:\n{}\n\n方法定律:\n{}\n\n多准则定律:\n{}\n",
            self.theory_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.method_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.multi_criteria_laws().iter()
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
    fn test_decision_science_laws() {
        let laws = DecisionScienceLaws::new();
        assert!(!laws.theory_laws().is_empty());
        assert!(!laws.bias_laws().is_empty());
    }
}