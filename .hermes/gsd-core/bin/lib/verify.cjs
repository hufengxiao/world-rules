"use strict";
/**
 * Verify — Verification suite, consistency, and health validation
 *
 * ADR-457 build-at-publish: the hand-written bin/lib/verify.cjs collapsed to
 * a TypeScript source of truth, compiled by tsc to a gitignored .cjs at the
 * same require() path. Behaviour preserved byte-for-behaviour; only types are added.
 */
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
const node_fs_1 = __importDefault(require("node:fs"));
const node_path_1 = __importDefault(require("node:path"));
const node_os_1 = __importDefault(require("node:os"));
const validate_cjs_1 = require("./validate.cjs");
const validate_cjs_2 = require("./validate.cjs");
// eslint-disable-next-line @typescript-eslint/no-require-imports -- core.cjs is an export= CommonJS module
const core = require("./core.cjs");
// eslint-disable-next-line @typescript-eslint/no-require-imports -- planning-workspace.cjs is an export= CommonJS module
const planningWorkspace = require("./planning-workspace.cjs");
// eslint-disable-next-line @typescript-eslint/no-require-imports -- frontmatter.cjs is an export= CommonJS module
const frontmatterMod = require("./frontmatter.cjs");
// eslint-disable-next-line @typescript-eslint/no-require-imports -- state.cjs is an export= CommonJS module
const stateMod = require("./state.cjs");
// eslint-disable-next-line @typescript-eslint/no-require-imports -- model-profiles.cjs is an export= CommonJS module
const modelProfilesMod = require("./model-profiles.cjs");
const shell_command_projection_cjs_1 = require("./shell-command-projection.cjs");
const package_identity_cjs_1 = require("./package-identity.cjs");
const runtime_slash_cjs_1 = require("./runtime-slash.cjs");
const schema_detect_cjs_1 = require("./schema-detect.cjs");
const artifacts_cjs_1 = require("./artifacts.cjs");
const { loadConfig, normalizePhaseName, phaseTokenMatches, escapeRegex, findPhaseInternal, getMilestoneInfo, stripShippedMilestones, extractCurrentMilestone, output, error, checkAgentsInstalled, CONFIG_DEFAULTS, inspectWorktreeHealth, } = core;
const { planningDir } = planningWorkspace;
const { extractFrontmatter, parseMustHavesBlock } = frontmatterMod;
const { writeStateMd } = stateMod;
const { MODEL_PROFILES } = modelProfilesMod;
// Unused but imported for structural parity
void stripShippedMilestones;
void schema_detect_cjs_1.detectSchemaFiles;
function cmdVerifySummary(cwd, summaryPath, checkFileCount, raw) {
    if (!summaryPath) {
        error('summary-path required');
    }
    const fullPath = node_path_1.default.join(cwd, summaryPath);
    const checkCount = checkFileCount || 2;
    if (!node_fs_1.default.existsSync(fullPath)) {
        const result = {
            passed: false,
            checks: {
                summary_exists: false,
                files_created: { checked: 0, found: 0, missing: [] },
                commits_exist: false,
                self_check: 'not_found',
            },
            errors: ['SUMMARY.md not found'],
        };
        output(result, raw, 'failed');
        return;
    }
    const content = node_fs_1.default.readFileSync(fullPath, 'utf-8');
    const errors = [];
    const mentionedFiles = new Set();
    const patterns = [
        /`([^`]+\.[a-zA-Z]+)`/g,
        /(?:Created|Modified|Added|Updated|Edited):\s*`?([^\s`]+\.[a-zA-Z]+)`?/gi,
    ];
    for (const pattern of patterns) {
        let m;
        while ((m = pattern.exec(content)) !== null) {
            const filePath = m[1];
            if (filePath && !filePath.startsWith('http') && filePath.includes('/')) {
                mentionedFiles.add(filePath);
            }
        }
    }
    const filesToCheck = Array.from(mentionedFiles).slice(0, checkCount);
    const missing = [];
    for (const file of filesToCheck) {
        if (!node_fs_1.default.existsSync(node_path_1.default.join(cwd, file))) {
            missing.push(file);
        }
    }
    const commitHashPattern = /\b[0-9a-f]{7,40}\b/g;
    const hashes = content.match(commitHashPattern) || [];
    let commitsExist = false;
    if (hashes.length > 0) {
        for (const hash of hashes.slice(0, 3)) {
            const result = (0, shell_command_projection_cjs_1.execGit)(['cat-file', '-t', hash], { cwd });
            if (result.exitCode === 0 && result.stdout.trim() === 'commit') {
                commitsExist = true;
                break;
            }
        }
    }
    let selfCheck = 'not_found';
    const selfCheckPattern = /##\s*(?:Self[- ]?Check|Verification|Quality Check)/i;
    if (selfCheckPattern.test(content)) {
        const passPattern = /(?:all\s+)?(?:pass|✓|✅|complete|succeeded)/i;
        const failPattern = /(?:fail|✗|❌|incomplete|blocked)/i;
        const checkSection = content.slice(content.search(selfCheckPattern));
        if (failPattern.test(checkSection)) {
            selfCheck = 'failed';
        }
        else if (passPattern.test(checkSection)) {
            selfCheck = 'passed';
        }
    }
    if (missing.length > 0)
        errors.push('Missing files: ' + missing.join(', '));
    if (!commitsExist && hashes.length > 0)
        errors.push('Referenced commit hashes not found in git history');
    if (selfCheck === 'failed')
        errors.push('Self-check section indicates failure');
    const checks = {
        summary_exists: true,
        files_created: { checked: filesToCheck.length, found: filesToCheck.length - missing.length, missing },
        commits_exist: commitsExist,
        self_check: selfCheck,
    };
    const passed = missing.length === 0 && selfCheck !== 'failed';
    const result = { passed, checks, errors };
    output(result, raw, passed ? 'passed' : 'failed');
}
function cmdVerifyPlanStructure(cwd, filePath, raw) {
    if (!filePath) {
        error('file path required');
    }
    const fullPath = node_path_1.default.isAbsolute(filePath) ? filePath : node_path_1.default.join(cwd, filePath);
    const content = (0, shell_command_projection_cjs_1.platformReadSync)(fullPath);
    if (!content) {
        output({ error: 'File not found', path: filePath }, raw);
        return;
    }
    const fm = extractFrontmatter(content);
    const errors = [];
    const warnings = [];
    const required = ['phase', 'plan', 'type', 'wave', 'depends_on', 'files_modified', 'autonomous', 'must_haves'];
    for (const field of required) {
        if (fm[field] === undefined)
            errors.push(`Missing required frontmatter field: ${field}`);
    }
    const taskPattern = /<task[^>]*>([\s\S]*?)<\/task>/g;
    const tasks = [];
    let taskMatch;
    while ((taskMatch = taskPattern.exec(content)) !== null) {
        const taskContent = taskMatch[1];
        const nameMatch = taskContent.match(/<name>([\s\S]*?)<\/name>/);
        const taskName = nameMatch ? nameMatch[1].trim() : 'unnamed';
        const hasFiles = /<files>/.test(taskContent);
        const hasAction = /<action>/.test(taskContent);
        const hasVerify = /<verify>/.test(taskContent);
        const hasDone = /<done>/.test(taskContent);
        if (!nameMatch)
            errors.push('Task missing <name> element');
        if (!hasAction)
            errors.push(`Task '${taskName}' missing <action>`);
        if (!hasVerify)
            warnings.push(`Task '${taskName}' missing <verify>`);
        if (!hasDone)
            warnings.push(`Task '${taskName}' missing <done>`);
        if (!hasFiles)
            warnings.push(`Task '${taskName}' missing <files>`);
        tasks.push({ name: taskName, hasFiles, hasAction, hasVerify, hasDone });
    }
    if (tasks.length === 0)
        warnings.push('No <task> elements found');
    if (fm['wave'] &&
        parseInt(fm['wave']) > 1 &&
        (!fm['depends_on'] ||
            (Array.isArray(fm['depends_on']) && fm['depends_on'].length === 0))) {
        warnings.push('Wave > 1 but depends_on is empty');
    }
    const hasCheckpoints = /<task\s+type=["']?checkpoint/.test(content);
    // eslint-disable-next-line @typescript-eslint/no-base-to-string -- FrontmatterValue comparison
    if (hasCheckpoints && fm['autonomous'] !== 'false' && String(fm['autonomous']) !== 'false') {
        errors.push('Has checkpoint tasks but autonomous is not false');
    }
    output({
        valid: errors.length === 0,
        errors,
        warnings,
        task_count: tasks.length,
        tasks,
        frontmatter_fields: Object.keys(fm),
    }, raw, errors.length === 0 ? 'valid' : 'invalid');
}
function cmdVerifyPhaseCompleteness(cwd, phase, raw) {
    if (!phase) {
        error('phase required');
    }
    const phaseInfoRaw = findPhaseInternal(cwd, phase);
    if (!phaseInfoRaw || !phaseInfoRaw['found']) {
        output({ error: 'Phase not found', phase }, raw);
        return;
    }
    const phaseInfo = phaseInfoRaw;
    const errors = [];
    const warnings = [];
    const phaseDir = node_path_1.default.join(cwd, phaseInfo['directory']);
    let files;
    try {
        files = node_fs_1.default.readdirSync(phaseDir);
    }
    catch {
        output({ error: 'Cannot read phase directory' }, raw);
        return;
    }
    const plans = files.filter((f) => f.match(/-PLAN\.md$/i));
    const summaries = files.filter((f) => f.match(/-SUMMARY\.md$/i));
    const planIds = new Set(plans.map((p) => p.replace(/-PLAN\.md$/i, '')));
    const summaryIds = new Set(summaries.map((s) => s.replace(/-SUMMARY\.md$/i, '')));
    const incompletePlans = [...planIds].filter((id) => !summaryIds.has(id));
    if (incompletePlans.length > 0) {
        errors.push(`Plans without summaries: ${incompletePlans.join(', ')}`);
    }
    const orphanSummaries = [...summaryIds].filter((id) => !planIds.has(id));
    if (orphanSummaries.length > 0) {
        warnings.push(`Summaries without plans: ${orphanSummaries.join(', ')}`);
    }
    output({
        complete: errors.length === 0,
        phase: phaseInfo['phase_number'],
        plan_count: plans.length,
        summary_count: summaries.length,
        incomplete_plans: incompletePlans,
        orphan_summaries: orphanSummaries,
        errors,
        warnings,
    }, raw, errors.length === 0 ? 'complete' : 'incomplete');
}
function cmdVerifyReferences(cwd, filePath, raw) {
    if (!filePath) {
        error('file path required');
    }
    const fullPath = node_path_1.default.isAbsolute(filePath) ? filePath : node_path_1.default.join(cwd, filePath);
    const content = (0, shell_command_projection_cjs_1.platformReadSync)(fullPath);
    if (!content) {
        output({ error: 'File not found', path: filePath }, raw);
        return;
    }
    const found = [];
    const missing = [];
    const atRefs = content.match(/@([^\s\n,)]+\/[^\s\n,)]+)/g) || [];
    for (const ref of atRefs) {
        const cleanRef = ref.slice(1);
        const resolved = cleanRef.startsWith('~/')
            ? node_path_1.default.join(process.env['HOME'] || '', cleanRef.slice(2))
            : node_path_1.default.join(cwd, cleanRef);
        if (node_fs_1.default.existsSync(resolved)) {
            found.push(cleanRef);
        }
        else {
            missing.push(cleanRef);
        }
    }
    const backtickRefs = content.match(/`([^`]+\/[^`]+\.[a-zA-Z]{1,10})`/g) || [];
    for (const ref of backtickRefs) {
        const cleanRef = ref.slice(1, -1);
        if (cleanRef.startsWith('http') || cleanRef.includes('${') || cleanRef.includes('{{'))
            continue;
        if (found.includes(cleanRef) || missing.includes(cleanRef))
            continue;
        const resolved = node_path_1.default.join(cwd, cleanRef);
        if (node_fs_1.default.existsSync(resolved)) {
            found.push(cleanRef);
        }
        else {
            missing.push(cleanRef);
        }
    }
    output({
        valid: missing.length === 0,
        found: found.length,
        missing,
        total: found.length + missing.length,
    }, raw, missing.length === 0 ? 'valid' : 'invalid');
}
function cmdVerifyCommits(cwd, hashes, raw) {
    if (!hashes || hashes.length === 0) {
        error('At least one commit hash required');
    }
    const valid = [];
    const invalid = [];
    for (const hash of hashes) {
        const result = (0, shell_command_projection_cjs_1.execGit)(['cat-file', '-t', hash], { cwd });
        if (result.exitCode === 0 && result.stdout.trim() === 'commit') {
            valid.push(hash);
        }
        else {
            invalid.push(hash);
        }
    }
    output({
        all_valid: invalid.length === 0,
        valid,
        invalid,
        total: hashes.length,
    }, raw, invalid.length === 0 ? 'valid' : 'invalid');
}
function cmdVerifyArtifacts(cwd, planFilePath, raw) {
    if (!planFilePath) {
        error('plan file path required');
    }
    const fullPath = node_path_1.default.isAbsolute(planFilePath) ? planFilePath : node_path_1.default.join(cwd, planFilePath);
    const content = (0, shell_command_projection_cjs_1.platformReadSync)(fullPath);
    if (!content) {
        output({ error: 'File not found', path: planFilePath }, raw);
        return;
    }
    const artifacts = parseMustHavesBlock(content, 'artifacts');
    if (artifacts.length === 0) {
        output({ error: 'No must_haves.artifacts found in frontmatter', path: planFilePath }, raw);
        return;
    }
    const results = [];
    for (const artifact of artifacts) {
        if (typeof artifact === 'string')
            continue;
        const artPath = artifact['path'];
        if (!artPath)
            continue;
        const artFullPath = node_path_1.default.join(cwd, artPath);
        const exists = node_fs_1.default.existsSync(artFullPath);
        const check = { path: artPath, exists, issues: [], passed: false };
        if (exists) {
            const fileContent = (0, shell_command_projection_cjs_1.platformReadSync)(artFullPath) || '';
            const lineCount = fileContent.split('\n').length;
            if (artifact['min_lines'] && lineCount < artifact['min_lines']) {
                check['issues'].push(`Only ${lineCount} lines, need ${artifact['min_lines']}`);
            }
            if (artifact['contains'] && !fileContent.includes(artifact['contains'])) {
                check['issues'].push(`Missing pattern: ${artifact['contains']}`);
            }
            if (artifact['exports']) {
                const exports = Array.isArray(artifact['exports'])
                    ? artifact['exports']
                    : [artifact['exports']];
                for (const exp of exports) {
                    if (!fileContent.includes(exp))
                        check['issues'].push(`Missing export: ${exp}`);
                }
            }
            check['passed'] = check['issues'].length === 0;
        }
        else {
            check['issues'].push('File not found');
        }
        results.push(check);
    }
    const passed = results.filter((r) => r['passed']).length;
    output({
        all_passed: passed === results.length,
        passed,
        total: results.length,
        artifacts: results,
    }, raw, passed === results.length ? 'valid' : 'invalid');
}
function cmdVerifyKeyLinks(cwd, planFilePath, raw) {
    if (!planFilePath) {
        error('plan file path required');
    }
    const fullPath = node_path_1.default.isAbsolute(planFilePath) ? planFilePath : node_path_1.default.join(cwd, planFilePath);
    const content = (0, shell_command_projection_cjs_1.platformReadSync)(fullPath);
    if (!content) {
        output({ error: 'File not found', path: planFilePath }, raw);
        return;
    }
    const keyLinks = parseMustHavesBlock(content, 'key_links');
    if (keyLinks.length === 0) {
        output({ error: 'No must_haves.key_links found in frontmatter', path: planFilePath }, raw);
        return;
    }
    const results = [];
    for (const link of keyLinks) {
        if (typeof link === 'string')
            continue;
        const check = {
            from: link['from'],
            to: link['to'],
            via: link['via'] || '',
            verified: false,
            detail: '',
        };
        const sourceContent = (0, shell_command_projection_cjs_1.platformReadSync)(node_path_1.default.join(cwd, link['from'] || ''));
        if (!sourceContent) {
            check['detail'] = 'Source file not found';
        }
        else if (link['pattern']) {
            try {
                const regex = new RegExp(link['pattern']);
                if (regex.test(sourceContent)) {
                    check['verified'] = true;
                    check['detail'] = 'Pattern found in source';
                }
                else {
                    const targetContent = (0, shell_command_projection_cjs_1.platformReadSync)(node_path_1.default.join(cwd, link['to'] || ''));
                    if (targetContent && regex.test(targetContent)) {
                        check['verified'] = true;
                        check['detail'] = 'Pattern found in target';
                    }
                    else {
                        check['detail'] = `Pattern "${link['pattern']}" not found in source or target`;
                    }
                }
            }
            catch {
                check['detail'] = `Invalid regex pattern: ${link['pattern']}`;
            }
        }
        else {
            if (sourceContent.includes(link['to'] || '')) {
                check['verified'] = true;
                check['detail'] = 'Target referenced in source';
            }
            else {
                check['detail'] = 'Target not referenced in source';
            }
        }
        results.push(check);
    }
    const verified = results.filter((r) => r['verified']).length;
    output({
        all_verified: verified === results.length,
        verified,
        total: results.length,
        links: results,
    }, raw, verified === results.length ? 'valid' : 'invalid');
}
function listMilestoneArchiveDirs(planBase) {
    const milestonesDir = node_path_1.default.join(planBase, 'milestones');
    try {
        return node_fs_1.default
            .readdirSync(milestonesDir, { withFileTypes: true })
            .filter((e) => e.isDirectory() && validate_cjs_2.MILESTONE_ARCHIVE_DIR_RE.test(e.name))
            .map((e) => node_path_1.default.join(milestonesDir, e.name))
            .sort((a, b) => node_path_1.default.basename(a).localeCompare(node_path_1.default.basename(b), undefined, { numeric: true }));
    }
    catch {
        return [];
    }
}
function forEachArchivedPhaseToken(planBase, onPhase) {
    for (const archiveDir of listMilestoneArchiveDirs(planBase)) {
        try {
            const entries = node_fs_1.default.readdirSync(archiveDir, { withFileTypes: true });
            for (const e of entries) {
                if (!e.isDirectory())
                    continue;
                const m = e.name.match(validate_cjs_2.PHASE_TOKEN_FROM_DIR_RE);
                if (m)
                    onPhase(m[1]);
            }
        }
        catch {
            /* archive dir absent/unreadable */
        }
    }
}
function getActiveMilestoneArchiveDir(planBase) {
    const archiveDirs = listMilestoneArchiveDirs(planBase);
    if (archiveDirs.length === 0)
        return null;
    try {
        const statePath = node_path_1.default.join(planBase, 'STATE.md');
        if (node_fs_1.default.existsSync(statePath)) {
            const state = node_fs_1.default.readFileSync(statePath, 'utf-8');
            const m = state.match(/^\s*(?:\*\*)?milestone(?:\*\*)?:\s*\*{0,2}\s*([^\s*\r\n#][^\s\r\n#]*)/mi);
            if (m && m[1]) {
                const milestone = m[1].trim();
                const candidate = node_path_1.default.join(planBase, 'milestones', `${milestone}-phases`);
                return archiveDirs.includes(candidate) ? candidate : null;
            }
        }
    }
    catch {
        /* intentionally empty — fall through to version-sort below */
    }
    return archiveDirs[archiveDirs.length - 1];
}
function collectPhaseRoots(planBase) {
    const roots = [];
    const flatPhasesDir = node_path_1.default.join(planBase, 'phases');
    if (node_fs_1.default.existsSync(flatPhasesDir))
        roots.push(flatPhasesDir);
    const activeArchive = getActiveMilestoneArchiveDir(planBase);
    if (activeArchive)
        roots.push(activeArchive);
    return roots;
}
function collectDiskPhases(planBase) {
    const diskPhases = new Set();
    const phaseRoots = collectPhaseRoots(planBase);
    const scanDir = (dir) => {
        try {
            const entries = node_fs_1.default.readdirSync(dir, { withFileTypes: true });
            for (const e of entries) {
                if (e.isDirectory()) {
                    const m = e.name.match(validate_cjs_2.PHASE_TOKEN_FROM_DIR_RE);
                    if (m)
                        diskPhases.add(m[1]);
                }
            }
        }
        catch {
            /* dir absent */
        }
    };
    for (const root of phaseRoots)
        scanDir(root);
    return diskPhases;
}
function checkMilestonePrefixMismatches(roadmapContent, { getMilestoneFromPhaseId }) {
    const mismatches = [];
    const sections = [];
    const sectionRx = /^#{1,3}\s+(?:\[[^\]]+\]\s*)?.*v(\d+\.\d+)/gim;
    let m;
    while ((m = sectionRx.exec(roadmapContent)) !== null) {
        if (sections.length > 0)
            sections[sections.length - 1].end = m.index;
        sections.push({ version: `v${m[1]}`, start: m.index, end: roadmapContent.length });
    }
    for (const section of sections) {
        const content = roadmapContent.slice(section.start, section.end);
        const phaseRx = /#{2,4}\s*(?:\[[^\]]+\]\s*)?Phase\s+([\w][\w.-]*)\s*:/gi;
        let pm;
        while ((pm = phaseRx.exec(content)) !== null) {
            const phaseId = pm[1];
            const expectedMilestone = getMilestoneFromPhaseId(phaseId);
            if (expectedMilestone !== null && expectedMilestone !== section.version) {
                mismatches.push({
                    phaseId,
                    foundInMilestone: section.version,
                    expectedMilestone,
                });
            }
        }
    }
    return mismatches;
}
function cmdValidateConsistency(cwd, raw) {
    const planBase = planningDir(cwd);
    const roadmapPath = node_path_1.default.join(planBase, 'ROADMAP.md');
    const errors = [];
    const warnings = [];
    if (!node_fs_1.default.existsSync(roadmapPath)) {
        errors.push('ROADMAP.md not found');
        output({ passed: false, errors, warnings }, raw, 'failed');
        return;
    }
    const roadmapContentRaw = node_fs_1.default.readFileSync(roadmapPath, 'utf-8');
    const roadmapContent = extractCurrentMilestone(roadmapContentRaw, cwd);
    const { roadmapPhases } = (0, validate_cjs_1.buildRoadmapPhaseVariants)(roadmapContent);
    const { roadmapPhaseVariants: fullRoadmapPhaseVariants } = (0, validate_cjs_1.buildRoadmapPhaseVariants)(roadmapContentRaw);
    const diskPhases = collectDiskPhases(planBase);
    for (const p of roadmapPhases) {
        if (!diskPhases.has(p) && !diskPhases.has(normalizePhaseName(p))) {
            warnings.push(`Phase ${p} in ROADMAP.md but no directory on disk`);
        }
    }
    for (const p of diskPhases) {
        const variants = (0, validate_cjs_1.phaseVariants)(p);
        if (![...variants].some((v) => fullRoadmapPhaseVariants.has(v))) {
            warnings.push(`Phase ${p} exists on disk but not in ROADMAP.md`);
        }
    }
    const config = loadConfig(cwd);
    if (config.phase_naming !== 'custom') {
        const integerPhases = [...diskPhases]
            .filter((p) => !p.includes('.'))
            .map((p) => parseInt(p, 10))
            .sort((a, b) => a - b);
        for (let i = 1; i < integerPhases.length; i++) {
            if (integerPhases[i] !== integerPhases[i - 1] + 1) {
                warnings.push(`Gap in phase numbering: ${integerPhases[i - 1]} → ${integerPhases[i]}`);
            }
        }
    }
    const phaseRoots = collectPhaseRoots(planBase);
    for (const phaseRoot of phaseRoots) {
        try {
            const entries = node_fs_1.default.readdirSync(phaseRoot, { withFileTypes: true });
            const dirs = entries
                .filter((e) => e.isDirectory())
                .map((e) => e.name)
                .sort();
            for (const dir of dirs) {
                const phasePath = node_path_1.default.join(phaseRoot, dir);
                const phaseLabel = node_path_1.default.relative(planBase, phasePath).replace(/\\/g, '/');
                const phaseFiles = node_fs_1.default.readdirSync(phasePath);
                const plans = phaseFiles.filter((f) => f.endsWith('-PLAN.md')).sort();
                const planNums = plans
                    .map((p) => {
                    const pm = p.match(/-(\d{2})-PLAN\.md$/);
                    return pm ? parseInt(pm[1], 10) : null;
                })
                    .filter((n) => n !== null);
                for (let i = 1; i < planNums.length; i++) {
                    if (planNums[i] !== planNums[i - 1] + 1) {
                        warnings.push(`Gap in plan numbering in ${phaseLabel}: plan ${planNums[i - 1]} → ${planNums[i]}`);
                    }
                }
                const summaries = phaseFiles.filter((f) => f.endsWith('-SUMMARY.md'));
                const planIds = new Set(plans.map((p) => p.replace('-PLAN.md', '')));
                const summaryIds = new Set(summaries.map((s) => s.replace('-SUMMARY.md', '')));
                for (const sid of summaryIds) {
                    if (!planIds.has(sid)) {
                        warnings.push(`Summary ${sid}-SUMMARY.md in ${phaseLabel} has no matching PLAN.md`);
                    }
                }
                for (const plan of plans) {
                    const content = node_fs_1.default.readFileSync(node_path_1.default.join(phasePath, plan), 'utf-8');
                    const fmData = extractFrontmatter(content);
                    if (!fmData['wave']) {
                        warnings.push(`${phaseLabel}/${plan}: missing 'wave' in frontmatter`);
                    }
                }
            }
        }
        catch {
            /* intentionally empty */
        }
    }
    const passed = errors.length === 0;
    output({ passed, errors, warnings, warning_count: warnings.length }, raw, passed ? 'passed' : 'failed');
}
function cmdValidateHealth(cwd, options, raw) {
    const resolved = node_path_1.default.resolve(cwd);
    if (resolved === node_os_1.default.homedir()) {
        output({
            status: 'error',
            errors: [
                {
                    code: 'E010',
                    message: `CWD is home directory (${resolved}) — health check would read the wrong .planning/ directory. Run from your project root instead.`,
                    fix: 'cd into your project directory and retry',
                },
            ],
            warnings: [],
            info: [{ code: 'I010', message: `Resolved CWD: ${resolved}` }],
            repairable_count: 0,
        }, raw);
        return;
    }
    const planBase = planningDir(cwd);
    const projectPath = node_path_1.default.join(planBase, 'PROJECT.md');
    const roadmapPath = node_path_1.default.join(planBase, 'ROADMAP.md');
    const statePath = node_path_1.default.join(planBase, 'STATE.md');
    const configPath = node_path_1.default.join(planBase, 'config.json');
    const phasesDir = node_path_1.default.join(planBase, 'phases');
    const _slashRuntime = (0, runtime_slash_cjs_1.resolveRuntime)(cwd);
    const slash = (name) => (0, runtime_slash_cjs_1.formatGsdSlash)(name, _slashRuntime);
    const errors = [];
    const warnings = [];
    const info = [];
    const repairs = [];
    const addIssue = (severity, code, message, fix, repairable = false) => {
        const issue = { code, message, fix, repairable };
        if (severity === 'error')
            errors.push(issue);
        else if (severity === 'warning')
            warnings.push(issue);
        else
            info.push(issue);
    };
    if (!node_fs_1.default.existsSync(planBase)) {
        addIssue('error', 'E001', '.planning/ directory not found', `Run ${slash('new-project')} to initialize`);
        output({ status: 'broken', errors, warnings, info, repairable_count: 0 }, raw);
        return;
    }
    if (!node_fs_1.default.existsSync(projectPath)) {
        addIssue('error', 'E002', 'PROJECT.md not found', `Run ${slash('new-project')} to create`);
    }
    else {
        const content = node_fs_1.default.readFileSync(projectPath, 'utf-8');
        const requiredSections = ['## What This Is', '## Core Value', '## Requirements'];
        for (const section of requiredSections) {
            if (!content.includes(section)) {
                addIssue('warning', 'W001', `PROJECT.md missing section: ${section}`, 'Add section manually');
            }
        }
    }
    if (!node_fs_1.default.existsSync(roadmapPath)) {
        addIssue('error', 'E003', 'ROADMAP.md not found', `Run ${slash('new-milestone')} to create roadmap`);
    }
    if (!node_fs_1.default.existsSync(statePath)) {
        addIssue('error', 'E004', 'STATE.md not found', `Run ${slash('health')} --repair to regenerate`, true);
        repairs.push('regenerateState');
    }
    else {
        const stateContent = node_fs_1.default.readFileSync(statePath, 'utf-8');
        const phaseRefs = [...stateContent.matchAll(/[Pp]hase\s+(\d+[A-Z]?(?:\.\d+)*)/g)].map((m) => m[1]);
        const validPhases = collectDiskPhases(planBase);
        try {
            if (node_fs_1.default.existsSync(roadmapPath)) {
                const roadmapRaw = node_fs_1.default.readFileSync(roadmapPath, 'utf-8');
                const all = [...roadmapRaw.matchAll(/#{2,4}\s*Phase\s+(\d+[A-Z]?(?:\.\d+)*)/gi)];
                for (const m of all)
                    validPhases.add(m[1]);
            }
        }
        catch {
            /* intentionally empty */
        }
        forEachArchivedPhaseToken(planBase, (token) => validPhases.add(token));
        const normalizedValid = new Set();
        for (const p of validPhases) {
            normalizedValid.add(p);
            const dotIdx = p.indexOf('.');
            const head = dotIdx === -1 ? p : p.slice(0, dotIdx);
            const tail = dotIdx === -1 ? '' : p.slice(dotIdx);
            if (/^\d+$/.test(head)) {
                normalizedValid.add(head.padStart(2, '0') + tail);
            }
        }
        for (const ref of phaseRefs) {
            const dotIdx = ref.indexOf('.');
            const head = dotIdx === -1 ? ref : ref.slice(0, dotIdx);
            const tail = dotIdx === -1 ? '' : ref.slice(dotIdx);
            const padded = /^\d+$/.test(head) ? head.padStart(2, '0') + tail : ref;
            if (!normalizedValid.has(ref) && !normalizedValid.has(padded)) {
                if (normalizedValid.size > 0) {
                    addIssue('warning', 'W002', `STATE.md references phase ${ref}, but only phases ${[...validPhases].sort((a, b) => a.localeCompare(b, undefined, { numeric: true })).join(', ')} are declared`, `Review STATE.md manually before changing it; ${slash('health')} --repair will not overwrite an existing STATE.md for phase mismatches`);
                }
            }
        }
    }
    if (!node_fs_1.default.existsSync(configPath)) {
        addIssue('warning', 'W003', 'config.json not found', `Run ${slash('health')} --repair to create with defaults`, true);
        repairs.push('createConfig');
    }
    else {
        try {
            const rawCfg = node_fs_1.default.readFileSync(configPath, 'utf-8');
            const parsed = JSON.parse(rawCfg);
            const validProfiles = ['quality', 'balanced', 'budget', 'inherit'];
            if (parsed['model_profile'] && !validProfiles.includes(parsed['model_profile'])) {
                addIssue('warning', 'W004', `config.json: invalid model_profile "${parsed['model_profile']}"`, `Valid values: ${validProfiles.join(', ')}`);
            }
        }
        catch (err) {
            addIssue('error', 'E005', `config.json: JSON parse error - ${err instanceof Error ? err.message : String(err)}`, `Run ${slash('health')} --repair to reset to defaults`, true);
            repairs.push('resetConfig');
        }
    }
    if (node_fs_1.default.existsSync(configPath)) {
        try {
            const configRaw = node_fs_1.default.readFileSync(configPath, 'utf-8');
            const configParsed = JSON.parse(configRaw);
            const workflow = configParsed['workflow'];
            if (workflow && workflow['nyquist_validation'] === undefined) {
                addIssue('warning', 'W008', 'config.json: workflow.nyquist_validation absent (defaults to enabled but agents may skip)', `Run ${slash('health')} --repair to add key`, true);
                if (!repairs.includes('addNyquistKey'))
                    repairs.push('addNyquistKey');
            }
            if (workflow && workflow['ai_integration_phase'] === undefined) {
                addIssue('warning', 'W016', `config.json: workflow.ai_integration_phase absent (defaults to enabled — run ${slash('ai-integration-phase')} before planning AI system phases)`, `Run ${slash('health')} --repair to add key`, true);
                if (!repairs.includes('addAiIntegrationPhaseKey'))
                    repairs.push('addAiIntegrationPhaseKey');
            }
        }
        catch {
            /* intentionally empty */
        }
    }
    let phaseDirEntries = [];
    const phaseDirFiles = new Map();
    try {
        phaseDirEntries = node_fs_1.default
            .readdirSync(phasesDir, { withFileTypes: true })
            .filter((e) => e.isDirectory());
        for (const e of phaseDirEntries) {
            try {
                phaseDirFiles.set(e.name, node_fs_1.default.readdirSync(node_path_1.default.join(phasesDir, e.name)));
            }
            catch {
                phaseDirFiles.set(e.name, []);
            }
        }
    }
    catch {
        /* intentionally empty */
    }
    for (const e of phaseDirEntries) {
        if (!e.name.match(validate_cjs_2.phaseDirNameRe)) {
            addIssue('warning', 'W005', `Phase directory "${e.name}" doesn't follow NN-name format`, 'Rename to match pattern (e.g., 01-setup)');
        }
    }
    for (const e of phaseDirEntries) {
        const phaseFiles = phaseDirFiles.get(e.name) || [];
        const plans = phaseFiles.filter((f) => f.endsWith('-PLAN.md') || f === 'PLAN.md');
        const summaries = phaseFiles.filter((f) => f.endsWith('-SUMMARY.md') || f === 'SUMMARY.md');
        const summaryBases = new Set();
        for (const s of summaries) {
            const summaryBase = s.replace('-SUMMARY.md', '').replace('SUMMARY.md', '');
            summaryBases.add(summaryBase);
            summaryBases.add((0, validate_cjs_2.canonicalPlanStem)(summaryBase));
        }
        for (const plan of plans) {
            const planBase = plan.replace('-PLAN.md', '').replace('PLAN.md', '');
            const canonicalBase = (0, validate_cjs_2.canonicalPlanStem)(planBase);
            if (!summaryBases.has(planBase) && !summaryBases.has(canonicalBase)) {
                addIssue('info', 'I001', `${e.name}/${plan} has no SUMMARY.md`, 'May be in progress');
            }
        }
    }
    for (const e of phaseDirEntries) {
        const phaseFiles = phaseDirFiles.get(e.name) || [];
        const hasResearch = phaseFiles.some((f) => f.endsWith('-RESEARCH.md'));
        const hasValidation = phaseFiles.some((f) => f.endsWith('-VALIDATION.md'));
        if (hasResearch && !hasValidation) {
            const researchFile = phaseFiles.find((f) => f.endsWith('-RESEARCH.md'));
            try {
                const researchContent = node_fs_1.default.readFileSync(node_path_1.default.join(phasesDir, e.name, researchFile), 'utf-8');
                if (researchContent.includes('## Validation Architecture')) {
                    addIssue('warning', 'W009', `Phase ${e.name}: has Validation Architecture in RESEARCH.md but no VALIDATION.md`, `Re-run ${slash('plan-phase')} with --research to regenerate`);
                }
            }
            catch {
                /* intentionally empty */
            }
        }
    }
    try {
        const agentStatus = checkAgentsInstalled();
        if (!agentStatus.agents_installed) {
            if ((agentStatus.installed_agents).length === 0) {
                addIssue('warning', 'W010', `No GSD agents found in ${agentStatus.agents_dir} — Task(subagent_type="gsd-*") will fall back to general-purpose`, `Run the GSD installer: npx ${package_identity_cjs_1.PACKAGE_NAME}@latest`);
            }
            else {
                addIssue('warning', 'W010', `Missing ${(agentStatus.missing_agents).length} GSD agents: ${(agentStatus.missing_agents).join(', ')} — affected workflows will fall back to general-purpose`, `Run the GSD installer: npx ${package_identity_cjs_1.PACKAGE_NAME}@latest`);
            }
        }
    }
    catch {
        /* intentionally empty — agent check is non-blocking */
    }
    if (node_fs_1.default.existsSync(roadmapPath)) {
        const roadmapContentRaw = node_fs_1.default.readFileSync(roadmapPath, 'utf-8');
        const roadmapContent = extractCurrentMilestone(roadmapContentRaw, cwd);
        const { roadmapPhases } = (0, validate_cjs_1.buildRoadmapPhaseVariants)(roadmapContent);
        const { roadmapPhaseVariants: fullRoadmapPhaseVariants } = (0, validate_cjs_1.buildRoadmapPhaseVariants)(roadmapContentRaw);
        const diskPhases = collectDiskPhases(planBase);
        forEachArchivedPhaseToken(planBase, (token) => diskPhases.add(token));
        const activeDiskPhases = collectDiskPhases(planBase);
        const notStartedPhases = (0, validate_cjs_1.buildNotStartedPhaseVariants)(roadmapContent);
        for (const p of roadmapPhases) {
            const variants = (0, validate_cjs_1.phaseVariants)(p);
            const existsOnDisk = [...variants].some((v) => diskPhases.has(v));
            if (!existsOnDisk) {
                const isNotStarted = [...variants].some((v) => notStartedPhases.has(v));
                if (isNotStarted)
                    continue;
                addIssue('warning', 'W006', `Phase ${p} in ROADMAP.md but no directory on disk`, 'Create phase directory or remove from roadmap');
            }
        }
        for (const p of activeDiskPhases) {
            const variants = (0, validate_cjs_1.phaseVariants)(p);
            if (![...variants].some((v) => fullRoadmapPhaseVariants.has(v))) {
                addIssue('warning', 'W007', `Phase ${p} exists on disk but not in ROADMAP.md`, 'Add to roadmap or remove directory');
            }
        }
    }
    if (node_fs_1.default.existsSync(statePath) && node_fs_1.default.existsSync(roadmapPath)) {
        try {
            const stateContent = node_fs_1.default.readFileSync(statePath, 'utf-8');
            const roadmapContentFull = node_fs_1.default.readFileSync(roadmapPath, 'utf-8');
            const currentPhaseMatch = stateContent.match(/\*\*Current Phase:\*\*\s*(\S+)/i) ||
                stateContent.match(/Current Phase:\s*(\S+)/i);
            if (currentPhaseMatch) {
                const statePhase = currentPhaseMatch[1].replace(/^0+/, '');
                const phaseCheckboxRe = new RegExp(`-\\s*\\[x\\].*Phase\\s+0*${escapeRegex(statePhase)}[:\\s]`, 'i');
                if (phaseCheckboxRe.test(roadmapContentFull)) {
                    const stateStatus = stateContent.match(/\*\*Status:\*\*\s*(.+)/i);
                    const statusVal = stateStatus ? stateStatus[1].trim().toLowerCase() : '';
                    if (statusVal !== 'complete' && statusVal !== 'done') {
                        addIssue('warning', 'W011', `STATE.md says current phase is ${statePhase} (status: ${statusVal || 'unknown'}) but ROADMAP.md shows it as [x] complete — state files may be out of sync`, `Run ${slash('progress')} to re-derive current position, or manually update STATE.md`);
                    }
                }
            }
        }
        catch {
            /* intentionally empty — cross-validation is advisory */
        }
    }
    if (node_fs_1.default.existsSync(configPath)) {
        try {
            const configRaw = node_fs_1.default.readFileSync(configPath, 'utf-8');
            const configParsed = JSON.parse(configRaw);
            const validStrategies = ['none', 'phase', 'milestone'];
            if (configParsed['branching_strategy'] &&
                !validStrategies.includes(configParsed['branching_strategy'])) {
                addIssue('warning', 'W012', `config.json: invalid branching_strategy "${configParsed['branching_strategy']}"`, `Valid values: ${validStrategies.join(', ')}`);
            }
            if (configParsed['context_window'] !== undefined) {
                const cw = configParsed['context_window'];
                if (typeof cw !== 'number' || cw <= 0 || !Number.isInteger(cw)) {
                    addIssue('warning', 'W013', `config.json: context_window should be a positive integer, got "${cw}"`, 'Set to 200000 (default) or 1000000 (for 1M models)');
                }
            }
            if (configParsed['phase_branch_template'] &&
                !configParsed['phase_branch_template'].includes('{phase}')) {
                addIssue('warning', 'W014', 'config.json: phase_branch_template missing {phase} placeholder', 'Template must include {phase} for phase number substitution');
            }
            if (configParsed['milestone_branch_template'] &&
                !configParsed['milestone_branch_template'].includes('{milestone}')) {
                addIssue('warning', 'W015', 'config.json: milestone_branch_template missing {milestone} placeholder', 'Template must include {milestone} for version substitution');
            }
        }
        catch {
            /* parse error already caught in Check 5 */
        }
    }
    try {
        const worktreeHealth = inspectWorktreeHealth(cwd, { staleAfterMs: 60 * 60 * 1000 }, { execGit: shell_command_projection_cjs_1.execGit, existsSync: node_fs_1.default.existsSync, statSync: node_fs_1.default.statSync });
        if (!worktreeHealth['ok']) {
            if (worktreeHealth['reason'] === 'git_timed_out') {
                addIssue('warning', 'W020', 'Worktree health check degraded: git worktree list timed out after 10s — orphan/stale worktrees could not be inspected', 'Run: git worktree list --porcelain to diagnose; check for .git/index.lock or a hung git process');
            }
            if (worktreeHealth['reason'] === 'git_list_failed') {
                addIssue('warning', 'W020', 'Worktree health check degraded: git worktree list failed — orphan/stale worktrees could not be inspected', 'Run: git worktree list --porcelain to diagnose; check git repository state and permissions');
            }
        }
        else {
            for (const finding of worktreeHealth['findings']) {
                if (finding['kind'] === 'orphan') {
                    addIssue('warning', 'W017', `Orphan git worktree: ${finding['path']} (path no longer exists on disk)`, 'Run: git worktree prune');
                    continue;
                }
                if (finding['kind'] === 'stale') {
                    addIssue('warning', 'W017', `Stale git worktree: ${finding['path']} (last modified ${finding['ageMinutes']} minutes ago)`, `Run: git worktree remove ${finding['path']} --force`);
                }
            }
        }
    }
    catch {
        /* git worktree not available or not a git repo — skip silently */
    }
    try {
        const phaseConvention = (() => {
            if (!node_fs_1.default.existsSync(configPath))
                return null;
            try {
                const configRaw = node_fs_1.default.readFileSync(configPath, 'utf-8');
                const configParsed = JSON.parse(configRaw);
                return configParsed['phase_id_convention'] || null;
            }
            catch {
                return null;
            }
        })();
        if (phaseConvention === 'milestone-prefixed') {
            if (node_fs_1.default.existsSync(roadmapPath)) {
                const roadmapContent = node_fs_1.default.readFileSync(roadmapPath, 'utf-8');
                const { getMilestoneFromPhaseId } = core;
                const mismatches = checkMilestonePrefixMismatches(roadmapContent, {
                    getMilestoneFromPhaseId: getMilestoneFromPhaseId,
                });
                for (const mm of mismatches) {
                    addIssue('warning', 'W021', `Phase ${mm.phaseId}: integer prefix implies ${mm.expectedMilestone} but listed under ${mm.foundInMilestone}`, 'Run `gsd-tools roadmap upgrade --convention milestone-prefixed` to migrate (dry-run by default)');
                }
            }
        }
    }
    catch {
        /* W021 check is advisory — skip on error */
    }
    const milestonesPath = node_path_1.default.join(planBase, 'MILESTONES.md');
    const milestonesArchiveDir = node_path_1.default.join(planBase, 'milestones');
    const missingFromRegistry = [];
    try {
        if (node_fs_1.default.existsSync(milestonesArchiveDir)) {
            const archiveFiles = node_fs_1.default.readdirSync(milestonesArchiveDir);
            const archivedVersions = archiveFiles
                .map((f) => f.match(/^(v\d+\.\d+(?:\.\d+)?)-ROADMAP\.md$/))
                .filter(Boolean)
                .map((m) => m[1]);
            if (archivedVersions.length > 0) {
                const registryContent = node_fs_1.default.existsSync(milestonesPath)
                    ? node_fs_1.default.readFileSync(milestonesPath, 'utf-8')
                    : '';
                for (const ver of archivedVersions) {
                    if (!registryContent.includes(`## ${ver}`)) {
                        missingFromRegistry.push(ver);
                    }
                }
                if (missingFromRegistry.length > 0) {
                    addIssue('warning', 'W018', `MILESTONES.md missing ${missingFromRegistry.length} archived milestone(s): ${missingFromRegistry.join(', ')}`, `Run ${slash('health')} --backfill to synthesize missing entries from archive snapshots`, true);
                    repairs.push('backfillMilestones');
                }
            }
        }
    }
    catch {
        /* intentionally empty — milestone sync check is advisory */
    }
    try {
        const entries = node_fs_1.default.readdirSync(planBase, { withFileTypes: true });
        for (const entry of entries) {
            if (!entry.isFile())
                continue;
            if (!entry.name.endsWith('.md'))
                continue;
            if (!(0, artifacts_cjs_1.isCanonicalPlanningFile)(entry.name)) {
                addIssue('warning', 'W019', `Unrecognized .planning/ file: ${entry.name} — not a canonical GSD artifact`, 'Move to .planning/milestones/ archive subdir or delete if stale. See templates/README.md for the canonical artifact list.', false);
            }
        }
    }
    catch {
        /* artifact check is advisory — skip on error */
    }
    try {
        if (node_fs_1.default.existsSync(statePath) && node_fs_1.default.existsSync(roadmapPath)) {
            const stateRaw = node_fs_1.default.readFileSync(statePath, 'utf-8');
            const statusMatch = stateRaw.match(/^status:\s*(.+)/im);
            const stateStatus = statusMatch ? statusMatch[1].trim().toLowerCase() : '';
            const isMarkedComplete = /milestone complete|archived/.test(stateStatus);
            if (isMarkedComplete) {
                const roadmapRaw = node_fs_1.default.readFileSync(roadmapPath, 'utf-8');
                const scopedContent = extractCurrentMilestone(roadmapRaw, cwd);
                const phasePattern = /#{2,4}\s*Phase\s+(\d+[A-Z]?(?:\.\d+)*)\s*:\s*([^\n]+)/gi;
                const unstarted = [];
                let pm;
                // Non-hoisted: load-order matters (circular dep guard)
                // eslint-disable-next-line @typescript-eslint/no-require-imports -- planning-workspace.cjs is an export= CommonJS module
                const planningWorkspace2 = require('./planning-workspace.cjs');
                const phasesDir2 = planningWorkspace2.planningPaths(cwd).phases;
                const phaseDirNames2 = (() => {
                    try {
                        return node_fs_1.default
                            .readdirSync(phasesDir2, { withFileTypes: true })
                            .filter((e) => e.isDirectory())
                            .map((e) => e.name);
                    }
                    catch {
                        return [];
                    }
                })();
                while ((pm = phasePattern.exec(scopedContent)) !== null) {
                    const phaseNum = pm[1];
                    const normalizedPh = normalizePhaseName(phaseNum);
                    const hasDirectory = phaseDirNames2.some((d) => phaseTokenMatches(d, normalizedPh));
                    if (!hasDirectory) {
                        unstarted.push(phaseNum);
                    }
                }
                if (unstarted.length > 0) {
                    addIssue('warning', 'W021', `STATE says milestone complete but ROADMAP lists ${unstarted.length} unstarted phase(s) (e.g. Phase ${unstarted[0]})`, 'Run validate consistency or re-run complete-milestone after verifying all phases are done');
                }
            }
        }
    }
    catch {
        /* W021 check is advisory — skip on error */
    }
    // ─── Perform repairs if requested ─────────────────────────────────────────
    const repairActions = [];
    if (options['repair'] && repairs.length > 0) {
        for (const repair of repairs) {
            try {
                switch (repair) {
                    case 'createConfig':
                    case 'resetConfig': {
                        const defaults = {
                            model_profile: CONFIG_DEFAULTS.model_profile,
                            commit_docs: CONFIG_DEFAULTS.commit_docs,
                            search_gitignored: CONFIG_DEFAULTS.search_gitignored,
                            branching_strategy: CONFIG_DEFAULTS.branching_strategy,
                            phase_branch_template: CONFIG_DEFAULTS.phase_branch_template,
                            milestone_branch_template: CONFIG_DEFAULTS.milestone_branch_template,
                            quick_branch_template: CONFIG_DEFAULTS.quick_branch_template,
                            workflow: {
                                research: CONFIG_DEFAULTS.research,
                                plan_check: CONFIG_DEFAULTS.plan_checker,
                                verifier: CONFIG_DEFAULTS.verifier,
                                nyquist_validation: CONFIG_DEFAULTS.nyquist_validation,
                            },
                            parallelization: CONFIG_DEFAULTS.parallelization,
                            brave_search: CONFIG_DEFAULTS.brave_search,
                        };
                        (0, shell_command_projection_cjs_1.platformWriteSync)(configPath, JSON.stringify(defaults, null, 2));
                        repairActions.push({ action: repair, success: true, path: 'config.json' });
                        break;
                    }
                    case 'regenerateState': {
                        if (node_fs_1.default.existsSync(statePath)) {
                            const timestamp = new Date().toISOString().replace(/[:.]/g, '-').slice(0, 19);
                            const backupPath = `${statePath}.bak-${timestamp}`;
                            node_fs_1.default.copyFileSync(statePath, backupPath);
                            repairActions.push({ action: 'backupState', success: true, path: backupPath });
                        }
                        const milestone = getMilestoneInfo(cwd);
                        const projectRef = node_path_1.default
                            .relative(cwd, node_path_1.default.join(planningDir(cwd), 'PROJECT.md'))
                            .split(node_path_1.default.sep)
                            .join('/');
                        let stateContent = `# Session State\n\n`;
                        stateContent += `## Project Reference\n\n`;
                        stateContent += `See: ${projectRef}\n\n`;
                        stateContent += `## Position\n\n`;
                        stateContent += `**Milestone:** ${milestone.version} ${milestone.name}\n`;
                        stateContent += `**Current phase:** (determining...)\n`;
                        stateContent += `**Status:** Resuming\n\n`;
                        stateContent += `## Session Log\n\n`;
                        stateContent += `- ${new Date().toISOString().split('T')[0]}: STATE.md regenerated by ${slash('health')} --repair\n`;
                        writeStateMd(statePath, stateContent, cwd);
                        repairActions.push({ action: repair, success: true, path: 'STATE.md' });
                        break;
                    }
                    case 'addNyquistKey': {
                        if (node_fs_1.default.existsSync(configPath)) {
                            try {
                                const configRaw = node_fs_1.default.readFileSync(configPath, 'utf-8');
                                const configParsed = JSON.parse(configRaw);
                                if (!configParsed['workflow'])
                                    configParsed['workflow'] = {};
                                const wf = configParsed['workflow'];
                                if (wf['nyquist_validation'] === undefined) {
                                    wf['nyquist_validation'] = true;
                                    (0, shell_command_projection_cjs_1.platformWriteSync)(configPath, JSON.stringify(configParsed, null, 2));
                                }
                                repairActions.push({ action: repair, success: true, path: 'config.json' });
                            }
                            catch (err) {
                                repairActions.push({
                                    action: repair,
                                    success: false,
                                    error: err instanceof Error ? err.message : String(err),
                                });
                            }
                        }
                        break;
                    }
                    case 'addAiIntegrationPhaseKey': {
                        if (node_fs_1.default.existsSync(configPath)) {
                            try {
                                const configRaw = node_fs_1.default.readFileSync(configPath, 'utf-8');
                                const configParsed = JSON.parse(configRaw);
                                if (!configParsed['workflow'])
                                    configParsed['workflow'] = {};
                                const wf = configParsed['workflow'];
                                if (wf['ai_integration_phase'] === undefined) {
                                    wf['ai_integration_phase'] = true;
                                    (0, shell_command_projection_cjs_1.platformWriteSync)(configPath, JSON.stringify(configParsed, null, 2));
                                }
                                repairActions.push({ action: repair, success: true, path: 'config.json' });
                            }
                            catch (err) {
                                repairActions.push({
                                    action: repair,
                                    success: false,
                                    error: err instanceof Error ? err.message : String(err),
                                });
                            }
                        }
                        break;
                    }
                    case 'backfillMilestones': {
                        if (!options['backfill'] && !options['repair'])
                            break;
                        const today = new Date().toISOString().split('T')[0];
                        let backfilled = 0;
                        for (const ver of missingFromRegistry) {
                            try {
                                const snapshotPath = node_path_1.default.join(milestonesArchiveDir, `${ver}-ROADMAP.md`);
                                const snapshot = (0, shell_command_projection_cjs_1.platformReadSync)(snapshotPath);
                                const titleMatch = snapshot && snapshot.match(/^#\s+(.+)$/m);
                                const milestoneName = titleMatch
                                    ? titleMatch[1].replace(/^Milestone\s+/i, '').replace(/^v[\d.]+\s*/, '').trim()
                                    : ver;
                                const entry = `## ${ver}${milestoneName && milestoneName !== ver ? ` ${milestoneName}` : ''} (Backfilled: ${today})\n\n**Note:** Synthesized from archive snapshot by \`${slash('health')} --backfill\`. Original completion date unknown.\n\n---\n\n`;
                                const milestonesContent = node_fs_1.default.existsSync(milestonesPath)
                                    ? node_fs_1.default.readFileSync(milestonesPath, 'utf-8')
                                    : '';
                                if (!milestonesContent.trim()) {
                                    (0, shell_command_projection_cjs_1.platformWriteSync)(milestonesPath, `# Milestones\n\n${entry}`);
                                }
                                else {
                                    const headerMatch = milestonesContent.match(/^(#{1,3}\s+[^\n]*\n\n?)/);
                                    if (headerMatch) {
                                        const header = headerMatch[1];
                                        const rest = milestonesContent.slice(header.length);
                                        (0, shell_command_projection_cjs_1.platformWriteSync)(milestonesPath, header + entry + rest);
                                    }
                                    else {
                                        (0, shell_command_projection_cjs_1.platformWriteSync)(milestonesPath, entry + milestonesContent);
                                    }
                                }
                                backfilled++;
                            }
                            catch {
                                /* intentionally empty — partial backfill is acceptable */
                            }
                        }
                        repairActions.push({
                            action: repair,
                            success: true,
                            detail: `Backfilled ${backfilled} milestone(s) into MILESTONES.md`,
                        });
                        break;
                    }
                }
            }
            catch (err) {
                repairActions.push({
                    action: repair,
                    success: false,
                    error: err instanceof Error ? err.message : String(err),
                });
            }
        }
    }
    let status;
    if (errors.length > 0) {
        status = 'broken';
    }
    else if (warnings.length > 0) {
        status = 'degraded';
    }
    else {
        status = 'healthy';
    }
    const repairableCount = errors.filter((e) => e.repairable).length + warnings.filter((w) => w.repairable).length;
    const result = {
        status,
        errors,
        warnings,
        info,
        repairable_count: repairableCount,
        repairs_performed: repairActions.length > 0 ? repairActions : undefined,
    };
    output(result, raw);
    return result;
}
function cmdValidateAgents(cwd, raw) {
    const agentStatus = checkAgentsInstalled();
    const expected = Object.keys(MODEL_PROFILES);
    output({
        agents_dir: agentStatus.agents_dir,
        agents_found: agentStatus.agents_installed,
        installed: agentStatus.installed_agents,
        missing: agentStatus.missing_agents,
        expected,
    }, raw);
}
function cmdVerifySchemaDrift(cwd, phaseArg, skipFlag, raw) {
    if (!phaseArg) {
        error('Usage: verify schema-drift <phase> [--skip]');
        return;
    }
    const pDir = planningDir(cwd);
    const phasesDir = node_path_1.default.join(pDir, 'phases');
    if (!node_fs_1.default.existsSync(phasesDir)) {
        output({ drift_detected: false, blocking: false, message: 'No phases directory' }, raw);
        return;
    }
    let phaseDir = null;
    const entries = node_fs_1.default.readdirSync(phasesDir, { withFileTypes: true });
    for (const entry of entries) {
        if (entry.isDirectory() && entry.name.includes(phaseArg)) {
            phaseDir = node_path_1.default.join(phasesDir, entry.name);
            break;
        }
    }
    if (!phaseDir) {
        const exact = node_path_1.default.join(phasesDir, phaseArg);
        if (node_fs_1.default.existsSync(exact))
            phaseDir = exact;
    }
    if (!phaseDir) {
        output({ drift_detected: false, blocking: false, message: `Phase directory not found: ${phaseArg}` }, raw);
        return;
    }
    const allFiles = [];
    const planFiles = node_fs_1.default.readdirSync(phaseDir).filter((f) => f.endsWith('-PLAN.md'));
    for (const pf of planFiles) {
        const content = node_fs_1.default.readFileSync(node_path_1.default.join(phaseDir, pf), 'utf-8');
        const fmMatch = content.match(/files_modified:\s*\[([^\]]*)\]/);
        if (fmMatch) {
            const files = fmMatch[1].split(',').map((f) => f.trim()).filter(Boolean);
            allFiles.push(...files);
        }
    }
    let executionLog = '';
    const summaryFiles = node_fs_1.default.readdirSync(phaseDir).filter((f) => f.endsWith('-SUMMARY.md'));
    for (const sf of summaryFiles) {
        executionLog += node_fs_1.default.readFileSync(node_path_1.default.join(phaseDir, sf), 'utf-8') + '\n';
    }
    const gitLog = (0, shell_command_projection_cjs_1.execGit)(['log', '--oneline', '--all', '-50'], { cwd });
    if (gitLog.exitCode === 0) {
        executionLog += '\n' + gitLog.stdout;
    }
    const result = (0, schema_detect_cjs_1.checkSchemaDrift)(allFiles, executionLog, { skipCheck: !!skipFlag });
    output({
        drift_detected: result['driftDetected'],
        blocking: result['blocking'],
        schema_files: result['schemaFiles'],
        orms: result['orms'],
        unpushed_orms: result['unpushedOrms'],
        message: result['message'],
        skipped: result['skipped'] || false,
    }, raw);
}
function cmdVerifyCodebaseDrift(cwd, raw) {
    // Non-hoisted: load-order matters for circular dep guard
    // eslint-disable-next-line @typescript-eslint/no-require-imports -- drift.cjs is an export= CommonJS module
    const drift = require('./drift.cjs');
    const emit = (payload) => output(payload, raw);
    try {
        const codebaseDir = node_path_1.default.join(planningDir(cwd), 'codebase');
        const structurePath = node_path_1.default.join(codebaseDir, 'STRUCTURE.md');
        if (!node_fs_1.default.existsSync(structurePath)) {
            emit({
                skipped: true,
                reason: 'no-structure-md',
                action_required: false,
                directive: 'none',
                elements: [],
            });
            return;
        }
        let structureMd;
        try {
            structureMd = node_fs_1.default.readFileSync(structurePath, 'utf-8');
        }
        catch (err) {
            emit({
                skipped: true,
                reason: 'cannot-read-structure-md: ' + (err instanceof Error ? err.message : String(err)),
                action_required: false,
                directive: 'none',
                elements: [],
            });
            return;
        }
        const lastMapped = drift['readMappedCommit'](structurePath);
        const revProbe = (0, shell_command_projection_cjs_1.execGit)(['rev-parse', 'HEAD'], { cwd });
        if (revProbe.exitCode !== 0) {
            emit({
                skipped: true,
                reason: 'not-a-git-repo',
                action_required: false,
                directive: 'none',
                elements: [],
            });
            return;
        }
        const EMPTY_TREE = '4b825dc642cb6eb9a060e54bf8d69288fbee4904';
        let base = lastMapped;
        if (!base) {
            base = EMPTY_TREE;
        }
        else {
            const verify = (0, shell_command_projection_cjs_1.execGit)(['cat-file', '-t', base], { cwd });
            if (verify.exitCode !== 0)
                base = EMPTY_TREE;
        }
        const diff = (0, shell_command_projection_cjs_1.execGit)(['diff', '--name-status', base, 'HEAD'], { cwd });
        if (diff.exitCode !== 0) {
            emit({
                skipped: true,
                reason: 'git-diff-failed',
                action_required: false,
                directive: 'none',
                elements: [],
            });
            return;
        }
        const added = [];
        const modified = [];
        const deleted = [];
        for (const line of diff.stdout.split(/\r?\n/)) {
            if (!line.trim())
                continue;
            const m = line.match(/^([A-Z])\d*\t(.+?)(?:\t(.+))?$/);
            if (!m)
                continue;
            const status = m[1];
            const file = m[3] || m[2];
            if (status === 'A' || status === 'R' || status === 'C')
                added.push(file);
            else if (status === 'M')
                modified.push(file);
            else if (status === 'D')
                deleted.push(file);
        }
        const config = loadConfig(cwd);
        const wf = config?.workflow;
        const threshold = Number.isInteger(wf?.drift_threshold) && wf?.drift_threshold >= 1
            ? wf?.drift_threshold
            : 3;
        const action = wf?.drift_action === 'auto-remap' ? 'auto-remap' : 'warn';
        const driftResult = drift['detectDrift']({
            addedFiles: added,
            modifiedFiles: modified,
            deletedFiles: deleted,
            structureMd,
            threshold,
            action,
            runtime: (0, runtime_slash_cjs_1.resolveRuntime)(cwd),
        });
        emit({
            skipped: !!driftResult['skipped'],
            reason: driftResult['reason'] || null,
            action_required: !!driftResult['actionRequired'],
            directive: driftResult['directive'],
            spawn_mapper: !!driftResult['spawnMapper'],
            affected_paths: driftResult['affectedPaths'] || [],
            elements: driftResult['elements'] || [],
            threshold,
            action,
            last_mapped_commit: lastMapped,
            message: driftResult['message'] || '',
        });
    }
    catch (err) {
        emit({
            skipped: true,
            reason: 'exception: ' + (err && err instanceof Error ? err.message : String(err)),
            action_required: false,
            directive: 'none',
            elements: [],
        });
    }
}
module.exports = {
    cmdVerifySummary,
    cmdVerifyPlanStructure,
    cmdVerifyPhaseCompleteness,
    cmdVerifyReferences,
    cmdVerifyCommits,
    cmdVerifyArtifacts,
    cmdVerifyKeyLinks,
    cmdValidateConsistency,
    cmdValidateHealth,
    cmdValidateAgents,
    cmdVerifySchemaDrift,
    cmdVerifyCodebaseDrift,
};
