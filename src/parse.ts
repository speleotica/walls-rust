import avro from "avsc";

const x = avro.Type.forSchema({
  name: "Foo",
  type: "record",
  fields: [
    { name: "a", type: "long" },
    { name: "b", type: "string" },
    { name: "c", type: "bytes" },
    { name: "d", type: ["null", "string"] },
  ],
});
avro.createFileDecoder("value.avro").on("data", console.log);
