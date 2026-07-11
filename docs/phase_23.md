# Phase 23: E6 水上运动扩充

## 概述

Phase 23 添加了 **25种水上运动规则**，包括游泳、跳水、冲浪、赛艇、龙舟等多种水上竞技项目。

**完成日期**: 2026-07-11

**规则总数**: 25种新规则

---

## 23-01: 游泳规则 (10种)

### 添加的规则

1. **奥运游泳规则** (`SwimmingOlympicRules`)
   - 奥运会游泳比赛规则
   - 包含所有奥运游泳项目
   - 泳池规格、资格赛制、比赛轮次

2. **世界游泳锦标赛规则** (`SwimmingWorldChampionshipRules`)
   - FINA世界游泳锦标赛规则
   - 包含奖金分配、参赛资格
   - 兴奋剂检测规定

3. **短池游泳规则** (`SwimmingShortCourseRules`)
   - 25米短池游泳规则
   - 与长池游泳的主要区别
   - 短池特有的转身规则

4. **公开水域游泳规则** (`SwimmingOpenWaterRules`)
   - 5km/10km/25km公开水域规则
   - 安全规则和医疗保障
   - 环境条件要求

5. **残奥游泳规则** (`SwimmingParalympicRules`)
   - 残疾人游泳分级系统
   - S1-S15分级标准
   - 适应性游泳规则

6. **成人游泳规则** (`SwimmingMastersRules`)
   - 25岁以上成人游泳
   - 年龄组划分（25-29, 30-34等）
   - 成人游泳比赛规则

7. **自由泳规则** (`SwimmingFreestyleRules`)
   - 自由泳技术规则
   - 爬泳技术要求
   - 出发和转身规则

8. **仰泳规则** (`SwimmingBackstrokeRules`)
   - 仰泳技术规则
   - 水中出发规则
   - 转身和终点触壁

9. **蛙泳规则** (`SwimmingBreaststrokeRules`)
   - 蛙泳技术规则
   - 蛙腿蹬腿要求
   - 双手同时触壁

10. **蝶泳规则** (`SwimmingButterflyRules`)
    - 蝶泳技术规则
    - 海豚腿技术
    - 双臂同时动作

---

## 23-02: 水上运动规则 (10种)

### 添加的规则

1. **艺术游泳详细规则** (`ArtisticSwimmingDetailedRules`)
   - 技术自选和自由自选
   - 评分系统（执行分、艺术分、难度分）
   - 犯规与扣分规则

2. **跳台跳水规则** (`DivingPlatformRules`)
   - 10米跳台跳水
   - 动作难度系数
   - 裁判评分系统

3. **跳板跳水规则** (`DivingSpringboardRules`)
   - 3米跳板跳水
   - 跳板弹性利用
   - 跳板技术要求

4. **高台跳水规则** (`HighDivingRules`)
   - 20米/27米高台跳水
   - 安全规则和医疗保障
   - 高台跳水特有规则

5. **冲浪竞赛规则** (`SurfingCompetitionRules`)
   - 短板和长板冲浪
   - 优先权规则
   - 评分系统（0.1-10分）

6. **帆板规则** (`WindsurfingRules`)
   - 帆板装备要求
   - 操帆技术
   - 帆板竞赛规则

7. **竞技桨板规则** (`PaddleboardRacingRules`)
   - 站立桨板竞赛
   - 冲刺和长距离项目
   - 桩板技术要求

8. **水球世界联赛规则** (`WaterPoloWorldLeagueRules`)
   - 世界联赛结构
   - 比赛时间（四节各8分钟）
   - 队员配置（7人场上）

9. **龙舟世界锦标赛规则** (`DragonBoatWorldChampionshipRules`)
   - 标准龙舟和小龙舟
   - 人员配置（20划手+鼓手+舵手）
   - 比赛距离（200m/500m/1000m）

10. **赛艇世界杯规则** (`RowingWorldCupRules`)
    - 世界杯系列赛结构
    - 船艇类型（单人、双人、八人）
    - 标准距离2000米

---

## 23-03: 潜水规则 (5种)

### 添加的规则

1. **水肺潜水规则** (`ScubaDivingRules`)
   - PADI认证等级（OW, AOW, 救援, 潜水长）
   - 潜伴制度和安全停留
   - 深度限制（18m/30m/40m）
   - 手势信号和环境保护

2. **水下曲棍球规则** (`UnderwaterHockeyRules`)
   - 水下球场规格（21-25米长）
   - 6人制比赛
   - 装备要求（面镜、脚蹼、球杆）
   - 犯规和处罚规则

3. **水下橄榄球规则** (`UnderwaterRugbyRules`)
   - 水下球场规格（12-18米深）
   - 篮筐进球规则
   - 接触规则和犯规
   - 团队战术

4. **竞技屏气潜水规则** (`ApneaDivingRules`)
   - 竞赛项目（静态屏气、恒重下潜、无限制）
   - 安全规则（安全潜水员、医疗）
   - 评分系统（白卡/红卡）
   - 禁止行为（晕厥、LMC）

5. **技术潜水规则** (`TechnicalDivingRules`)
   - 深潜、洞穴、沉船潜水
   - Trimix认证要求
   - 双气瓶和减压程序
   - 气体规划（三分之一法则）

---

## 测试覆盖

### 测试文件

- `tests/phase_23_rules.rs` - Phase 23 集成测试

### 测试用例数量

- **游泳规则**: 30+ 测试用例
- **水上运动规则**: 40+ 测试用例
- **潜水规则**: 50+ 测试用例

**总计**: 120+ 测试用例

### 测试内容

每个规则包含以下测试：
- 基本规则测试（metadata, explain, category）
- 方法返回值验证
- 数据完整性检查
- 关键字段存在性验证

---

## 使用示例

### 游泳规则

```rust
use world_rules::prelude::*;
use world_rules::rules::sports::SwimmingOlympicRules;

let rules = SwimmingOlympicRules::new();

// 查看比赛项目
let events = rules.events();
println!("奥运游泳项目: {}", events.len());

// 泳池规格
let pool = rules.pool_specifications();
assert!(pool.iter().any(|p| p.contains("50米")));

// 资格赛制
let qual = rules.qualification_system();
println!("资格要求: {}", qual.join("\n"));
```

### 艺术游泳

```rust
use world_rules::rules::sports::ArtisticSwimmingDetailedRules;

let rules = ArtisticSwimmingDetailedRules::new();

// 评分系统
let scoring = rules.scoring_system();
assert!(scoring.iter().any(|s| s.contains("执行分")));
assert!(scoring.iter().any(|s| s.contains("难度分")));

// 犯规与扣分
let penalties = rules.penalties();
println!("扣分规则: {}", penalties.len());
```

### 潜水规则

```rust
use world_rules::rules::sports::ScubaDivingRules;

let rules = ScubaDivingRules::new();

// 认证等级
let levels = rules.certification_levels();
assert!(levels.iter().any(|l| l.contains("开放水域")));

// 深度限制
let limits = rules.depth_limits();
assert!(limits.iter().any(|l| l.contains("40米")));

// 环境保护
let env = rules.environmental_rules();
println!("环保规则: {}", env.join("\n"));
```

---

## 规则来源

### 国际组织

- **World Aquatics** (世界泳联) - 游泳、跳水、艺术游泳、水球
- **FINA** - 国际泳联（世界游泳锦标赛）
- **IOC** - 国际奥委会（奥运游泳）
- **IDBF** - 国际龙舟联合会
- **FISA** - 世界赛艇联合会
- **ISA** - 国际冲浪协会
- **PADI** - 专业潜水教练协会
- **AIDA** - 国际自由潜水发展协会

---

## 下一步

Phase 24: E7 冬季运动扩充
- 10种滑雪规则
- 5种滑冰规则
- 5种其他冬季规则

---

**Phase 23 完成 ✅**
- 25种水上运动规则
- 120+ 测试用例
- 完整的文档覆盖