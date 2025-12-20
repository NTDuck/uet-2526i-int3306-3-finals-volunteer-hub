import { mkdir, writeFile } from "node:fs/promises";
import { join } from "node:path";
import { randomUUID } from "node:crypto";
import { Buffer } from "node:buffer";
import process from "node:process";

export async function uploadFile(bytes: Uint8Array): Promise<string> {
  const buffer = Buffer.from(bytes);

  const fileName = `${randomUUID()}`;

  const uploadDir = join(process.cwd(), "static", "uploads");
  const filePath = join(uploadDir, fileName);

  await mkdir(uploadDir, { recursive: true });
  await writeFile(filePath, buffer);

  return `/uploads/${fileName}`;
}
