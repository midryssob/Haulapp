# Hauling Pyro – desktopová aplikace pro Windows

Evidence hauling kontraktů a plánovač trasy pro Star Citizen (systém Pyro), zabalená jako samostatná aplikace pro Windows (Tauri 2).

Neoficiální fanouškovský nástroj, bez vazby na Cloud Imperium Games ani Roberts Space Industries. Lokace, výtahy a vzdálenosti: Powered by UEX (uexcorp.space).

## Co desktopová verze přidává oproti webové
- Vlastní okno a ikona, instalátor (.exe / .msi).
- **Navrchu**: tlačítko v horní liště drží okno nad ostatními, i nad hrou v režimu *Borderless*.
- Data se průběžně ukládají do souboru `%APPDATA%\cz.haulingpyro.app\hauling-pyro.json`.
- Záloha jedním tlačítkem do složky Dokumenty, obnova z libovolného souboru zálohy.

## Sestavení přes GitHub (bez instalace čehokoli na PC)
1. Na github.com založ nový repozitář (třeba `hauling-pyro`).
2. Nahraj do něj **obsah** této složky včetně skryté složky `.github` (Add file → Upload files, nebo přes GitHub Desktop).
3. Otevři záložku **Actions**. Sestavení „Sestavení pro Windows“ se spustí samo po nahrání do větve `main` a trvá asi 10–15 minut.
4. Po dokončení otevři běh sestavení a dole v části **Artifacts** stáhni `hauling-pyro-windows` (zip s instalátorem `.exe` a `.msi`).

### Vydání pro ostatní
Vytvoř v repozitáři **Release** se značkou začínající `v` (např. `v1.0.0`). Sestavení pak instalátory samo přiloží k vydání na stránce *Releases* a odtud si je může stáhnout kdokoli.
Novou verzi vydáš tak, že změníš číslo verze v `src-tauri/tauri.conf.json` a `src-tauri/Cargo.toml` a vytvoříš nový Release (např. `v1.0.1`).

## Instalace a první spuštění
- Spusť `Hauling Pyro_1.0.0_x64-setup.exe` (doporučeno), případně `.msi`.
- Windows SmartScreen může ukázat „Systém Windows ochránil počítač“, protože instalátor není podepsaný certifikátem. Klikni na **Další informace → Přesto spustit**.
- Aplikace potřebuje Microsoft WebView2 (ve Windows 10/11 bývá). Pokud chybí, instalátor ho stáhne sám.

## Úpravy aplikace
Celá aplikace je v jednom souboru `src/index.html`. Po úpravě stačí nahrát změnu na GitHub a sestavení proběhne znovu.

Lokální vývoj (volitelné): nainstaluj Node.js a Rust (rustup.rs) a spusť `npm install` a pak `npm run dev`.

## Struktura
```
src/index.html                     aplikace (HTML + JS)
src-tauri/src/lib.rs               ukládání souboru, záloha, okno navrchu
src-tauri/tauri.conf.json          název, okno, instalátor
src-tauri/icons/                   ikony
.github/workflows/build-windows.yml  automatické sestavení
```
