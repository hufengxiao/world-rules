//! 传播学定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 传播学定律集合
pub struct CommunicationLaws {
    metadata: RuleMetadata,
}

impl CommunicationLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "传播学定律",
                "传播学基本定律"
            )
            .with_origin("社会科学")
            .with_tags(vec!["科学".into(), "传播".into()]),
        }
    }

    /// 传播过程定律
    pub fn process_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("传播者定律", "发送者角色", "传播者角色功能"),
            ("媒介定律", "传播渠道", "媒介传播功能"),
            ("内容定律", "传播信息", "传播内容特征"),
            ("受众定律", "接收者特征", "受众群体特征"),
            ("效果定律", "传播效果", "传播效果规律"),
            ("反馈定律", "反馈机制", "传播反馈机制"),
            ("噪音定律", "干扰因素", "传播干扰因素"),
        ]
    }

    /// 传播模式定律
    pub fn model_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("线性模式定律", "单向传播", "单向传播模式"),
            ("循环模式定律", "双向互动", "双向互动传播"),
            ("螺旋模式定律", "螺旋上升", "螺旋传播过程"),
            ("网状模式定律", "网络传播", "网络传播结构"),
            ("互动模式定律", "交互传播", "交互传播模式"),
            ("扩散模式定律", "信息扩散", "信息扩散规律"),
        ]
    }

    /// 传播效果定律
    pub fn effect_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("强效果定律", "强力影响", "传播强力效果"),
            ("有限效果定律", "有限影响", "传播有限效果"),
            ("适度效果定律", "适度影响", "传播适度效果"),
            ("议程设置定律", "议程影响", "媒介议程设置"),
            ("沉默螺旋定律", "沉默效应", "沉默螺旋效应"),
            ("知沟定律", "知识差距", "知识差距扩大"),
            ("使用满足定律", "需求满足", "受众使用满足"),
        ]
    }

    /// 媒介定律
    pub fn media_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("媒介进化定律", "媒介发展", "媒介进化规律"),
            ("媒介融合定律", "融合趋势", "媒介融合趋势"),
            ("媒介生态定律", "生态系统", "媒介生态系统"),
            ("媒介技术定律", "技术驱动", "媒介技术驱动"),
            ("媒介权力定律", "媒介影响", "媒介权力影响"),
            ("媒介经济定律", "经济规律", "媒介经济规律"),
        ]
    }

    /// 传播类型
    pub fn communication_types(&self) -> Vec<&'static str> {
        vec![
            "人际传播",
            "组织传播",
            "大众传播",
            "网络传播",
            "跨文化传播",
            "政治传播",
            "商业传播",
            "科学传播",
        ]
    }

    /// 传播理论
    pub fn theories(&self) -> Vec<&'static str> {
        vec![
            "经验功能理论",
            "批判理论",
            "符号互动理论",
            "社会学习理论",
            "认知理论",
            "建构理论",
            "效果理论",
            "受众理论",
        ]
    }
}

impl Default for CommunicationLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for CommunicationLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("communication")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【传播学定律】\n\n过程定律:\n{}\n\n效果定律:\n{}\n\n媒介定律:\n{}\n",
            self.process_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.effect_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.media_laws().iter()
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
    fn test_communication_laws() {
        let laws = CommunicationLaws::new();
        assert!(!laws.process_laws().is_empty());
        assert!(!laws.effect_laws().is_empty());
    }
}