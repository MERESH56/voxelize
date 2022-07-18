import { NdArray } from "ndarray";
import CullWorker from "web-worker:./workers/cull-worker.ts";

import { WorkerPool } from "../../libs/worker-pool";
import { Coords3 } from "../../types";

export type MeshResultType = {
  positions: Float32Array;
  normals: Float32Array;
  indices: Float32Array;
};

export type CullOptionsType = {
  min: Coords3;
  max: Coords3;
  realMin: Coords3;
  realMax: Coords3;
  dimensions: Coords3;
};

const cullPool = new WorkerPool(CullWorker, {
  maxWorker: 2,
});

async function cull(
  array: NdArray,
  options: CullOptionsType
): Promise<MeshResultType> {
  const { stride, data } = array;
  const { dimensions, min, max, realMin, realMax } = options;

  return new Promise<MeshResultType>((resolve) => {
    cullPool.addJob({
      message: {
        data,
        configs: {
          min,
          max,
          dimensions,
          stride,
          realMin,
          realMax,
        },
      },
      resolve,
      buffers: [(<Uint8Array>data).buffer.slice(0)],
    });
  });
}

export { cull };