//! 心理学定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 心理学规则
pub struct PsychologyRules {
    metadata: RuleMetadata,
}

impl PsychologyRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("心理学定律", "心理学基本定律和效应")
                .with_origin("心理学")
                .with_tags(vec!["科学".into(), "心理学".into()]),
        }
    }

    /// 常见心理效应
    pub fn psychological_effects(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            ("首因效应", "第一印象对后续认知的影响"),
            ("近因效应", "最新信息对印象的改变"),
            ("光环效应", "一个优点影响整体评价"),
            ("从众效应", "跟随大众行为的倾向"),
            ("破窗效应", "小问题不管会导致大问题"),
        ]
    }

    /// 认知偏差
    pub fn cognitive_biases(&self) -> Vec<&'static str> {
        vec![
            "确认偏差: 只寻找支持自己观点的证据",
            "锚定效应: 过度依赖第一条信息",
            "幸存者偏差: 只看到成功案例",
            "达克效应: 无知者往往自信",
            "损失厌恶: 损失的痛苦大于获得的快乐",
        ]
    }

    /// 学习理论
    pub fn learning_theories(&self) -> Vec<&'static str> {
        vec![
            "经典条件反射: 巴甫洛夫的狗",
            "操作性条件反射: 行为后果影响行为",
            "社会学习理论: 观察模仿学习",
            "认知学习理论: 理解和思考在学习中的作用",
        ]
    }

    /// 记忆规律
    pub fn memory_laws(&self) -> Vec<&'static str> {
        vec![
            "艾宾浩斯遗忘曲线: 遗忘先快后慢",
            "前摄抑制: 之前的学习干扰新学习",
            "倒摄抑制: 新学习干扰旧记忆",
            "系列位置效应: 首尾记得最牢",
        ]
    }

    /// 动机理论
    pub fn motivation_theories(&self) -> Vec<&'static str> {
        vec![
            "马斯洛需求层次: 生理→安全→社交→尊重→自我实现",
            "赫茨伯格双因素: 保健因素和激励因素",
            "期望理论: 努力导致绩效，绩效导致奖励",
        ]
    }

    /// 认知心理学
    pub fn cognitive_psychology(&self) -> Vec<&'static str> {
        vec![
            "工作记忆模型: 包含中央执行系统语音环路和视空间画板",
            "选择性注意: 人类只能有选择地处理部分环境信息",
            "认知负荷理论: 工作记忆容量有限教学设计需考虑负荷",
            "图式理论: 人们用已有知识框架解释和组织新信息",
            "元认知: 对自身认知过程的觉察监控和调控",
            "双加工理论: 认知分为自动化加工和控制加工两个系统",
            "前景理论: 人们对损失比等量收益更加敏感",
        ]
    }

    /// 发展心理学
    pub fn developmental_psychology(&self) -> Vec<&'static str> {
        vec![
            "皮亚杰认知发展: 感知运动→前运算→具体运算→形式运算四阶段",
            "维果茨基最近发展区: 独立解决问题与在指导下解决问题之间的差距",
            "埃里克森心理社会发展: 八个阶段的危机与转机理论",
            "科尔伯格道德发展: 前习俗→习俗→后习俗三水平六阶段",
            "鲍尔比依恋理论: 早期依恋关系影响终生社会情感发展",
            "班杜拉社会学习: 观察学习包括注意保持再现和动机四个过程",
        ]
    }

    /// 社会心理学
    pub fn social_psychology(&self) -> Vec<&'static str> {
        vec![
            "从众效应: 个体受群体影响改变行为或信念",
            "服从权威: 米尔格拉姆实验揭示的权威服从现象",
            "认知失调: 态度与行为不一致时产生心理不适",
            "基本归因错误: 高估个人因素低估情境因素的倾向",
            "旁观者效应: 在场人数越多个人施助可能性越低",
            "刻板印象: 对某群体成员的过度简化认知",
            "晕轮效应: 对某人某方面好感影响对其整体评价",
        ]
    }

    /// 临床心理学
    pub fn clinical_psychology(&self) -> Vec<&'static str> {
        vec![
            "认知行为疗法: 通过改变不良认知模式改善情绪和行为",
            "精神分析: 通过自由联想和梦的解析探索无意识",
            "人本主义疗法: 强调自我实现和个人成长的治疗取向",
            "暴露疗法: 系统接触恐惧刺激以减少焦虑反应",
            "正念减压: 通过冥想训练关注当下减少压力",
            "行为激活: 通过增加积极活动改善抑郁症状",
            "辩证行为疗法: 结合认知行为和正念技能训练",
        ]
    }
}

impl Default for PsychologyRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for PsychologyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("psychology")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        let effects = self.psychological_effects();
        format!(
            "【心理学定律】\n\n\
            常见心理效应:\n{}\n\n\
            认知偏差:\n{}\n\n\
            学习理论:\n{}\n\n\
            记忆规律:\n{}\n",
            effects
                .iter()
                .map(|(n, d)| format!("  • {}: {}", n, d))
                .collect::<Vec<_>>()
                .join("\n"),
            self.cognitive_biases()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.learning_theories()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.memory_laws()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_psychology_rules() {
        let rules = PsychologyRules::new();
        assert!(!rules.psychological_effects().is_empty());
    }
}
