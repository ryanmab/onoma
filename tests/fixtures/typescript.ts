// ==============================
// Module
// ==============================
export namespace MyModule { }

// ==============================
// Classes & Interfaces
// ==============================
class MyClass {
    constructor() { }
    myMethod() { }
    get myGetter() { return 1; }
    set mySetter(v: number) { }
    field!: string;
    property!: string;
}

interface MyInterface {
    myInterfaceMethod(): void;
}

// ==============================
// Enums
// ==============================
enum MyEnum {
    First,            // identifier
    Second = 2,       // assignment
    Third = 3 // computed property name
}

// ==============================
// Type aliases
// ==============================
type MyAlias = string;

// ==============================
// Functions
// ==============================
function myFunction(a: number, b?: string) { }

// ==============================
// Parameters & Type Parameters
// ==============================
function genericFunction<T>(x: T, y?: number) { }

// ==============================
// Variables & Constants
// ==============================
let myVar = 42;
const myConst = "constValue";

// ==============================
// Imports
// ==============================
import { importedValue } from "some-module";

// ==============================
// Object literal keys
// ==============================
const obj = {
    key: "value",
    anotherKey: 123
};

// ==============================
// Literals
// ==============================
const n: number = 10;
const s: string = "hello";
const bTrue: boolean = true;
const bFalse: boolean = false;
const nullValue = null;
