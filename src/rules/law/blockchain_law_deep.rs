//! 区块链法深度规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: BlockchainLawDeepRules,
    name: "区块链法深度规则",
    desc: "区块链技术的法律规则解析",
    origin: "中国",
    tags: ["法律", "区块链"]
}

impl BlockchainLawDeepRules {
    /// 区块链应用监管深度规则
    pub fn blockchain_supervision_detailed(&self) -> Vec<&'static str> {
        vec![
            "备案管理: 区块链信息服务提供者应当在提供服务之日起十个工作日内向国家互联网信息办公室备案备案内容包括服务名称服务形式应用领域服务器地址等",
            "实名认证: 区块链信息服务提供者应当按照《中华人民共和国网络安全法》的规定对使用其服务的用户进行基于移动电话号码等方式的真实身份信息认证",
            "内容审核: 区块链信息服务提供者应当建立健全信息内容审核制度对上链信息进行审核发现违法信息的应当及时采取消除等处置措施防止信息扩散",
            "安全评估: 区块链信息服务提供者开发上线新产品新应用新功能的应当按照有关规定进行安全评估",
            "备案变更: 区块链信息服务提供者变更服务名称服务形式应用领域等备案事项的应当在变更之日起十个工作日内办理变更备案",
            "备案注销: 区块链信息服务提供者终止服务的应当在终止服务前公告并报告国家互联网信息办公室办理备案注销",
            "监督检查: 国家互联网信息办公室对区块链信息服务进行监督检查区块链信息服务提供者应当配合监督检查如实说明情况提供必要的技术支持和协助",
            "违法处理: 区块链信息服务提供者违反规定的由国家互联网信息办公室依据职责采取警告责令限期改正罚款等措施",
            "信用管理: 国家互联网信息办公室建立区块链信息服务提供者信用管理制度将违法违规行为纳入信用记录",
            "行业标准: 鼓励区块链信息服务提供者制定并执行行业标准提高服务质量和安全水平",
        ]
    }

    /// 智能合约法律深度规则
    pub fn smart_contract_legal_detailed(&self) -> Vec<&'static str> {
        vec![
            "合约定义: 智能合约是以计算机代码形式存储在区块链上的自执行合约当预设的条件满足时合约自动执行相应的操作",
            "法律效力: 智能合约符合《中华人民共和国民法典》关于合同成立和生效要件的具有法律效力当事人应当按照约定履行自己的义务",
            "意思表示: 当事人通过智能合约作出意思表示的应当以明示的方式作出智能合约代码应当真实反映当事人的合意",
            "代码解释: 智能合约的代码应当清晰明确避免歧义对代码理解有争议的应当按照通常理解予以解释",
            "执行责任: 智能合约自动执行产生的法律后果由当事人承担当事人应当按照智能合约的约定履行义务",
            "执行异常: 因技术原因导致智能合约执行异常或者错误的当事人应当及时采取措施减少损失并协商解决",
            "合约变更: 经当事人协商一致可以变更智能合约变更应当以智能合约支持的方式进行",
            "合约终止: 当事人协商一致或者出现约定的终止情形时智能合约终止终止应当符合智能合约的约定",
            "争议解决: 因智能合约发生争议的当事人可以协商解决也可以通过调解仲裁诉讼等方式解决",
            "证据效力: 智能合约及其执行记录可以作为证据使用符合电子证据规则的具有证据效力",
        ]
    }

    /// 数字资产确权深度规则
    pub fn digital_asset_rights_detailed(&self) -> Vec<&'static str> {
        vec![
            "资产定义: 数字资产是指以数字化形式存在的具有经济价值的资产包括数字货币虚拟商品虚拟服务数字作品等",
            "权利认定: 数字资产的权利人是指对数字资产享有占有使用收益处分等权利的主体权利人可以依法行使其权利",
            "权利登记: 数字资产权利可以通过区块链等技术进行登记登记应当真实准确完整",
            "权利转移: 数字资产权利转移应当符合法律规定和当事人约定转移自交付时发生效力",
            "权利保护: 数字资产权利受法律保护任何组织和个人不得侵害他人的数字资产权利",
            "权利限制: 数字资产权利的行使不得违反法律规定不得损害国家利益社会公共利益或者他人合法权益",
            "权利证明: 权利人可以通过区块链记录等方式证明其享有数字资产权利区块链记录具有证明效力",
            "权利纠纷: 因数字资产权利发生纠纷的当事人可以通过协商调解仲裁诉讼等方式解决",
            "跨境权利: 涉及跨境的数字资产权利应当符合相关国家和地区的法律规定",
            "权利继承: 数字资产可以作为遗产继承继承人应当符合法律规定的继承条件",
        ]
    }

    /// 区块链数据治理深度规则
    pub fn blockchain_data_governance_detailed(&self) -> Vec<&'static str> {
        vec![
            "数据归属: 区块链上的数据应当明确数据归属主体数据归属主体对数据享有相应的权利",
            "数据质量: 区块链上的数据应当真实准确完整数据提供者应当对数据质量负责",
            "数据安全: 区块链节点运营者和数据使用者应当采取技术措施保障数据安全防止数据泄露篡改破坏",
            "数据共享: 区块链数据共享应当遵循自愿原则数据归属主体同意后方可共享数据",
            "数据使用: 区块链数据使用应当符合数据归属主体的授权范围不得超范围使用数据",
            "数据删除: 区块链数据删除应当符合法律规定和当事人约定删除应当彻底并做好删除记录",
            "数据跨境: 区块链数据跨境传输应当符合数据出境安全管理的有关规定",
            "数据审计: 区块链数据的使用和处理情况应当进行审计审计记录应当保存一定期限",
            "数据备份: 重要的区块链数据应当进行备份备份应当与原始数据具有同等的保护水平",
            "数据标准: 区块链数据应当符合数据标准确保数据的互操作性和可用性",
        ]
    }

    /// 区块链安全责任深度规则
    pub fn blockchain_safety_liability_detailed(&self) -> Vec<&'static str> {
        vec![
            "责任主体: 区块链节点运营者和应用提供者应当对其运营的区块链节点和应用承担安全责任",
            "安全义务: 区块链节点运营者和应用提供者应当建立健全安全管理制度采取必要的技术措施保障区块链系统的安全",
            "风险评估: 区块链节点运营者和应用提供者应当定期进行安全风险评估识别和防范安全风险",
            "漏洞修复: 区块链节点运营者和应用提供者发现系统漏洞应当及时修复防止被利用",
            "应急预案: 区块链节点运营者和应用提供者应当制定安全事件应急预案定期进行应急演练",
            "事件报告: 发生安全事件区块链节点运营者和应用提供者应当立即采取措施防止损失扩大并报告主管部门",
            "损害赔偿: 因区块链系统安全问题造成损害的节点运营者和应用提供者应当依法承担赔偿责任",
            "连带责任: 因多个区块链节点或者应用相互作用造成损害的各责任主体应当承担连带责任",
            "第三方责任: 因第三方原因导致区块链系统安全问题的节点运营者和应用提供者赔偿后有权向第三人追偿",
            "保险制度: 鼓励区块链节点运营者和应用提供者投保区块链安全责任保险分散安全风险",
        ]
    }
}

impl Rule for BlockchainLawDeepRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("blockchain_law_deep")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "区块链法深度规则",
            &[
                ("应用监管", &self.blockchain_supervision_detailed()),
                ("智能合约", &self.smart_contract_legal_detailed()),
                ("数字资产确权", &self.digital_asset_rights_detailed()),
                ("数据治理", &self.blockchain_data_governance_detailed()),
                ("安全责任", &self.blockchain_safety_liability_detailed()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blockchain_law_deep_rules() {
        let rules = BlockchainLawDeepRules::new();
        assert_eq!(rules.metadata().name, "区块链法深度规则");

        // 测试各个方法
        assert!(!rules.blockchain_supervision_detailed().is_empty());
        assert!(!rules.smart_contract_legal_detailed().is_empty());
        assert!(!rules.digital_asset_rights_detailed().is_empty());
        assert!(!rules.blockchain_data_governance_detailed().is_empty());
        assert!(!rules.blockchain_safety_liability_detailed().is_empty());
    }

    #[test]
    fn test_explain() {
        let rules = BlockchainLawDeepRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("应用监管"));
        assert!(explanation.contains("智能合约"));
        assert!(explanation.contains("数字资产确权"));
    }
}
