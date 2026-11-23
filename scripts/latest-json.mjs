// Vytvoří latest.json pro automatické aktualizace z podepsaného instalátoru NSIS.
import fs from 'node:fs';
import path from 'node:path';
const tag = process.argv[2];
const repo = process.env.GITHUB_REPOSITORY;
const dir = 'src-tauri/target/release/bundle/nsis';
const conf = JSON.parse(fs.readFileSync('src-tauri/tauri.conf.json', 'utf8'));
const exe = fs.readdirSync(dir).find(f => f.endsWith('-setup.exe'));
if (!exe || !fs.existsSync(path.join(dir, exe + '.sig'))) {
  console.log('Podepsaný instalátor nenalezen – latest.json se nevytváří.');
  process.exit(0);
}
// GitHub v názvech příloh nahrazuje mezery tečkami.
const asset = exe.replace(/ /g, '.');
const out = {
  version: conf.version,
  notes: `Hauling Pyro ${conf.version}`,
  pub_date: new Date().toISOString(),
  platforms: {
    'windows-x86_64': {
      signature: fs.readFileSync(path.join(dir, exe + '.sig'), 'utf8').trim(),
      url: `https://github.com/${repo}/releases/download/${encodeURIComponent(tag)}/${encodeURIComponent(asset)}`
    }
  }
};
fs.writeFileSync(path.join(dir, 'latest.json'), JSON.stringify(out, null, 2));
console.log('latest.json:', out.platforms['windows-x86_64'].url);
