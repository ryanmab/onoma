// Module
mod my_module {
    // @Module

    // Struct
    pub struct Point {
        // @Struct
        x: i32, // @Field
        y: i32, // @Field
    }

    // Enum
    pub enum Color {
        // @Enum
        Red,   // @EnumMember
        Green, // @EnumMember
        Blue,  // @EnumMember
    }

    // Union
    pub union Value {
        // @Union
        i: i32,
        f: f32,
    }

    // Trait
    pub trait Display {
        // @Trait
        fn fmt(&self); // @TraitMethod
    }

    // Type alias
    type MyInt = i32; // @TypeAlias

    // Constants & statics
    const MAX: i32 = 100; // @Constant
    static mut COUNTER: i32 = 0; // @StaticVariable

    // Variables
    fn vars_example(a: i32, b: i32) {
        // @Function
        let x = a + b; // @Variable
        let y = a - b; // @Variable
        println!("{} {}", x, y);
    }

    // Impl methods
    impl Point {
        fn move_by(&mut self, dx: i32, dy: i32) {
            // @Method
            self.x += dx; // @SelfParameter
            self.y += dy; // @SelfParameter
        }
    }

    // Macro
    macro_rules! hello {
        // @Macro
        () => {
            println!("Hello, world!");
        };
    }
}
