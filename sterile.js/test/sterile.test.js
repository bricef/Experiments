let expect = require('chai').expect;
let sterile = require('../sterile.js');
const clonedeep = require('lodash').cloneDeep;

let sanitize = sterile.sanitize;
let sterilize = sterile.sterilize;

describe('sanitize', () => {


  /*
   * Simple tests
   */
  let simple_tests = [
    {
      description: "Will not modify the identity function",
      inputs: [42],
      fn: (x) => x,
      outputs: [42]
    },
    {
      description: "Will not allow a function to modify its input",
      inputs: [[1,2,3]],
      outputs: [[1,2,3,4]],
      fn: (a) => { a.push(4); return a }
    }
  ];

  describe('sanitize(...)', ()=>{
    simple_tests.forEach(({description, inputs, fn, outputs})=>{
      it(description, ()=>{
        let input_copy =  clonedeep(inputs);
        let sanitised_function = sanitize(fn)
        
        let output = sanitised_function(...inputs)
        
        expect(output).to.deep.equal(...outputs)
        expect(inputs).to.deep.equal(input_copy)
      })
    })

    it("Will allow a closure from modifying its enclosed variables", ()=>{
      let a = 1;
      let my_function = () => {a = 2};

      let clean_function = sanitize(my_function);
      clean_function()

      expect(a).to.equal(2)
    })
  });

  /*
   * Check mutated arguments
   */
  describe('sanitize(..., pass_output_values=true)',()=>{

    // Check that the pass_output flag doesn't affect the wrapped function
    simple_tests.forEach(({description, inputs, fn, outputs, this_before})=>{
      simple_tests.forEach(({description, inputs, fn, outputs, this_before})=>{
        it(description, ()=>{
          let input_copy =  clonedeep(inputs);
          let this_copy = clonedeep(this_before)
          let sanitised_function = sanitize(fn, true)
          
          let {output} = sanitised_function.apply(this_copy, inputs)
          
          expect(output).to.deep.equal(...outputs)
          expect(inputs).to.deep.equal(input_copy)
          expect(this_copy).to.deep.equal(this_before)
        })
      })
    })

    it("Will return the modified 'this' argument when requested", ()=>{
      let original_object = {
        fn: function(){ this.foo=123 }
      }
      let object_clone = clonedeep(original_object)
      let this_modified_to = {foo: 123}

      // Sanitise
      original_object.fn = sanitize(original_object.fn, true)
      
      // Call the sanitised function
      let out = original_object.fn();
      let {thisArg} = out;
      
      // check we haven't modified the 'this' input argument
      expect(original_object.foo).to.not.equal(123)

      // check we have modified the output argument
      expect(thisArg.foo).to.deep.equal(123)
  
    })
    it("Will return the modified input arguments when requested", ()=>{
      let inputs = [[1,2,3]];
      let fn = (x) => x.push(4)
      let modified_inputs = [[1,2,3,4]];

      let sanitised_function = sanitize(fn, true);
      let {inputsCopy} = sanitised_function(...inputs)

      expect(inputsCopy).to.deep.equal(modified_inputs)
    })
  })

  /* 
   * Check we can skip cloning 'this'
   */
  describe('sanitize(..., pass_output_values=false, clone_this=false)', ()=>{
    simple_tests.forEach(({description, inputs, fn, outputs, this_before})=>{
      simple_tests.forEach(({description, inputs, fn, outputs, this_before})=>{
        it(description, ()=>{
          let input_copy =  clonedeep(inputs);
          let this_copy = clonedeep(this_before)
          let sanitised_function = sanitize(fn, false, false)
          
          let output = sanitised_function.apply(this_copy, inputs)
          
          expect(output).to.deep.equal(...outputs)
          expect(inputs).to.deep.equal(input_copy)
          expect(this_copy).to.deep.equal(this_before)
        })
      })
    })

    it("Will not clone the this argument when configured not to.", ()=>{
      let original_object = {
        fn: function(){ this.foo=123 }
      }

      original_object.fn = sanitize(original_object.fn, false, false);
      original_object.fn()

      expect(original_object.foo).to.equal(123)
    })
  })
})