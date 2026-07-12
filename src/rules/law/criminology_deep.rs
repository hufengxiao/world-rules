//! 犯罪学深度规则
//!
//! 涵盖犯罪学的详细内容，包括：
//! - 犯罪成因理论详解
//! - 犯罪类型分析详解
//! - 犯罪预防策略详解
//! - 犯罪统计分析详解

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: CriminologyDeepRules,
    name: "犯罪学深度规则",
    desc: "犯罪学的详细规则解析",
    origin: "中国",
    tags: ["法律", "刑法", "犯罪学"]
}

impl CriminologyDeepRules {
    /// 犯罪成因理论详解
    pub fn crime_causation_detailed(&self) -> Vec<&'static str> {
        vec![
            "社会结构理论: 犯罪是社会结构不合理导致的结果如贫富差距、社会阶层分化等",
            "社会控制理论: 犯罪是社会控制机制失效的结果如家庭教育缺失、社区监管不力等",
            "社会学习理论: 犯罪是通过社会学习获得的行为如模仿犯罪、犯罪亚文化影响等",
            "心理分析理论: 犯罪是心理问题导致的结果如人格缺陷、心理创伤、情绪失控等",
            "理性选择理论: 犯罪是理性计算的结果如犯罪收益大于犯罪成本时选择犯罪",
            "生物学理论: 犯罪与生物因素有关如遗传、脑部结构异常、激素水平异常等",
            "标签理论: 犯罪是社会标签导致的结果如被标签为犯罪者后自我认同为犯罪者",
            "冲突理论: 犯罪是社会冲突导致的结果如阶级冲突、种族冲突、文化冲突等",
            "机会理论: 犯罪是机会增加导致的结果如犯罪目标、犯罪工具、犯罪场所等",
            "综合理论: 犯罪是多种因素综合作用的结果需要综合分析犯罪成因",
        ]
    }

    /// 犯罪类型分析详解
    pub fn crime_types_detailed(&self) -> Vec<&'static str> {
        vec![
            "暴力犯罪: 以暴力手段实施的犯罪如杀人、伤害、抢劫、强奸等",
            "财产犯罪: 以获取财产为目的的犯罪如盗窃、诈骗、侵占、敲诈勒索等",
            "经济犯罪: 违反经济法律法规的犯罪如贪污、贿赂、金融诈骗、偷税漏税等",
            "毒品犯罪: 与毒品有关的犯罪如走私、贩卖、运输、制造毒品等",
            "性犯罪: 与性行为有关的犯罪如强奸、猥亵、组织卖淫等",
            "网络犯罪: 利用网络实施的犯罪如网络诈骗、网络盗窃、网络攻击等",
            "组织犯罪: 有组织实施的犯罪如黑社会性质组织犯罪、恐怖组织犯罪等",
            "职务犯罪: 利用职务便利实施的犯罪如贪污、渎职、滥用职权等",
            "青少年犯罪: 青少年实施的犯罪具有年龄特点和行为特征",
            "女性犯罪: 女性实施的犯罪具有性别特点和行为特征",
        ]
    }

    /// 犯罪预防策略详解
    pub fn crime_prevention_detailed(&self) -> Vec<&'static str> {
        vec![
            "社会预防: 通过改善社会环境预防犯罪如缩小贫富差距、加强社会保障等",
            "情境预防: 通过改变犯罪情境预防犯罪如加强治安巡逻、安装监控设备等",
            "发展预防: 通过促进个人发展预防犯罪如加强家庭教育、改善学校教育等",
            "社区预防: 通过社区力量预防犯罪如社区巡逻、社区调解、社区帮教等",
            "法律预防: 通过法律手段预防犯罪如完善法律法规、加强执法力度等",
            "技术预防: 通过技术手段预防犯罪如应用人工智能、大数据分析等",
            "心理预防: 通过心理干预预防犯罪如心理咨询、心理治疗、心理教育等",
            "教育预防: 通过教育手段预防犯罪如法制教育、道德教育、职业教育等",
            "就业预防: 通过就业安置预防犯罪如提供就业机会、职业培训等",
            "综合治理预防: 通过综合治理预防犯罪如政府、社会、家庭、学校协同预防",
        ]
    }

    /// 犯罪统计分析详解
    pub fn crime_statistics_detailed(&self) -> Vec<&'static str> {
        vec![
            "犯罪率统计: 统计犯罪案件数量与人口比例反映犯罪发生频率",
            "犯罪类型统计: 统计不同类型犯罪的数量比例分析犯罪结构",
            "犯罪地区统计: 统计不同地区犯罪数量分析犯罪地理分布",
            "犯罪时间统计: 统计不同时间犯罪数量分析犯罪时间规律",
            "犯罪人群统计: 统计犯罪人群特征分析犯罪人群构成",
            "犯罪后果统计: 统计犯罪造成的损失评估犯罪社会影响",
            "犯罪破案率统计: 统计案件破案率评估公安机关破案能力",
            "犯罪起诉率统计: 统计案件起诉率评估检察机关办案质量",
            "犯罪判决率统计: 统计案件判决率评估人民法院审判效率",
            "犯罪重犯率统计: 统计罪犯重犯率评估刑罚执行效果",
        ]
    }

    /// 犯罪心理分析详解
    pub fn criminal_psychology_detailed(&self) -> Vec<&'static str> {
        vec![
            "犯罪动机: 推动犯罪行为的内在原因如利益驱动、情感驱动、仇恨驱动等",
            "犯罪人格: 犯罪者的人格特征如反社会人格、冲动型人格、攻击型人格等",
            "犯罪认知: 犯罪者对犯罪行为的认知如合理化犯罪、否认犯罪危害等",
            "犯罪情绪: 犯罪者的情绪状态如愤怒、恐惧、焦虑、抑郁等",
            "犯罪意志: 犯罪者的意志特征如意志薄弱、意志坚定、意志扭曲等",
            "犯罪习惯: 犯罪者的行为习惯如长期犯罪形成的犯罪习惯",
            "犯罪团伙心理: 犯罪团伙的心理特征如团伙归属感、团伙忠诚等",
            "犯罪者心理改造: 对犯罪者进行心理改造如认知矫正、行为矫正、情感矫正等",
            "犯罪预测心理: 通过心理特征预测犯罪可能性如风险评估、心理测试等",
            "犯罪者心理评估: 对犯罪者进行心理评估如人格测试、心理诊断等",
        ]
    }

    /// 犯罪社会影响详解
    pub fn crime_social_impact_detailed(&self) -> Vec<&'static str> {
        vec![
            "犯罪对被害人的影响: 被害人遭受身心伤害、财产损失、生活质量下降等",
            "犯罪对被害人家庭的影响: 家庭成员遭受心理创伤、经济损失、家庭关系破裂等",
            "犯罪对社会秩序的影响: 破坏社会秩序、影响社会稳定、降低社会安全感等",
            "犯罪对经济发展的影响: 增加社会成本、影响投资环境、阻碍经济发展等",
            "犯罪对公共安全的影响: 危害公共安全、影响公共秩序、降低公共安全感等",
            "犯罪对教育的影响: 影响学校教育、家庭教育、社会教育等",
            "犯罪对文化的影响: 破坏社会文化、影响社会风气、降低社会道德水平等",
            "犯罪对法治的影响: 破坏法治秩序、影响法治建设、降低法治公信力等",
            "犯罪对国家的影响: 危害国家安全、影响国家形象、增加国家治理成本等",
            "犯罪综合治理影响: 需要综合治理、多部门协同、全社会参与应对犯罪",
        ]
    }
}

impl Rule for CriminologyDeepRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("criminology_deep")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "犯罪学深度规则",
            &[
                ("犯罪成因理论详解", &self.crime_causation_detailed()),
                ("犯罪类型分析详解", &self.crime_types_detailed()),
                ("犯罪预防策略详解", &self.crime_prevention_detailed()),
                ("犯罪统计分析详解", &self.crime_statistics_detailed()),
                ("犯罪心理分析详解", &self.criminal_psychology_detailed()),
                ("犯罪社会影响详解", &self.crime_social_impact_detailed()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_criminology_deep_rules() {
        let rules = CriminologyDeepRules::new();
        assert_eq!(rules.metadata().name, "犯罪学深度规则");
        assert!(!rules.crime_causation_detailed().is_empty());
        assert!(!rules.crime_types_detailed().is_empty());
        assert!(!rules.crime_prevention_detailed().is_empty());
        assert!(!rules.crime_statistics_detailed().is_empty());
        assert!(!rules.criminal_psychology_detailed().is_empty());
        assert!(!rules.crime_social_impact_detailed().is_empty());
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_sections_count() {
        let rules = CriminologyDeepRules::new();
        assert_eq!(rules.crime_causation_detailed().len(), 10);
        assert_eq!(rules.crime_types_detailed().len(), 10);
        assert_eq!(rules.crime_prevention_detailed().len(), 10);
        assert_eq!(rules.crime_statistics_detailed().len(), 10);
        assert_eq!(rules.criminal_psychology_detailed().len(), 10);
        assert_eq!(rules.crime_social_impact_detailed().len(), 10);
    }
}