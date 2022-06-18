const fs = require('fs');

const wasmBuffer = fs.readFileSync("./out.wasm");

WebAssembly.instantiate(wasmBuffer).then(results => {
  console.log("Loaded wasm module");
  instance = results.instance;
  console.log("instance", instance);
  var white = 2;
  var black = 1;
  var crowned_white = 6; var crowned_black = 5;
  console.log("Calling offset");
  var offset = instance.exports.offsetForPosition(3,4);
  console.log("Offset for 3,4 is ",offset);
  console.debug("White is white?", instance.exports.isWhite(white));
  console.debug("Black is black?", instance.exports.isBlack(black));
  console.debug("Black is white?", instance.exports.isWhite(black));
  console.debug("Uncrowned white",
    instance.exports.isWhite(instance.exports.withoutCrown(crowned_white)));
  console.debug("Uncrowned black",
    instance.exports.isBlack(instance.exports.withoutCrown(crowned_black)));
  console.debug("Crowned is crowned",
    instance.exports.isCrowned(crowned_black));
  console.debug("Crowned is crowned (b)",
    instance.exports.isCrowned(crowned_white));

  console.debug("setPiece(0, 0, 4) -> set (0, 0) to crown", instance.exports.setPiece(0, 0, 4));
  console.debug("getPiece(0, 0) ", instance.exports.getPiece(0, 0));
  //console.debug("getPiece(9, 0) -> piece outside of range will cause panic", instance.exports.getPiece(9, 0));
  console.debug("inRange(0, 7, 8)", instance.exports.inRange(0, 7, 8));
});

