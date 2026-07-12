//! 刑法量刑指南深度规则
//!
//! 涵盖刑法量刑的详细内容，包括：
//! - 量刑原则详解
//! - 量刑情节详解
//! - 量刑方法详解
//! - 量刑程序详解

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: SentencingGuidelineDeepRules,
    name: "刑法量刑指南深度规则",
    desc: "刑法量刑指南的详细规则解析",
    origin: "中国",
    tags: ["法律", "刑法", "量刑"]
}

impl SentencingGuidelineDeepRules {
    /// 量刑原则详解
    pub fn sentencing_principles_detailed(&self) -> Vec<&'static str> {
        vec![
            "以犯罪事实为根据原则: 量刑必须以查清的犯罪事实为依据包括犯罪行为、犯罪性质、犯罪情节、犯罪后果等",
            "以刑法为准绳原则: 量刑必须在刑法规定的量刑幅度内进行不得超出法定刑的范围",
            "罪责刑相适应原则: 刑罚的轻重应当与犯罪分子所犯罪行和承担的刑事责任相适应",
            "量刑个别化原则: 根据犯罪分子的具体情况包括主观恶性、人身危险性、社会危害性等进行个别化量刑",
            "量刑公开原则: 量刑过程应当公开人民法院应当在判决书中说明量刑的理由和依据",
            "量刑均衡原则: 同类案件在同一地区应当保持量刑的基本均衡避免量刑畸轻畸重",
            "量刑宽容原则: 对于符合法定条件的犯罪分子应当依法从宽处理体现刑法的宽容性",
            "量刑严格原则: 对于严重犯罪、累犯等应当依法从严处理体现刑法的严厉性",
            "量刑公正原则: 量刑应当公正合理保护被害人权益维护社会公平正义",
            "量刑效果原则: 量刑应当考虑刑罚的社会效果有利于预防犯罪、改造罪犯、维护社会秩序",
        ]
    }

    /// 量刑情节详解
    pub fn sentencing_circumstances_detailed(&self) -> Vec<&'static str> {
        vec![
            "法定从重情节: 累犯、教唆未成年人犯罪、在刑罚执行期间犯罪、利用职务犯罪、实施相同犯罪等",
            "法定从轻情节: 未成年人犯罪、又聋又哑的人或盲人犯罪、防卫过当、避险过当、中止犯等",
            "法定减轻情节: 未成年人犯罪、中止犯没有造成损害的、自首又有重大立功表现、在国外已受过刑罚处罚等",
            "法定免除情节: 犯罪情节轻微不需要判处刑罚的、中止犯没有造成损害的、自首又有重大立功表现等",
            "酌定从重情节: 犯罪动机卑劣、犯罪手段残忍、犯罪后果严重、认罪态度不好、无悔罪表现等",
            "酌定从轻情节: 犯罪动机不卑劣、犯罪手段不残忍、犯罪后果不严重、认罪态度好、有悔罪表现等",
            "犯罪前科: 有犯罪前科的可以酌情从重处罚但前科已过追诉时效的除外",
            "犯罪后的表现: 犯罪后积极抢救被害人、积极赔偿被害人损失、取得被害人谅解等可以从轻处罚",
            "被害人过错: 被害人有过错的可以酌情对被告人从轻处罚",
            "社会影响: 犯罪造成恶劣社会影响的可以从重处罚",
        ]
    }

    /// 量刑方法详解
    pub fn sentencing_methods_detailed(&self) -> Vec<&'static str> {
        vec![
            "量刑基准确定: 根据犯罪的基本事实确定量刑基准点作为量刑的起点",
            "量刑情节适用: 根据法定和酌定量刑情节对基准刑进行调整",
            "量刑幅度计算: 在法定刑幅度内根据量刑情节计算具体刑期",
            "数罪并罚计算: 对一人犯数罪的按照数罪并罚原则计算刑罚",
            "主刑附加刑适用: 主刑只能独立适用附加刑可以独立或附加适用",
            "缓刑适用判断: 根据犯罪情节、悔罪表现、再犯罪风险等因素判断是否适用缓刑",
            "减刑假释计算: 根据罪犯的表现和刑期计算减刑、假释的条件和期限",
            "量刑建议提出: 人民检察院可以提出量刑建议供人民法院参考",
            "量刑辩论进行: 在法庭辩论阶段控辩双方可以对量刑问题进行辩论",
            "量刑裁判作出: 人民法院在综合考虑各方意见后作出量刑裁判",
        ]
    }

    /// 量刑程序详解
    pub fn sentencing_procedure_detailed(&self) -> Vec<&'static str> {
        vec![
            "量刑程序启动: 在刑事案件审理过程中应当启动量刑程序确保量刑的公正性",
            "量刑事实调查: 在法庭调查阶段应当查明与量刑有关的事实和情节",
            "量刑证据审查: 对与量刑有关的证据进行审查核实确保量刑事实清楚",
            "量刑辩论程序: 在法庭辩论阶段控辩双方应当就量刑问题进行辩论",
            "量刑建议提出: 公诉人应当提出量刑建议并说明量刑建议的理由和依据",
            "量刑辩护提出: 辩护人应当提出量刑辩护意见并说明从宽处罚的理由",
            "量刑被害人意见: 被害人及其代理人可以就量刑问题发表意见",
            "量刑被告人陈述: 被告人可以就量刑问题进行最后陈述",
            "量刑评议程序: 合议庭应当对量刑问题进行评议确定具体刑期",
            "量刑裁判说明: 在判决书中应当说明量刑的理由和依据",
        ]
    }

    /// 未成年人量刑详解
    pub fn juvenile_sentencing_detailed(&self) -> Vec<&'static str> {
        vec![
            "未成年人量刑原则: 对未成年人犯罪应当坚持教育为主、惩罚为辅的原则",
            "未成年人量刑从宽: 对未成年人犯罪应当从轻或减轻处罚",
            "未成年人不适用死刑: 犯罪时不满18周岁的人不适用死刑",
            "未成年人缓刑适用: 对符合条件的未成年罪犯应当优先适用缓刑",
            "未成年人记录封存: 对未成年罪犯的犯罪记录应当依法封存",
            "未成年人社区矫正: 对未成年罪犯应当优先适用社区矫正",
            "未成年人量刑考量: 应当考虑未成年人的身心特点、成长经历、家庭环境等因素",
            "未成年人量刑保护: 应当保护未成年人的合法权益避免二次伤害",
            "未成年人量刑教育: 应当通过量刑教育引导未成年人改过自新",
            "未成年人量刑跟踪: 对未成年罪犯应当进行跟踪帮教防止再犯罪",
        ]
    }

    /// 量刑规范化详解
    pub fn sentencing_standardization_detailed(&self) -> Vec<&'static str> {
        vec![
            "量刑指导意见: 最高人民法院制定的量刑指导意见为量刑提供参考",
            "量刑实施细则: 各高级人民法院根据本地实际情况制定量刑实施细则",
            "量刑常见犯罪: 对常见犯罪的量刑应当遵循量刑指导意见的规定",
            "量刑情节量化: 对量刑情节进行量化计算确保量刑的客观性",
            "量刑均衡机制: 建立量刑均衡机制防止量刑畸轻畸重",
            "量刑监督机制: 建立量刑监督机制对量刑过程进行监督",
            "量刑评估机制: 建立量刑评估机制对量刑效果进行评估",
            "量刑调整机制: 根据社会发展和犯罪变化适时调整量刑标准",
            "量刑公开机制: 建立量刑公开机制提高量刑透明度",
            "量刑信息化: 推进量刑信息化建设提高量刑效率和质量",
        ]
    }
}

impl Rule for SentencingGuidelineDeepRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("sentencing_guideline_deep")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "刑法量刑指南深度规则",
            &[
                ("量刑原则详解", &self.sentencing_principles_detailed()),
                ("量刑情节详解", &self.sentencing_circumstances_detailed()),
                ("量刑方法详解", &self.sentencing_methods_detailed()),
                ("量刑程序详解", &self.sentencing_procedure_detailed()),
                ("未成年人量刑详解", &self.juvenile_sentencing_detailed()),
                ("量刑规范化详解", &self.sentencing_standardization_detailed()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sentencing_guideline_deep_rules() {
        let rules = SentencingGuidelineDeepRules::new();
        assert_eq!(rules.metadata().name, "刑法量刑指南深度规则");
        assert!(!rules.sentencing_principles_detailed().is_empty());
        assert!(!rules.sentencing_circumstances_detailed().is_empty());
        assert!(!rules.sentencing_methods_detailed().is_empty());
        assert!(!rules.sentencing_procedure_detailed().is_empty());
        assert!(!rules.juvenile_sentencing_detailed().is_empty());
        assert!(!rules.sentencing_standardization_detailed().is_empty());
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_sections_count() {
        let rules = SentencingGuidelineDeepRules::new();
        assert_eq!(rules.sentencing_principles_detailed().len(), 10);
        assert_eq!(rules.sentencing_circumstances_detailed().len(), 10);
        assert_eq!(rules.sentencing_methods_detailed().len(), 10);
        assert_eq!(rules.sentencing_procedure_detailed().len(), 10);
        assert_eq!(rules.juvenile_sentencing_detailed().len(), 10);
        assert_eq!(rules.sentencing_standardization_detailed().len(), 10);
    }
}