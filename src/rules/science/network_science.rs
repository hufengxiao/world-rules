//! 网络科学定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 网络科学定律集合
pub struct NetworkScienceLaws {
    metadata: RuleMetadata,
}

impl NetworkScienceLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "网络科学定律",
                "网络科学基本定律"
            )
            .with_origin("科学")
            .with_tags(vec!["科学".into(), "网络".into()]),
        }
    }

    /// 网络结构定律
    pub fn structure_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("小世界定律", "六度分隔", "节点距离很短"),
            ("无标度定律", "幂律分布", "少数节点高度连接"),
            ("聚类定律", "局部聚类", "节点形成聚类"),
            ("中心性定律", "中心节点", "核心节点识别"),
            ("连接定律", "连接规律", "节点连接规律"),
            ("度分布定律", "度值分布", "节点度分布"),
            ("路径定律", "路径特性", "节点路径长度"),
            ("连通定律", "网络连通", "网络连通性"),
        ]
    }

    /// 网络动力学定律
    pub fn dynamics_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("传播定律", "信息传播", "信息在网络传播"),
            ("扩散定律", "物质扩散", "物质网络扩散"),
            ("同步定律", "网络同步", "节点同步现象"),
            ("演化定律", "网络演化", "网络动态变化"),
            ("涌现定律", "涌现现象", "网络涌现行为"),
            ("相变定律", "相变现象", "网络相变过程"),
            ("鲁棒性定律", "抗破坏", "网络抗破坏能力"),
            ("脆弱性定律", "脆弱节点", "关键节点脆弱"),
        ]
    }

    /// 网络模型定律
    pub fn model_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("随机网络定律", "随机连接", "ER随机图模型"),
            ("规则网络定律", "规则连接", "规则网络模型"),
            ("小世界模型定律", "WS模型", "小世界网络构建"),
            ("无标度模型定律", "BA模型", "无标度网络构建"),
            ("层次模型定律", "层次结构", "层次网络模型"),
            ("空间网络定律", "空间约束", "空间嵌入网络"),
        ]
    }

    /// 网络分析方法
    pub fn analysis_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("拓扑分析定律", "拓扑特性", "网络拓扑分析"),
            ("统计分析定律", "统计特性", "网络统计分析"),
            ("结构分析定律", "结构识别", "网络结构分析"),
            ("功能分析定律", "功能特性", "网络功能分析"),
            ("社区发现定律", "社区结构", "网络社区发现"),
            ("链预测定律", "链路预测", "预测未来连接"),
        ]
    }

    /// 网络类型
    pub fn network_types(&self) -> Vec<&'static str> {
        vec![
            "社交网络",
            "互联网",
            "交通网络",
            "生物网络",
            "经济网络",
            "知识网络",
            "电力网络",
            "通信网络",
        ]
    }

    /// 网络指标
    pub fn metrics(&self) -> Vec<&'static str> {
        vec![
            "度",
            "路径长度",
            "聚类系数",
            "中心性",
            "连通度",
            "密度",
            "直径",
            "模块度",
        ]
    }
}

impl Default for NetworkScienceLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for NetworkScienceLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("network_science")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【网络科学定律】\n\n结构定律:\n{}\n\n动力学定律:\n{}\n\n模型定律:\n{}\n",
            self.structure_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.dynamics_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.model_laws().iter()
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
    fn test_network_science_laws() {
        let laws = NetworkScienceLaws::new();
        assert!(!laws.structure_laws().is_empty());
        assert!(!laws.dynamics_laws().is_empty());
    }
}