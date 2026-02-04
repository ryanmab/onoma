// Module (export)
export const moduleValue = 1;

// Type alias
type ID = number;

// Interface
interface Props {
    title: string;
}

// Enum
enum Color {
    Red,
    Green,
    Blue,
}

// Class
class Counter {
    count: number = 0; // Field

    increment(step: number) { // Method + Parameter
        this.count += step;
    }
}

// Function
function add(a: number, b: number) { // Function + Parameters
    return a + b;
}

// Arrow function
const multiply = (x: number) => x * 2; // Function + Parameter

// Variables
let value = 10;
const MAX_COUNT = 100; // Constant

// JSX Component
function App(props: Props) { // Function + Parameter
    return (
        <div className="container">
            <Header title={props.title} />
            <span>hello</span>
        </div>
    );
}

// JSX Component (capitalized)
function Header({ title }: { title: string }) {
    return <h1>{title}</h1>;
}
