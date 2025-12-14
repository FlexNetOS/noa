import fs from 'node:fs';
import path from 'node:path';

const repoRoot = path.resolve(process.cwd());
const docsRoots = [
  path.join(repoRoot, 'README.md'),
  path.join(repoRoot, 'QUICKSTART.md'),
  path.join(repoRoot, 'CONTRIBUTING.md'),
  path.join(repoRoot, 'config', 'README.md'),
  path.join(repoRoot, 'ai', 'shared', 'policy'),
  path.join(repoRoot, 'docs', 'index.md'),
  path.join(repoRoot, 'docs', '00-guides', 'integrators-getting-started.md'),
  path.join(repoRoot, 'docs', '00-guides', 'integrations-map.md'),
  path.join(repoRoot, 'docs', '00-guides', 'provider-integration.md'),
  path.join(repoRoot, 'docs', '00-guides', 'provider-catalog.md'),
  path.join(repoRoot, 'docs', '00-guides', 'agent-tool-authoring.md'),
  path.join(repoRoot, 'docs', '00-guides', 'schemas-and-contracts.md'),
  path.join(repoRoot, 'docs', 'run-book', 'integrator-troubleshooting.md'),
  path.join(repoRoot, 'docs', 'api', 'README.md'),
];

function isMarkdownFile(p) {
  return p.toLowerCase().endsWith('.md');
}

function walk(dirOrFile) {
  const st = fs.statSync(dirOrFile);
  if (st.isFile()) return [dirOrFile];
  const out = [];
  const entries = fs.readdirSync(dirOrFile, { withFileTypes: true });
  for (const e of entries) {
    if (e.name.startsWith('.')) continue;
    const p = path.join(dirOrFile, e.name);
    if (e.isDirectory()) out.push(...walk(p));
    else out.push(p);
  }
  return out;
}

function extractLinks(md) {
  const links = [];
  const re = /\[[^\]]*?\]\(([^)]+)\)/g;
  let m;
  while ((m = re.exec(md)) !== null) {
    links.push(m[1]);
  }
  return links;
}

function normalizeLinkTarget(raw) {
  const trimmed = raw.trim();
  const noTitle = trimmed.replace(/\s+["'][^"']*["']\s*$/, '');
  const noAnchor = noTitle.split('#')[0];
  return noAnchor;
}

function shouldSkipLink(target) {
  if (!target) return true;
  // Template placeholders (intentionally unresolved)
  if (target.includes('{') || target.includes('}')) return true;
  if (target.startsWith('http://') || target.startsWith('https://')) return true;
  if (target.startsWith('mailto:')) return true;
  if (target.startsWith('tel:')) return true;
  if (target.startsWith('#')) return true;
  return false;
}

const mdFiles = [];
for (const root of docsRoots) {
  if (!fs.existsSync(root)) continue;
  for (const f of walk(root)) {
    if (fs.statSync(f).isFile() && isMarkdownFile(f)) mdFiles.push(f);
  }
}

const errors = [];

for (const file of mdFiles) {
  const content = fs.readFileSync(file, 'utf8');
  const links = extractLinks(content);
  for (const rawTarget of links) {
    const target = normalizeLinkTarget(rawTarget);
    if (shouldSkipLink(target)) continue;

    // Treat absolute-looking paths as repo-root relative (e.g. /docs/foo.md)
    const resolved = target.startsWith('/')
      ? path.join(repoRoot, target)
      : path.resolve(path.dirname(file), target);

    // Allow linking to directories (Jekyll-style); just check existence.
    if (!fs.existsSync(resolved)) {
      errors.push(`${path.relative(repoRoot, file)} -> missing link target: ${rawTarget}`);
    }
  }
}

if (errors.length) {
  console.error('Broken markdown links found:\n');
  for (const e of errors) console.error(`- ${e}`);
  process.exit(1);
}

console.log(`OK: ${mdFiles.length} markdown files checked, no broken relative links found.`);


