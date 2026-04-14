//! 土地管理法基础规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 土地管理法规则
pub struct LandAdministrationLawRules {
    metadata: RuleMetadata,
}

impl LandAdministrationLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "土地管理法规则",
                "中国土地管理法基础知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "土地管理".into()]),
        }
    }

    /// 土地所有权类型
    pub fn land_ownership(&self) -> Vec<&'static str> {
        vec![
            "国有土地: 国家所有土地",
            "集体土地: 集体所有土地",
            "城市土地国家所有",
            "农村土地集体所有",
            "宅基地集体所有",
            "自留地集体所有",
            "荒山荒地所有权",
            "林地所有权归属",
        ]
    }

    /// 土地使用权
    pub fn land_use_rights(&self) -> Vec<&'static str> {
        vec![
            "建设用地使用权",
            "农用地使用权",
            "宅基地使用权",
            "土地承包经营权",
            "林地使用权",
            "草地使用权",
            "水域使用权",
            "海域使用权",
        ]
    }

    /// 土地用途分类
    pub fn land_use_classification(&self) -> Vec<&'static str> {
        vec![
            "农用地: 农业生产用地",
            "耕地: 种植农作物",
            "林地: 林业生产用地",
            "草地: 畜牧业用地",
            "建设用地: 建筑用地",
            "住宅用地: 住宅建设",
            "商业用地: 商业开发",
            "工业用地: 工业建设",
            "未利用地: 待开发土地",
        ]
    }

    /// 土地征收规则
    pub fn land_expropriation(&self) -> Vec<&'static str> {
        vec![
            "征收条件: 公共利益需要",
            "征收审批: 征收审批程序",
            "征收公告: 征收公告发布",
            "征收补偿: 征收补偿标准",
            "安置补助: 安置补助费用",
            "土地补偿: 土地补偿费用",
            "附着物补偿: 附着物赔偿",
            "社会保障: 社保安置措施",
        ]
    }

    /// 土地出让规则
    pub fn land_transfer(&self) -> Vec<&'static str> {
        vec![
            "出让方式: 土地出让方式",
            "招标出让: 招标竞争出让",
            "拍卖出让: 拍卖竞价出让",
            "挂牌出让: 挂牌出让方式",
            "协议出让: 协议出让方式",
            "出让年限: 土地出让年限",
            "出让价格: 土地出让金",
            "出让合同: 出让合同签订",
        ]
    }

    /// 土地登记规则
    pub fn land_registration(&self) -> Vec<&'static str> {
        vec![
            "初始登记: 土地初始登记",
            "变更登记: 土地变更登记",
            "转移登记: 土地转移登记",
            "注销登记: 土地注销登记",
            "登记申请: 登记申请程序",
            "登记审查: 登记审查程序",
            "登记公告: 登记公告程序",
            "登记证书: 登记证书发放",
        ]
    }

    /// 土地保护规则
    pub fn land_protection(&self) -> Vec<&'static str> {
        vec![
            "耕地保护: 耕地保护制度",
            "基本农田保护",
            "耕地红线保护",
            "耕地占用补偿",
            "土地复垦: 土地恢复利用",
            "土地整治: 土地综合治理",
            "用途管制: 土地用途管制",
            "违法查处: 违法用地查处",
        ]
    }

    /// 土地违法行为
    pub fn land_violations(&self) -> Vec<&'static str> {
        vec![
            "非法占用土地",
            "非法转让土地",
            "非法批地: 非法批准用地",
            "破坏耕地行为",
            "闲置土地: 土地闲置行为",
            "改变用途违法",
            "超标用地违法",
            "未批先建违法",
        ]
    }
}

impl Default for LandAdministrationLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for LandAdministrationLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("land_administration")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【土地管理法规则】\n\n土地所有权:\n{}\n\n土地使用权:\n{}\n\n用途分类:\n{}\n",
            self.land_ownership().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.land_use_rights().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.land_use_classification().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_land_administration_law_rules() {
        let rules = LandAdministrationLawRules::new();
        assert!(!rules.land_ownership().is_empty());
        assert!(!rules.land_use_rights().is_empty());
    }
}