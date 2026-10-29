import fs from 'node:fs';
import path from 'node:path';
import zlib from 'node:zlib';

function createPng(width, height, r, g, b) {
  // Construct raw uncompressed scanlines: each row starts with filter byte 0, followed by RGBA pixels
  const rowSize = 1 + width * 4;
  const rawData = Buffer.alloc(rowSize * height);

  for (let y = 0; y < height; y++) {
    const rowOffset = y * rowSize;
    rawData[rowOffset] = 0; // Filter: None
    for (let x = 0; x < width; x++) {
      const px = rowOffset + 1 + x * 4;
      // Border or background
      const isBorder = (x === 0 || x === width - 1 || y === 0 || y === height - 1);
      rawData[px] = isBorder ? 245 : r;     // R
      rawData[px + 1] = isBorder ? 158 : g; // G
      rawData[px + 2] = isBorder ? 11 : b;  // B
      rawData[px + 3] = 255;                // A
    }
  }

  const compressed = zlib.deflateSync(rawData);

  // PNG Signature
  const signature = Buffer.from([137, 80, 78, 71, 13, 10, 26, 10]);

  // IHDR chunk
  const ihdr = Buffer.alloc(13);
  ihdr.writeUInt32BE(width, 0);
  ihdr.writeUInt32BE(height, 4);
  ihdr[8] = 8; // bit depth
  ihdr[9] = 6; // color type: RGBA
  ihdr[10] = 0; // compression
  ihdr[11] = 0; // filter
  ihdr[12] = 0; // interlace
  const ihdrChunk = makeChunk('IHDR', ihdr);

  // IDAT chunk
  const idatChunk = makeChunk('IDAT', compressed);

  // IEND chunk
  const iendChunk = makeChunk('IEND', Buffer.alloc(0));

  return Buffer.concat([signature, ihdrChunk, idatChunk, iendChunk]);
}

function crc32(buf) {
  let crc = 0xffffffff;
  for (let i = 0; i < buf.length; i++) {
    const byte = buf[i];
    crc ^= byte;
    for (let j = 0; j < 8; j++) {
      crc = (crc >>> 1) ^ (-(crc & 1) & 0xedb88320);
    }
  }
  return (crc ^ 0xffffffff) >>> 0;
}

function makeChunk(type, data) {
  const len = Buffer.alloc(4);
  len.writeUInt32BE(data.length, 0);

  const typeBuf = Buffer.from(type, 'ascii');
  const body = Buffer.concat([typeBuf, data]);

  const crcBuf = Buffer.alloc(4);
  crcBuf.writeUInt32BE(crc32(body), 0);

  return Buffer.concat([len, body, crcBuf]);
}

function createIco(png32) {
  const header = Buffer.alloc(6);
  header.writeUInt16LE(0, 0); // reserved
  header.writeUInt16LE(1, 2); // ICO type
  header.writeUInt16LE(1, 4); // 1 image

  const dirEntry = Buffer.alloc(16);
  dirEntry[0] = 32; // width
  dirEntry[1] = 32; // height
  dirEntry[2] = 0;  // palette colors
  dirEntry[3] = 0;  // reserved
  dirEntry.writeUInt16LE(1, 4);  // color planes
  dirEntry.writeUInt16LE(32, 6); // bpp
  dirEntry.writeUInt32LE(png32.length, 8); // size of image data
  dirEntry.writeUInt32LE(22, 12); // offset (6 + 16 = 22)

  return Buffer.concat([header, dirEntry, png32]);
}

const iconsDir = path.resolve('src-tauri', 'icons');
const png32 = createPng(32, 32, 22, 24, 30);
const png128 = createPng(128, 128, 22, 24, 30);
const ico = createIco(png32);

fs.writeFileSync(path.join(iconsDir, '32x32.png'), png32);
fs.writeFileSync(path.join(iconsDir, '128x128.png'), png128);
fs.writeFileSync(path.join(iconsDir, '128x128@2x.png'), png128);
fs.writeFileSync(path.join(iconsDir, 'icon.png'), png128);
fs.writeFileSync(path.join(iconsDir, 'icon.ico'), ico);

console.log('Generated icon assets in', iconsDir);
