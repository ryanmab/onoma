// Module / export
export const ModuleExample = "module";

//////////////////////////
// Classes
//////////////////////////
class MyComponent {
    constructor() {
        this.field = 42; // Member expression / field
    }

    methodExample(param1, param2) {
        const localVar = 10;
        return this.field + localVar + param1 + param2;
    }
}

//////////////////////////
// Functions / Arrow Functions
//////////////////////////
function regularFunction(x) {
    return x * 2;
}

const arrowFunc = (y) => y + 3;

//////////////////////////
// Functional Components
//////////////////////////
function FunctionalComponent({ title }) {
    return (
        <div>
            <span>{title}</span>
            <NestedComponent count={42} />
        </div>
    );
}

const ArrowFunctional = ({ name }) => (
    <section>
        <p>Hello {name}</p>
    </section>
);

//////////////////////////
// Variables / Constants / TypeAlias
//////////////////////////
const MAX_VALUE = 100;       // Constant
const MyTypeAlias = Number;  // TypeAlias
let variableExample = 5;     // Variable

//////////////////////////
// Member expressions
//////////////////////////
const obj = { prop: "value" };
const valueFromMember = obj.prop;

//////////////////////////
// Literals
//////////////////////////
const str = "hello";
const num = 123;
const boolTrue = true;
const boolFalse = false;
const nul = null;

//////////////////////////
// JSX
//////////////////////////
const jsxVar = (
    <MyComponent
        title="Hello World"
        count={num}
    >
        Some JSX Text
        <ChildComponent childProp="child">
            Nested Text
            <GrandChild count={1} />
        </ChildComponent>
    </MyComponent>
);

const selfClosing = <SelfClosingComponent name="Self" />;
