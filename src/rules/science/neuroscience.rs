//! 神经科学定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 神经科学定律集合
pub struct NeuroscienceLaws {
    metadata: RuleMetadata,
}

impl NeuroscienceLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "神经科学定律",
                "神经科学基本定律"
            )
            .with_origin("神经科学")
            .with_tags(vec!["科学".into(), "神经".into()]),
        }
    }

    /// 神经传导定律
    pub fn conduction_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("动作电位定律", "全或无原则", "神经信号要么完全激发要么不激发"),
            ("离子通道定律", "选择性通透", "离子通道选择性通过特定离子"),
            ("突触传递定律", "化学传递", "神经信号通过化学递质传递"),
            ("神经编码定律", "频率编码", "神经信号通过频率编码信息"),
            ("神经整合定律", "时空总和", "多个输入信号的整合"),
            ("神经可塑性定律", "赫布定律", "同时激活的神经元连接增强"),
        ]
    }

    /// 大脑功能定律
    pub fn brain_function_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("大脑分区定律", "功能定位", "不同脑区负责不同功能"),
            ("半球分工定律", "左逻辑右创意", "左右半球功能差异"),
            ("记忆定律", "编码存储提取", "记忆三阶段"),
            ("学习定律", "神经可塑性", "学习改变大脑结构"),
            ("注意定律", "选择性注意", "大脑选择性处理信息"),
            ("情绪定律", "边缘系统", "情绪由边缘系统控制"),
        ]
    }

    /// 认知定律
    pub fn cognitive_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("认知负荷定律", "有限容量", "工作记忆容量有限"),
            ("感知定律", "感觉阈值", "感知的最小刺激强度"),
            ("反应定律", "反应时间", "刺激到反应的时间"),
            ("决策定律", "成本收益", "决策基于成本收益分析"),
            ("启发式定律", "快速决策", "使用启发式快速决策"),
            ("偏差定律", "认知偏差", "系统性决策偏差"),
        ]
    }

    /// 神经现象
    pub fn phenomena(&self) -> Vec<&'static str> {
        vec![
            "睡眠周期",
            "做梦",
            "意识状态",
            "冥想",
            "幻觉",
            "错觉",
            "记忆闪回",
            "反射",
            "习惯形成",
            "条件反射",
            "无条件反射",
            "睡眠剥夺",
        ]
    }

    /// 神经递质
    pub fn neurotransmitters(&self) -> Vec<&'static str> {
        vec![
            "多巴胺: 奖赏与动机",
            "血清素: 情绪调节",
            "乙酰胆碱: 学习记忆",
            "去甲肾上腺素: 注意警觉",
            "谷氨酸: 兴奋性",
            "GABA: 抑制性",
            "内啡肽: 止痛",
        ]
    }

    /// 神经元与突触
    pub fn neuron_synapse(&self) -> Vec<&'static str> {
        vec![
            "动作电位: 神经元去极化达到阈值后产生的全或无电信号",
            "突触传递: 神经递质从突触前膜释放经间隙作用于后膜",
            "长时程增强: 高频刺激后突触传递效率持久增强的可塑性",
            "赫布学习规则: 同时激活的神经元之间连接强度增加",
            "神经可塑性: 大脑神经网络结构和功能随经验改变的能力",
            "髓鞘化: 少突胶质细胞包裹轴突加速神经信号传导",
            "神经递质: 多巴胺血清素乙酰胆碱等化学信号分子",
        ]
    }

    /// 脑区功能
    pub fn brain_regions(&self) -> Vec<&'static str> {
        vec![
            "布洛卡区: 左额叶下回损伤导致运动性失语",
            "韦尼克区: 左颞叶上回损伤导致感觉性失语",
            "海马体: 将短时记忆转化为长时记忆的关键脑区",
            "杏仁核: 情绪加工特别是恐惧反应的核心结构",
            "前额叶皮层: 执行功能决策判断和工作记忆的中枢",
            "小脑: 运动协调平衡维持和运动学习的重要结构",
            "基底神经节: 随意运动启动和程序性学习的调节中枢",
        ]
    }


    /// 认知神经科学
    pub fn cognitive_neuroscience(&self) -> Vec<&'static str> {
        vec![
            "意识的神经关联: 丘脑皮层回路与意识体验相关",
            "注意网络: 警觉定向和执行控制三个注意网络",
            "记忆巩固: 海马依赖的记忆逐渐转移到皮层",
            "决策神经基础: 前额叶和基底神经节参与决策",
            "语言网络: 布洛卡区韦尼克区和弓状束连接",
            "情绪调节: 前额叶对杏仁核活动的自上而下调控",
            "社会认知: 内侧前额叶和颞顶联合区参与心理理论",
        ]
    }

}

impl Default for NeuroscienceLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for NeuroscienceLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("neuroscience")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【神经科学定律】\n\n神经传导定律:\n{}\n\n大脑功能定律:\n{}\n\n认知定律:\n{}\n",
            self.conduction_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.brain_function_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.cognitive_laws().iter()
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
    fn test_neuroscience_laws() {
        let laws = NeuroscienceLaws::new();
        assert!(!laws.conduction_laws().is_empty());
    }
}