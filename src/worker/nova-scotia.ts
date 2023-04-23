// Taken from https://github.com/nalinbhardwaj/Nova-Scotia

import { expose } from "comlink";

async function generate_params(r1csUrl: string) {
  const multiThread = await import("nova_scotia_browser");
  await multiThread.default();
  await multiThread.initThreadPool(navigator.hardwareConcurrency);

  return await multiThread.generate_params(r1csUrl);
}

async function generate_proof(pp: string, inputs: any, r1csUrl: string, wasmUrl: string) {
  const multiThread = await import("nova_scotia_browser");
  await multiThread.default();
  await multiThread.initThreadPool(navigator.hardwareConcurrency);

  return await multiThread.generate_proof(pp, inputs, r1csUrl, wasmUrl);
}

async function verify_proof(pp: string, inputs: any, proof: string) {
  const multiThread = await import("nova_scotia_browser");
  await multiThread.default();
  await multiThread.initThreadPool(navigator.hardwareConcurrency);

  return await multiThread.verify_compressed_proof(pp, inputs, proof);
}

const exports = {
  generate_params,
  generate_proof,
  verify_proof,
};
export type NovaScotiaWorker = typeof exports;

expose(exports);
