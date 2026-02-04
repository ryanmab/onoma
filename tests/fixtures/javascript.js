// -------------------- Module --------------------
export const MODULE_NAME = "my_module"; // @Module

// -------------------- Type / Class --------------------
class MyClass {                           // @Class
    constructor(x, y) {                     // @Method
        this.x = x;                            // @Field
        this.y = y;                            // @Field
    }

    sum(a, b) {                              // @Method
        return a + b;                          // @Function (nested value)
    }
}

// Type alias (heuristic: PascalCase const)       // @TypeAlias
const PointType = { x: 0, y: 0 };

// Enum-like object                                // @Enum
const Color = {
    Red: "red",                                   // @EnumMember
    Blue: "blue"                                  // @EnumMember
};

// -------------------- Functions --------------------
function add(a, b) {                          // @Function
    const result = a + b;                        // @Variable
    return result;                               // @Value
}

// Arrow function                                // @Function
const multiply = (x, y) => x * y;

// -------------------- Constants --------------------
const MAX_RETRIES = 5;                        // @Constant

// -------------------- Variables --------------------
let count = 0;                                // @Variable
let done = true;                              // @Boolean
let name = null;                              // @Null
let greeting = "hello";                       // @String
let score = 42;                               // @Number

// -------------------- Properties --------------------
const obj = {
    age: 30,                                     // @Property
    active: true                                 // @Property
};

// -------------------- Function Parameters --------------------
function greet(person) {                       // @Function
    return "Hello " + person;                    // @Parameter usage inside
}
