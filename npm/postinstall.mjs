import * as process from "node:process";
import * as fs from "node:fs";
import * as path from "node:path";
import * as url from "node:url";
import * as stream from "node:stream";
import * as tar from "./tar.cjs";

const __dirname = path.dirname(url.fileURLToPath(import.meta.url));
const __bin = path.join(__dirname, "apvm.exe");
const __bin_unix = path.join(__dirname, "apvm");

const APVM_BIN_PATH = process.env.APVM_BIN_PATH;
const APVM_SKIP_DOWNLOAD = process.env.APVM_SKIP_DOWNLOAD;
const APVM_FORCE_TAG = process.env.APVM_FORCE_TAG;

const os = {
  darwin: "macos",
  linux: "linux",
  win32: "windows",
}[process.platform];

const arch = {
  arm64: "arm64",
  x64: "amd64",
}[process.arch];

void (async function main() {
  if (APVM_SKIP_DOWNLOAD === "true") {
    // Do nothing
  } else if (APVM_BIN_PATH && fs.existsSync(APVM_BIN_PATH)) {
    await fs.promises.rm(__bin, { recursive: true, force: true });
    await fs.promises.symlink(__bin, APVM_BIN_PATH);
  } else {
    await downloadBin();
  }
})();

async function downloadBin() {
  if (!arch || !os) {
    throw new Error(
      "Unable to determine what Atlaspack Version Manager binary to download"
    );
  }

  let tag = APVM_FORCE_TAG || "latest";
  if (fs.existsSync(path.join(__dirname, "tag"))) {
    tag = (
      await fs.promises.readFile(path.join(__dirname, "tag"), "utf8")
    ).trim();
  }

  let url = `https://github.com/alshdavid/atlaspack-version-manager/releases/latest/download/apvm-${os}-${arch}.tar.gz`;
  if (tag !== "latest") {
    url = `https://github.com/alshdavid/atlaspack-version-manager/releases/download/${tag}/apvm-${os}-${arch}.tar.gz`;
  }

  const response = await globalThis.fetch(url);
  if (!response.ok) {
    throw new Error("Unable to fetch Atlaspack Version Manager binary");
  }

  const body = await response.bytes();
  const file = new stream.Duplex();

  file.push(body);
  file.push(null);

  if (fs.existsSync(__bin)) {
    fs.rmSync(__bin, { recursive: true, force: true });
  }

  let writable = tar.x({
    C: __dirname,
    chmod: true,
  });

  file.pipe(writable);
  await new Promise((res) => writable.on("close", res));

  await fs.promises.rm(__bin, { recursive: true, force: true });
  if (fs.existsSync(__bin_unix)) {
    await fs.promises.rename(__bin_unix, __bin);
  }
  await fs.promises.chmod(__bin, "755");
}
