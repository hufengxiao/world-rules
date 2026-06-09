//! 网络科学定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 网络科学定律集合
pub struct NetworkScienceLaws {
    metadata: RuleMetadata,
}

impl NetworkScienceLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("网络科学定律", "网络科学基本定律")
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

    /// 网络控制定律
    pub fn control_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("网络可控性定律", "控制节点", "最少驱动节点控制网络"),
            ("网络可观性定律", "观测节点", "网络状态可观测性"),
            ("网络传播控制定律", "传播干预", "控制网络传播过程"),
            ("网络级联定律", "级联失效", "网络级联故障机制"),
            ("网络修复定律", "恢复机制", "网络故障后恢复策略"),
            ("网络免疫定律", "免疫策略", "网络免疫保护策略"),
            ("网络同步控制定律", "同步调控", "网络同步行为调控"),
            ("网络诱导定律", "舆论引导", "网络舆论引导策略"),
        ]
    }

    /// 网络应用定律
    pub fn application_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("推荐网络定律", "推荐系统", "社交网络推荐机制"),
            ("影响力定律", "影响力传播", "网络影响力最大化"),
            ("网络搜索定律", "搜索算法", "网络信息搜索策略"),
            ("网络路由定律", "路由优化", "网络路由效率优化"),
            ("网络博弈定律", "网络博弈", "网络上的博弈行为"),
            ("网络压缩定律", "数据压缩", "网络数据压缩传输"),
            ("网络安全定律", "安全防护", "网络安全防护机制"),
            ("网络可视化定律", "可视化", "网络结构可视化方法"),
        ]
    }

    /// 时序网络定律
    pub fn temporal_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("时序网络定律", "时间网络", "考虑时间维度的网络"),
            ("接触序列定律", "接触模式", "时序接触序列分析"),
            ("网络记忆定律", "记忆效应", "网络历史影响当前"),
            ("网络节奏定律", "周期模式", "网络活动周期规律"),
            ("网络预测定律", "链路预测", "预测未来网络连接"),
            ("网络趋势定律", "趋势分析", "网络发展趋势分析"),
            ("网络衰减定律", "关系衰减", "社交关系时间衰减"),
            ("网络爆发定律", "突发活动", "网络突发活动模式"),
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

    fn explain(&self) -> String {
        format!(
            "【网络科学定律】\n\n结构定律:\n{}\n\n动力学定律:\n{}\n\n模型定律:\n{}\n\n网络控制定律:\n{}\n\n网络应用定律:\n{}\n\n时序网络定律:\n{}\n",
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
                .join("\n"),
            self.control_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.application_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.temporal_laws().iter()
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
