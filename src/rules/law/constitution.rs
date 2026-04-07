//! 宪法基础知识

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 宪法规则
pub struct ConstitutionRules {
    metadata: RuleMetadata,
}

impl ConstitutionRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "宪法规则",
                "中国宪法基础知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "宪法".into()]),
        }
    }

    /// 宪法地位
    pub fn constitution_status(&self) -> Vec<&'static str> {
        vec![
            "国家的根本法",
            "具有最高法律效力",
            "其他法律不得与宪法相抵触",
            "是治国安邦的总章程",
            "一切组织和个人的根本活动准则",
        ]
    }

    /// 国家性质
    pub fn state_nature(&self) -> Vec<&'static str> {
        vec![
            "中华人民共和国是社会主义国家",
            "工人阶级领导的工农联盟为基础",
            "人民民主专政",
            "社会主义制度是根本制度",
            "中国共产党领导是中国特色社会主义最本质特征",
        ]
    }

    /// 公民基本权利
    pub fn citizen_rights(&self) -> Vec<&'static str> {
        vec![
            "平等权: 法律面前人人平等",
            "政治权利: 选举权被选举权、言论出版等自由",
            "人身自由: 人身不受侵犯、住宅不受侵犯",
            "社会经济权利: 劳动权、休息权、受教育权",
            "文化权利: 科学研究、文艺创作自由",
            "监督权: 批评建议、申诉控告检举权",
            "获得救济权: 国家赔偿和补偿",
        ]
    }

    /// 公民基本义务
    pub fn citizen_obligations(&self) -> Vec<&'static str> {
        vec![
            "维护国家统一和民族团结",
            "遵守宪法和法律",
            "维护国家安全荣誉和利益",
            "保卫祖国、抵抗侵略",
            "依法纳税",
            "劳动和受教育",
            "计划生育",
        ]
    }

    /// 国家机构
    pub fn state_organs(&self) -> Vec<&'static str> {
        vec![
            "全国人民代表大会: 最高国家权力机关",
            "国务院: 最高国家行政机关",
            "国家监察委员会: 最高监察机关",
            "最高人民法院: 最高审判机关",
            "最高人民检察院: 最高检察机关",
            "中央军事委员会: 最高军事领导机关",
        ]
    }

    /// 选举制度
    pub fn election_system(&self) -> Vec<&'static str> {
        vec![
            "选举权被选举权: 18周岁以上未被剥夺",
            "直接选举: 县乡两级人大代表",
            "间接选举: 市级以上人大代表",
            "一人一票",
            "秘密投票",
            "差额选举",
        ]
    }

    /// 宪法修改
    pub fn amendment_procedure(&self) -> Vec<&'static str> {
        vec![
            "提议: 全国人大常委会或五分之一以上全国人大代表",
            "通过: 全国人大代表三分之二以上多数",
            "宪法修改案由全国人大公布",
        ]
    }
}

impl Default for ConstitutionRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ConstitutionRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("constitution")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【宪法规则】\n\n\
            宪法地位:\n{}\n\n\
            公民基本权利:\n{}\n\n\
            公民基本义务:\n{}\n\n\
            国家机构:\n{}\n",
            self.constitution_status().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.citizen_rights().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.citizen_obligations().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.state_organs().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constitution_rules() {
        let rules = ConstitutionRules::new();
        assert!(!rules.constitution_status().is_empty());
    }
}