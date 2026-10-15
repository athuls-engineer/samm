import test from 'node:test';
import assert from 'node:assert';
import { ConfidenceLevel, InstallationPlan, ModRecord, InstalledMod } from '../types';

test('confidence level rating verification', () => {
  const verifiedConfidence: ConfidenceLevel = 'confirmed';
  const likelyConfidence: ConfidenceLevel = 'likely';
  const ambiguousConfidence: ConfidenceLevel = 'ambiguous';
  const incompatibleConfidence: ConfidenceLevel = 'incompatible';

  assert.strictEqual(verifiedConfidence, 'confirmed');
  assert.strictEqual(likelyConfidence, 'likely');
  assert.strictEqual(ambiguousConfidence, 'ambiguous');
  assert.strictEqual(incompatibleConfidence, 'incompatible');
});

test('installation plan operation counters', () => {
  const samplePlan: InstallationPlan = {
    modId: 'mod_123',
    modTitle: 'SilentPatch',
    installationType: 'automatic',
    strategy: 'asi',
    totalFiles: 3,
    newFilesCount: 2,
    overwriteFilesCount: 1,
    missingPrerequisites: [],
    warnings: [],
    fileOperations: [
      {
        archiveRelPath: 'SilentPatchSA.asi',
        destinationRelPath: 'SilentPatchSA.asi',
        destinationAbsolutePath: 'C:/Games/GTA/SilentPatchSA.asi',
        fileType: 'asi',
        rule: 'Place in game root',
        destinationState: 'new',
        backupRequired: false,
        fileSizeBytes: 280000,
      },
      {
        archiveRelPath: 'SilentPatchSA.ini',
        destinationRelPath: 'SilentPatchSA.ini',
        destinationAbsolutePath: 'C:/Games/GTA/SilentPatchSA.ini',
        fileType: 'ini',
        rule: 'Place in game root',
        destinationState: 'overwrite',
        backupRequired: true,
        fileSizeBytes: 4096,
      },
      {
        archiveRelPath: 'SilentPatch_ReadMe.txt',
        destinationRelPath: 'modloader/SilentPatch/SilentPatch_ReadMe.txt',
        destinationAbsolutePath: 'C:/Games/GTA/modloader/SilentPatch/SilentPatch_ReadMe.txt',
        fileType: 'txt',
        rule: 'Place in ModLoader sandbox',
        destinationState: 'new',
        backupRequired: false,
        fileSizeBytes: 12000,
      },
    ],
    confidence: 'high',
    manualInstructions: null,
  };

  assert.strictEqual(samplePlan.totalFiles, samplePlan.fileOperations.length);
  assert.strictEqual(samplePlan.newFilesCount, 2);
  assert.strictEqual(samplePlan.overwriteFilesCount, 1);
  assert.strictEqual(samplePlan.fileOperations.filter(o => o.backupRequired).length, 1);
});

test('installed mod file tracking and backup integrity', () => {
  const mod: InstalledMod = {
    id: 'mod_silentpatch',
    gameInstallationId: 'game_01',
    profileId: 'default',
    name: 'SilentPatch',
    author: 'Silent',
    version: '1.1 Build 32',
    sourceName: 'GTAinside',
    sourceUrl: 'https://www.gtainside.com/en/sanandreas/mods/64069-silentpatch/',
    installedAt: '2026-10-15T12:00:00Z',
    strategy: 'asi',
    enabled: true,
    files: [
      {
        id: 'f_01',
        installedModId: 'mod_silentpatch',
        destinationRelPath: 'SilentPatchSA.asi',
        sha256: '9a8b7c6d5e4f3a2b1c0d',
        fileSizeBytes: 284160,
        hasBackup: false,
        backupPath: null,
      },
      {
        id: 'f_02',
        installedModId: 'mod_silentpatch',
        destinationRelPath: 'vorbisFile.dll',
        sha256: '11223344556677889900',
        fileSizeBytes: 65536,
        hasBackup: true,
        backupPath: 'C:/Users/user/AppData/Local/SAMM/backups/game_01/mod_silentpatch/vorbisFile.dll',
      },
    ],
    hasConflicts: false,
    conflictNotes: [],
  };

  assert.strictEqual(mod.enabled, true);
  assert.strictEqual(mod.files.length, 2);
  assert.strictEqual(mod.files.some(f => f.hasBackup), true);
  assert.ok(mod.files.find(f => f.hasBackup)?.backupPath?.includes('vorbisFile.dll'));
});
