//! 神经生物学规则
//!
//! 神经系统的基础生物学原理，包括神经元结构、突触传递、
//! 神经可塑性、神经编码等核心概念。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 神经生物学规则集合
pub struct NeurobiologyRules {
    metadata: RuleMetadata,
}

impl NeurobiologyRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("神经生物学规则", "神经系统基础生物学原理")
                .with_origin("神经生物学")
                .with_tags(vec!["科学".into(), "生命科学".into(), "神经科学".into()]),
        }
    }

    /// 神经元结构定律
    pub fn neuron_structure_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("极化定律", "树突接收轴突输出", "神经元信息流向的单向性"),
            ("胞体定律", "代谢中心", "神经元细胞体是代谢和整合中心"),
            ("轴突定律", "信号传导", "轴突负责长距离信号传导"),
            ("髓鞘定律", "跳跃传导", "髓鞘使动作电位跳跃式传导"),
            ("突触定律", "信号传递", "神经元间通过突触传递信息"),
            ("树突定律", "信息整合", "树突接收并整合输入信号"),
            ("郎飞结定律", "去极化点", "髓鞘间隙处的去极化"),
        ]
    }

    /// 突触传递定律
    pub fn synaptic_transmission_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("化学突触定律", "神经递质传递", "化学突触通过神经递质传递"),
            ("电突触定律", "电信号直接传递", "电突触通过缝隙连接传递"),
            ("兴奋性突触定律", "EPSP产生", "兴奋性突触产生去极化电位"),
            ("抑制性突触定律", "IPSP产生", "抑制性突触产生超极化电位"),
            ("量子释放定律", "囊泡量子释放", "神经递质以量子形式释放"),
            ("突触延迟定律", "传递延迟", "化学突触存在传递延迟"),
            ("突触清除定律", "递质清除", "突触间隙递质快速清除"),
        ]
    }

    /// 动作电位定律
    pub fn action_potential_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("全或无定律", "动作电位特性", "动作电位要么全发生要么不发生"),
            ("阈电位定律", "触发条件", "膜电位达到阈值触发动作电位"),
            ("不应期定律", "绝对不应期", "动作电位后短暂不应期"),
            ("相对不应期定律", "兴奋性降低", "不应期后兴奋性逐渐恢复"),
            ("扩布定律", "传导特性", "动作电位沿膜扩布不衰减"),
            ("离子通道定律", "Na+/K+流动", "动作电位由钠钾离子流动产生"),
            ("复极化定律", "电位恢复", "膜电位恢复到静息状态"),
        ]
    }

    /// 神经可塑性定律
    pub fn neuroplasticity_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("长时程增强LTP", "突触强化", "高频刺激导致突触持久增强"),
            ("长时程抑制LTD", "突触弱化", "低频刺激导致突触持久减弱"),
            ("赫布定律", "同步强化", "一起激发的神经元连接加强"),
            ("突触重塑定律", "结构改变", "突触结构可随经验改变"),
            ("神经发生定律", "新生神经元", "成人大脑可产生新神经元"),
            ("髓鞘可塑性定律", "髓鞘变化", "髓鞘可随经验改变"),
            ("补偿定律", "功能替代", "脑损伤后功能可被其他区域补偿"),
        ]
    }

    /// 神经编码定律
    pub fn neural_coding_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("频率编码定律", "频率代表强度", "神经信号频率编码信息强度"),
            ("时间编码定律", "时间模式", "神经信号时间模式编码信息"),
            ("群体编码定律", "神经元群体", "群体神经元协同编码"),
            ("位置编码定律", "拓扑映射", "神经元位置映射信息"),
            ("相位编码定律", "振荡相位", "振荡相位编码信息"),
            ("稀疏编码定律", "少数激活", "少数神经元激活代表信息"),
            ("冗余编码定律", "鲁棒性", "冗余编码增加信息鲁棒性"),
        ]
    }

    /// 神经递质定律
    pub fn neurotransmitter_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("谷氨酸定律", "主要兴奋性递质", "谷氨酸是中枢主要兴奋性递质"),
            ("GABA定律", "主要抑制性递质", "GABA是中枢主要抑制性递质"),
            ("多巴胺定律", "奖赏运动", "多巴胺调节奖赏和运动"),
            ("血清素定律", "情绪睡眠", "血清素调节情绪和睡眠"),
            ("乙酰胆碱定律", "神经肌肉接头", "乙酰胆碱是神经肌肉接头递质"),
            ("去甲肾上腺素定律", "注意唤醒", "去甲肾上腺素调节注意和唤醒"),
            ("内啡肽定律", "镇痛愉悦", "内啡肽产生镇痛和愉悦感"),
        ]
    }

    /// 感觉神经定律
    pub fn sensory_neural_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("感受器定律", "刺激转导", "感受器将物理刺激转为神经信号"),
            ("感受野定律", "响应区域", "神经元响应特定区域刺激"),
            ("适应定律", "敏感性变化", "持续刺激下感受器敏感性降低"),
            ("侧抑制定律", "对比增强", "相邻感受器相互抑制增强对比"),
            ("特征检测定律", "特征提取", "神经元检测特定特征"),
            ("拓扑映射定律", "有序投射", "感觉表面有序投射到皮层"),
            ("多模态整合定律", "信息融合", "多感觉信息在大脑中整合"),
        ]
    }

    /// 运动神经定律
    pub fn motor_neural_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("运动单位定律", "功能单元", "运动神经元及其支配肌纤维"),
            ("大小原则定律", "招募顺序", "运动单位按大小顺序招募"),
            ("协同肌定律", "协调运动", "协同肌协调完成运动"),
            ("拮抗肌定律", "运动控制", "拮抗肌调控运动精度"),
            ("运动程序定律", "模式生成", "中枢模式发生器产生节律运动"),
            ("反馈控制定律", "误差纠正", "感觉反馈纠正运动误差"),
            ("前馈控制定律", "预测控制", "预期信号预先调整运动"),
        ]
    }

    /// 神经发育定律
    pub fn neural_development_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("神经诱导定律", "命运决定", "信号分子诱导神经细胞命运"),
            ("神经迁移定律", "位置确定", "神经元迁移到正确位置"),
            ("轴突引导定律", "路径寻找", "轴突沿引导分子生长"),
            ("突触形成定律", "连接建立", "突触按特定模式形成"),
            ("突触修剪定律", "优化连接", "多余突触被消除"),
            ("关键期定律", "可塑性窗口", "特定发育期可塑性最高"),
            ("髓鞘化定律", "成熟标志", "髓鞘化标志神经回路成熟"),
        ]
    }

    /// 神经再生定律
    pub fn neural_regeneration_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("外周神经再生定律", "再生能力", "外周神经损伤后可再生"),
            ("中枢神经限制定律", "再生受限", "中枢神经再生能力有限"),
            ("沃勒变性能定律", "轴突变性", "轴突切断后远端变性"),
            ("神经胶质瘢痕定律", "再生障碍", "胶质瘢痕阻碍再生"),
            ("神经营养因子定律", "促进再生", "神经营养因子促进神经生长"),
            ("抑制因子定律", "抑制再生", "中枢存在抑制再生因子"),
            ("干细胞定律", "替代修复", "干细胞可分化为神经细胞"),
        ]
    }
}

impl Default for NeurobiologyRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for NeurobiologyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("neurobiology")
    }

    fn explain(&self) -> String {
        format!(
            "【神经生物学规则】\n\n\
            神经生物学是研究神经系统的科学，涵盖从分子到行为的多层次研究。\n\n\
            神经元结构定律:\n{}\n\n\
            突触传递定律:\n{}\n\n\
            动作电位定律:\n{}\n\n\
            神经可塑性定律:\n{}\n\n\
            神经编码定律:\n{}\n\n\
            神经递质定律:\n{}\n\n\
            感觉神经定律:\n{}\n\n\
            运动神经定律:\n{}\n\n\
            神经发育定律:\n{}\n\n\
            神经再生定律:\n{}",
            self.neuron_structure_laws()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.synaptic_transmission_laws()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.action_potential_laws()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.neuroplasticity_laws()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.neural_coding_laws()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.neurotransmitter_laws()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.sensory_neural_laws()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.motor_neural_laws()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.neural_development_laws()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.neural_regeneration_laws()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neurobiology_rules() {
        let rules = NeurobiologyRules::new();
        assert_eq!(rules.neuron_structure_laws().len(), 7);
        assert_eq!(rules.synaptic_transmission_laws().len(), 7);
        assert_eq!(rules.action_potential_laws().len(), 7);
        assert_eq!(rules.neuroplasticity_laws().len(), 7);
        assert_eq!(rules.neural_coding_laws().len(), 7);
        assert_eq!(rules.neurotransmitter_laws().len(), 7);
        assert_eq!(rules.sensory_neural_laws().len(), 7);
        assert_eq!(rules.motor_neural_laws().len(), 7);
        assert_eq!(rules.neural_development_laws().len(), 7);
        assert_eq!(rules.neural_regeneration_laws().len(), 7);
    }

    #[test]
    fn test_neurobiology_metadata() {
        let rules = NeurobiologyRules::new();
        assert_eq!(rules.metadata().name, "神经生物学规则");
    }
}
