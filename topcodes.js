import init, {getTopcodes} from './pkg/topcodes_wasm.js';

let initialized = false;

async function run() {
  await init();

  initialized = true;
}

run();

onmessage = (e) => {
  if (!initialized) {
    postMessage({ type: "SCANNER_NOT_INITIALIZED"});
    return;
  }

  const {type, payload} = e.data;

  switch (type) {
    case "SEND_IMAGE_BUFFER": {
      if (!payload.arrayBuffer) {
        postMessage({ type: "TOPCODE_INVALID_INPUT" });
        return;
      }

      const buffer = new Uint32Array(payload.arrayBuffer);
      const topcodes = getTopcodes(buffer).map(json => JSON.parse(json));
      postMessage({ type: "TOPCODE_RESULT", payload: { topcodes } });
      break;
    }
  }
};
