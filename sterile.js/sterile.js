const clonedeep = require('lodash').cloneDeep;

function sanitise(fn, pass_output_values = false, clone_this = true){
    return function(...args){
        let args_copy = args.map((x)=>clonedeep(x))
        let _this = this;
        if(clone_this){
            _this = clonedeep(this)
        }
        
        let return_val = fn.apply(_this, args_copy)

        if(pass_output_values){
            let out = {
                output: return_val, 
                thisArg: _this, 
                inputsCopy: args_copy
            }
            return out
        }
        return return_val
    }
}




function sterilise(fn){}

exports.sterilise = sterilise
exports.sterilize = sterilise
exports.sanitise = sanitise
exports.sanitize = sanitise

