import { encodeHex } from "https://deno.land/std/encoding/hex.ts"

let source_code_path = Deno.args[0];
let dylib_path = Deno.args[1];

let source_code = await Deno.readTextFile(source_code_path);

let file = await Deno.open(dylib_path);

let buffer = new Uint8Array(64*1024*1024);
let numberOfBytesRead = await file.read(buffer);



let encode_dylib = encodeHex(buffer.slice(0, numberOfBytesRead));
let encode_code = source_code.replace(
    "// EmbedDyLib: 766572696679206d6521",
    "// EmbedDyLib: "+encode_dylib
)
await Deno.writeTextFile(source_code_path, encode_code)

