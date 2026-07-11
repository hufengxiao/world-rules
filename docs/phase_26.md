# Phase 26: E9 中华文化礼仪扩充

## 概述

Phase 26 扩展了中华文化礼仪规则，新增 10 种传统礼仪规则，涵盖成年礼、寿礼、禁忌、祭祀、服饰、待人接物、书信、仕途、宗族等方面。

## 新增规则列表 (26-01)

### 1. 中国传统成年礼 (ChineseComingOfAgeRules)

**文件**: `src/rules/social/chinese_coming_of_age.rs`

**内容**:
- 冠礼流程（男子成年礼）
- 笄礼流程（女子成年礼）
- 冠礼三加意义
- 笄礼三加意义
- 成年礼象征意义
- 现代意义

### 2. 中国寿礼礼仪 (ChineseBirthdayEtiquetteRules)

**文件**: `src/rules/social/chinese_birthday_etiquette.rs`

**内容**:
- 寿礼等级（花甲、古稀、耄耋、期颐）
- 寿礼筹备流程
- 祝寿仪式流程
- 寿礼禁忌
- 传统寿礼礼品
- 祝寿词
- 家族责任

### 3. 中国传统禁忌 (ChineseTabooRules)

**文件**: `src/rules/social/chinese_taboo.rs`

**内容**:
- 语言禁忌
- 饮食禁忌
- 行为禁忌
- 礼仪禁忌
- 节日禁忌
- 数字禁忌
- 婚丧禁忌
- 居家禁忌

### 4. 中国传统礼仪基础 (ChineseEtiquetteBasicsRules)

**文件**: `src/rules/social/chinese_etiquette_basics.rs`

**内容**:
- 五常（仁义礼智信）
- 三纲
- 四维（礼义廉耻）
- 八德（孝悌忠信礼义廉耻）
- 五伦
- 六艺
- 基本礼仪规范
- 现代诠释

### 5. 中国祭祀礼仪 (ChineseRitualSacrificeRules)

**文件**: `src/rules/social/chinese_ritual_sacrifice.rs`

**内容**:
- 祭祀对象分类
- 祭品类型（太牢、少牢等）
- 祭祀程序
- 祭祖礼仪
- 清明祭扫
- 祭祀禁忌
- 祭祀祝文格式
- 现代简化祭祀

### 6. 中国传统服饰礼仪 (ChineseTraditionalDressRules)

**文件**: `src/rules/social/chinese_traditional_dress.rs`

**内容**:
- 传统服饰类型（深衣、袍服、襦裙等）
- 服饰等级制度
- 场合着装规范
- 配饰礼仪
- 穿戴规范
- 禁忌规范
- 颜色象征意义
- 现代传承

### 7. 中国待人接物礼仪 (ChineseInterpersonalEtiquetteRules)

**文件**: `src/rules/social/chinese_interpersonal_etiquette.rs`

**内容**:
- 见面礼仪（拱手礼、作揖礼、鞠躬礼）
- 称呼礼仪
- 待客礼仪
- 作客礼仪
- 送礼礼仪
- 饮茶礼仪
- 言谈礼仪
- 交往禁忌

### 8. 中国传统书信礼仪 (ChineseCorrespondenceRules)

**文件**: `src/rules/social/chinese_correspondence.rs`

**内容**:
- 书信格式
- 称呼格式
- 启辞用语
- 祝颂语
- 谦称用语
- 敬称用语
- 书信禁忌
- 现代书信

### 9. 中国传统仕途礼仪 (ChineseOfficialEtiquetteRules)

**文件**: `src/rules/social/chinese_official_etiquette.rs`

**内容**:
- 官职等级
- 官服制度（补子、顶戴）
- 朝见礼仪
- 官员交往
- 升迁礼仪
- 辞官礼仪
- 官场禁忌
- 官员修养

### 10. 中国传统宗族礼仪 (ChineseClanEtiquetteRules)

**文件**: `src/rules/social/chinese_clan_etiquette.rs`

**内容**:
- 宗族组织结构
- 家族辈分制度
- 祭祖礼仪
- 家族集会
- 家规家训
- 犯禁惩罚
- 宗族义务
- 现代意义

## 测试覆盖

**测试文件**: `tests/phase_26_rules.rs`

- 每种规则包含 4-8 个测试用例
- 共 55+ 测试用例
- 测试覆盖所有方法功能

## 规则统计

| 规则类型 | 文件数 | 方法数 | 测试用例数 |
|---------|--------|--------|-----------|
| 传统礼仪 | 10 | 80+ | 55+ |

## 文化价值

这些规则系统化整理了中华传统文化中的礼仪规范，涵盖：

1. **成人礼**: 冠礼笄礼，传承古代成人仪式
2. **寿礼**: 祝寿规范，体现敬老传统
3. **禁忌**: 行为规范，维护社会秩序
4. **礼仪基础**: 五常八德，中华文化核心价值
5. **祭祀**: 祭祖规范，延续慎终追远传统
6. **服饰**: 穿着规范，展现礼仪文化
7. **待人接物**: 交往规范，体现尊重礼仪
8. **书信**: 书信格式，传承文书礼仪
9. **仕途**: 官场规范，展现古代仕途文化
10. **宗族**: 家族规范，体现家族治理

---

*创建时间: 2026-07-11*
*Phase: 26-01*
*状态: 完成*