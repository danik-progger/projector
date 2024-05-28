import { Operation } from "../src/config";
import Projector, { Data } from "../src/projector";

function createData(): Data {
    return {
        projector: {
            "/": {
                foo: "bar1",
                fem: "is_great",
            },
            "/foo": {
                foo: "bar2",
            },
            "/foo/bar": {
                foo: "bar3",
            },
        },
    };
}

function getProjector(pwd: string, data = createData()): Projector {
    return new Projector(
        {
            args: [],
            operation: Operation.Print,
            pwd,
            config: "hello",
        },
        data
    );
}

test("getValueAll", function () {
    const proj = getProjector("/foo/bar");
    expect(proj.getAll()).toEqual({
        fem: "is_great",
        foo: "bar3",
    });
});

test("getValue", function () {
    let proj = getProjector("/foo/bar");
    expect(proj.getValue("foo")).toEqual("bar3");
    proj = getProjector("/foo");
    expect(proj.getValue("foo")).toEqual("bar2");
    expect(proj.getValue("fem")).toEqual("is_great");
});

test("setValue", function () {
    let proj = getProjector("/foo/bar");
    proj.setValue("foo", "baz");
    expect(proj.getValue("foo")).toEqual("baz");
    proj.setValue("fem", "is_even_better_than_great");
    expect(proj.getValue("fem")).toEqual("is_even_better_than_great");

    proj = getProjector("/");
    expect(proj.getValue("fem")).toEqual("is_great");
});

test("removeValue", function () {
    let proj = getProjector("/foo/bar");
    proj.removeValue("fem");
    expect(proj.getValue("fem")).toEqual("is_great");

    proj.removeValue("foo");
    expect(proj.getValue("foo")).toEqual("bar2");
});
