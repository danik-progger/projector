import getOps from "./parseCLI";
import {config,  Operation } from "./config";
import Projector from "./projector";

const opts = getOps();
const conf = config(opts);
const proj = Projector.fromConfig(conf);

if (conf.operation === Operation.Print) {
    if (conf.args.length > 0) {
        const value = proj.getValue(conf.args[0]);
        if (value) {
            console.log(value);
        }
        else {
            console.log("Nothing found");
        }
    }
    else {
        console.log(JSON.stringify(proj.getAll()));
    }
}

if (conf.operation === Operation.Add) {
    proj.setValue(conf.args[0], conf.args[1]);
    proj.save();
}

if (conf.operation === Operation.Remove) {
    proj.removeValue(conf.args[0]);
    proj.save();
}