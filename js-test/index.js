const { normalizers, Reader } = require('ckb-js-toolkit');
const lumos = require('@ckb-lumos/base');

const lockLen = 2082;
const witnessArgs = {
  lock: '0x' + '0'.repeat(lockLen),
  input_type: '',
  output_type: '',
};

const nomalizedArgs = normalizers.NormalizeWitnessArgs(witnessArgs);
console.log('args', nomalizedArgs);
const serializedWitness = new Reader(lumos.core.SerializeWitnessArgs(nomalizedArgs)).serializeJson();
console.log('serialized', serializedWitness);

