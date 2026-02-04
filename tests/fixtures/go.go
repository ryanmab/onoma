package myapp  // @Module

import "fmt"   // @Namespace
import "math"  // @Namespace

// Types
type MyType int          // @Type
type Point struct {      // @Struct
    X int                // @Field
    Y int                // @Field
}
type Reader interface {  // @Interface
    Read(p []byte) int   // @Method
}

// Constants
const Pi = 3.14          // @Constant

// Variables
var globalVar int         // @Variable
localVar := 42            // @Variable

// Functions
func Add(a int, b int) int {  // @Function
    return a + b
}

// Methods
func (p Point) Move(dx int, dy int) {  // @Method
    p.X += dx                        // @Variable (parameter dx)
    p.Y += dy                        // @Variable (parameter dy)
}

// Function with parameters
func Multiply(x int, y int) int {  // @Function
    return x * y                   // @Variable (parameters)
}

