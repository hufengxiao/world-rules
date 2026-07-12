//! 系统生物学定律
//!
//! 系统生物学研究生物系统的整体性质和动态行为，
//! 通过整合多层次数据理解生物系统的复杂性和涌现性。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 系统生物学定律集合
pub struct SystemsBiologyLaws {
    metadata: RuleMetadata,
}

impl SystemsBiologyLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("系统生物学定律", "系统生物学基本定律和系统分析方法")
                .with_origin("生物学")
                .with_tags(vec!["科学".into(), "生物".into(), "系统".into()]),
        }
    }

    /// 系统涌现定律
    pub fn emergence_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("涌现性质定律", "整体大于部分", "系统整体涌现新性质"),
            ("层次结构定律", "多层次组织", "系统多层次结构"),
            ("网络效应定律", "网络作用", "网络连接产生效应"),
            ("反馈定律", "反馈调节", "正负反馈调节系统"),
            ("自组织定律", "自发组织", "系统自发组织"),
            ("协同定律", "协同作用", "系统组分协同"),
            ("非线性定律", "非线性关系", "系统非线性响应"),
            ("动态平衡定律", "动态稳态", "系统动态稳定"),
        ]
    }

    /// 网络生物学定律
    pub fn network_biology_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("代谢网络定律", "代谢通路", "代谢反应网络"),
            ("调控网络定律", "调控关系", "基因调控网络"),
            ("信号网络定律", "信号传导", "细胞信号网络"),
            ("蛋白质网络定律", "相互作用", "蛋白质相互作用网络"),
            ("神经网络定律", "神经连接", "神经元网络"),
            ("生态系统网络定律", "生态关系", "生态系统网络"),
            ("社会网络定律", "社会关系", "生物社会网络"),
            ("疾病网络定律", "疾病关联", "疾病相关网络"),
        ]
    }

    /// 系统建模定律
    pub fn modeling_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("动力学定律", "动态建模", "系统动力学模型"),
            ("稳态定律", "稳态分析", "系统稳态计算"),
            ("参数估计定律", "参数拟合", "模型参数估计"),
            ("敏感性定律", "敏感性分析", "系统敏感性分析"),
            ("优化定律", "系统优化", "系统性能优化"),
            ("控制定律", "系统控制", "系统控制策略"),
            ("预测定律", "系统预测", "系统行为预测"),
            ("验证定律", "模型验证", "模型实验验证"),
        ]
    }

    /// 整合生物学定律
    pub fn integrative_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("多组学定律", "组学整合", "基因组转录组蛋白组整合"),
            ("多层次定律", "层次整合", "分子细胞组织层次整合"),
            ("时序定律", "时间整合", "系统时间动态整合"),
            ("空间定律", "空间整合", "系统空间分布整合"),
            ("功能定律", "功能整合", "系统功能模块整合"),
            ("进化定律", "进化整合", "系统进化历程整合"),
            ("环境定律", "环境整合", "系统环境因素整合"),
            ("临床定律", "临床整合", "系统临床数据整合"),
        ]
    }

    /// 系统方法
    pub fn systems_methods(&self) -> Vec<&'static str> {
        vec![
            "网络分析: 构建和分析生物网络",
            "动态建模: 建立系统动力学模型",
            "参数估计: 从数据估计模型参数",
            "敏感性分析: 分析系统敏感参数",
            "路径分析: 分析系统路径和通路",
            "模块分析: 分析系统功能模块",
            "状态分析: 分析系统状态变化",
            "稳定性分析: 分析系统稳定性",
        ]
    }

    /// 系统类型
    pub fn system_types(&self) -> Vec<&'static str> {
        vec![
            "代谢系统: 物质代谢和能量代谢",
            "调控系统: 基因调控和表达控制",
            "信号系统: 细胞信号传导",
            "免疫系统: 免疫响应和防御",
            "神经系统: 信息处理和行为控制",
            "发育系统: 发育程序和形态形成",
            "生态系统: 生物与环境相互作用",
            "疾病系统: 疾病发生和发展",
        ]
    }

    /// 系统应用
    pub fn applications(&self) -> Vec<&'static str> {
        vec![
            "药物发现: 系统药物靶点发现",
            "疾病诊断: 系统疾病诊断标志物",
            "精准医学: 个体化系统医学",
            "合成生物学: 系统生物设计",
            "农业改良: 作物系统改良",
            "环境监测: 生态系统监测",
            "生物制造: 工业生物系统",
            "健康管理: 健康系统管理",
        ]
    }
}

impl Default for SystemsBiologyLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SystemsBiologyLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("systems_biology")
    }

    fn explain(&self) -> String {
        format!(
            "【系统生物学定律】\n\n\
            系统涌现定律:\n{}\n\n\
            网络生物学定律:\n{}\n\n\
            系统建模定律:\n{}\n\n\
            整合生物学定律:\n{}\n\n\
            系统方法:\n{}\n\n\
            系统类型:\n{}\n",
            self.emergence_laws()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.network_biology_laws()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.modeling_laws()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.integrative_laws()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.systems_methods()
                .iter()
                .map(|m| format!("  • {}", m))
                .collect::<Vec<_>>()
                .join("\n"),
            self.system_types()
                .iter()
                .map(|t| format!("  • {}", t))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_systems_biology_laws() {
        let rules = SystemsBiologyLaws::new();
        assert_eq!(rules.emergence_laws().len(), 8);
        assert_eq!(rules.network_biology_laws().len(), 8);
        assert_eq!(rules.modeling_laws().len(), 8);
        assert_eq!(rules.integrative_laws().len(), 8);
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_network_biology() {
        let rules = SystemsBiologyLaws::new();
        let laws = rules.network_biology_laws();
        assert!(laws.iter().any(|(n, _, _)| n.contains("代谢网络")));
    }

    #[test]
    fn test_system_types() {
        let rules = SystemsBiologyLaws::new();
        assert_eq!(rules.system_types().len(), 8);
    }
}