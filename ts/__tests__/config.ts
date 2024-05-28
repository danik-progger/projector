import { Operation, config } from "../src/config";
 
test("simple print all", function () {
    const conf = config({});
    expect(conf.operation).toEqual(Operation.Print);
    expect(conf.args).toEqual([]);
});

test("print key", function () {
    const conf = config({
        args: ["foo"],
    });
    expect(conf.operation).toEqual(Operation.Print);
    expect(conf.args).toEqual(["foo"]);
});

test("add key value", function () {
    const conf = config({
        args: ["add", "foo", "bar"],
    });
    expect(conf.operation).toEqual(Operation.Add);
    expect(conf.args).toEqual(["foo", "bar"]);
});

test("rm key", function () {
    const conf = config({
        args: ["rm", "foo"],
    });
    expect(conf.operation).toEqual(Operation.Remove);
    expect(conf.args).toEqual(["foo"]);
});