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

    /// 传播理论
    pub fn communication_theories(&self) -> Vec<&'static str> {
        vec![
            "香农信息论: 信息是消除不确定性的东西",
            "施拉姆模型: 传播是编码发送接收解码的循环过程",
            "议程设置理论: 媒体不能决定人们怎么想但能决定想什么",
            "沉默的螺旋: 持少数意见者倾向保持沉默",
            "知沟理论: 社会经济地位高者获取信息更快",
            "使用与满足: 受众主动选择媒体满足自身需求",
        ]
    }

    /// 媒介研究
    pub fn media_studies(&self) -> Vec<&'static str> {
        vec![
            "媒介即讯息: 媒介本身比其传播内容更影响社会",
            "冷热媒介: 热媒介高清晰度冷媒介低清晰度需受众参与",
            "地球村: 电子媒介使世界缩小为一个村庄",
            "媒介素养: 受众获取分析评价和传播信息的能力",
            "数字鸿沟: 不同群体间信息通信技术的差距",
            "信息茧房: 人们只接触自己感兴趣的信息形成封闭环境",
        ]
    }


    /// 数字传播
    pub fn digital_communication(&self) -> Vec<&'static str> {
        vec![
            "社交媒体: 用户生成内容的在线平台",
            "算法推荐: 根据用户行为个性化推送内容",
            "信息过载: 可用信息超过处理能力的现象",
            "后真相时代: 情感诉求比客观事实更能影响舆论",
            "深度伪造: 利用AI生成的虚假音视频",
            "网络舆论: 互联网上公众意见的形成和传播",
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