-- Enum-like table
Colors = { Red = 1, Green = 2, Blue = 3 }  -- @Enum
-- Enum members captured individually via table fields
RedValue = Colors.Red                          -- @EnumMember
GreenValue = Colors.Green                      -- @EnumMember

-- Function declarations
function add(a, b)                             -- @Function
    return a + b
end

myTable = {}                                   -- @Struct

-- Methods assigned to table fields
function myTable:printName()                  -- @Method
    print(self.name)
end

-- Function assigned via variable
subtract = function(x, y)                     -- @Function
    return x - y
end

-- Properties / fields
myTable.name = "Alice"                         -- @Property
myTable.age = 30                               -- @Property

-- Direct field (table literal)
person = { name = "Bob", age = 25 }           -- @Struct / table literal
person.name                                     -- @Field
person.age                                      -- @Field

-- Constants (uppercase names)
PI = 3.1415                                    -- @Constant
MAX_COUNT = 100                                -- @Constant

-- Variables
local score = 0                                -- @Variable
level = 1                                      -- @Variable

-- Literals
str = "hello"                                  -- @String
num = 42                                       -- @Number
flag = true                                    -- @Boolean
empty = nil                                    -- @Null

-- Function calls
print("Hello World")                            -- @Message
person:getName()                                -- @Message
