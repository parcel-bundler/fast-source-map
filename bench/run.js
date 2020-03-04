const ParcelSourceMap = require("./parcel-source-map").default;
const Benchmark = require("tiny-benchy");
const assert = require("assert");
const SourceMap = require("../");

const ITERATIONS = 50;

const test_maps = [
  {
    version: 3,
    file: "helloworld.js",
    sources: ["helloworld.coffee"],
    names: [],
    mappings: "AAAA;AAAA,EAAA,OAAO,CAAC,GAAR,CAAY,aAAZ,CAAA,CAAA;AAAA"
  },
  require("./maps/angular")
];
const suite = new Benchmark(ITERATIONS);

suite.add("@parcel/source-map#consume", async () => {
  for (let map of test_maps) {
    let sm = await ParcelSourceMap.fromRawSourceMap(map);
  }
});

suite.add("@parcel/source-map#consume->serialize->JSON.stringify", async () => {
  for (let map of test_maps) {
    let sm = await ParcelSourceMap.fromRawSourceMap(map);
    JSON.stringify(sm.serialize());
  }
});

suite.add(
  "@parcel/source-map#consume->serialize->JSON.stringify->JSON.parse->new SourceMap",
  async () => {
    for (let map of test_maps) {
      let sm = await ParcelSourceMap.fromRawSourceMap(map);
      let parsed = JSON.parse(JSON.stringify(sm.serialize()));
      sm = new ParcelSourceMap(map);
    }
  }
);

suite.add("cpp#consume", async () => {
  for (let map of test_maps) {
    let sm = new SourceMap(map.mappings, map.sources, map.names);
  }
});

suite.add("cpp#consume->stringify", async () => {
  for (let map of test_maps) {
    let sm = new SourceMap(map.mappings, map.sources, map.names);
    let s = sm.stringify();
  }
});

suite.add("cpp#consume->toBuffer", async () => {
  for (let map of test_maps) {
    let sm = new SourceMap(map.mappings, map.sources, map.names);
    let buff = sm.toBuffer();
  }
});

suite.add("cpp#consume->toBuffer->fromBuffer", async () => {
  for (let map of test_maps) {
    let sm = new SourceMap(map.mappings, map.sources, map.names);
    let buff = sm.toBuffer();
    sm = new SourceMap(buff);
  }
});

suite.run();
