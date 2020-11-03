Test = {}
Test.__index = Test

function Test:create(message)
    local instance = {}
    setmetatable(instance, Test)

    instance.message = message

    return instance
end

function Test:me_print()
    print(self.message)
end

obj = Test:create("I'm Test")
obj:me_print()