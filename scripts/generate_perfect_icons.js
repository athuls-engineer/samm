import fs from 'node:fs';
import path from 'node:path';
import zlib from 'node:zlib';

function createCrestBuffer(size) {
  // RGBA buffer
  const buf = Buffer.alloc(size * size * 4);

  const cx = size / 2;
  const cy = size / 2;
  const scale = size / 128;

  function setPixel(x, y, r, g, b, a) {
    if (x < 0 || x >= size || y < 0 || y >= size) return;
    const idx = (y * size + x) * 4;
    // Alpha blending
    const srcA = a / 255;
    const dstA = buf[idx + 3] / 255;
    const outA = srcA + dstA * (1 - srcA);
    if (outA <= 0) return;

    buf[idx] = Math.round((r * srcA + buf[idx] * dstA * (1 - srcA)) / outA);
    buf[idx + 1] = Math.round((g * srcA + buf[idx + 1] * dstA * (1 - srcA)) / outA);
    buf[idx + 2] = Math.round((b * srcA + buf[idx + 2] * dstA * (1 - srcA)) / outA);
    buf[idx + 3] = Math.round(outA * 255);
  }

  // Draw rounded plate
  const rPlate = Math.floor(22 * scale);
  const margin = Math.floor(4 * scale);
  for (let y = 0; y < size; y++) {
    for (let x = 0; x < size; x++) {
      // Check rounded rect distance
      const dx = Math.max(margin + rPlate - x, 0, x - (size - margin - rPlate - 1));
      const dy = Math.max(margin + rPlate - y, 0, y - (size - margin - rPlate - 1));
      const dist = Math.sqrt(dx * dx + dy * dy);

      if (dist <= rPlate) {
        // Gradient from top dark slate #1e222b to bottom #0d0f14
        const factor = y / size;
        const r = Math.round(30 * (1 - factor) + 13 * factor);
        const g = Math.round(34 * (1 - factor) + 15 * factor);
        const b = Math.round(43 * (1 - factor) + 20 * factor);

        // Border check
        const isBorder = (dist >= rPlate - Math.max(1, 2 * scale));
        if (isBorder) {
          setPixel(x, y, 75, 85, 105, 255);
        } else {
          setPixel(x, y, r, g, b, 255);
        }
      }
    }
  }

  // Draw Shield Contour
  // Shield coordinates (scaled from 128x128): Top tip (64, 18), Right (104, 34), Bottom tip (64, 108), Left (24, 34)
  for (let y = 0; y < size; y++) {
    for (let x = 0; x < size; x++) {
      const origX = x / scale;
      const origY = y / scale;

      // Inside shield test
      let inside = false;
      if (origY >= 18 && origY <= 108) {
        let halfWidth = 0;
        if (origY < 34) {
          halfWidth = ((origY - 18) / 16) * 40;
        } else {
          // Curved contour downwards to point at 108
          const progress = (origY - 34) / 74;
          halfWidth = 40 * Math.pow(1 - progress, 0.7);
        }

        if (Math.abs(origX - 64) <= halfWidth) {
          inside = true;
          const distToEdge = halfWidth - Math.abs(origX - 64);
          const strokeWidth = 4.5;

          if (distToEdge <= strokeWidth || origY <= 22) {
            // Gold border
            const goldR = Math.round(251 - (origY / 108) * 70);
            const goldG = Math.round(191 - (origY / 108) * 90);
            const goldB = Math.round(36 - (origY / 108) * 25);
            setPixel(x, y, goldR, goldG, goldB, 255);
          } else {
            // Inner shield dark fill
            const inR = Math.round(39 - (origY / 108) * 20);
            const inG = Math.round(44 - (origY / 108) * 22);
            const inB = Math.round(56 - (origY / 108) * 28);
            setPixel(x, y, inR, inG, inB, 255);
          }
        }
      }
    }
  }

  // Draw "S" and "A" Monogram in Center
  // S: from (34, 40) to (60, 84)
  // A: from (68, 40) to (94, 84)
  for (let y = 0; y < size; y++) {
    for (let x = 0; x < size; x++) {
      const ox = x / scale;
      const oy = y / scale;

      // Draw Letter S in white/silver
      if (ox >= 36 && ox <= 58 && oy >= 42 && oy <= 84) {
        // Upper bar
        const isTop = (oy >= 42 && oy <= 49 && ox >= 38 && ox <= 58);
        const isTopLeft = (oy >= 42 && oy <= 63 && ox >= 36 && ox <= 43);
        const isMid = (oy >= 60 && oy <= 66 && ox >= 38 && ox <= 56);
        const isBotRight = (oy >= 63 && oy <= 84 && ox >= 51 && ox <= 58);
        const isBot = (oy >= 77 && oy <= 84 && ox >= 36 && ox <= 56);

        if (isTop || isTopLeft || isMid || isBotRight || isBot) {
          setPixel(x, y, 248, 250, 252, 255);
        }
      }

      // Draw Letter A in Gold
      if (ox >= 68 && ox <= 94 && oy >= 40 && oy <= 84) {
        const t = (oy - 40) / 44; // 0 at top (apex 81), 1 at legs (68 and 94)
        const leftLegX = 81 - t * 13;
        const rightLegX = 81 + t * 13;
        const legWidth = 5.5;

        const onLeftLeg = Math.abs(ox - leftLegX) <= legWidth / 2;
        const onRightLeg = Math.abs(ox - rightLegX) <= legWidth / 2;
        const onCrossbar = (oy >= 63 && oy <= 69 && ox >= leftLegX && ox <= rightLegX);

        if (onLeftLeg || onRightLeg || onCrossbar) {
          setPixel(x, y, 245, 158, 11, 255);
        }
      }

      // Bottom golden gear dot (64, 95)
      const gearDist = Math.sqrt(Math.pow(ox - 64, 2) + Math.pow(oy - 95, 2));
      if (gearDist <= 5) {
        if (gearDist <= 2.2) {
          setPixel(x, y, 19, 22, 29, 255);
        } else {
          setPixel(x, y, 251, 191, 36, 255);
        }
      }
    }
  }

  return buf;
}

function encodePng(width, height, rgbaBuffer) {
  const rowSize = 1 + width * 4;
  const rawData = Buffer.alloc(rowSize * height);

  for (let y = 0; y < height; y++) {
    const rowOffset = y * rowSize;
    rawData[rowOffset] = 0; // Filter: None
    const srcOffset = y * width * 4;
    rgbaBuffer.copy(rawData, rowOffset + 1, srcOffset, srcOffset + width * 4);
  }

  const compressed = zlib.deflateSync(rawData);
  const signature = Buffer.from([137, 80, 78, 71, 13, 10, 26, 10]);

  const ihdr = Buffer.alloc(13);
  ihdr.writeUInt32BE(width, 0);
  ihdr.writeUInt32BE(height, 4);
  ihdr[8] = 8;  // 8 bits
  ihdr[9] = 6;  // RGBA
  ihdr[10] = 0; // default compression
  ihdr[11] = 0; // filter
  ihdr[12] = 0; // no interlace

  return Buffer.concat([
    signature,
    makeChunk('IHDR', ihdr),
    makeChunk('IDAT', compressed),
    makeChunk('IEND', Buffer.alloc(0))
  ]);
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

function encodeDibBmp(width, height, rgbaBuffer) {
  // Standard Windows DIB: BITMAPINFOHEADER + bottom-to-top BGRA + 1bpp AND mask
  const biSize = 40;
  const biWidth = width;
  const biHeight = height * 2; // XOR + AND mask
  const biPlanes = 1;
  const biBitCount = 32;
  const biCompression = 0; // BI_RGB
  const biSizeImage = width * height * 4;

  const header = Buffer.alloc(40);
  header.writeUInt32LE(biSize, 0);
  header.writeInt32LE(biWidth, 4);
  header.writeInt32LE(biHeight, 8);
  header.writeUInt16LE(biPlanes, 12);
  header.writeUInt16LE(biBitCount, 14);
  header.writeUInt32LE(biCompression, 16);
  header.writeUInt32LE(biSizeImage, 20);

  const pixelData = Buffer.alloc(width * height * 4);
  // Bottom-up BGRA
  for (let y = 0; y < height; y++) {
    const srcY = height - 1 - y;
    for (let x = 0; x < width; x++) {
      const srcIdx = (srcY * width + x) * 4;
      const dstIdx = (y * width + x) * 4;
      pixelData[dstIdx] = rgbaBuffer[srcIdx + 2];     // B
      pixelData[dstIdx + 1] = rgbaBuffer[srcIdx + 1]; // G
      pixelData[dstIdx + 2] = rgbaBuffer[srcIdx];     // R
      pixelData[dstIdx + 3] = rgbaBuffer[srcIdx + 3]; // A
    }
  }

  // AND mask: 1 bit per pixel, padded to 32-bit row boundary
  const andRowBytes = Math.ceil(width / 32) * 4;
  const andMask = Buffer.alloc(andRowBytes * height, 0); // all transparent bits are handled by 32bpp alpha

  return Buffer.concat([header, pixelData, andMask]);
}

function buildMultiIco(frames) {
  // frames: Array of { width, height, data }
  const count = frames.length;
  const header = Buffer.alloc(6);
  header.writeUInt16LE(0, 0); // reserved
  header.writeUInt16LE(1, 2); // 1 = ICO
  header.writeUInt16LE(count, 4);

  let currentOffset = 6 + count * 16;
  const entries = [];
  const payloads = [];

  for (const frame of frames) {
    const entry = Buffer.alloc(16);
    entry[0] = frame.width >= 256 ? 0 : frame.width;
    entry[1] = frame.height >= 256 ? 0 : frame.height;
    entry[2] = 0; // color palette
    entry[3] = 0; // reserved
    entry.writeUInt16LE(1, 4);  // color planes
    entry.writeUInt16LE(32, 6); // bpp
    entry.writeUInt32LE(frame.data.length, 8);
    entry.writeUInt32LE(currentOffset, 12);

    entries.push(entry);
    payloads.push(frame.data);
    currentOffset += frame.data.length;
  }

  return Buffer.concat([header, ...entries, ...payloads]);
}

// Generate all target sizes
console.log('Rendering golden San Andreas crest icons...');

const rgba16 = createCrestBuffer(16);
const rgba32 = createCrestBuffer(32);
const rgba48 = createCrestBuffer(48);
const rgba64 = createCrestBuffer(64);
const rgba128 = createCrestBuffer(128);
const rgba256 = createCrestBuffer(256);

const dib16 = encodeDibBmp(16, 16, rgba16);
const dib32 = encodeDibBmp(32, 32, rgba32);
const dib48 = encodeDibBmp(48, 48, rgba48);
const dib64 = encodeDibBmp(64, 64, rgba64);
const png128 = encodePng(128, 128, rgba128);
const png256 = encodePng(256, 256, rgba256);
const png32 = encodePng(32, 32, rgba32);

const icoBuffer = buildMultiIco([
  { width: 16, height: 16, data: dib16 },
  { width: 32, height: 32, data: dib32 },
  { width: 48, height: 48, data: dib48 },
  { width: 64, height: 64, data: dib64 },
  { width: 128, height: 128, data: png128 },
  { width: 256, height: 256, data: png256 },
]);

const iconsDir = path.resolve('src-tauri', 'icons');
fs.mkdirSync(iconsDir, { recursive: true });

fs.writeFileSync(path.join(iconsDir, '32x32.png'), png32);
fs.writeFileSync(path.join(iconsDir, '128x128.png'), png128);
fs.writeFileSync(path.join(iconsDir, '128x128@2x.png'), png256);
fs.writeFileSync(path.join(iconsDir, 'icon.png'), png256);
fs.writeFileSync(path.join(iconsDir, 'icon.ico'), icoBuffer);

console.log(`Generated authentic multi-resolution Windows ICO (${icoBuffer.length} bytes) and PNG icons!`);
